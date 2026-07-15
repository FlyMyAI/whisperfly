//! WhisperFly cloud mode: after a local transcription finishes, ship the saved
//! WAV to a FlyMyAI agent (cloud Whisper STT + LLM cleanup + keywords + Notion
//! filing). Fire-and-forget: the local paste path is never blocked; failures
//! are logged and never surface as transcription errors.

use std::path::PathBuf;
use std::time::Duration;

use log::{debug, error, info};
use serde_json::{json, Value};
use tauri::AppHandle;

use crate::settings::get_settings;

const DEFAULT_BACKEND: &str = "https://backend.flymy.ai/api/v1";
const POLL_INTERVAL: Duration = Duration::from_secs(2);
const POLL_ATTEMPTS: u32 = 120; // 4 min ceiling; agent runs are ~40-60s

fn backend_base() -> String {
    std::env::var("FLYMYAI_BACKEND")
        .ok()
        .map(|s| s.trim().trim_end_matches('/').to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| DEFAULT_BACKEND.to_string())
}

/// Resolve credentials: settings first, env fallback (FLYMYAI_API_KEY /
/// WHISPERFLY_AGENT_UUID) so a dev build works without touching the UI.
fn cloud_config(app: &AppHandle) -> Option<(String, String)> {
    let settings = get_settings(app);
    if !settings.whisperfly_cloud_enabled {
        return None;
    }
    let key = Some(settings.flymyai_api_key.trim().to_string())
        .filter(|s| !s.is_empty())
        .or_else(|| std::env::var("FLYMYAI_API_KEY").ok().map(|s| s.trim().to_string()))
        .filter(|s| !s.is_empty());
    let agent = Some(settings.flymyai_agent_uuid.trim().to_string())
        .filter(|s| !s.is_empty())
        .or_else(|| std::env::var("WHISPERFLY_AGENT_UUID").ok().map(|s| s.trim().to_string()))
        .filter(|s| !s.is_empty());
    match (key, agent) {
        (Some(k), Some(a)) => Some((k, a)),
        _ => {
            debug!("WhisperFly cloud enabled but api key / agent uuid missing");
            None
        }
    }
}

/// Spawn the detached cloud-filing task. Call after the WAV is saved; never
/// blocks or fails the local transcription flow.
pub fn spawn_file_voice_note(app: &AppHandle, wav_path: PathBuf) {
    let Some((api_key, agent_uuid)) = cloud_config(app) else {
        return;
    };
    tauri::async_runtime::spawn(async move {
        match file_voice_note(&api_key, &agent_uuid, &wav_path).await {
            Ok(result) => {
                let notion = result["notion_url"].as_str().unwrap_or("");
                let keywords = result["keywords"]
                    .as_array()
                    .map(|a| {
                        a.iter()
                            .filter_map(|k| k.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                    .unwrap_or_default();
                info!("WhisperFly: filed to Notion [{}] {}", keywords, notion);
            }
            Err(e) => error!("WhisperFly: cloud filing failed: {}", e),
        }
    });
}

async fn file_voice_note(api_key: &str, agent_uuid: &str, wav: &PathBuf) -> Result<Value, String> {
    let base = backend_base();
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|e| e.to_string())?;

    // 1) Upload the WAV -> public URL the agent's whisper tool can read.
    let bytes = tokio::fs::read(wav).await.map_err(|e| format!("read wav: {e}"))?;
    let external_id = format!(
        "whisperfly-{}",
        chrono::Utc::now().timestamp_millis()
    );
    let part = reqwest::multipart::Part::bytes(bytes)
        .file_name("voice-note.wav")
        .mime_str("audio/wav")
        .map_err(|e| e.to_string())?;
    let form = reqwest::multipart::Form::new()
        .text("external_id", external_id)
        .part("file", part);
    let up: Value = client
        .post(format!("{base}/agents/agent-file-chat-upload/"))
        .header("X-API-KEY", api_key)
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("upload: {e}"))?
        .error_for_status()
        .map_err(|e| format!("upload: {e}"))?
        .json()
        .await
        .map_err(|e| format!("upload json: {e}"))?;
    let audio_url = up["public_url"]
        .as_str()
        .ok_or("upload response missing public_url")?
        .to_string();

    // 2) Run the (frozen) agent via run-loop on the task uuid.
    let run: Value = client
        .post(format!("{base}/agents/tasks/{agent_uuid}/run-loop/"))
        .header("X-API-KEY", api_key)
        .json(&json!({"variables": {"audio_url": audio_url, "source": "whisperfly"}}))
        .send()
        .await
        .map_err(|e| format!("run-loop: {e}"))?
        .error_for_status()
        .map_err(|e| format!("run-loop: {e}"))?
        .json()
        .await
        .map_err(|e| format!("run-loop json: {e}"))?;
    let execution_id = run["id"]
        .as_str()
        .ok_or("run-loop response missing execution id")?
        .to_string();
    debug!("WhisperFly: cloud run {} started", execution_id);

    // 3) Poll until settled.
    for _ in 0..POLL_ATTEMPTS {
        tokio::time::sleep(POLL_INTERVAL).await;
        let st: Value = client
            .get(format!("{base}/agents/executions/{execution_id}/"))
            .header("X-API-KEY", api_key)
            .send()
            .await
            .map_err(|e| format!("poll: {e}"))?
            .json()
            .await
            .map_err(|e| format!("poll json: {e}"))?;
        match st["status"].as_str() {
            Some("completed") => return Ok(st["agent_result"].clone()),
            Some("failed") | Some("cancelled") => {
                return Err(format!(
                    "run {} {}: {}",
                    execution_id,
                    st["status"].as_str().unwrap_or("?"),
                    st["error"].as_str().unwrap_or("")
                ))
            }
            _ => {}
        }
    }
    Err(format!("run {} timed out", execution_id))
}

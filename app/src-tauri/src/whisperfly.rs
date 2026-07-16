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
// Fast polls early (agent runs are trending toward ~10-20s), then back off.
const POLL_FAST: Duration = Duration::from_millis(800);
const POLL_FAST_ATTEMPTS: u32 = 25; // ~20s of fast polling
const POLL_INTERVAL: Duration = Duration::from_secs(2);
const POLL_ATTEMPTS: u32 = 140; // ~4.5 min ceiling total

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
    // Env override so a terminal launch can exercise cloud mode before the
    // settings UI existed / without touching the store: WHISPERFLY_CLOUD=1.
    let env_enabled = std::env::var("WHISPERFLY_CLOUD").map(|v| v == "1").unwrap_or(false);
    if !settings.whisperfly_cloud_enabled && !env_enabled {
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

/// Cloud-first mode: the FlyMy.AI agent does the transcription (and the Notion
/// filing) - active when cloud is configured and NO local model is selected.
/// Picking a local model in advanced settings restores the local-instant path.
pub fn cloud_only_mode(app: &AppHandle) -> bool {
    let settings = get_settings(app);
    settings.selected_model.is_empty() && cloud_config(app).is_some()
}

/// Synchronous (awaited) cloud transcription for the paste path: upload the
/// WAV, run the agent, return the cleaned text. The agent also files the note
/// to Notion server-side - the app never talks to Notion itself.
pub async fn transcribe_via_cloud(app: &AppHandle, wav_path: PathBuf) -> Result<String, String> {
    let (api_key, agent_uuid) =
        cloud_config(app).ok_or("FlyMy.AI cloud is not configured (API key / agent id)")?;
    let result = file_voice_note(&api_key, &agent_uuid, &wav_path).await?;
    if let Some(text) = result["text"].as_str() {
        let keywords = result["keywords"]
            .as_array()
            .map(|a| a.iter().filter_map(|k| k.as_str()).collect::<Vec<_>>().join(", "))
            .unwrap_or_default();
        info!(
            "WhisperFly: cloud transcription done [{}] {}",
            keywords,
            result["notion_url"].as_str().unwrap_or("")
        );
        Ok(text.to_string())
    } else {
        Err(result["error"]
            .as_str()
            .unwrap_or("agent returned no text")
            .to_string())
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

    // 3) Poll until settled: fast at first, then every 2s.
    for attempt in 0..POLL_ATTEMPTS {
        tokio::time::sleep(if attempt < POLL_FAST_ATTEMPTS {
            POLL_FAST
        } else {
            POLL_INTERVAL
        })
        .await;
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

/// Resolve whatever the user pasted as "agent id" into a runnable task uuid.
/// Accepts: a task uuid (returned as-is), or a share/chat id like
/// "abc-defg-hij" (the tail of app.flymy.ai/agents/chat/<id>) which is an
/// EXECUTION id - resolved via the API to its owning task uuid. Users paste
/// the chat id constantly (we did too), so the app must just handle it.
#[tauri::command]
#[specta::specta]
pub async fn resolve_flymyai_agent(reference: String, api_key: String) -> Result<String, String> {
    let r = reference.trim().trim_end_matches('/').to_string();
    let r = r.rsplit('/').next().unwrap_or(&r).to_string(); // tolerate full URLs
    let is_uuid = r.len() == 36
        && r.chars().enumerate().all(|(i, c)| match i {
            8 | 13 | 18 | 23 => c == '-',
            _ => c.is_ascii_hexdigit(),
        });
    if is_uuid {
        return Ok(r);
    }
    let looks_like_chat_id =
        r.len() >= 8 && r.chars().all(|c| c.is_ascii_lowercase() || c == '-') && r.contains('-');
    if !looks_like_chat_id {
        return Err("Not an agent uuid or a chat link id".to_string());
    }
    let key = api_key.trim();
    if key.is_empty() {
        return Err("Set the API key first - it is needed to resolve a chat id".to_string());
    }
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(20))
        .build()
        .map_err(|e| e.to_string())?;
    let st: Value = client
        .get(format!("{}/agents/executions/{}/", backend_base(), r))
        .header("X-API-KEY", key)
        .send()
        .await
        .map_err(|e| format!("resolve: {e}"))?
        .error_for_status()
        .map_err(|_| "Chat id not found on your account - clone the agent first, then paste YOUR copy's id".to_string())?
        .json()
        .await
        .map_err(|e| format!("resolve json: {e}"))?;
    st["user_agent_task_uuid"]
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| "Could not resolve the chat id to an agent".to_string())
}

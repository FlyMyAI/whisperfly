# CLAUDE.md — adapting WhisperFly

You are likely being asked to adapt this project to a new FlyMyAI account or to change what happens with voice notes. Map first, then edit.

## Architecture (3 loosely-coupled parts)

1. **`app/` - macOS Tauri app** (fork of Handy, Rust + React). Local flow is untouched upstream code: hotkey -> record -> local ASR -> paste. Our addition is ONE hook + ONE module:
   - Hook: `app/src-tauri/src/actions.rs` - right after the WAV is saved (`wav_saved`), calls `crate::whisperfly::spawn_file_voice_note(&ah, wav_path)`. Fire-and-forget; never blocks the paste.
   - Module: `app/src-tauri/src/whisperfly.rs` - uploads the WAV, runs the agent, polls. Settings fields (with env fallbacks): `whisperfly_cloud_enabled`, `flymyai_api_key` / `FLYMYAI_API_KEY`, `flymyai_agent_uuid` / `WHISPERFLY_AGENT_UUID` (declared in `settings.rs`, defaults in `get_default_settings()`).
2. **`agent/` - the FlyMyAI cloud agent** (prompt.md). Does STT -> cleanup -> keywords -> Notion. Frozen into a compilation for cheap fixed-pipeline runs.
3. **`client/voicenote.py`** - stdlib CLI speaking the same HTTP contract; fastest way to test the cloud path without the app.

## FlyMyAI HTTP contract (X-API-KEY auth, base https://backend.flymy.ai/api/v1)

```
POST /agents/agent-file-chat-upload/         multipart: file, external_id  -> {public_url}
POST /agents/tasks/{agent_uuid}/run-loop/    {"variables": {"audio_url": ..., "source": ...}} -> {id}
GET  /agents/executions/{id}/                poll until status completed/failed -> {agent_result}
```

`agent_result` is whatever the agent's save_result returned: `{"text", "keywords", "notion_url"}`.

## Common adaptations

- **New FlyMyAI account**: create Notion DB, create agent from `agent/prompt.md` (swap database id), run once, `freeze_agent`, put api key + agent task uuid into app settings. Nothing in Rust changes.
- **Different sink (Slack/Obsidian/email instead of Notion)**: change ONLY the agent prompt (steps 3-5) to use another connected FlyMyAI tool; keep the JSON reply shape so the app stays compatible.
- **Different STT**: swap step 1 of the prompt to another FlyMyAI STT tool (e.g. elevenlabs speech_to_text - pricier), or bypass cloud STT entirely: send the LOCAL transcript text instead of audio (add a `text` variable to the agent and skip whisper).
- **Prompt-engineering gotchas we already paid for** (keep these rules): small models RETYPE long URLs (keep "copy character-for-character"), raw Notion block JSON fails on small models (keep `notion_append_text`), require honest failure reporting (`append_failed`) or the agent will claim success on partial work.

## Build

`cd app && bun install && bun run tauri build` (needs Rust stable + bun). If the bundler fails at the xattr/dmg step, the .app is already built under `app/src-tauri/target/release/bundle/macos/`; `xattr -cr WhisperFly.app` + `hdiutil create` finishes the job.

## Style

English-only in code and docs. Keep upstream Handy code style in `app/`; keep our additions minimal and hook-shaped (one call site in upstream files).

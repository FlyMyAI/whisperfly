# The prompt that builds this project

Prerequisite: an AI coding agent (Claude Code / Codex / Antigravity) with the FlyMyAI MCP connected (see README, one line) and a FlyMyAI account with Notion connected.

## Option A — reproduce THIS app on your account (~5 min, reliable)

Paste into your agent:

```text
Clone https://github.com/FlyMyAI/whisperfly and read its CLAUDE.md.
Set it up end-to-end on MY FlyMyAI account using the FlyMyAI MCP tools:
1. Create a Notion database "Voice Notes" (properties: Name title, Date date,
   Keywords multi_select, Source select, Words number) under a page my FlyMyAI
   Notion integration can access, and remember its database id.
2. Create the agent from agent/prompt.md with MY database id, model gpt-4.1-mini,
   tools whisper + notion. Run it once on any short audio URL, verify a Notion
   row appears with keywords and the full transcript in the page body, then
   freeze it.
3. Build the macOS app from app/ (bun install && bun run tauri build; if the
   bundler fails at the xattr step, finish with codesign ad-hoc + hdiutil).
4. Tell me exactly what to put into the app settings (my API key env var and
   the agent uuid) and how to test it with the hotkey.
Show me the real billed cost of the test run via get_execution_price.
```

## Option B — build your own from scratch (what we originally did)

```text
I want to build a hotkey voice-dictation app for macOS with an AI-cloud twist,
like the WhisperFly demo (github.com/FlyMyAI/whisperfly):
- Fork the best MIT open-source local dictation app (research the current
  options: Handy, OpenWhispr, Amical, OpenLess) as the frontend.
- Keep local ASR for instant paste. After each note is saved, fire-and-forget
  the WAV to a FlyMyAI cloud agent.
- Build that agent with the FlyMyAI MCP tools: cloud STT (whisper tool) ->
  LLM cleanup + keyword extraction (spoken "tag ..." markers too) -> file the
  note into a sink I choose (Notion database / Slack / Obsidian). Freeze the
  agent after one verified run so repeat runs are a cheap fixed pipeline.
- Measure the REAL billed cost per note with get_execution_price and put the
  numbers in a BUILD_LOG.md as you go.
Hard-won rules from the original build - bake them into the agent prompt:
copy long URLs character-for-character (small models retype them wrong), use
plain-text append tools instead of hand-built Notion block JSON, and require
honest failure reporting (never claim success for a step that failed).
```

Both options end the same way: your API key + your agent uuid go into the app settings, and every voice note lands in your own sink, billed on your own account at cents per note.

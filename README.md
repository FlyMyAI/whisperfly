# WhisperFly 🎙️

**Push a hotkey, speak, done: text lands at your cursor instantly (local ASR), and a cloud agent quietly transcribes, cleans, tags and files every voice note into your Notion.**

Built in one day with **Claude Code** as the brain and **[FlyMy.AI](https://flymy.ai)** as the cloud runtime - and this repo is the full recipe: code, agent prompt, and the honest build log with real billed costs. Clone it, feed it to Claude, adapt it to your own account.

```
you speak (hotkey)
      │
      ▼
WhisperFly.app (Handy fork, local)          FlyMy.AI cloud (async, fire-and-forget)
├─ local ASR (Whisper/Parakeet) ──paste──▶  your cursor, instantly
└─ saved WAV ───────upload────────────────▶ WhisperFly agent
                                            ├─ cloud Whisper STT
                                            ├─ LLM cleanup + keywords (spoken "tag ..."/"тег ..." too)
                                            ├─ Notion row (Name/Date/Keywords/Source/Words + full text)
                                            └─ ~$0.08 and ~40s per note (o4-mini, measured)
```

## What's in the box

| Path | What it is |
|---|---|
| [`app/`](app/) | The macOS app - a fork of [Handy](https://github.com/cjpais/Handy) (MIT, see [NOTICE](app/NOTICE.md)) + `whisperfly.rs` cloud mode |
| [`agent/`](agent/) | The FlyMyAI agent: [prompt](agent/prompt.md) + [how to create/freeze/clone it](agent/README.md) |
| [`client/`](client/) | Minimal Python CLI (record -> agent -> Notion) - the pre-app prototype, still handy for scripting |
| [`BUILD_LOG.md`](BUILD_LOG.md) | The whole story: timestamps, real billed prices, bugs we hit (incl. platform bugs), naming detours |

## Quick start (use the app)

1. Grab `WhisperFly.dmg` from Releases (or build it yourself: `bun install && bun run tauri build` in `app/`, needs Rust + bun).
2. First launch: right-click -> Open (the demo build is ad-hoc signed).
3. Pick a local ASR model when prompted, grant mic + accessibility - dictation already works fully offline.
4. Cloud mode (the Notion inbox): you need a FlyMyAI account + your own agent - next section. Then set in settings (or env): `whisperfly_cloud_enabled`, `flymyai_api_key` (env `FLYMYAI_API_KEY`), `flymyai_agent_uuid` (env `WHISPERFLY_AGENT_UUID`).

## Set up YOUR cloud agent (~5 min)

1. Sign up at [app.flymy.ai](https://app.flymy.ai), connect **Notion** (share a parent page with the integration), get your API key.
2. Create a Notion database "Voice Notes" with properties: `Name` (title), `Date` (date), `Keywords` (multi-select), `Source` (select), `Words` (number).
3. Create the agent from [`agent/prompt.md`](agent/prompt.md) (swap in your database id) - via the FlyMyAI MCP tools from Claude, or just paste the prompt into the FlyMyAI chat and ask it to build the agent. Or clone our public WhisperFly agent once it's published and edit only the database id.
4. Run it once on any audio URL, then **freeze** it - frozen runs are fixed-pipeline, cheap and fast.
5. Put your API key + agent uuid into the app settings. Speak. Check Notion.

## Measured economics (real billed numbers, see BUILD_LOG)

| Metric | Value |
|---|---|
| Cost per voice note (cloud path) | **$0.083** (o4-mini agent + cloud whisper + notion writes) |
| Latency of cloud filing | ~42 s, async - invisible, local paste is instant |
| First naive version | $0.256/note - BUILD_LOG shows exactly how we cut it 3x |

## Adapting with Claude

Read [`CLAUDE.md`](CLAUDE.md): architecture map, the FlyMyAI API contracts used, and where to change what (a different sink instead of Notion, different STT, your own post-processing). Drop the repo into Claude Code and ask.

## License

MIT. `app/` is a fork of [Handy](https://github.com/cjpais/Handy) by CJ Pais (MIT) - thank you for the excellent base. ❤️

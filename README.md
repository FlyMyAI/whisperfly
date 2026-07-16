# WhisperFly 🎙️

**Push a hotkey, speak, done: text lands at your cursor instantly (local ASR), and a cloud agent quietly transcribes, cleans, tags and files every voice note into your Notion.**

Built in one day with **Claude Code** as the brain and **[FlyMy.AI](https://flymy.ai)** as the AI cloud - and this repo is the full recipe. **The price difference, upfront:**

| | Dictation subscription (Wispr Flow Pro) | WhisperFly (this repo) |
|---|---|---|
| Hotkey dictation | $12-15/mo forever | **$0** - local, offline |
| Voice notes cleaned + tagged + filed to Notion | not a feature | **$0.083/note** (our real bill) |

## Build this yourself - 3 steps

**1. Connect the FlyMy.AI MCP to your coding agent** - one line:

```bash
# Claude Code
claude mcp add --transport http flymyai https://mcp-agents.flymy.ai/mcp
```
claude.ai / Claude Desktop: Settings -> Connectors -> add `https://mcp-agents.flymy.ai/mcp`. Codex, Antigravity or any MCP client: point it at the same URL. Sign in with your [flymy.ai](https://app.flymy.ai) account when prompted.

**2. Paste one prompt.** [`BUILD_PROMPT.md`](BUILD_PROMPT.md) has two ready-made ones: *reproduce this app on your account in ~5 min*, or *build your own from scratch* the way we did. In short:

```text
Clone github.com/FlyMyAI/whisperfly, read CLAUDE.md, and set it up on MY
FlyMyAI account: Notion database, agent from agent/prompt.md, freeze it,
build the app, tell me what to put in settings.
```

**3. Speak.** Text lands at your cursor instantly (local ASR); the cloud agent quietly files every note into your Notion.

```
   hotkey ──▶ WhisperFly.app ──paste──▶ your cursor          (local, $0, instant)
                    │
                    └──WAV──▶ FlyMy.AI agent ──▶ STT + cleanup + tags ──▶ Notion   ($0.083, async)
```

## Why this exists (vs a $15/mo dictation subscription)

The thing dictation subscriptions charge $144-180/year for - hotkey speech-to-text at your cursor - is **local, offline and $0** here. You own the stack: your keys, your data path, your routing rules.

**Cost** (our real billed numbers, not provider list prices - see BUILD_LOG for receipts):

| | WhisperFly | Wispr Flow Pro |
|---|---|---|
| Hotkey dictation, unlimited | **$0** (on-device Whisper/Parakeet) | $12-15/mo, cloud-only |
| Voice note -> cleaned + tagged + filed to Notion | **$0.083/note** (measured, FlyMyAI billing) | not a feature |
| Works offline | yes (dictation) | no |
| Free tier ceiling | none | 2,000 words/week (~285 words/day) |

**Footprint**: native Tauri/Rust app - tens of MB of RAM. Wispr Flow's desktop client is Electron; users measured ~800 MB RAM and ~8% CPU at idle on Windows.

**Privacy** (literally true, no marketing stretch): no account, no telemetry. Dictation runs fully on-device - audio never leaves your machine unless you enable cloud mode. Recordings/history live only on your disk, under your control. Cloud mode is opt-in and runs on YOUR FlyMyAI account with YOUR keys; notes land in YOUR Notion. (The cloud-only incumbent had a documented 2025 controversy around screenshot capture; with a local-first, your-keys stack that class of problem is structurally absent.)

**Quality, honestly**: on clean English the top STT engines are within 1-2 points of each other - we do not claim "more accurate" without a side-by-side. What you DO get: a **custom dictionary** for your jargon/brand names (the most-complained-about weakness of the incumbents) - via initial-prompt biasing locally and the whisper `prompt` keyterm biasing in the cloud agent - plus a cleanup prompt you can edit, model choice per task, and routing rules (Notion today; Slack/Obsidian/tasks = edit one prompt).

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

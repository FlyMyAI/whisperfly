# WhisperFly 🎙️

**We killed Wispr Flow.** Built this app from **one prompt** - Claude as the builder, **FlyMy.AI cloud** as the backend.

- **Safe**: local-first. Your voice never leaves your Mac unless YOU flip cloud mode on. No account, no telemetry.
- **Cheap**: dictation costs **$0 forever** (they charge $12-15/mo for it). Cloud notes to Notion: **$0.083 each** on our real bill - a heavy month is ~$4 vs their $180/yr. **3-18x cheaper, and the core feature is free.**

## How we did it

```
   hotkey ──▶ WhisperFly.app ──paste──▶ your cursor                (local, $0, instant)
                    │
                    └──▶ FlyMy.AI cloud agent ──▶ STT + cleanup + tags ──▶ your Notion   ($0.083, async)
```

Open-source app (fork of [Handy](https://github.com/cjpais/Handy), MIT) + one FlyMy.AI agent, frozen into a fixed pipeline. That's the whole product.

## Build it yourself

1. **Connect FlyMy.AI to your coding agent** - one line:
   ```bash
   claude mcp add --transport http flymyai https://mcp-agents.flymy.ai/mcp
   ```
   (claude.ai / Codex / Antigravity: add the same URL as an MCP connector, sign in with [flymy.ai](https://app.flymy.ai).)
2. **Paste one prompt** from [BUILD_PROMPT.md](BUILD_PROMPT.md) - it clones this repo, creates YOUR agent + Notion base, builds the app, and shows you the real bill.
3. **Speak.** Done.

## Use the app directly

Download `WhisperFly.dmg` from Releases, right-click -> Open (demo build is ad-hoc signed), pick a local model, grant mic + accessibility. Dictation works immediately, offline. For the Notion inbox: open **FlyMy.AI Cloud** in settings, paste your [flymy.ai](https://app.flymy.ai) API key, then **clone the [public WhisperFly agent](https://app.flymy.ai/agents/chat/qte-mkye-seb) to your account** (agents run on YOUR account) and paste your copy's ID. Or create your own from [agent/prompt.md](agent/prompt.md).

## Numbers, receipts, dead ends

| | Wispr Flow Pro | WhisperFly |
|---|---|---|
| Hotkey dictation | $12-15/mo, cloud-only | **$0, on-device, offline** |
| Note -> cleaned + tagged -> Notion | not a feature | **$0.083/note** (measured) |
| Free tier | 2,000 words/week | unlimited |
| Idle footprint | ~800 MB RAM (Electron, user-measured) | tens of MB (native Tauri/Rust) |

Quality, honestly: top STT engines sit within 1-2 points of each other - we don't claim "more accurate". We claim a **custom dictionary** for your jargon (their #1 complaint), an editable cleanup prompt, and routing rules you own.

Everything else - full timestamped build history, real billed prices, every bug and naming detour - is in [BUILD_LOG.md](BUILD_LOG.md). Adapting via Claude? Read [CLAUDE.md](CLAUDE.md).

## License

MIT. `app/` is a fork of [Handy](https://github.com/cjpais/Handy) by CJ Pais (MIT) - thank you for the excellent base. ❤️

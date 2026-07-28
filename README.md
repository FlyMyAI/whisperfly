# WhisperFly 🎙️

[![part of Build with FlyMy.AI](https://img.shields.io/badge/part%20of-Build%20with%20FlyMy.AI-b6ff3b?style=flat-square&labelColor=0b0d10)](https://github.com/FlyMyAI/build-with-flymyai)

**We killed Wispr Flow.** WhisperFly is a working replacement: you hit a hotkey, speak, and a cleaned-up, tagged note lands in your Notion - **$0.031 a note** instead of $12-15 every month. It exists because nobody had to build the hard part. The entire AI backend behind it - transcription, cleanup, tagging, filing into Notion - was *described in plain text* on [FlyMy.AI](https://flymy.ai/media), assembled into an agent, frozen into an API and hosted there, in one sitting. No servers, no model keys in the app, no pipeline code. That is the real headline: **a full-featured clone of a $700M product is now an afternoon of typing** - and the same infrastructure is sitting there for whatever you want to rebuild next.

Here is the whole build, verbatim - paste it into a coding agent with the FlyMy.AI MCP connected:

```text
Claude, build me a dictation backend on FlyMy.AI: I record audio, an agent
transcribes it, cleans up the text, tags it and files it into my Notion.
Freeze it into an API, host it, then give me a tiny local client that sends
the audio and shows the Notion link. Tell me the real cost per note. Go.
```

- **Safe**: no telemetry, no third-party APIs in the app - it talks ONLY to your FlyMy.AI account; the agent holds the Notion credentials server-side. Optional fully-local offline mode (pick a model in advanced settings).
- **Cheap**: no subscription. **$0.031 per note** on our real bill (transcription + cleanup + tags + Notion filing included) - a heavy month is ~$3-6 vs their $144-180/yr, and nothing to install-time-download.

## How it got built: one prompt in your terminal

<img src="docs/one-prompt.gif" alt="connect the FlyMy.AI MCP, type one prompt, the cloud hosts the agent" width="820">

Connect the MCP once, say what app you want, and the agent is created, run and hosted in the FlyMy.AI cloud - you get back a frozen endpoint and the real bill. (Recreated from [BUILD_LOG.md](BUILD_LOG.md): frozen compilation 244, verified $0.031/note, 29s.)

## How we did it

```mermaid
flowchart LR
    U(["🎙️ you press a hotkey<br/>and <b>speak</b>"]) --> APP

    subgraph APP["💻 WhisperFly (open-source app)"]
        L["local ASR →<br/>instant paste"]
    end

    APP -->|audio| CLOUD
    subgraph CLOUD["☁️ FlyMy.AI cloud agent — the engine"]
        STT["cloud STT → cleanup →<br/>keywords → file it"]
    end

    APP --> CUR(["⌨️ text at your cursor<br/>instantly"])
    CLOUD --> NOTION(["🗒️ your Notion<br/>cleaned + tagged · <b>$0.031/note</b>"])

    classDef you fill:#0b7285,stroke:#0b7285,color:#fff;
    classDef app fill:#5f3dc4,stroke:#5f3dc4,color:#fff;
    classDef cloud fill:#1864ab,stroke:#1864ab,color:#fff;
    classDef out fill:#2b8a3e,stroke:#2b8a3e,color:#fff;
    class U you; class L app; class STT cloud; class CUR,NOTION out;
```

Open-source app shell (fork of [Handy](https://github.com/cjpais/Handy), MIT) + one FlyMy.AI agent, frozen into a fixed pipeline. The agent IS the engine - the app just records and pastes.

## Build it yourself

1. **Connect FlyMy.AI to your coding agent** - one line:
   ```bash
   claude mcp add --transport http flymyai https://mcp-agents.flymy.ai/mcp
   ```
   (claude.ai / Codex / Antigravity: add the same URL as an MCP connector, sign in with [flymy.ai](https://app.flymy.ai).)
2. **Paste one prompt** from [BUILD_PROMPT.md](BUILD_PROMPT.md) - it clones this repo, creates YOUR agent + Notion base, builds the app, and shows you the real bill.
3. **Speak.** Done.

## Use the app directly

Download `WhisperFly.dmg` from Releases, right-click -> Open (demo build is ad-hoc signed), pick a local model, grant mic + accessibility. Dictation works immediately, offline. For the Notion inbox: open **FlyMy.AI Cloud** in settings, paste your [flymy.ai](https://app.flymy.ai) API key, then **clone the [public WhisperFly agent](https://app.flymy.ai/agents/chat/kff-gefa-yjr) to your account** (agents run on YOUR account) and paste your copy's ID. Or create your own from [agent/prompt.md](agent/prompt.md).

## Share it with a friend

Send them the dmg. First launch walks them through everything: grant permissions -> paste THEIR free [flymy.ai](https://app.flymy.ai) API key -> clone the [public WhisperFly agent](https://app.flymy.ai/agents/chat/kff-gefa-yjr) to THEIR account (one Copy click; the app accepts the id straight from the URL of their copy) -> speak. Their notes go to their Notion, billed to their account at ~$0.03/note. The app ships with zero baked-in credentials.

## Numbers, receipts, dead ends

| | Wispr Flow Pro | WhisperFly |
|---|---|---|
| Price | $12-15/mo forever | **$0.031/note, pay-per-use** (or $0 in local offline mode) |
| Note -> cleaned + tagged -> Notion | not a feature | **included in the same run** |
| Offline mode | no | yes (advanced: pick a local model) |
| Idle footprint | ~800 MB RAM (Electron, user-measured) | tens of MB (native Tauri/Rust) |

Quality, honestly: top STT engines sit within 1-2 points of each other - we don't claim "more accurate". We claim a **custom dictionary** for your jargon (their #1 complaint), an editable cleanup prompt, and routing rules you own.

Everything else - full timestamped build history, real billed prices, every bug and naming detour - is in [BUILD_LOG.md](BUILD_LOG.md). Adapting via Claude? Read [CLAUDE.md](CLAUDE.md).

## Part of Build with FlyMy.AI

WhisperFly is one demo in **[Build with FlyMy.AI](https://github.com/FlyMyAI/build-with-flymyai)** - a series where each app rebuilds a venture-funded product from a single prompt, with Claude as the builder and the FlyMy.AI agentic cloud as the backend, and publishes the real bill. The umbrella repo holds the shared playbook, the agent rules and the other demos:

- **WhisperFly** (you are here) - dictation straight into Notion, ~$0.03 a note
- [replifly](https://github.com/FlyMyAI/replifly) - "deploy my code to prod" on your own accounts
- [higfly](https://github.com/FlyMyAI/higfly) - cinematic AI video, ~$0.20-0.50 a clip

Want to build your own kill? Start with the [playbook](https://github.com/FlyMyAI/build-with-flymyai/blob/main/PLAYBOOK.md).

## License

MIT. `app/` is a fork of [Handy](https://github.com/cjpais/Handy) by CJ Pais (MIT) - thank you for the excellent base. ❤️

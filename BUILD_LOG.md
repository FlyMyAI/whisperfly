# Voice Inbox — Build Log

Goal: E2E demo — local dictation client -> FlyMyAI cloud agent -> top-tier cloud STT -> transcripts stored locally + filed into Notion with keyword support. Built live with Claude (brain) + FlyMyAI MCP (hands). Candidate for a public repo + blog post.

All timestamps UTC.

## 2026-07-15

- **22:4x** — Research phase 1 (feasibility): 6-agent parallel workflow (Wispr Flow deep-dive, market, macOS tech, FlyMyAI catalog audit, unit economics, devil's advocate). **332,051 subagent tokens, 118 tool calls, ~5.2 min.** Key findings: Wispr = cloud-only, $15/mo, <700ms target (1-2s observed); FlyMyAI catalog has ZERO STT in run_model (whisper exists only as an agent tool, $0.000733/call observed) and no streaming STT; cloud ASR economics are 83-96% gross margin at realistic usage.
- **23:0x** — Research phase 2 (OSS parity): 3-agent workflow (VoiceInk audit, Handy/OpenWhispr current state, discovery of other OSS Wispr-likes). **174,643 subagent tokens, 54 tool calls, ~7.6 min.** VoiceInk ≈ 90% Wispr parity but GPL-3.0 + macOS-only; Handy (MIT, 26.6k stars) now has mainline LLM cleanup + basic dictionary + streaming ASR (v0.9.0) — best fork base; upstream feature-freeze confirmed (fork-friendly).
- **23:07** — Created Notion database **🎙️ Voice Notes** (`39e2285e-8394-814c-afa5-c5105d19c3c5`) under top-level "Projects" page via FlyMyAI `notion` tool. Properties: Name (title), Date, Keywords (multi_select), Source (select), Words (number).
- **23:08** — Created FlyMyAI agent **Voice Inbox** (`057ad1c9-8528-40f4-9b74-bfa50c112b76`), effort=low, tools pinned: `whisper` (2667) + `notion` (2868). Prompt bakes concrete DB id + action names (no tool discovery at runtime). Input schema: `{audio_url, source?}`.
- **23:09** — Generated Russian test audio via `elevenlabs.text_to_speech` (agent file 2591, public GCS URL) — includes spoken tags ("Тег демо. Тег войс инбокс.") to exercise keyword parsing.
- **23:09:22** — E2E test run started: `det-ubiu-xuo` (audio_url -> whisper -> clean -> keywords -> Notion row -> JSON reply).

### Platform bugs found along the way (filed to backlog)
1. `notion_retrieve_block` action returns raw HTTP 500 (unhandled error, reproducible) — same class as the `telegram_send_message` 500.
2. `notion_search` has no pagination cursor: `has_more=true` with no way to fetch the next page; workspaces with >100 objects are partially unreachable.
3. `notion_search` param is `limit`, not `page_size` — unknown params are silently ignored (schema-validation gap).
4. (From research) run_model catalog: no STT/TTS models at all; no streaming STT anywhere; no lightweight LLM text endpoint; `claude-sonnet-4-6` bills $0.00 (missing LLMChargeRates row).

- **23:10:22** — E2E run 1 **completed** (60s wall): perfect RU transcript, spoken tags («Демо», «Voice Inbox») parsed + 5 content keywords, Notion row created.
- **23:10:40** — Froze execution -> **compilation 240** (status `compiled` at 23:11:38). Pipeline: whisper -> clean+keywords -> notion_create_database_page -> append_block_children -> save_result.
- **23:13** — Wrote local client `client/voicenote.py` (stdlib-only Python + ffmpeg): record mic (avfoundation :0) / `--file` -> multipart upload `agent-file-chat-upload/` -> `run-loop` on task uuid -> poll `executions/{id}/` -> append `~/VoiceNotes/notes.md` + `.jsonl` -> pbcopy + macOS notification.
- **23:14** — Client E2E run 2 (`msu-nkau-fmj`) **PASSED**: upload 0.7s + agent 48.7s. New Notion row, local files written, clipboard + notification OK.

## Real billed costs (from get_execution_price, prod)

| Run | Total | LLM (gpt-5.5) | whisper | notion | sandbox |
|---|---|---|---|---|---|
| det-ubiu-xuo (draft) | **$0.2558** | $0.1749 (6 calls) | $0.0346 | $0.0242 | $0.0221 |
| msu-nkau-fmj (frozen, via client) | **$0.2150** | $0.1460 (6 calls) | $0.0276 | $0.0053 | $0.0361 (2 calls) |

**≈ $0.22 per voice note.** ~50s latency per note.

### vs Wispr Flow ($12/mo annual, $15 monthly)
- Break-even vs Wispr: ~56 notes/month (~1.9/day). A real user (10 notes/day = 300/mo) costs **$64/mo — 5x MORE expensive than Wispr today.** Honest result.
- Where the money goes: the **agent loop, not the work**. 6 LLM turns (gpt-5.5, ~12k ctx each) = 68% of cost; the actual transcription is $0.028 and Notion writes $0.005.
- Also notable: whisper tool billed $0.028-0.035 for a ~21s clip (execution-duration billing) ≈ $0.08-0.10/audio-min vs OpenAI list $0.006/min — our own tool-billing markup is ~14x.

### Optimization path (identified, not yet applied)
1. Drop the sandbox URL-extraction step — `page.url` is already in the create response (-$0.03, -1-2 LLM turns).
2. Cheap model for this agent (task is trivial cleanup+keywords): gpt-5.5 ($5/$30 per 1M) -> nano/mini tier = LLM cost /30-50.
3. Collapse to ~3 turns via tighter frozen instruction.
   -> realistic optimized: **~$0.04/note** (300/mo = $12 — Wispr parity, but WITH Notion filing which Wispr doesn't do).
4. Platform primitives (the real fix): direct STT endpoint + light LLM text endpoint, no agent loop -> ~$0.01/note, ~2-3s latency. This is the same platform gap the research flagged (no STT in run_model, no cheap text endpoint).

## OSS frontend research (workflow 2 conclusions)
- Functionally closest to Wispr: **TypeWhisper** (GPL-3, macOS, full checklist), **FluidVoice** (GPL-3 open-core, 8k stars, streaming preview), **VoiceInk** (GPL-3, macOS, ~90% parity).
- Best MIT fork bases: **Amical** (MIT, Mac+Win, Electron/TS — the only OSS with Wispr-style auto per-app tone), **OpenLess** (MIT, Tauri Rust+React, Mac+Win+Android — deliberate Typeless clone, China-centric defaults to strip), **Handy** (MIT, 26.6k stars, Tauri/Rust — minimal but rock-solid base; upstream feature-freeze = fork-friendly).

## 2026-07-15 (continued) — optimization, naming, FlyVoice

- **23:20** — Agent optimized: model gpt-5.5 -> **o4-mini**, prompt v2: forbid sandbox, build notion_url from page_id (no extraction call), target 3 tool calls + save_result.
- **23:21** — Optimized run `awq-pcls-iiy`: exactly 4 tool calls, ~38s, **$0.041** vs $0.215 (5.2x). BUT all 5 o4-mini LLM calls billed **$0.000000** — SECOND missing LLMChargeRates row (after claude-sonnet-4-6). Froze as compilation 241.
- **23:2x** — Name check (background agent, 55k tokens): **VoiceFly = RISKY** — active US voice-AI startup voiceflyai.com (DropFly, daily commits) + VOICEFLOW live USPTO application in classes 9/42 + VOICEFLIGHT registered in speech recognition. Denis picked **FlyVoice** instead (FlyVoice/FlySay/Dictafly backup check running).
- **23:2x** — Denis added the o4-mini LLMChargeRates row: $1.10 in / $0.275 cache-read / $4.40 out per 1M.
- **23:30** — Frozen run `tut-yiix-cfp` (compilation 241): completed but **dirty** — $0.090, 10 LLM calls, 6 tool exceptions:
  - whisper 404 x2: **o4-mini RETYPED the long GCS URL with a typo** (`ba9735e` vs `ba973e`) — small models reconstruct args instead of copying; self-corrected on attempt 3.
  - notion_append_block_children 400 x4: o4-mini builds the `children` block shape wrong; gave up — **page body left EMPTY while the agent reported success** (silent partial failure).
  - Billing side confirmed fixed: every o4-mini call now bills (~$0.005/call).
- **23:33** — Prompt v3: use `notion_append_text` (server builds the block — kills the 400 class), "COPY audio_url character-for-character", honest `append_failed` reporting rule.
- **23:34** — Run `qte-mkye-seb`: **CLEAN** — exactly 4 tool calls, zero retries, ~42s, **$0.083** ($0.042 LLM + $0.024 whisper + $0.017 notion). Page body verified present. Frozen as **compilation 242 (FINAL)**.
- Note: whisper tool billed $0.014-0.035 across runs for the SAME clip — execution-duration billing makes per-call cost non-deterministic.

### Updated cost picture (real, billed)
| Version | $/note | Latency | Notes |
|---|---|---|---|
| v1 gpt-5.5 draft | $0.256 | ~60s | 6 LLM turns + sandbox waste |
| v1 gpt-5.5 frozen | $0.215 | ~49s | still sandbox URL-extraction |
| v3 o4-mini frozen (242) | **$0.083** | **~42s** | 4 calls, clean, honest billing |

vs Wispr $12/mo: break-even ~145 notes/mo (~5/day); at 10 notes/day **$25/mo** — 2x Wispr, down from 5x. Remaining fat: whisper execution-duration billing (~$0.07/audio-min ≈ 11x OpenAI list) + agent-loop LLM overhead (~12k ctx/turn).

### Naming
~~Voice Inbox~~ (working title) -> ~~VoiceFly~~ (blocked: voiceflyai.com + Voiceflow USPTO 9/42) -> **FlyVoice** (Denis's pick, collision check pending).

## Next steps
- [x] Optimize the agent, re-freeze, re-measure $/note ($0.215 -> $0.083, compilation 242).
- [ ] macOS app: fork **Handy** -> FlyVoice .dmg (npm/cargo build awaiting Denis's explicit repo confirmation for the sandbox).
- [ ] Hotkey wrapper fallback (Hammerspoon around voicenote.py) if app path stalls.
- [ ] Platform: STT into run_model + cheap text endpoint + sonnet-4-6 rate row (o4-mini DONE by Denis).

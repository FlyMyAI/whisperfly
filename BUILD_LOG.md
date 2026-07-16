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

## 2026-07-15 — final name: WhisperFly
- Denis picked **WhisperFly** (over FlySay which scanned cleanest). Risk accepted and documented: phonetic proximity to Wispr Flow ("Wispr" = stylized "whisper"), crowded whisper-* namespace (superwhisper, MacWhisper, OpenWhispr), OpenAI model-name branding guidelines. Product name only; can be re-skinned in minutes if it ever bites.
- Vanilla Handy build: first attempt failed (repo requires **bun** for beforeBuildCommand); installed bun 1.3.14, rebuild running.
- Integration design locked: hook right after Handy's WAV save (`actions.rs`), detached async task -> upload WAV -> run-loop compilation 242 -> Notion; local paste stays instant, cloud filing is fire-and-forget. New `src-tauri/src/whisperfly.rs`, settings fields `whisperfly_enabled` + `flymyai_api_key` (env fallback), reqwest needs `multipart` feature.

## 2026-07-15/16 — .dmg built; GTM cost-model verified against real billing

- **WhisperFly.app + WhisperFly_0.9.3_aarch64.dmg built** (40MB app, ad-hoc signed; tauri bundler's xattr step fails in this environment - finished manually with codesign + hdiutil). Repos pushed: FlyMyAI/whisperfly + FlyMyAI/built-with-flymyai (hub + submodule).
- **Verified the internal GTM cost model (kill_demo_01_wisprflow) against measured billing.** The provider-price math ($0.27-0.32/audio-hour all-in, 36x/4x cheaper scenarios) is internally consistent BUT is a theoretical floor, not our bill:
  - whisper tool bills by execution duration: ~$4-6/audio-hour observed = 11-16x provider list ($0.36/hr);
  - agent-loop LLM overhead: ~$0.04/note vs ~$0.0005 assumed = ~80x;
  - elevenlabs (Scribe) tool observed $0.03-1.71/call, not $0.22/hr list.
  - Conclusion applied to marketing: lead with "dictation is local and $0" + measured "$0.083/note" for cloud filing; provider-floor numbers only as clearly-labeled theory. Closing the floor-vs-bill gap = the platform-primitives work (direct STT endpoint, light LLM endpoint).
- Custom dictionary (the GTM differentiator) confirmed implementable TODAY on both paths: whisper tool has a `prompt` keyterm param; local Handy has custom words via initial_prompt.
- Marketing/quality/privacy claim rules codified in the hub's **PLAYBOOK.md** (only own-bill numbers, no unverifiable accuracy claims, literally-true privacy wording, name-check before naming).

## 2026-07-16 — Cloud settings UI, launch-and-go interface, final self-test

- **00:1x-00:2x** — App v0.2: new sidebar section **FlyMy.AI Cloud** (enable toggle, masked API key, agent id prefilled with the public WhisperFly agent, Connect Notion / Open FlyMy.AI buttons). Interface trimmed for launch-and-go: Advanced + History hidden behind a "Show advanced sections" toggle in General (UI-only gate, zero code removed). Env overrides for terminal testing: WHISPERFLY_CLOUD=1, FLYMYAI_API_KEY, WHISPERFLY_AGENT_UUID. Rust: 4 new settings commands; bindings.ts patched in generator style (runtime regeneration will produce the same).
- **00:20** — Self-test per the new PLAYBOOK bar: frozen compilation 242 run `hlk-sbzw-dww` - CLEAN (4 tool calls, 0 retries, ~47s, **$0.096**), Notion row verified INCLUDING page body. Cost range over 3 frozen runs: $0.08-0.10 (whisper duration-billing variance).
- **00:23** — Final **WhisperFly_0.9.3_aarch64.dmg** (40MB app, ad-hoc signed, dmg mount-verified; binary smoke: cloud settings symbols + UI strings present in bundle). Hub renamed to **build-with-flymyai**; AGENTS.md + build-flymyai-app skill added there.

## 2026-07-16 — v0.3: cloud-first, full rebrand, zero-download install

- Product decision (Denis): NO local model at install. Default mode = the FlyMy.AI agent does transcription AND Notion filing; the app records and pastes the agent's cleaned text (~40-60s to paste today). Local models remain an advanced opt-in (pick one -> instant local paste + async cloud filing).
- Onboarding = permissions only; new users land straight on the FlyMy.AI Cloud tab. onboarding_completed set without any download.
- Agent made PUBLIC: https://app.flymy.ai/agents/chat/qte-mkye-seb (anon-verified 200). Cloud tab: "Open public agent" + clone-flow copy (agents are per-account - prefilling the author's uuid would 403; found via Denis's upgraded settings store where the prefill silently stayed empty).
- Full visible rebrand: WhisperFly wordmark (bolt + text) replaces the Handy logo, tray tooltip, all locale strings; upstream credit moved to About + NOTICE. Lightning tray icons (theme-aware, red dot recording, outline transcribing). App icon generated with our own nano-banana-pro ($0.134).
- Rules codified in hub PLAYBOOK 3c (cloud-first) + 3d (full rebrand).

## 2026-07-16 — v0.4: first live user dictation + guided first-run wizard

- **01:25 — FIRST LIVE E2E FROM VOICE**: Denis dictated via hotkey; run `due-qzau-owk` (source=whisperfly): text pasted from the cloud agent, Notion row + keywords created, **$0.062 billed**. Three real-user issues found and fixed on the way: wrong clipboard content in the key field, chat-link id pasted as agent id (404), Bluetooth AirPods as default mic capturing 0 samples.
- v0.4: **CloudOnboarding wizard** on first launch (get key -> clone public agent -> paste id from URL; live-API validation; Skip available). New Rust command `resolve_flymyai_agent`: accepts agent uuid, chat-link id, or full URL and resolves via the API - in the wizard AND the settings tab. Zero baked credentials in the bundle.
- Rules -> hub PLAYBOOK 3e (guided first-run, zero baked credentials, auto-resolve pasted ids).

## 2026-07-16 — model battery: nano vs mini, latency work (steps 1-3)

Backend MR 590 shipped (prod v3.4394): gpt-4.1-mini/nano in the agent model list + `text` param on notion_create_database_page (page+body in ONE call, -1 LLM turn). MR 591: mypy + model-list test fixed properly. App: fast early polling (0.8s).

**6-case recognition battery** (RU tags / self-corrections / EN jargon / RU-EN code-switch / numbers-dates / long structured list):
- **gpt-4.1-nano: REJECTED.** Blazing (7-11s/run) but undisciplined: malformed nested `properties` JSON (Notion 400s) until given a literal template; then 4/6 runs returned empty results, one run INVENTED a transcript, JSON-in-JSON in another. Cheap is worthless if wrong.
- **gpt-4.1-mini: SHIPPED.** 6/6 clean with the template prompt. Numbers/dates/percent conversions flawless ("двадцать пять тысяч долларов" -> "25 000 долларов"). Prompt v6 also fixed: spoken self-corrections now resolved (example-driven rule), notion_url copied from the response's page.url (models mangle retyped ids - the mini variant of the URL-retype disease).
- Known residual: ~1-in-8 runs mini introduces small typos when re-emitting a long transcript between turns. Documented, acceptable for v1; escalate to claude-sonnet-4-6 (rates row now exists) if quality complaints.

**Frozen compilation 244** (v6, gpt-4.1-mini): verified run = **$0.0307/note, 29s, 0 exceptions**.

| Metric | v1 (gpt-5.5) | v3 (o4-mini) | v6 (gpt-4.1-mini) |
|---|---|---|---|
| $/note | $0.256 | $0.083 | **$0.031** |
| Latency | ~60s | ~42s | **~29s** |
| LLM turns / tool calls | 6 / 5 | 5 / 4 | 4 / 3 |

8x cheaper and 2x faster than the first working version. Remaining latency = whisper tool (~5-10s) + celery pickup + 4 mini turns; the next big step is script-mode frozen execution (platform work, deferred).

## 2026-07-16 — v0.5: Models tab removed, onboarding wizard reordered for the friend flow
- Models sidebar tab removed entirely (enabled:()=>false; component + code kept). Local model card now only under General -> Show advanced sections.
- CloudOnboarding reordered to match the sharing flow exactly: step 1 = "Clone the WhisperFly agent to your account" (opens the public agent URL, clone + connect Notion, paste the URL of YOUR copy - app resolves full URL or id), step 2 = paste your FlyMy.AI API key. Finish validates live, then the app runs under the friend's account.

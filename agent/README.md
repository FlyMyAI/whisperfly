# WhisperFly cloud agent (FlyMyAI)

The cloud half of WhisperFly: one FlyMyAI agent that takes a voice-note audio URL and does STT -> cleanup -> keywords -> Notion filing, returning compact JSON. Frozen into a compilation so every run is a fixed, cheap pipeline (no tool discovery, no planning).

- Model: `gpt-4.1-mini`, effort `low`
- Tools: `whisper` (platform OpenAI Whisper, zero-config) + `notion`
- Measured: ~$0.031/note, ~29s, 3 tool calls (gpt-4.1-mini, prompt v6)
- Input variables: `audio_url` (public HTTPS), optional `source`

## Agent prompt (v6)

See [prompt.md](prompt.md). Three hard-won rules baked in (each one fixed a real failure we hit):

1. `COPY audio_url character-for-character` — small models RETYPE long URLs and introduce typos (we caught o4-mini 404-ing on a hallucinated re-spelling of a GCS URL).
2. Give small models a LITERAL argument JSON template for notion_create_database_page (with the inline `text` param that fills the page body in one call) — they mangle hand-built nested JSON (400s). One Notion call, not two.
3. `never claim success for something that did not happen` — the agent once left the page body empty after failed appends and still reported success.

## Reproduce

```text
create_agent(name="WhisperFly", model="gpt-4.1-mini", effort="low",
             available_tools=[<your notion tool id>, <your whisper tool id>],
             input_schema={audio_url: string (required), source: string},
             user_prompt=<prompt.md>)
run_agent(agent_id, variables={audio_url: "<public https audio url>"})
freeze_agent(execution_id)        # -> compilation id
run_frozen(compilation_id, variables={...})   # cheap repeat runs / API
```

Create your Notion database first (properties: Name title, Date date, Keywords multi_select, Source select, Words number) and put its id into the prompt.

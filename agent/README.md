# WhisperFly cloud agent (FlyMyAI)

The cloud half of WhisperFly: one FlyMyAI agent that takes a voice-note audio URL and does STT -> cleanup -> keywords -> Notion filing, returning compact JSON. Frozen into a compilation so every run is a fixed, cheap pipeline (no tool discovery, no planning).

- Model: `o4-mini`, effort `low`
- Tools: `whisper` (platform OpenAI Whisper, zero-config) + `notion`
- Measured: ~$0.08/note, ~42s, exactly 4 tool calls
- Input variables: `audio_url` (public HTTPS), optional `source`

## Agent prompt (v3)

See [prompt.md](prompt.md). Three hard-won rules baked in (each one fixed a real failure we hit):

1. `COPY audio_url character-for-character` — small models RETYPE long URLs and introduce typos (we caught o4-mini 404-ing on a hallucinated re-spelling of a GCS URL).
2. Use `notion_append_text` (server builds the paragraph block) instead of raw `notion_append_block_children` — small models format Notion block JSON wrong (4x 400s).
3. `never claim success for something that did not happen` — the agent once left the page body empty after failed appends and still reported success.

## Reproduce

```text
create_agent(name="WhisperFly", model="o4-mini", effort="low",
             available_tools=[<your notion tool id>, <your whisper tool id>],
             input_schema={audio_url: string (required), source: string},
             user_prompt=<prompt.md>)
run_agent(agent_id, variables={audio_url: "<public https audio url>"})
freeze_agent(execution_id)        # -> compilation id
run_frozen(compilation_id, variables={...})   # cheap repeat runs / API
```

Create your Notion database first (properties: Name title, Date date, Keywords multi_select, Source select, Words number) and put its id into the prompt.

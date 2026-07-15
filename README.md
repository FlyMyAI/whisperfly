# WhisperFly

WhisperFly (naming history: Voice Inbox -> VoiceFly -> FlyVoice -> WhisperFly, see BUILD_LOG). E2E demo: dictate a voice note locally -> FlyMyAI cloud agent transcribes (cloud STT), cleans it up, extracts keywords (incl. spoken tags like "тег демо") -> files it into a Notion database -> returns the text to your clipboard + local archive (~/VoiceNotes).

Built live with Claude Code (brain) + FlyMyAI MCP (hands). See BUILD_LOG.md for the full timestamped build history, real billed costs, and the Wispr Flow comparison.

## Parts
- `client/voicenote.py` - local CLI client (record / --file, upload, run agent, poll, save, notify)
- FlyMyAI agent "Voice Inbox" (uuid 057ad1c9-8528-40f4-9b74-bfa50c112b76, frozen compilation 240)
- Notion database "🎙️ Voice Notes" (39e2285e-8394-814c-afa5-c5105d19c3c5)

## Usage
```
python3 client/voicenote.py            # record mic, Enter to stop
python3 client/voicenote.py --file x.m4a
```
Needs: ffmpeg, macOS, FlyMyAI API key (~/Projects/FlyMyAI/.flymy_key or $FLYMYAI_API_KEY).

#!/usr/bin/env python3
"""Voice Inbox — local client (v0, CLI).

Record a voice note from the mic (or take an audio file), send it to the
FlyMyAI Voice Inbox agent (cloud STT + cleanup + keywords), store the result
locally (~/VoiceNotes) and in Notion (the agent files it), notify via
macOS notification, and copy the text to the clipboard.

Usage:
  python3 voicenote.py            # record from mic, Enter to stop
  python3 voicenote.py --file x.m4a   # use an existing audio file

Requires: ffmpeg (mic recording), macOS (pbcopy/osascript), a FlyMyAI API key
in $FLYMYAI_API_KEY or ~/Projects/FlyMyAI/.flymy_key.
"""

import argparse
import datetime as dt
import json
import mimetypes
import os
import pathlib
import subprocess
import sys
import time
import urllib.error
import urllib.request
import uuid

BACKEND = os.environ.get("FLYMYAI_BACKEND", "https://backend.flymy.ai/api/v1")
AGENT_UUID = "057ad1c9-8528-40f4-9b74-bfa50c112b76"  # Voice Inbox
MIC_DEVICE = os.environ.get("VOICENOTE_MIC", ":0")  # avfoundation audio index
NOTES_DIR = pathlib.Path.home() / "VoiceNotes"
POLL_INTERVAL_S = 1.5
POLL_TIMEOUT_S = 180


def api_key() -> str:
    key = os.environ.get("FLYMYAI_API_KEY", "").strip()
    if not key:
        key_file = pathlib.Path.home() / "Projects/FlyMyAI/.flymy_key"
        if key_file.exists():
            key = key_file.read_text().strip()
    if not key:
        sys.exit("No API key: set FLYMYAI_API_KEY or create ~/Projects/FlyMyAI/.flymy_key")
    return key


def record(out_path: pathlib.Path) -> None:
    print("● Recording — press Enter to stop...", flush=True)
    proc = subprocess.Popen(
        [
            "ffmpeg", "-hide_banner", "-loglevel", "error",
            "-f", "avfoundation", "-i", MIC_DEVICE,
            "-ac", "1", "-ar", "16000", "-c:a", "aac", "-b:a", "48k",
            "-y", str(out_path),
        ],
        stdin=subprocess.PIPE,
    )
    try:
        input()
    except KeyboardInterrupt:
        pass
    proc.stdin.write(b"q")  # graceful ffmpeg stop → valid container
    proc.stdin.flush()
    proc.wait(timeout=10)
    if not out_path.exists() or out_path.stat().st_size < 1000:
        sys.exit("Recording failed or too short")
    print(f"  saved {out_path} ({out_path.stat().st_size // 1024} KB)")


def http(method: str, url: str, key: str, body: bytes | None = None,
         content_type: str | None = None) -> dict:
    req = urllib.request.Request(url, data=body, method=method)
    req.add_header("X-API-KEY", key)
    if content_type:
        req.add_header("Content-Type", content_type)
    try:
        with urllib.request.urlopen(req, timeout=60) as resp:
            return json.loads(resp.read().decode() or "{}")
    except urllib.error.HTTPError as e:
        sys.exit(f"{method} {url} -> {e.code}: {e.read().decode()[:300]}")


def upload(path: pathlib.Path, key: str) -> tuple[str, str]:
    """Multipart upload -> (public_url, external_id)."""
    external_id = f"voicenote-{uuid.uuid4().hex[:12]}"
    boundary = uuid.uuid4().hex
    ctype = mimetypes.guess_type(path.name)[0] or "application/octet-stream"
    parts = []
    parts.append(
        f"--{boundary}\r\nContent-Disposition: form-data; name=\"external_id\"\r\n\r\n{external_id}\r\n".encode()
    )
    parts.append(
        f"--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; "
        f"filename=\"{path.name}\"\r\nContent-Type: {ctype}\r\n\r\n".encode()
    )
    parts.append(path.read_bytes())
    parts.append(f"\r\n--{boundary}--\r\n".encode())
    body = b"".join(parts)
    data = http(
        "POST", f"{BACKEND}/agents/agent-file-chat-upload/", key,
        body=body, content_type=f"multipart/form-data; boundary={boundary}",
    )
    return data["public_url"], data["external_id"]


def run_agent(audio_url: str, key: str) -> str:
    body = json.dumps({"variables": {"audio_url": audio_url, "source": "voice"}}).encode()
    data = http(
        "POST", f"{BACKEND}/agents/tasks/{AGENT_UUID}/run-loop/", key,
        body=body, content_type="application/json",
    )
    return data["id"]


def poll(execution_id: str, key: str) -> dict:
    deadline = time.time() + POLL_TIMEOUT_S
    while time.time() < deadline:
        data = http("GET", f"{BACKEND}/agents/executions/{execution_id}/", key)
        status = data.get("status")
        if status == "completed":
            return data.get("agent_result") or {}
        if status in ("failed", "cancelled"):
            sys.exit(f"Run {execution_id} {status}: {data.get('error')}")
        time.sleep(POLL_INTERVAL_S)
    sys.exit(f"Run {execution_id} timed out after {POLL_TIMEOUT_S}s")


def save_local(result: dict) -> pathlib.Path:
    NOTES_DIR.mkdir(exist_ok=True)
    now = dt.datetime.now()
    text = result.get("text", "")
    keywords = result.get("keywords", [])
    notion_url = result.get("notion_url", "")
    md = NOTES_DIR / "notes.md"
    with md.open("a") as f:
        f.write(f"\n## {now:%Y-%m-%d %H:%M}\n")
        if keywords:
            f.write(f"tags: {', '.join(keywords)}\n\n")
        f.write(text + "\n")
        if notion_url:
            f.write(f"\n[notion]({notion_url})\n")
    with (NOTES_DIR / "notes.jsonl").open("a") as f:
        f.write(json.dumps({"ts": now.isoformat(), **result}, ensure_ascii=False) + "\n")
    return md


def notify(result: dict) -> None:
    text = result.get("text", "")
    subprocess.run(["pbcopy"], input=text.encode(), check=False)
    head = (text[:120] + "…") if len(text) > 120 else text
    subprocess.run(
        ["osascript", "-e",
         f'display notification "{head}" with title "Voice Inbox" subtitle "copied to clipboard"'],
        check=False,
    )


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--file", help="use an existing audio file instead of recording")
    args = ap.parse_args()
    key = api_key()

    t0 = time.time()
    if args.file:
        audio = pathlib.Path(args.file)
        if not audio.exists():
            sys.exit(f"No such file: {audio}")
    else:
        audio = pathlib.Path(f"/tmp/voicenote-{dt.datetime.now():%Y%m%d-%H%M%S}.m4a")
        record(audio)

    t1 = time.time()
    public_url, _ = upload(audio, key)
    t2 = time.time()
    print(f"↑ uploaded ({t2 - t1:.1f}s)")

    execution_id = run_agent(public_url, key)
    print(f"⚙ run {execution_id} ...")
    result = poll(execution_id, key)
    t3 = time.time()

    md = save_local(result)
    notify(result)

    print(f"\n{result.get('text', '')}\n")
    print(f"tags:   {', '.join(result.get('keywords', []))}")
    print(f"notion: {result.get('notion_url', '')}")
    print(f"local:  {md}")
    print(f"⏱ upload {t2 - t1:.1f}s + agent {t3 - t2:.1f}s = {t3 - t1:.1f}s after recording")


if __name__ == "__main__":
    main()

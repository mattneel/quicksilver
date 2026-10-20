#!/usr/bin/env python3
"""Run scaffold verification with explicit command results."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--offline", action="store_true")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    if shutil.which("cargo") is None:
        raise SystemExit("Cargo is unavailable. Install the pinned Rust toolchain before verification.")
    extra = ["--offline"] if args.offline else []
    binary = root / "target/debug" / ("quicksilver.exe" if os.name == "nt" else "quicksilver")
    if os.environ.get("CARGO_TARGET_DIR"):
        binary = Path(os.environ["CARGO_TARGET_DIR"]).resolve() / "debug" / binary.name
    commands = [
        [sys.executable, "scripts/validate_kit.py"],
        ["cargo", "fmt", "--all", "--", "--check"],
        ["cargo", "test", "--locked", *extra, "--all-targets"],
        ["cargo", "clippy", "--locked", *extra, "--all-targets", "--", "-D", "warnings"],
        ["cargo", "build", "--locked", *extra],
        [sys.executable, "scripts/lsp_smoke.py", "--server", str(binary)],
    ]
    records = []
    environment = os.environ.copy()
    environment["CARGO_TERM_COLOR"] = "never"
    for command in commands:
        print("Run:", " ".join(command), flush=True)
        start = time.monotonic()
        result = subprocess.run(command, cwd=root, capture_output=True, text=True, env=environment)
        diagnostic_error = command[0] == "cargo" and any(
            line.startswith(("error:", "error[")) for line in result.stderr.splitlines()
        )
        record = {"command": command, "exit_code": result.returncode, "diagnostic_error": diagnostic_error, "duration_seconds": round(time.monotonic() - start, 3), "stdout": result.stdout, "stderr": result.stderr}
        records.append(record)
        print(result.stdout, end="")
        print(result.stderr, end="", file=sys.stderr)
        if result.returncode or diagnostic_error:
            break
    passed = len(records) == len(commands) and all(record["exit_code"] == 0 and not record["diagnostic_error"] for record in records)
    output = root / "reports/local/verification.json"
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps({"schema_version": 1, "evidence_class": "scaffold", "status": "passed" if passed else "failed", "commands": records}, indent=2) + "\n")
    print("Scaffold verification:", "passed" if passed else "failed")
    raise SystemExit(0 if passed else 1)


if __name__ == "__main__":
    main()


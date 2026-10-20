#!/usr/bin/env python3
"""Measure the scaffold's edit-to-symbol path. This is not a release benchmark."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import subprocess
import tempfile
import time

from lsp_client import LspClient, flatten_symbols


def percentiles(samples: list[float]) -> dict:
    if not samples:
        return {"p50_ms": None, "p95_ms": None, "p99_ms": None, "max_ms": None}
    ordered = sorted(samples)
    result = {}
    for name, proportion in (("p50_ms", 0.50), ("p95_ms", 0.95), ("p99_ms", 0.99)):
        result[name] = round(ordered[max(0, math.ceil(len(ordered) * proportion) - 1)], 4)
    result["max_ms"] = round(ordered[-1], 4)
    return result


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--server", required=True)
    parser.add_argument("--iterations", type=int, default=200)
    parser.add_argument("--functions", type=int, default=250)
    parser.add_argument("--output", default="reports/local/scaffold-benchmark.json")
    args = parser.parse_args()
    if args.iterations < 20 or args.functions < 1:
        parser.error("Use at least 20 iterations and one function.")
    server = Path(args.server).resolve()
    baseline = "defmodule Bench do\n" + "".join(f"  def item_{index}(x), do: x + {index}\n" for index in range(args.functions)) + "end\n"
    samples = []
    failures = []
    version = subprocess.run([str(server), "--version"], capture_output=True, text=True, check=True).stdout.strip()
    with tempfile.TemporaryDirectory(prefix="quicksilver-bench-") as folder:
        root = Path(folder)
        start = time.perf_counter_ns()
        client = LspClient(str(server), root)
        try:
            client.initialize(root.as_uri())
            uri = (root / "bench.ex").as_uri()
            client.open_document(uri, baseline)
            outline = flatten_symbols(client.request("textDocument/documentSymbol", {"textDocument": {"uri": uri}}))
            if len(outline) != args.functions + 1:
                raise AssertionError("The cold result omitted declarations.")
            cold_ms = (time.perf_counter_ns() - start) / 1_000_000
            client.diagnostics(uri, 1)
            for index in range(1, args.iterations + 1):
                text = baseline.replace("item_0", f"edit_marker_{index}", 1).replace("x + 0", f"x + {index}", 1)
                start = time.perf_counter_ns()
                try:
                    client.change_document(uri, text, index + 1)
                    outline = flatten_symbols(client.request("textDocument/documentSymbol", {"textDocument": {"uri": uri}}))
                    if len(outline) != args.functions + 1:
                        raise AssertionError("A warm result omitted declarations.")
                    if not any(item["name"] == f"edit_marker_{index}/1" for item in outline):
                        raise AssertionError("A warm result contains stale declarations.")
                    elapsed_ms = (time.perf_counter_ns() - start) / 1_000_000
                    client.diagnostics(uri, index + 1)
                    samples.append(elapsed_ms)
                except Exception as error:
                    failures.append({"iteration": index, "error": str(error), "elapsed_ms": (time.perf_counter_ns() - start) / 1_000_000})
                    break
            exit_code = client.shutdown()
            if exit_code != 0:
                failures.append({"phase": "shutdown", "exit_code": exit_code})
        finally:
            client.cleanup()
    result = {
        "schema_version": 1,
        "evidence_class": "scaffold",
        "status": "passed" if not failures and len(samples) == args.iterations else "failed",
        "workload": "full-text declaration rename and body edit followed by fresh documentSymbol for one open document",
        "server_version": version,
        "binary_sha256": hashlib.sha256(server.read_bytes()).hexdigest(),
        "platform": platform.platform(),
        "cpu_count": os.cpu_count(),
        "python": platform.python_version(),
        "corpus": {"files": 1, "functions": args.functions, "bytes": len(baseline.encode("utf-8")), "sha256": hashlib.sha256(baseline.encode()).hexdigest()},
        "cache_state": "fresh server, then resident document tree",
        "cold_process_to_symbols_ms": round(cold_ms, 4),
        "requested_samples": args.iterations,
        "successful_samples": len(samples),
        "unattempted_samples": args.iterations - len(samples) - len([item for item in failures if "iteration" in item]),
        "warm": percentiles(samples),
        "raw_warm_ms": [round(value, 4) for value in samples],
        "failures": failures,
        "limitations": [
            "One synthetic document only.",
            "No semantic resolver or BEAM worker exists in this scaffold.",
            "One cold sample cannot establish a cold percentile.",
            "Successful-sample percentiles exclude failures, which remain listed separately.",
            "These measurements do not satisfy production performance gates.",
        ],
    }
    destination = Path(args.output)
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({key: value for key, value in result.items() if key != "raw_warm_ms"}, indent=2))
    if result["status"] != "passed":
        raise SystemExit(1)


if __name__ == "__main__":
    main()


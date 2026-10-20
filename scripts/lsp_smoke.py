#!/usr/bin/env python3
"""Verify the bootstrap through a real stdio subprocess."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import platform
import subprocess
import tempfile
import time

from lsp_client import LspClient, LspError, flatten_symbols, selected_text


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def expect_error(client: LspClient, method: str, params: object, code: int) -> None:
    try:
        client.request(method, params)
    except LspError as error:
        require(error.error["code"] == code, f"Expected error {code}, received {error.error}")
    else:
        raise AssertionError(f"Expected an error for {method}.")


def send_fragmented(client: LspClient, message: dict) -> None:
    payload = json.dumps(message, ensure_ascii=False, separators=(",", ":")).encode("utf-8")
    header = f"Content-Length: {len(payload)}\r\n\r\n".encode("ascii")
    frame = header + payload
    cuts = {0, 1, 12, len(header) - 3, len(header) - 1, len(header), len(header) + 1,
            len(header) + len(payload) // 2, len(frame) - 1, len(frame)}
    for index, byte in enumerate(payload):
        if byte >= 0xC0:
            cuts.add(len(header) + index + 1)
            break
    boundaries = sorted(cuts)
    for start, end in zip(boundaries, boundaries[1:]):
        client._write_all(frame[start:end])
        time.sleep(0.002)


def run(server: str) -> dict:
    executable = Path(server).resolve()
    tests: list[str] = []
    start = time.monotonic()
    version = subprocess.run([str(executable), "--version"], check=True, capture_output=True, text=True, timeout=10).stdout.strip()
    require("quicksilver" in version.lower(), "The version output does not identify Quicksilver.")
    tests.append("CLI version")
    with tempfile.TemporaryDirectory(prefix="quicksilver-lsp-test-") as folder:
        root = Path(folder)
        uri = (root / "smoke.ex").as_uri()
        for encoding in ("utf-16", "utf-8"):
            client = LspClient(str(executable), root)
            try:
                expect_error(client, "textDocument/documentSymbol", {"textDocument": {"uri": uri}}, -32002)
                initialized = client.initialize(root.as_uri(), encoding)
                capabilities = initialized["capabilities"]
                require(capabilities["positionEncoding"] == encoding, "The server did not honor the supported encoding.")
                require(capabilities.get("documentSymbolProvider") is True, "Document symbols are not advertised.")
                require(capabilities["textDocumentSync"]["change"] == 1, "The scaffold must advertise full-text synchronization.")
                require(not capabilities.get("definitionProvider"), "The scaffold falsely advertises definitions.")
                require(not capabilities.get("referencesProvider"), "The scaffold falsely advertises references.")
                expect_error(client, "quicksilver/unknownTestMethod", {}, -32601)
                expect_error(client, "initialize", {}, -32600)
                tests.append(f"{encoding}: lifecycle and truthful capabilities")

                text = 'defmodule Smoke do\r\n  @moduledoc "💡 café"\r\n  def greet(name), do: {:ok, name}\r\nend\r\n'
                client.open_document(uri, text)
                require(client.diagnostics(uri, 1)["diagnostics"] == [], "Valid code produced syntax errors.")
                outline = flatten_symbols(client.request("textDocument/documentSymbol", {"textDocument": {"uri": uri}}))
                names = [item["name"] for item in outline]
                require("Smoke" in names and "greet/1" in names, f"Missing source declarations: {names}")
                for item in outline:
                    expected = "Smoke" if item["name"] == "Smoke" else "greet"
                    require(selected_text(text, item["selectionRange"], encoding) == expected, f"Wrong declaration range: {item}")
                tests.append(f"{encoding}: CRLF and exact declaration ranges")

                same_line = 'defmodule Smoke do\n  def first, do: "💡"; def after_emoji, do: :ok\nend\n'
                client.change_document(uri, same_line, 2)
                require(client.diagnostics(uri, 2)["diagnostics"] == [], "The Unicode fixture did not parse.")
                outline = flatten_symbols(client.request("textDocument/documentSymbol", {"textDocument": {"uri": uri}}))
                symbol = next(item for item in outline if item["name"] == "after_emoji/0")
                require(selected_text(same_line, symbol["selectionRange"], encoding) == "after_emoji", "The emoji shifted a declaration range.")
                tests.append(f"{encoding}: non-BMP character before a symbol")

                client.change_document(uri, "defmodule Broken do\n  def missing(\n", 3)
                errors = client.diagnostics(uri, 3)["diagnostics"]
                require(bool(errors), "Malformed syntax produced no diagnostics.")
                require(all(item.get("source") == "quicksilver-syntax" for item in errors), "A syntax diagnostic has the wrong provenance.")
                client.change_document(uri, text, 4)
                require(client.diagnostics(uri, 4)["diagnostics"] == [], "Recovered syntax did not clear its diagnostics.")
                tests.append(f"{encoding}: malformed edit and recovery")

                client.change_document(uri, "defmodule Obsolete do end\n", 3)
                names = [item["name"] for item in flatten_symbols(client.request("textDocument/documentSymbol", {"textDocument": {"uri": uri}}))]
                require("Smoke" in names and "Obsolete" not in names, "A stale revision replaced current source text.")
                tests.append(f"{encoding}: stale edit rejection")

                status = client.request("quicksilver/status", {})
                require(status.get("projectExecution") is False, "The scaffold reports project execution.")
                require(status.get("semanticReady") is False, "The scaffold falsely reports semantic readiness.")
                require(status.get("diagnosticCoverage") == "syntax-only", "The scaffold overstates diagnostic coverage.")
                client.notify("textDocument/didClose", {"textDocument": {"uri": uri}})
                require(client.diagnostics(uri, None)["diagnostics"] == [], "Document close did not clear diagnostics.")
                expect_error(client, "textDocument/documentSymbol", {"textDocument": {"uri": uri}}, -32602)
                require(client.shutdown() == 0, "The graceful shutdown failed.")
                tests.append(f"{encoding}: status, document close, and shutdown")
            finally:
                client.cleanup()

        client = LspClient(str(executable), root)
        try:
            client.initialize(root.as_uri(), hierarchical=False)
            client.open_document(uri, "defmodule Flat do\n  def item, do: :ok\nend\n")
            symbols = client.request("textDocument/documentSymbol", {"textDocument": {"uri": uri}})
            require(len(symbols) == 2 and {item["name"] for item in symbols} == {"Flat", "item/0"}, "The flat response omitted or added declarations.")
            require(all("location" in item and "selectionRange" not in item for item in symbols), "The flat client received hierarchical symbols.")
            require(client.shutdown() == 0, "The flat client failed to shut down.")
            tests.append("flat SymbolInformation compatibility")
        finally:
            client.cleanup()

        client = LspClient(str(executable), root, timeout=5)
        try:
            client.initialize(root.as_uri(), encoding="utf-8")
            text = 'defmodule Fragmented do\n  def marker, do: "💡"\nend\n'
            send_fragmented(client, {"jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {
                "textDocument": {"uri": uri, "languageId": "elixir", "version": 1, "text": text}
            }})
            require(client.diagnostics(uri, 1)["diagnostics"] == [], "Fragmented source text did not parse.")
            client.sequence += 1
            request_id = client.sequence
            send_fragmented(client, {"jsonrpc": "2.0", "id": request_id, "method": "textDocument/documentSymbol", "params": {
                "textDocument": {"uri": uri}
            }})
            symbols = flatten_symbols(client.response(request_id))
            require(len(symbols) == 2 and {item["name"] for item in symbols} == {"Fragmented", "marker/0"}, "Fragmented frames lost declaration data.")
            for symbol in symbols:
                expected = "Fragmented" if symbol["name"] == "Fragmented" else "marker"
                require(selected_text(text, symbol["selectionRange"], "utf-8") == expected, "A fragmented frame changed a declaration range.")
            require(client.shutdown() == 0, "The fragmented client failed to shut down.")
            tests.append("fragmented headers, bodies, and UTF-8 bytes")
        finally:
            client.cleanup()

        client = LspClient(str(executable), root)
        try:
            client.notify("exit")
            require(client.process.wait(timeout=5) == 1, "Exit without shutdown must return status 1.")
            tests.append("exit without shutdown")
        finally:
            client.cleanup()

    return {
        "schema_version": 1,
        "evidence_class": "scaffold",
        "status": "passed",
        "server_version": version,
        "binary_sha256": hashlib.sha256(executable.read_bytes()).hexdigest(),
        "platform": platform.platform(),
        "python": platform.python_version(),
        "duration_seconds": round(time.monotonic() - start, 3),
        "passed_groups": len(tests),
        "groups": tests,
        "excluded": ["semantic resolution", "BEAM worker", "OMP integration", "release performance"],
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--server", required=True)
    parser.add_argument("--output")
    args = parser.parse_args()
    result = run(args.server)
    payload = json.dumps(result, indent=2) + "\n"
    if args.output:
        destination = Path(args.output)
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_text(payload)
    print(payload, end="")


if __name__ == "__main__":
    main()


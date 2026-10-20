"""A small external LSP test client with bounded waits."""

from __future__ import annotations

import json
import os
from pathlib import Path
import queue
import subprocess
import threading
import time
from typing import Any


class LspError(RuntimeError):
    def __init__(self, error: dict[str, Any]):
        self.error = error
        super().__init__(f"LSP error {error.get('code')}: {error.get('message')}")


class LspClient:
    def __init__(self, server: str, cwd: str | Path, timeout: float = 10.0):
        executable = Path(server).resolve()
        if not executable.is_file():
            raise FileNotFoundError(f"The server executable does not exist: {executable}")
        self.timeout = timeout
        self.sequence = 0
        self.notifications: list[dict[str, Any]] = []
        self.responses: dict[int, dict[str, Any]] = {}
        self.events: queue.Queue[Any] = queue.Queue()
        self.stderr_lines: list[str] = []
        self.process = subprocess.Popen(
            [str(executable), "--stdio"],
            cwd=str(cwd),
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env=os.environ.copy(),
            bufsize=0,
        )
        self.reader = threading.Thread(target=self._read_frames, daemon=True)
        self.stderr_reader = threading.Thread(target=self._read_stderr, daemon=True)
        self.reader.start()
        self.stderr_reader.start()

    def _read_exact(self, size: int) -> bytes:
        assert self.process.stdout is not None
        pieces = bytearray()
        while len(pieces) < size:
            piece = self.process.stdout.read(size - len(pieces))
            if not piece:
                raise EOFError("The server closed stdout inside a frame.")
            pieces.extend(piece)
        return bytes(pieces)

    def _read_frames(self) -> None:
        assert self.process.stdout is not None
        try:
            while True:
                headers: dict[str, str] = {}
                total = 0
                while True:
                    line = self.process.stdout.readline(8193)
                    if not line:
                        raise EOFError("The server closed stdout.")
                    total += len(line)
                    if total > 8192:
                        raise ValueError("The LSP header exceeds 8192 bytes.")
                    if line in (b"\r\n", b"\n"):
                        break
                    name, separator, value = line.decode("ascii").partition(":")
                    if not separator:
                        raise ValueError("The server wrote an invalid LSP header.")
                    key = name.lower().strip()
                    if key in headers:
                        raise ValueError("The server wrote a duplicate LSP header.")
                    headers[key] = value.strip()
                size = int(headers["content-length"])
                if not 0 <= size <= 16 * 1024 * 1024:
                    raise ValueError("The LSP frame exceeds the test limit.")
                message = json.loads(self._read_exact(size))
                if not isinstance(message, dict) or message.get("jsonrpc") != "2.0":
                    raise ValueError("The server wrote an invalid JSON-RPC object.")
                self.events.put(message)
        except BaseException as error:
            self.events.put(error)

    def _read_stderr(self) -> None:
        assert self.process.stderr is not None
        for line in iter(self.process.stderr.readline, b""):
            self.stderr_lines.append(line.decode("utf-8", errors="replace").rstrip())
            del self.stderr_lines[:-100]

    def send(self, message: dict[str, Any]) -> None:
        payload = json.dumps(message, ensure_ascii=False, separators=(",", ":")).encode("utf-8")
        frame = f"Content-Length: {len(payload)}\r\n\r\n".encode("ascii") + payload
        self._write_all(frame)

    def _write_all(self, data: bytes) -> None:
        assert self.process.stdin is not None
        remaining = memoryview(data)
        while remaining:
            written = self.process.stdin.write(remaining)
            if written is None or written <= 0:
                raise BrokenPipeError("The test client did not write a complete LSP frame.")
            remaining = remaining[written:]
        self.process.stdin.flush()

    def notify(self, method: str, params: Any = None) -> None:
        self.send({"jsonrpc": "2.0", "method": method, "params": params})

    def send_request(self, method: str, params: Any = None) -> int:
        self.sequence += 1
        self.send({"jsonrpc": "2.0", "id": self.sequence, "method": method, "params": params})
        return self.sequence

    def _receive(self, deadline: float) -> dict[str, Any]:
        remaining = deadline - time.monotonic()
        if remaining <= 0:
            raise TimeoutError("The LSP test deadline expired.")
        try:
            item = self.events.get(timeout=remaining)
        except queue.Empty as error:
            detail = "\n".join(self.stderr_lines[-10:])
            raise TimeoutError(f"The server did not respond before the deadline.\n{detail}") from error
        if isinstance(item, BaseException):
            raise RuntimeError(f"The LSP transport failed: {item}") from item
        if "method" in item:
            if "id" in item:
                self.send({"jsonrpc": "2.0", "id": item["id"], "error": {
                    "code": -32601, "message": "The test client does not implement this method."
                }})
            else:
                self.notifications.append(item)
                if len(self.notifications) > 4096:
                    raise RuntimeError("The server exceeded the notification test limit.")
        elif "id" in item:
            self.responses[item["id"]] = item
        return item

    def response(self, request_id: int, timeout: float | None = None) -> Any:
        deadline = time.monotonic() + (timeout or self.timeout)
        while request_id not in self.responses:
            self._receive(deadline)
        message = self.responses.pop(request_id)
        if "error" in message:
            raise LspError(message["error"])
        if "result" not in message:
            raise ValueError("The response has neither a result nor an error.")
        return message["result"]

    def request(self, method: str, params: Any = None, timeout: float | None = None) -> Any:
        return self.response(self.send_request(method, params), timeout)

    def initialize(self, uri: str, encoding: str = "utf-16", hierarchical: bool = True) -> dict[str, Any]:
        result = self.request("initialize", {
            "processId": os.getpid(),
            "rootUri": uri,
            "capabilities": {
                "general": {"positionEncodings": [encoding]},
                "textDocument": {
                    "documentSymbol": {"hierarchicalDocumentSymbolSupport": hierarchical},
                    "publishDiagnostics": {"versionSupport": True, "dataSupport": True},
                },
            },
            "clientInfo": {"name": "quicksilver-independent-test", "version": "1"},
        })
        self.notify("initialized", {})
        return result

    def open_document(self, uri: str, text: str, version: int = 1) -> None:
        self.notify("textDocument/didOpen", {"textDocument": {
            "uri": uri, "languageId": "elixir", "version": version, "text": text
        }})

    def change_document(self, uri: str, text: str, version: int) -> None:
        self.notify("textDocument/didChange", {
            "textDocument": {"uri": uri, "version": version},
            "contentChanges": [{"text": text}],
        })

    def diagnostics(self, uri: str, version: int | None, timeout: float | None = None) -> dict[str, Any]:
        deadline = time.monotonic() + (timeout or self.timeout)
        while True:
            for index, message in enumerate(self.notifications):
                params = message.get("params", {})
                if message.get("method") != "textDocument/publishDiagnostics":
                    continue
                version_matches = "version" not in params if version is None else params.get("version") == version
                if params.get("uri") == uri and version_matches:
                    self.notifications.pop(index)
                    return params
            self._receive(deadline)

    def shutdown(self) -> int:
        self.request("shutdown")
        self.notify("exit")
        result = self.process.wait(timeout=self.timeout)
        self.cleanup()
        return result

    def cleanup(self) -> None:
        if self.process.poll() is None:
            self.process.terminate()
            try:
                self.process.wait(timeout=2)
            except subprocess.TimeoutExpired:
                self.process.kill()
                self.process.wait(timeout=2)
        for reader in (self.reader, self.stderr_reader):
            reader.join(timeout=2)
        for stream in (self.process.stdin, self.process.stdout, self.process.stderr):
            if stream:
                stream.close()


def flatten_symbols(symbols: list[dict[str, Any]]) -> list[dict[str, Any]]:
    result: list[dict[str, Any]] = []
    for symbol in symbols:
        result.append(symbol)
        result.extend(flatten_symbols(symbol.get("children", [])))
    return result


def selected_text(text: str, span: dict[str, Any], encoding: str) -> str:
    lines = text.split("\n")
    line_starts = [0]
    for line in lines[:-1]:
        line_starts.append(line_starts[-1] + len(line) + 1)
    offsets = []
    codec = "utf-16-le" if encoding == "utf-16" else "utf-8"
    unit = 2 if encoding == "utf-16" else 1
    for key in ("start", "end"):
        position = span[key]
        line = position["line"]
        column = position["character"]
        if type(line) is not int or type(column) is not int or not 0 <= line < len(lines) or column < 0:
            raise AssertionError(f"The range is outside the document: {span}")
        content = lines[line]
        if line < len(lines) - 1 and content.endswith("\r"):
            content = content[:-1]
        encoded = content.encode(codec)
        if column * unit > len(encoded):
            raise AssertionError(f"The range is outside its source line: {span}")
        try:
            prefix = encoded[:column * unit].decode(codec)
        except UnicodeDecodeError as error:
            raise AssertionError(f"The range splits a Unicode character: {span}") from error
        offsets.append(line_starts[line] + len(prefix))
    if offsets[0] > offsets[1]:
        raise AssertionError("The selection range is reversed.")
    return text[offsets[0]:offsets[1]]


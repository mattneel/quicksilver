#!/usr/bin/env python3
"""Verify kit structure and task evidence. Release mode requires complete gates."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import re
import sys
import tomllib


def validate(root: Path, release: bool = False) -> dict:
    failures = []
    required = [
        "README.md", "START_HERE.md", "AGENTS.md", "LICENSE", "Cargo.toml", "Cargo.lock", "rust-toolchain.toml",
        "prompts/IMPLEMENT.md", "prompts/RESUME.md", "prompts/REVIEW.md",
        "docs/VISION.md", "docs/ARCHITECTURE.md", "docs/ELIXIR_SEMANTICS.md", "docs/PROTOCOL.md",
        "docs/DECISIONS.md", "docs/IMPLEMENTATION_PLAN.md", "docs/OMP_INTEGRATION.md",
        "docs/QUALITY_GATES.md", "docs/PERFORMANCE.md", "docs/FEATURE_MATRIX.md", "docs/RESEARCH.md",
        "docs/OPERATIONS.md", "planning/tasks.json", "planning/PROGRESS.md", "fixtures/cases.json",
        "scripts/lsp_client.py", "scripts/lsp_smoke.py", "scripts/benchmark.py", "scripts/verify.py",
        "scripts/task_status.py", "examples/omp/quicksilver.lsp.json",
    ]
    for name in required:
        if not (root / name).is_file():
            failures.append(f"Required file is absent: {name}")

    def load(relative: str):
        try:
            return json.loads((root / relative).read_text())
        except (OSError, ValueError) as error:
            failures.append(f"Cannot read {relative}: {error}")
            return {}

    plan = load("planning/tasks.json")
    tasks = plan.get("tasks", [])
    identifiers = [task.get("id") for task in tasks]
    if not tasks or len(identifiers) != len(set(identifiers)):
        failures.append("Task IDs must be nonempty and unique.")
    by_id = {task.get("id"): task for task in tasks}
    statuses = {"pending", "in_progress", "blocked", "done"}
    for task in tasks:
        identifier = task.get("id", "missing-id")
        for field in ("title", "phase", "status", "depends_on", "deliverables", "acceptance", "verify", "evidence"):
            if field not in task:
                failures.append(f"{identifier} has no {field} field.")
        if task.get("status") not in statuses:
            failures.append(f"{identifier} has an invalid status.")
        if len(task.get("acceptance", [])) < 3:
            failures.append(f"{identifier} requires at least three acceptance conditions.")
        for dependency in task.get("depends_on", []):
            if dependency not in by_id:
                failures.append(f"{identifier} has an unknown dependency: {dependency}")
            elif task.get("status") == "done" and by_id[dependency].get("status") != "done":
                failures.append(f"{identifier} is done before dependency {dependency}.")
        if task.get("status") == "done":
            if not task.get("evidence"):
                failures.append(f"{identifier} has no evidence.")
            for item in task.get("evidence", []):
                path = (root / item).resolve()
                if not path.is_relative_to(root.resolve()) or not path.is_file():
                    failures.append(f"{identifier} has an invalid evidence path: {item}")
        if release and task.get("required", True) and task.get("status") != "done":
            failures.append(f"Release gate is incomplete: {identifier}")

    visited = set()
    active = set()
    def visit(identifier):
        if identifier in active:
            failures.append(f"The task graph contains a cycle at {identifier}.")
            return
        if identifier in visited or identifier not in by_id:
            return
        active.add(identifier)
        for dependency in by_id[identifier].get("depends_on", []):
            visit(dependency)
        active.remove(identifier)
        visited.add(identifier)
    for identifier in by_id:
        visit(identifier)

    catalog = load("fixtures/cases.json")
    cases = catalog.get("cases", [])
    case_ids = [case.get("id") for case in cases]
    if not cases or len(case_ids) != len(set(case_ids)):
        failures.append("The fixture catalog needs unique case IDs.")
    anchor_count = 0

    def verify_anchors(value, case_id, case_files):
        nonlocal anchor_count
        if isinstance(value, dict):
            if "anchor" in value and "file" in value:
                anchor_count += 1
                name = value["file"]
                if name not in case_files:
                    failures.append(f"Fixture {case_id} has an anchor outside its source snapshot: {name}")
                    return
                path = (root / name).resolve()
                if not path.is_relative_to(root.resolve()) or not path.is_file():
                    return
                source = path.read_bytes().decode("utf-8")
                anchor = value["anchor"]
                selected = value.get("selectionText", "")
                occurrence = value.get("occurrence", 1)
                if not anchor or source.count(anchor) != 1:
                    failures.append(f"Fixture {case_id} has an absent or ambiguous anchor: {anchor}")
                    return
                prefix = source[:source.index(anchor)].rsplit("\n", 1)[-1]
                if not selected or not isinstance(occurrence, int) or occurrence < 1 or prefix.count(selected) < occurrence:
                    failures.append(f"Fixture {case_id} has an invalid anchor selection: {anchor}")
            for item in value.values():
                verify_anchors(item, case_id, case_files)
        elif isinstance(value, list):
            for item in value:
                verify_anchors(item, case_id, case_files)

    for case in cases:
        for field in ("category", "required_phase", "files", "scenario", "expected", "oracle", "status"):
            if field not in case:
                failures.append(f"Fixture {case.get('id')} has no {field} field.")
        for name in case.get("files", []):
            path = (root / name).resolve()
            if not path.is_relative_to(root.resolve()) or not path.is_file():
                failures.append(f"Fixture {case.get('id')} names an absent file: {name}")
        verify_anchors(case.get("expected", {}), case.get("id"), case.get("files", []))

    config = load("examples/omp/quicksilver.lsp.json").get("servers", {})
    if not config.get("elixirls", {}).get("disabled") or not config.get("expert", {}).get("disabled"):
        failures.append("The OMP example must disable both legacy Elixir servers.")
    if config.get("quicksilver", {}).get("args") != ["--stdio"]:
        failures.append("The OMP example must invoke the stdio executable.")
    if (root / ".omp/lsp.json").exists():
        failures.append("The kit must not activate its example Elixir server in the Rust implementation workspace.")

    for document in root.rglob("*.md"):
        if "target" in document.relative_to(root).parts:
            continue
        text = document.read_text()
        for target in re.findall(r"\[[^\]]+\]\(([^)]+)\)", text):
            target = target.split("#", 1)[0]
            if not target or "://" in target or target.startswith(("mailto:", "sandbox:")):
                continue
            if not (document.parent / target).exists():
                failures.append(f"Broken local link in {document.relative_to(root)}: {target}")
    try:
        metadata = tomllib.loads((root / "Cargo.toml").read_text())
        if metadata["package"]["name"] != "quicksilver":
            failures.append("The Cargo package must be named quicksilver.")
        if not release and metadata["package"].get("publish") is not False:
            failures.append("Registry publication must remain disabled in the seed.")
    except (OSError, ValueError, KeyError) as error:
        failures.append(f"Cannot verify Cargo metadata: {error}")
    return {
        "schema_version": 1, "evidence_class": "structure", "mode": "release" if release else "kit",
        "status": "failed" if failures else "passed", "task_count": len(tasks), "fixture_count": len(cases),
        "fixture_anchor_count": anchor_count,
        "failures": failures,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--release", action="store_true")
    args = parser.parse_args()
    result = validate(Path(__file__).resolve().parents[1], args.release)
    print(json.dumps(result, indent=2))
    sys.exit(0 if result["status"] == "passed" else 1)


if __name__ == "__main__":
    main()


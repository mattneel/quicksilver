#!/usr/bin/env python3
"""Print the implementation ledger without any mutation."""

import argparse
from collections import Counter
import json
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--all", action="store_true")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    plan = json.loads((root / "planning/tasks.json").read_text())
    tasks = plan["tasks"]
    completed = {task["id"] for task in tasks if task["status"] == "done"}
    counts = Counter(task["status"] for task in tasks)
    print("Quicksilver task ledger")
    print(" ".join(f"{name}={counts[name]}" for name in ("pending", "in_progress", "blocked", "done")))
    for task in tasks:
        ready = task["status"] in ("pending", "in_progress") and set(task["depends_on"]) <= completed
        if args.all or ready:
            marker = "ready" if ready else task["status"]
            print(f"{task['id']} [{task['phase']}] {marker}: {task['title']}")
            if args.all and task["depends_on"]:
                print("  dependencies: " + ", ".join(task["depends_on"]))
    if not args.all and not any(task["status"] in ("pending", "in_progress") and set(task["depends_on"]) <= completed for task in tasks):
        print("No task is ready. Inspect incomplete dependencies or external blockers.")


if __name__ == "__main__":
    main()


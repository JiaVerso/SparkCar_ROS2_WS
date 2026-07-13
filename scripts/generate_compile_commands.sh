#!/usr/bin/env bash

set -euo pipefail

workspace_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output_file="${workspace_root}/compile_commands.json"

python3 - "${workspace_root}" "${output_file}" <<'PY'
import json
import os
import sys

workspace_root = os.path.realpath(sys.argv[1])
output_file = os.path.realpath(sys.argv[2])
entries_by_file = {}
database_paths = []

for root, dirs, files in os.walk(workspace_root):
    dirs[:] = [directory for directory in dirs if directory not in {".git", "install", "log"}]
    if "compile_commands.json" not in files:
        continue

    path = os.path.realpath(os.path.join(root, "compile_commands.json"))
    package_build_dir = os.path.dirname(path)
    if path == output_file or os.path.basename(os.path.dirname(package_build_dir)) != "build":
        continue
    database_paths.append(path)

for path in sorted(database_paths):
    with open(path, encoding="utf-8") as database:
        entries = json.load(database)
    for entry in entries:
        source_file = entry.get("file")
        if not source_file:
            continue
        if not os.path.isabs(source_file):
            source_file = os.path.join(entry.get("directory", workspace_root), source_file)
        entries_by_file[os.path.realpath(source_file)] = entry

with open(output_file, "w", encoding="utf-8") as database:
    json.dump(list(entries_by_file.values()), database, indent=2)
    database.write("\n")

print(f"Merged {len(database_paths)} databases and {len(entries_by_file)} source files")
print(f"Output: {output_file}")
PY

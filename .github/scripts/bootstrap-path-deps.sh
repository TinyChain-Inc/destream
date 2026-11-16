#!/usr/bin/env bash
set -euo pipefail

declare -A seen
queue=("$PWD")
seen["$PWD"]=1

while ((${#queue[@]})); do
  dir="${queue[0]}"
  queue=("${queue[@]:1}")

  while IFS= read -r cargo; do
    while IFS= read -r dep; do
      [[ -n "$dep" ]] || continue
      dep_dir="$(cd "$dir/.." && pwd)/$dep"

      if [[ ! -d "$dep_dir" ]]; then
        echo "Cloning dependency repo: $dep"
        git clone --depth 1 "https://github.com/TinyChain-Inc/$dep.git" "$dep_dir"
      fi

      if [[ -z "${seen[$dep_dir]+x}" ]]; then
        queue+=("$dep_dir")
        seen["$dep_dir"]=1
      fi
    done < <(
      grep -hoE 'path[[:space:]]*=[[:space:]]*"\.\./[^"]+"' "$cargo" \
        | sed -E 's/.*"\.\.\/([^"]+)".*/\1/' \
        | sort -u || true
    )
  done < <(find "$dir" -name Cargo.toml -type f)
done


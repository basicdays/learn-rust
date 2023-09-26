#!/usr/bin/env bash
set -Eeu -o pipefail

for dir in */; do
	if [[ -f "$dir/Cargo.toml" ]]; then
		(cd "$dir" && cargo fmt)
	elif [[ -f "$dir/Makefile" ]]; then
		(cd "$dir" && make format)
	fi
done

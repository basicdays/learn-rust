#!/usr/bin/env bash
set -Eeou pipefail

for dir in */; do
	if [ -f "$dir/Cargo.toml" ]; then
		(cd "$dir" && cargo build)
	elif [ -f "$dir/Makefile" ]; then
		(cd "$dir" && make)
	fi
done

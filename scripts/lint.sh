#!/usr/bin/env bash
set -Eeou pipefail

for dir in */; do
	if [ -f "$dir/Cargo.toml" ]; then
		(cd "$dir" && cargo clippy)
	elif [ -f "$dir/Makefile" ]; then
		(cd "$dir" && make lint)
	fi
done

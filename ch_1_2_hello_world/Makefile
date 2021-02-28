all: main

main: main.rs
	@rustc $<

lint:
	clippy-driver main.rs

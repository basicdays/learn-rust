all: main

main: main.rs
	@rustc $<

run: main
	@./main

clean:
	@rm main

lint:
	@clippy-driver main.rs

format:
	@rustfmt *.rs

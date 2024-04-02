#Sample Makefile
build:
	cargo build
test:
	cargo test --package math_lib
dev:
	cargo tauri dev
release:
	cargo build --release
	cargo tauri build
# will generate docs for math_lib
docs:
	cargo doc --package math_lib
# will compile profiling but not run it
profiling:
	cargo build --package profiling

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
profiling:
	export CARGO_PROFILE_RELEASE_DEBUG=true
	cargo build --package profiling
	cargo flamegraph  --package profiling --bin profiling -o flamegraph_10.svg < profiling/test_case_10.txt

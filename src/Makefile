test_min=0.
test_max=1000000.

.PHONY: all build test clean dev release docs profiling

all:
pack:
clean:
	sudo rm -rf target
test:
	cargo test --package math_lib --lib parser
doc:
	cargo doc --package profiling
	cargo doc --package math_lib --document-private-items --open
run: dev
profiling:
	make flamegraph_{10,1000,1000000}.svg

dependencies:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	cargo install create-tauri-app --locked
	cargo install tauri-cli --locked
	cargo install flamegraph --locked
	cargo install cargo-deb
build:
	cargo build
dev:
	cargo tauri dev
release:
	cargo build --release
	cargo tauri build
	cargo deb --package profiling
test_case_%.txt:
	cargo run --package profiling --bin generator target/profiling/$@ $(test_min) $(test_max)
flamegraph_%.svg: test_case_%.txt 
	sudo cargo flamegraph --package profiling --bin profiling -o target/profiling/$@ < target/profiling/$<

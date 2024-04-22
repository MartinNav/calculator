test_min=0.
test_max=1000000.

.PHONY: build test dev release docs profiling

build:
	cargo build
test:
	cargo test --package math_lib --lib parser
dev:
	cargo tauri dev
release:
	cargo build --release
	cargo tauri build
docs:
	cargo doc --package math_lib --document-private-items --open
	cargo doc --package profiling
clean:
	sudo rm -rf target
profiling:
	make flamegraph_{10,1000,1000000}.svg
test_case_%.txt:
	cargo run --package profiling --bin generator target/profiling/$@ $(test_min) $(test_max)
flamegraph_%.svg: test_case_%.txt 
	sudo cargo flamegraph --package profiling --bin profiling -o target/profiling/$@ < target/profiling/$<

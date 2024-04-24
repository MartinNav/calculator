test_min=0.
test_max=1000000.

.PHONY: all build test clean dev release docs profiling

all:
	make doc
	make test
	make profiling
	make run
pack:
	make clean
	make release
	mkdir target/archive
	mkdir target/archive/doc
	make doc
	cp -r target/doc/* target/archive/doc
	mkdir target/archive/install
	cp target/debian/*.deb target/archive/install/
	cp target/release/bundle/deb/*.deb target/archive/install/
	mkdir target/archive/repo
	rm -rf target/debian target/debug target/doc target/release
	rsync -avm ../../calculator/ target/archive/repo/
	cd target/archive; \
	zip -r -D ../xnavram00_xgajdo33_xcernim00_xcapkar00.zip doc/ install/ repo/
clean:
	rm -rf target
test:
	cargo test --package math_lib --lib parser
doc:
	cargo doc --package profiling
	cargo doc --package math_lib --document-private-items --open
run: dev
profiling:
	make flamegraph_{10}.svg
	make flamegraph_{1000}.svg
	make flamegraph_{1000000}.svg

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
	cargo flamegraph --package profiling --bin profiling -o target/profiling/$@ < target/profiling/$<

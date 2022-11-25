dev: target/debug/kv

release: target/release/kv

target/debug/kv: $(shell find ./src Cargo.toml)
	cargo build

target/release/kv: $(shell find ./src Cargo.toml)
	cargo build --release

clean:
	-rm -rf target

install: release
	cp target/release/kv /usr/local/bin
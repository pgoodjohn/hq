.PHONY:release
release:
	cargo build --release --bin hq
	mkdir -p ~/.hq/bin/
	mv -f ./target/release/hq ~/.hq/bin/hq

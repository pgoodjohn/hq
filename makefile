.PHONY:release
release:
	cargo build --release --bin hq
	mkdir ~/.hq/
	mkdir ~/.hq/bin/
	mv ./target/release/hq ~/.hq/bin/hq

build:
	cargo build --release

pack: build
	upx --best --lzma target/release/collatz-cli

install: pack
	mv target/release/collatz-cli target/release/collatz
	sudo mv target/release/collatz /usr/local/bin/

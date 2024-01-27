build:
	cargo build --release

pack: build
	upx --best --lzma target/release/collatz

install: pack
	sudo mv target/release/collatz /usr/local/bin/

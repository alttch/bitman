all:
	rm -f bitman
	cargo build --target x86_64-unknown-linux-musl --release
	cp ./target/x86_64-unknown-linux-musl/release/bitman .
	strip bitman

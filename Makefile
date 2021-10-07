VERSION=$(shell grep ^version Cargo.toml|head -1|cut -d\" -f2)

all:
	@echo ${VERSION}
	@echo select target

tag:
	git tag -a v${VERSION} -m v${VERSION}
	git push origin --tags

release: tag pkg

pkg:
	rm -rf _build
	mkdir -p _build
	cargo build --target x86_64-unknown-linux-musl --release
	cargo build --target i686-unknown-linux-musl --release
	cargo build --target arm-unknown-linux-musleabihf --release
	cargo build --target aarch64-unknown-linux-musl --release
	cd target/x86_64-unknown-linux-musl/release && cp bitman ../../../_build/bitman-${VERSION}-x86_64
	cd target/i686-unknown-linux-musl/release && cp bitman ../../../_build/bitman-${VERSION}-i686
	cd target/arm-unknown-linux-musleabihf/release && bitman ../../../_build/bitman-${VERSION}-arm-musleabihf
	cd target/aarch64-unknown-linux-musl/release && aarch64-linux-gnu-strip bitman && \
		cp bitman ../../../_build/bitman-${VERSION}-aarch64
	cd _build && echo "" | gh release create v$(VERSION) -t "v$(VERSION)" \
			bitman-${VERSION}-x86_64 \
			bitman-${VERSION}-i686 \
			bitman-${VERSION}-arm-musleabihf \
			bitman-${VERSION}-aarch64

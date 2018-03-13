all: target/release/pop-theme-toggle

clean:
	cargo clean

target/release/pop-theme-toggle: src/* Cargo.toml Cargo.lock
	cargo build --release

install: target/release/pop-theme-toggle
	install $< /usr/local/bin/pop-theme-toggle

uninstall:
	rm -rf /usr/local/bin/pop-theme-toggle


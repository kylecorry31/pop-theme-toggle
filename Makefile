# Modified from https://github.com/pop-os/system76-power

prefix ?= /usr
sysconfdir ?= /etc
exec_prefix = $(prefix)
bindir = $(exec_prefix)/bin
libdir = $(exec_prefix)/lib
includedir = $(prefix)/include
datarootdir = $(prefix)/share
datadir = $(datarootdir)

.PHONY: all clean distclean install uninstall update

BIN=pop-theme-toggle

all: target/release/$(BIN)

clean:
	cargo clean

distclean: clean
	rm -rf .cargo vendor

install: all
	install -D -m 04755 "target/release/$(BIN)" "$(DESTDIR)$(bindir)/$(BIN)"

uninstall:
	rm -f "$(DESTDIR)$(bindir)/$(BIN)"

update:
	cargo update

.cargo/config: vendor_config
	mkdir -p .cargo
	cp $< $@

vendor: .cargo/config
	cargo vendor
	touch vendor

target/release/$(BIN): Cargo.lock Cargo.toml src/*.rs
	if [ -d vendor ]; \
	then \
		cargo build --release --frozen; \
	else \
		cargo build --release; \
	fi

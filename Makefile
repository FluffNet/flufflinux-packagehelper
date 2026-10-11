PREFIX ?= /usr
DESTDIR ?=
FAKEROOT ?= $(CURDIR)/fakeroot
BINARY := target/release/flufflinux-packagehelper

.PHONY: all build fakeroot install

all: fakeroot

build:
	cargo build --release

fakeroot: build
	$(MAKE) DESTDIR="$(FAKEROOT)" install
	install -Dm644 .PKGINFO "$(FAKEROOT)/.PKGINFO"
	install -Dm755 .INSTALL "$(FAKEROOT)/.INSTALL"

install:
	install -Dm755 "$(BINARY)" "$(DESTDIR)$(PREFIX)/bin/flufflinux-packagehelper"
	install -Dm644 flufflinux-packagehelper.desktop "$(DESTDIR)$(PREFIX)/share/applications/flufflinux-packagehelper.desktop"

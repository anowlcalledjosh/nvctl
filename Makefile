PREFIX ?= /usr/local
INSTALLDIR := $(PREFIX)/bin
FILE := target/release/nvctl

.PHONY: install uninstall

install: $(FILE)
	install -m 4755 $(FILE) $(INSTALLDIR)

uninstall:
	rm -f $(INSTALLDIR)/nvctl

$(FILE):
	cargo build --release

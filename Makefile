VERSION := $(shell fgrep 'const VERSION' src/main.rs | awk -F '"' '{ print $$2 }')
TOOLCHAIN := $(shell fgrep default_host_triple ~/.rustup/settings.toml | awk -F '"' '{ print $$2 }')

RELEASE_PRODUCT := target/release/pdf-urls


.PHONY: release
release: $(RELEASE_PRODUCT)

$(RELEASE_PRODUCT): src/*
	cargo build --release


.PHONY: doc
doc: doc/pdf-urls.1

doc/pdf-urls.1: doc/pdf-urls.1.txt
	a2x --no-xmllint --format manpage $<


.PHONY: dist-all
dist-all: dist/pdf-urls dist/pdf-urls.1

dist:
	mkdir -p dist

dist/pdf-urls: $(RELEASE_PRODUCT) dist
	cp $< $@

dist/pdf-urls.1: doc/pdf-urls.1 dist
	cp $< $@


.PHONY: pkg
pkg: pdf-urls_$(VERSION)_$(TOOLCHAIN).tar.bz2

pdf-urls_$(VERSION)_$(TOOLCHAIN).tar.bz2: dist-all
	tar cjv -s /dist/pdf-urls_$(VERSION)_$(TOOLCHAIN)/ -f $@ dist

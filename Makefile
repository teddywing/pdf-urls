.PHONY: doc
doc: doc/pdf-urls.1

doc/pdf-urls.1: doc/pdf-urls.1.txt
	a2x --no-xmllint --format manpage $<

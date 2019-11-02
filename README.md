PDF-URLs
========

Extract all URLs from a PDF file. Facilitates mouseless navigation.


## Examples
Hyperlinks are printed to standard output, one per line:

	$ pdf-urls ./testdata/Alice\'s\ Adventures\ in\ Wonderland.pdf
	http://www.gutenberg.org/ebooks/11
	https://science.nasa.gov/news-article/black-hole-image-makes-history
	https://ia800908.us.archive.org/6/items/alicesadventures19033gut/19033-h/images/i002.jpg

This allows them to be easily piped to other programs, like [urlview] or
[extract_url]:

	$ pdf-urls ./testdata/Alice\'s\ Adventures\ in\ Wonderland.pdf | urlview


## Install
On Mac OS X, PDF-URLs can be installed with Homebrew:

	$ brew install teddywing/formulae/pdf-urls

To compile from source or install on other platforms:

	$ cargo install --git https://github.com/teddywing/pdf-urls.git --root /usr/local


## Uninstall

	$ cargo uninstall --root /usr/local pdf-urls


## License
Copyright © 2019 Teddy Wing. Licensed under the GNU GPLv3+ (see the included
COPYING file).


[urlview]: https://github.com/sigpipe/urlview
[extract_url]: http://www.memoryhole.net/~kyle/extract_url/

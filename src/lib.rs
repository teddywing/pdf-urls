// Copyright (c) 2019  Teddy Wing
//
// This file is part of PDF-URLs.
//
// PDF-URLs is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// PDF-URLs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with PDF-URLs. If not, see <https://www.gnu.org/licenses/>.

mod errors;

extern crate lopdf;

use std::path::Path;
use std::str;
use std::string::String;
use std::vec::Vec;

use lopdf::{Document, Object};

use errors::Result;

/// Given a file path to a PDF, return a Vec of all URLs in the document.
pub fn get_urls_from_pdf<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let doc = Document::load(path)?;

    let mut urls = Vec::new();

    for (_, obj) in doc.objects {
        match obj {
            Object::Dictionary(d) => {
                for (k, v) in d.iter() {
                    let key = str::from_utf8(&k)?;

                    if object_is_link_annotation(key) {
                        let url_objects = v.as_dict()?;

                        for (k, v) in url_objects {
                            let key = str::from_utf8(&k)?;

                            if key == "URI" {
                                match v {
                                    Object::String(s, _) => {
                                        urls.push(String::from_utf8_lossy(s).to_string());
                                    },
                                    _ => (),
                                }
                            }
                        }
                    }
                }
            },
            _ => (),
        }
    }

    urls.dedup();

    Ok(urls)
}

/// Returns true if the given PDF object key is a link annotation.
fn object_is_link_annotation(key: &str) -> bool {
    key == "A"
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_urls_from_pdf_extracts_urls_from_pdf() {
        let expected = vec![
            "http://www.gutenberg.org/ebooks/11",
            "https://ia800908.us.archive.org/6/items/alicesadventures19033gut/19033-h/images/i002.jpg",
            "https://science.nasa.gov/news-article/black-hole-image-makes-history",
        ];

        let urls = get_urls_from_pdf("testdata/Alice's Adventures in Wonderland.pdf").unwrap();

        // Allow URLs to be out of order.
        for url in expected {
            assert!(urls.contains(&url.to_owned()));
        }
    }
}

mod errors;

extern crate lopdf;

use std::path::Path;
use std::str;
use std::string::String;
use std::vec::Vec;

use lopdf::{Document, Object};

use errors::Result;

pub fn get_urls_from_pdf<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let doc = Document::load(path)?;

    let mut urls = Vec::new();

    for (_, obj) in doc.objects {
        match obj {
            Object::Dictionary(d) => {
                for (k, v) in d.iter() {
                    let key = str::from_utf8(&k)?;

                    if key == "A" {
                        let url_objects = v.as_dict()?;

                        for (k, v) in url_objects {
                            let key = str::from_utf8(&k)?;

                            if key == "URI" {
                                match v {
                                    Object::String(s, _) => {
                                        urls.push(String::from_utf8(s.to_vec())?);
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

    Ok(urls)
}

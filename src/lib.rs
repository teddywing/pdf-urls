extern crate lopdf;

use std::path::Path;
use std::str;

use lopdf::{Document, Object};

pub fn get_urls_from_pdf<P: AsRef<Path>>(path: P) {
    let doc = Document::load(path).unwrap();

    for (_, obj) in doc.objects {
        match obj {
            Object::Dictionary(d) => {
                for (k, v) in d.iter() {
                    let key = str::from_utf8(&k).unwrap();

                    if key == "A" {
                        for (k, v) in v.as_dict().unwrap() {
                            let key = str::from_utf8(&k).unwrap();

                            if key == "URI" {

                                match v {
                                    Object::String(s, _) => {
                                        println!("{}", str::from_utf8(s).unwrap());

                                        ()
                                    },
                                    _ => (),
                                }
                            }
                        }
                    }
                }

                ()
            },
            _ => (),
        }
    }
}

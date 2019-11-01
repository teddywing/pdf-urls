mod errors;

extern crate lopdf;

use std::path::Path;
use std::str;

use lopdf::{Document, Object};

use errors::Result;

pub fn get_urls_from_pdf<P: AsRef<Path>>(path: P) -> Result<()> {
    let doc = Document::load(path)?;

    for (_, obj) in doc.objects {
        return match obj {
            Object::Dictionary(d) => {
                for (k, v) in d.iter() {
                    let key = str::from_utf8(&k)?;

                    if key == "A" {
                        let url_objects = v.as_dict()?;

                        for (k, v) in url_objects {
                            let key = str::from_utf8(&k)?;

                            if key == "URI" {

                                return match v {
                                    Object::String(s, _) => {
                                        println!("{}", str::from_utf8(s)?);

                                        Ok(())
                                    },
                                    _ => Ok(()),
                                }
                            }
                        }
                    }
                }

                Ok(())
            },
            _ => Ok(()),
        }
    }

    Ok(())
}

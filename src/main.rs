extern crate lopdf;

use lopdf::{Document, Object};

fn main() {
    let doc = Document::load("example.pdf").unwrap();
    // dbg!("{:?}", doc);

    for (id, obj) in doc.objects {
        // dbg!(obj.as_dict().unwrap());

        match obj {
            // Object::String(bs, format) => {
            //     dbg!(bs, format);
            //     ()
            // },
            Object::Dictionary(d) => {
                // dbg!(d);

                for (k, v) in d.iter() {
                    let key = ::std::str::from_utf8(&k).unwrap();

                    if key == "A" {
                        dbg!(v);

                        for (k, v) in v.as_dict().unwrap() {
                            let key = ::std::str::from_utf8(&k).unwrap();

                            // dbg!(key, v);
                            if key == "URI" {
                                dbg!(v);

                                match v {
                                    Object::String(s, _) => {
                                        dbg!(::std::str::from_utf8(s).unwrap());

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

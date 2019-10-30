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
                    dbg!(::std::str::from_utf8(&k).unwrap(), v);
                }

                ()
            },
            _ => (),
        }
    }
}

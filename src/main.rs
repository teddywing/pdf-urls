use std::path::Path;

use pdf_urls::get_urls_from_pdf;

fn main() {
    let path = Path::new("example.pdf");
    get_urls_from_pdf(&path);
}

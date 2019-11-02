use pdf_urls::get_urls_from_pdf;

fn main() {
    match get_urls_from_pdf("example.pdf") {
        Ok(_) => (),
        Err(err) => eprintln!("error: {}", err),
    }
}

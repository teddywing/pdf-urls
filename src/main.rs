extern crate exitcode;

use std::env;
use std::process;

use pdf_urls::get_urls_from_pdf;

fn print_usage() {
    println!("usage: pdf-urls FILE");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print_usage();

        process::exit(exitcode::USAGE);
    }

    match get_urls_from_pdf(&args[1]) {
        Ok(urls) => {
            for url in urls {
                println!("{}", url);
            }
        },
        Err(err) => eprintln!("error: {}", err),
    }
}

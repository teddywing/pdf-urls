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

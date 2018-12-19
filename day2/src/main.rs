extern crate common;
extern crate day2;

use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::process::exit;

fn main() {
    let ar: Vec<String> = args().collect();
    if ar.len() != 2 {
        eprintln!("Usage: {} <INPUT>", &ar[0]);
        exit(-1);
    }

    let input_file = File::open(&ar[1]).expect("Unable to open input file.");
    let words = common::read_lines(BufReader::new(input_file)).expect("Failed to read input.");
    let chksum = day2::checksum(&words);
    let similar = day2::locate_similar_packages(&words);
    println!(
        "Computed checksum: {}, similar packages: {:?}",
        chksum, similar
    );
}

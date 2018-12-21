extern crate common;
extern crate day3;

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
    let entries = common::read_lines(BufReader::new(input_file)).expect("Failed to read input.");
    let claims = day3::parse_claims(entries).expect("Failed to parse claims.");
    let bounds = day3::boundaries(&claims).expect("Expected to find boundaries.");
    let mut pixels = day3::DepthBuffer::create(bounds);

    for claim in &claims {
        pixels.add_claim(claim);
    }

    let count = pixels.count(1);

    println!("Found intersecting stuff: {}", count);
}

extern crate day1;

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
    let frequency =
        day1::compute_frequency(BufReader::new(input_file)).expect("Failed to compute frequency.");

    println!("Computed frequency: {}", frequency);
}

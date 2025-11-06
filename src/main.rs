use std::env;
use std::fs;

use crate::compress::Compress;

mod compress;
mod extract;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let mut file_contents =
        fs::read_to_string(file_path).expect("Yeah, that file could not be read.");
    file_contents.pop();

    let compr = Compress::new(file_contents.as_str());
    let compressed_str = compr.compress();

    println!("{compressed_str}");
}

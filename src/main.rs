#![forbid(unsafe_code)]

mod config;
mod entry;
mod write;

use crate::config::Config;
use crate::write::write;
use pico_args::Arguments;
use roxmltree::Document;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut args = Arguments::from_env();

    let input: String = args.value_from_str("-i").unwrap();
    let output: String = args.value_from_str("-o").unwrap();
    let config: String = args.value_from_str("-c").unwrap();

    let cfg = Config::new(read_to_string(config).unwrap().as_str());
    let xml = read_to_string(input).unwrap();
    let doc = Document::parse(&xml).unwrap();
    let root = doc.root_element();

    let mut file = File::create(output).unwrap();
    file.write_all(write(root, cfg).as_bytes()).unwrap();
}

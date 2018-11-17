extern crate clap;

use clap::{App, Arg};
use std::fs::read_to_string;
use std::fs::write;

mod lib;
use lib::set_crate_type;
use lib::CrateType;

fn main() {
    let matches = App::new("cargo crate-type")
        .version("1.0")
        .about("Edits the lib crate type to help building cross-platform libs")
        .arg(
            Arg::with_name("target")
                .required(true)
                .possible_values(&["dynamic", "static"]),
        ).arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .required(false)
                .default_value("Cargo.toml"),
        ).get_matches();

    let filename = matches.value_of("file").unwrap();
    let manifest = read_to_string(filename).expect("Not possible to read the given Cargo.toml");

    let target = match matches.value_of("target") {
        Some("static") => CrateType::Static,
        Some("dynamic") => CrateType::Dynamic,
        _ => return,
    };

    let new_manifest = set_crate_type(&manifest, target);
    write(filename, new_manifest).expect("Could not write to file");
}

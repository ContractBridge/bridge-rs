#[macro_use]
extern crate clap;
use clap::{crate_authors, crate_description, crate_version, crate_name, App};
extern crate bridge;

fn main() {
    let yaml = load_yaml!("cli/clap.yaml");
    let matches = App::from(yaml)
        .name(crate_name!())
        .version(crate_version!())
        .about(crate_description!()).author(crate_authors!(", "))
        .get_matches();

    if matches.is_present("deal") {
        println!("Deal!");
        bridge::deal();
    } else {
        println!("No deal!");
    }
}

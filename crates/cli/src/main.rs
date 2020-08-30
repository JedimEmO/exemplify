#[macro_use]
extern crate clap;

use clap::Clap;

use crate::layers::application::cli_params::ExemplifyCliParams;

mod layers;

fn main() {
    let params = ExemplifyCliParams::parse();
}

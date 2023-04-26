use crate::{cli::Args, pass::Pass};
use clap::Parser;

mod cli;
mod hash;
mod pass;

pub fn main() {
    let args = Args::parse();
    let pass = Pass::new(args.length);
    println!("{}", serde_json::to_string(&pass).unwrap());
}

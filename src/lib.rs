//! Easily generate random passwords of a given length,
//! with accomanying `sha-crypt` hash.
//!
//! # Installation
//!
//! ```bash
//! cargo install hardpass
//! ```
//!
//! # Usage
//!
//! ```bash
//! $ hardpass # default length is 30
//! {"plaintext":"ec5kdvqohdxNTRtS5GUUcsKgtE0uig","sha512":"$6$vINA2v96dp7xEn9L$mmnGVTuUiJf7ChxUq97lEKR2jXfI7tTH83SMI/T0dnVrHol9QDpOEp8kn1EKXRxRBxRSHncukCrbSSl7RpMaw."}
//! $ hardpass 15
//! {"plaintext":"n1gMiXa5oEAJkV0","sha512":"$6$lUHUWuYqrDncH0j5$BLmWjWj.yxuqOffII17RqRORzu.oIpaIt9sqaAh1XsC8JonTSwEUWwZ/3jtEoFfNOFzHEL5ru02fjH8GGpBAA0"}
//! ```
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

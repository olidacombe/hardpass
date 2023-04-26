use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Length of password to generate
    #[arg(default_value_t = 30)]
    pub length: usize,
}

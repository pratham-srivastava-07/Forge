mod cpp;

use clap::Parser;
use cpp::cli::{CPP, Commands};

fn main() {
    println!("Hello");

    let cpp = CPP::parse();

    match  cpp.commands {
        Commands::Build {  } => {},
        Commands::Init {  } => {},
        Commands::Run {  } => {}
    }
}
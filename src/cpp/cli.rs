use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct CPP {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Run{},
    Build{},
    Init{}
}
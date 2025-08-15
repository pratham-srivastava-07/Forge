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
    Init{
        project_name: String,
        
        #[arg(long = "type", default_value = "app")]
        template_type: String,

        #[arg(long, default_value = ".")]
        output_dir: String,
    }
}
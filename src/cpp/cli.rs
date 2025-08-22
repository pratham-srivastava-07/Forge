use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct CPP {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Run{
        project_name: String
    },
    Build{},
    Init{
        project_name: String,
        
        #[arg(long = "type", default_value = "app")]
        template_type: String,
    },  
    Install{}
}
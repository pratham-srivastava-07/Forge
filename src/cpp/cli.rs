use clap::{Parser, Subcommand, Args, ArgGroup};

#[derive(Parser)]
pub struct CPP {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Run {
        project_name: String,
    },
    Build {},
    Init(InitArgs),
    Install {
        library_name: String,
        repo_url: String,
    },
}

#[derive(Args)]
#[command(group(
    ArgGroup::new("template")
        .required(true)
        .args(&["app", "lib"]),
))]
pub struct InitArgs {
    pub project_name: String,

    /// Create an app template
    #[arg(long)]
    pub app: bool,

    /// Create a library template
    #[arg(long)]
    pub lib: bool,
}

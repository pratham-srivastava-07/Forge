pub mod commands;
pub mod utility;
mod cpp;
mod  template;
use clap::Parser;
use cpp::cli::{CPP, Commands};


fn main() {

    let cpp = CPP::parse();

    match  cpp.commands {
        Commands::Build {  } => {
            if let Err(e) = commands::build::build_project() {
                println!("Error building the project {}", e)
            } else {
                println!("Project built successfully!");
            }
        },  


       Commands::Init(args) => {
            let template_type = if args.app {
                "app"
            } else {
                "lib"
            };

            if let Err(e) = template::write_template(&args.project_name, template_type) {
                println!("Error generating template: {}", e)
            } else {
                println!("Project '{}' created successfully!", args.project_name);
            }
        }

        
        Commands::Run {project_name} => {
            if let Err(e) = commands::run::run_command(&project_name) {
                println!("Error running the project {}", e);
            } else {
                println!("Project ran successfully!");
            }
        }

        Commands::Install {library_name, repo_url} => {
            if let Err(e) = commands::install::install_command(&library_name, &repo_url) {
                println!("An error occured while installing the library {} {}", library_name, e);
            } else {
                println!("Library {} installed!!", library_name);
            }
        }
    }
}
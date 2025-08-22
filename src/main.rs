pub mod commands;
pub mod utility;
mod cpp;
mod  template;
use clap::Parser;
use cpp::cli::{CPP, Commands};


fn main() {
    println!("Hello");

    let cpp = CPP::parse();

    match  cpp.commands {
        Commands::Build {  } => {
            if let Err(e) = commands::build::build_project() {
                println!("Error building the project {}", e)
            } else {
                println!("Project built successfully!");
            }
        },


        Commands::Init { project_name, template_type} => {
            if let Err(e) = template::write_template(&project_name, &template_type) {
                println!("Error generating template: {}", e)
            } else {
                println!("Project '{}' created successfully!", project_name);
            }
        },

        
        Commands::Run {project_name} => {
            if let Err(e) = commands::run::run_command(&project_name) {
                println!("Error running the project {}", e);
            } else {
                println!("Project ran successfully!");
            }
        }

        Commands::Install {} => {}
    }
}
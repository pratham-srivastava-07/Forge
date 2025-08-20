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
        Commands::Build {  } => {},


        Commands::Init { project_name, template_type} => {
            if let Err(e) = template::write_template(&project_name, &template_type) {
                println!("Error generating template: {}", e)
            } else {
                println!("Project '{}' created successfully!", project_name);
            }
        },

        
        Commands::Run {} => {}

        Commands::Install {} => {}
    }
}
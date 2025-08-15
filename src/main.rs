mod cpp;
mod  template;
use clap::Parser;
use cpp::cli::{CPP, Commands};


fn main() {
    println!("Hello");

    let cpp = CPP::parse();

    match  cpp.commands {
        Commands::Build {  } => {},
        Commands::Init { project_name, template_type, output_dir } => {
            if let Err(e) = template::write_template(&project_name, &template_type, &output_dir) {
                println!("Error generating template: {}", e)
            } else {
                println!("Project '{}' created successfully!", project_name);
            }
        },
        Commands::Run {  } => {}
    }
}
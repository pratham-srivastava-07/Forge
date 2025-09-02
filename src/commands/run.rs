use crate::commands::build::{build_project};
use std::{env, process::Command};
use crate::commands::install::{detect_template, Template};
use crate::utility::helper::find_executable;


pub fn run_command(_project_name: &str) -> std::io::Result<()> {
    let cwd = env::current_dir()?;

    let template = detect_template(&cwd)?;
    let project_dir = match template {
        Template::App => cwd.join("app"),
        Template::Lib => cwd.join("lib"),
    };
    let build_dir = project_dir.join("build");

    println!("Ensuring project is built...");
    let _ = build_project(); 

    let exe_path = find_executable(&build_dir)?;

    if let Some(exe) = exe_path {
        println!("Running {:?}", exe);

        let status = Command::new(&exe)
            .current_dir(&build_dir)
            .status()?;

        if !status.success() {
            println!("Program exited with errors");
        }
    } else {
        println!("Executable not found even after build!");
    }

    Ok(())
}
use crate::commands::build;
use std::{env, process::Command};


pub fn run_command(project_name: &str) -> std::io::Result<()> {
    let cwd = env::current_dir()?;
    let build_dir = cwd.join("build");

    // build the project

    let _ = build::build_project();

    let executables = build_dir.join(project_name);
    println!("Executable path {:?}", executables);
    if !executables.exists() {
        println!("Executable path does not exist");
        return Ok(())
    }

    let status = Command::new(&executables)
        .status()?;

    if !status.success() {
        println!("Program exited with errors");
    }

    Ok(())

}
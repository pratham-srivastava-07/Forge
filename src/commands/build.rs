use std::{env, fs, process::Command};
use crate::utility::helper;
use crate::commands::install::{Template, detect_template};


pub fn build_project() -> std::io::Result<()> {
    let cwd = env::current_dir()?;

    // detect template type (App or Lib)
    let template = detect_template(&cwd)?;

    // resolve project dir based on template
    let project_dir = match template {
        Template::App => cwd.join("app"),
        Template::Lib => cwd.join("lib"),
    };

    let build_directory = project_dir.join("build");
    if !build_directory.exists() {
        fs::create_dir_all(&build_directory)?;
    }

    // check for dependencies
    if !helper::check_for_dependencies() {
        return Ok(());
    }

    // configure step
    let status = Command::new("cmake")
        .arg("..")
        .current_dir(&build_directory)
        .status()?;

    if !status.success() {
        println!("CMake configuration failed");
        return Ok(());
    }

    // build step
    let status = Command::new("cmake")
        .arg("--build")
        .arg(".")
        .current_dir(&build_directory)
        .status()?;

    if !status.success() {
        println!("Build failed");
        return Ok(());
    }

    println!("Build finished successfully");
    Ok(())
}
use std::{env, fs, process::Command};
use crate::utility::helper;


pub fn build_project() -> std::io::Result<()> {

    // getting current dir and inserting build dir
    let cwd = env::current_dir()?;
    let build_directory = cwd.join("build");

    //if build dir does not exist, create one
    if !build_directory.exists() {
        let _ = fs::create_dir_all(&build_directory);
    }

    // checking for dependencies
    if !helper::check_for_dependencies() {
        return Ok(());
    }

    // running cmake after making sure all the dependencies are present
    let status = Command::new("cmake").arg("..").current_dir(&build_directory).status()?;

    if !status.success() {
        println!("CMake configration failed");
        return Ok(());
    }

    println!("Build finished successfully");

    Ok(())
}

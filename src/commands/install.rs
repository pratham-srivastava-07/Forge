use std::path::PathBuf;
use std::process::Command;
use std::env;
use std::io;
use crate::utility::helper::update_cmake;
pub enum Template {
    App,
    Lib
}

pub fn detect_template(project_root: &PathBuf) -> io::Result<Template> {
    if project_root.join("app").exists() {
        Ok(Template::App)
    } else if project_root.join("lib").exists() {
        Ok(Template::Lib)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "No app/ or lib/ directory found"))
    }
}


pub fn install_command(library_name: &str, repo_url: &str) -> io::Result<()> {
    let project_root = env::current_dir()?;
    let template = detect_template(&project_root)?;

    let include_path = match template {
        Template::App => project_root.join("app/include").join(library_name),
        Template::Lib => project_root.join("lib/include").join(library_name),
    };

    if include_path.exists() {
        println!("{} already exists", library_name);
        return Ok(());
    }

    println!("Installing {}", library_name);

    let status = Command::new("git")
        .arg("clone")
        .arg("--depth=1")
        .arg(repo_url)
        .arg(&include_path)
        .status()?;

    if !status.success() {
        println!("Could not install the library {}", library_name);
        return Ok(());
    }

    // check if library has CMakeLists.txt
    let has_cmake = include_path.join("CMakeLists.txt").exists();

    // update project CMakeLists
    update_cmake(&project_root, library_name, has_cmake)?;

    Ok(())
}
use std::process::Command;


struct Dependencies {
    gcc: bool,
    clang: bool,
    msvc: bool, 
    cmake: bool
}

impl Dependencies {
    fn check_for_tools(tool: &str, args: &[&str]) -> bool {
        Command::new(tool).args(args).output().map(|out| out.status.success()).unwrap_or(false)
    }

    fn check() -> Self {
        Dependencies {
            gcc: Self::check_for_tools("gcc", &["--version"]),
            clang: Self::check_for_tools("clang", &["--version"]),
            msvc: Self::check_for_tools("cl", &["/?"]),
            cmake: Self::check_for_tools("cmake", &["--version"]),
        }
    }

    fn validate(&self) -> bool {
        let mut everything_present = true;

        if !self.gcc {
            println!("GCC compiler does not exist");
            everything_present = false;
        } else {
            println!("GCC found!!");
        }

        if !self.msvc {
            println!("MSVC missing!");
            everything_present = false;
        } else {
            println!("MSVC present");
        }

        if !self.clang {
            println!("Clang is not detected!");
            everything_present = false;
        } else {
            println!("Clang found");
        }

        if !self.cmake {
            println!("CMake not detected!");
            everything_present = false;
        } else {
            println!("CMake found!");
        }

        everything_present

    }
}

pub fn check_for_dependencies() -> bool {
    let dependencies = Dependencies::check();

    dependencies.validate()
}
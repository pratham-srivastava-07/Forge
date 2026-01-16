# Oxibuild

A powerful CLI tool to initialize, build, run, and manage C++ projects.

## Features

- **Init**: Create new C++ applications or libraries with a standard template.
- **Build**: Build your C++ projects easily.
- **Run**: Execute your C++ applications.
- **Install**: Install C++ libraries from remote repositories.

## Installation

```bash
cargo install forge
```

## Usage

### Initialize a new project

```bash
oxibuild init --project_name my_app --app  # Create an application
oxibuild init --project_name my_lib --lib  # Create a library
```

### Build the project

```bash
oxibuild build
```

### Run the project

```bash
oxibuild run --project_name my_app
```

### Install a library

```bash
oxibuild install --library_name external_lib --repo_url https://github.com/user/repo
```

## License

MIT

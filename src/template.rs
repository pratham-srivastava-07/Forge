use rust_embed::RustEmbed;
use std::{fs, env};

#[derive(RustEmbed)]
#[folder="templates/"]
struct Asset;

pub fn write_template(
    project_name: &str,
    template_type: &str,
) -> std::io::Result<()> {
    // Current working directory
    let cwd = env::current_dir()?;
    let project_root = cwd.join(project_name);

    for file in Asset::iter() {
        let path = file.as_ref();

        if path.starts_with(template_type) {
            let content = Asset::get(path).unwrap();
            let mut text =
                String::from_utf8_lossy(content.data.as_ref()).to_string();

            text = text.replace("{{project_name}}", project_name);

            
            let relative_path = path.strip_prefix(template_type).unwrap();
            let target_path = project_root.join(relative_path);

            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(target_path, text)?;
        }
    }

    Ok(())
}

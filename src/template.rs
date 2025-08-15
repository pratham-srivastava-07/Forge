use rust_embed::RustEmbed;
use std::{fs, path::Path};

#[derive(RustEmbed)]
#[folder="templates/"]
struct Asset;

pub fn write_template(project_name: &str, template_type: &str, output_dir: &str) -> std::io::Result<()> {
    for file in Asset::iter() {
        let path = file.as_ref();
        if path.starts_with(template_type) {
            let content = Asset::get(path).unwrap();
            let mut text = String::from_utf8_lossy(content.data.as_ref()).to_string();
            text = text.replace("{{project_name}}", project_name);

            let target_path = Path::new(output_dir).join(project_name).join(path.strip_prefix(template_type).unwrap());
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(target_path, text)?;
        }
    }
    Ok(())
}
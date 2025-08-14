use rust_embed::RustEmbed;
use std::{fs, path::Path};

#[derive(RustEmbed)]
#[folder="templates/"]
struct Asset;


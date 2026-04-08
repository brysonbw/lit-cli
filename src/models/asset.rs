use rust_embed::RustEmbed;

/// Embedded assets (like template files) compiled into the binary using the RustEmbed crate
#[derive(RustEmbed)]
#[folder = "templates/"]
pub struct Asset;

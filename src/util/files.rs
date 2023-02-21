use std::fs;

pub fn create_directory(filepath: &str) -> std::io::Result<()> {
    fs::create_dir_all(filepath)?;
    Ok(())
}

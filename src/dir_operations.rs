use std::{env, fs};

pub(crate) fn parent_dir() -> String {
    let mut current_dir: std::path::PathBuf = env::current_dir().unwrap();
    
    while let Some(parent) = current_dir.parent() {
        current_dir = parent.to_path_buf();
    }
    
    println!("Root directory path: {}", current_dir.display());
    current_dir.display().to_string()
}

pub(crate) fn is_dir(path: &String) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        return metadata.is_dir();
    } 
    false
}
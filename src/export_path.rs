use std::{path::PathBuf, sync::Arc};
use arguments::Args;
use std::fs;
use crate::arguments;

pub fn get_export_path(args: &Args) -> Arc<PathBuf> {
    let default_path: PathBuf = std::env::current_dir().unwrap().join("mysql-migrator");
    let export_path: PathBuf = args.export_path
        .clone()
        .map(PathBuf::from)
        .unwrap_or(default_path);
    let export_path_str: &str = export_path.to_str().unwrap();
    let mut export_path: PathBuf = PathBuf::new();
    export_path.push(export_path_str);
    return export_path.into();
}

pub fn create_export_dir(args: &Args, export_path: &PathBuf) {
    if !export_path.exists() {
        fs::create_dir_all(export_path).expect("Failed to create data directory");
    } else {
        if args.clean {
            fs::remove_dir_all(&export_path).expect("Failed to clear data directory");
            fs::create_dir_all(&export_path).expect("Failed to create data directory");
        }
    }
}
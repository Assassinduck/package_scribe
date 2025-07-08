mod types;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("Directory is empty")]
    EmptyDirectory,
    #[error("Directory does not contain package.json")]
    NoPackageJson,
    #[error("No package was found")]
    NoPackageFoundInApi,
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

use crate::types::PackageJson;

#[tokio::main]
#[tauri::command]
async fn get_pakage_info(package_name: &str, version: &str) -> Result<PackageJson, Error> {
    let request_url = format!(
        "https://registry.npmjs.org/{name}/{versionNum}",
        name = package_name,
        versionNum = version
    );

    let client = reqwest::Client::new();
    let res = client.get(request_url).send().await?;

    let package: PackageJson = res.json().await?;

    Ok(package)
}

#[tauri::command]
fn find_package_json(name: &str) -> Result<String, Error> {
    let dir_entries = fs::read_dir(name)?
        .map(|res| res.map(|e| e.file_name().to_string_lossy().to_string()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    const PACKAGE_NAME: &str = "package.json";

    if dir_entries.is_empty() {
        return Err(Error::EmptyDirectory);
    }

    if dir_entries.contains(&PACKAGE_NAME.to_string()) {
        return Result::Ok("its here".to_string());
    }

    Result::Err(Error::NoPackageJson)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![find_package_json, get_pakage_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

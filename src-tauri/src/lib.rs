mod types;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
use std::fs::{self, DirEntry, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::{env, io};

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
    #[error("something went wrong using the fime system")]
    StdError,
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

use crate::types::{Deps, PackageJson};
use crate::types::PackageObject;

#[tokio::main]
async fn get_package_info(package_name: &str, version: &str) -> Result<PackageJson, Error> {
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

#[tokio::main]
async fn create_package_object() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_dir()?;
    let file = path.join("package.json");
    let f = File::open(file)?;
    let reader = BufReader::new(f);

    let package_json: Option<PackageJson> = serde_json::from_reader(reader)?;
    let mut package_array: Vec<PackageObject> = Vec::new();
    let mut dev_dep_array: Vec<PackageObject> = Vec::new();

    if let Some(pkage) = package_json {
        if let Some(dependencies) = pkage.dependencies {
            for (package, version) in dependencies {
                let new_package = PackageObject { package, version };
                package_array.push(new_package);
            }
        }
        if let Some(devdep) = pkage.dev_dependencies {
            for (package, version) in devdep {
                let new_package: PackageObject = PackageObject { package, version };
                dev_dep_array.push(new_package);
            }
        }
    }

    package_array.sort_by(|a, b| a.package.cmp(&b.package));

    dev_dep_array.sort_by(|a, b| a.package.cmp(&b.package));

    let dep = Deps {
        deps: &package_array,
        devdeps: &dev_dep_array,
    };

    println!("Deps: {:#?}", &dep.deps);
    println!("DevDeps: {:#?}", &dep.devdeps);

    Ok(())
}

#[tauri::command]
fn find_package_json(path_name: &str) -> Result<String, Error> {
    let dir_entries: Vec<String> = fs::read_dir(path_name)?
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
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![find_package_json])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

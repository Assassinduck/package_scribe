mod npm;
mod types;

use regex::Regex;
use reqwest::Client;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use crate::npm::NpmPackageUnspecifiedVersion;
use crate::types::PackageJsonParsedObject;
use crate::types::{DepsWithMetadata, PackageJson, PackageVersionInfo, ParsedJsonDeps};
use node_semver::{Range, Version};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::PathBuf;
use std::{env, io};
use tokio::task::JoinHandle;

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
    #[error("bad package version found")]
    BadVersion,
    #[error(transparent)]
    Regex(#[from] regex::Error),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

async fn get_package_info(package_name: &str, version: &str, client: &Client) -> Result<(), Error> {
    let valid_version: Vec<&str> = version
        .split(|c| c == '^' || c == '~')
        .filter(|str| !str.is_empty())
        .collect();
    let split_version_num = valid_version.first().unwrap();

    let version_semver: Version = split_version_num.parse().unwrap();

    //let range: Range = version.parse().unwrap();

    
    let package_in_registry_req =
        format!("https://registry.npmjs.org/{name}/", name = package_name,);

    let registry_response = client.get(package_in_registry_req).send().await?;

    let npmPackage: Result<NpmPackageUnspecifiedVersion, reqwest::Error> =
        registry_response.json().await;

    let package_version = match npmPackage {
        Ok(package) => {
            let current_version = package.versions.get(split_version_num.to_owned());

            let current_latest_patch = package
                .versions
                .into_iter()
                .find(|(str, version_obj)| -> bool {
                        let version: Version =  version_obj.version.parse().unwrap();
                        version.major == version_semver.major &&
                        version.minor  == version_semver.minor &&
                        version.patch > version_semver.patch
                     });

            let latest = package.versions.get(&package.dist_tags.latest);

            let pvi = PackageVersionInfo {
                current: current_version,
                latest_patch: current_latest_patch,
                latest: latest_version,
                error: 
            };
        }
        Err(err) => {}
    };

    //let a: NpmPackageResponse = client.get(&latest_version).send().await?.json().await?;

    //println!("{:#?}", a);

    
    Ok(())
}

async fn flatten(
    handle: JoinHandle<Result<reqwest::Response, reqwest::Error>>,
) -> Result<reqwest::Response, Error> {
    match handle.await {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(reqwest_err)) => Err(Error::Reqwest(reqwest_err)),
        Err(_join_err) => Err(Error::StdError),
    }
}

fn create_package_object(
    path_name: &PathBuf,
) -> Result<ParsedJsonDeps, Box<dyn std::error::Error>> {
    let f = File::open(path_name)?;
    let reader = BufReader::new(f);

    let package_json: Option<PackageJson> = serde_json::from_reader(reader)?;
    let mut package_array: Vec<PackageJsonParsedObject> = Vec::new();
    let mut dev_dep_array: Vec<PackageJsonParsedObject> = Vec::new();

    if let Some(pkage) = package_json {
        if let Some(dependencies) = pkage.dependencies {
            for (package, version) in dependencies {
                let new_package = PackageJsonParsedObject { package, version };
                package_array.push(new_package);
            }
        }
        if let Some(devdep) = pkage.dev_dependencies {
            for (package, version) in devdep {
                let new_package: PackageJsonParsedObject =
                    PackageJsonParsedObject { package, version };
                dev_dep_array.push(new_package);
            }
        }
    }

    package_array.sort_by(|a, b| a.package.cmp(&b.package));

    dev_dep_array.sort_by(|a, b| a.package.cmp(&b.package));

    let dep = ParsedJsonDeps {
        deps: package_array,
        dev_deps: dev_dep_array,
    };

    Ok(dep)
}

#[tauri::command]
async fn find_package_json(path_name: &str) -> Result<(), Error> {
    let dir_entries = fs::read_dir(path_name)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    const PACKAGE_NAME: &str = "package.json";

    if dir_entries.is_empty() {
        return Err(Error::EmptyDirectory);
    }

    if let Some(package_path) = dir_entries.iter().find(|path| {
        path.file_name()
            .and_then(|name| Some(name.to_string_lossy().to_string()))
            .map(|name| -> bool { name == PACKAGE_NAME })
            .unwrap_or(false)
    }) {
        let package_vec = create_package_object(package_path).map_err(|_| Error::StdError)?;
        let client = reqwest::Client::new();

        let mut package_with_metadata = Vec::<PackageVersionInfo>::new();
        let mut dev_dep_package_with_metadata = Vec::<PackageVersionInfo>::new();
        for pack in package_vec.deps {
            let package = get_package_info(&pack.package, &pack.version, &client).await;

            /*  match package {
                Ok(pack) => package_with_metadata.push(pack),
                Err(err) => {
                    println!("{:#?}", &pack.package);
                    println!("{:#?}", &pack.version);
                    println!("{:#?}", &pack);
                    println!("{:#?}", err);
                    continue;
                }
            } */
        }

        /*   for pack in package_vec.dev_deps {
                   let dev_pack = get_package_info(&pack.package, &pack.version, &client).await;

                   match dev_pack {
                       Ok(pack) => dev_dep_package_with_metadata.push(pack),
                       Err(_) => continue,
                   }
               }
               let packages_with_metadata: DepsWithMetadata = DepsWithMetadata {
                   deps: package_with_metadata,
                   dev_deps: dev_dep_package_with_metadata,
               };
               //println!("Deps: {:#?}", packages_with_metadata);
        */
        return Ok(());
    } else {
        return Result::Err(Error::NoPackageJson);
    }
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

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageJson {
    pub name: String,
    #[serde(rename = "private")]
    pub is_private: bool,
    pub version: String,
    #[serde(rename = "type")]
    pub package_type: Option<String>,
    pub scripts: Option<IndexMap<String, String>>,
    pub dependencies: Option<IndexMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<IndexMap<String, String>>,
    #[serde(rename = "packageManager")]
    pub package_manager: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bugs {
    pub url: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageObject {
    pub(crate) package: String,
    pub(crate) version: String,
}

pub struct Deps<'a> {
    pub(crate) deps: &'a Vec<PackageObject>,
    pub(crate) devdeps: &'a Vec<PackageObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PnpmConfig {
    pub overrides: Option<IndexMap<String, String>>,
    #[serde(rename = "onlyBuiltDependencies")]
    pub only_built_dependencies: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dist {
    pub integrity: String,
    pub shasum: String,
    pub tarball: String,
    #[serde(rename = "fileCount")]
    pub file_count: Option<u32>,
    #[serde(rename = "unpackedSize")]
    pub unpacked_size: Option<u64>,
    pub attestations: Option<serde_json::Value>,
    pub signatures: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NpmUser {
    pub name: String,
    pub email: String,
    pub actor: Option<Actor>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Actor {
    pub name: String,
    pub email: String,
    #[serde(rename = "type")]
    pub actor_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Maintainer {
    pub name: String,
    pub email: String,
}

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::npm::{NpmPackageMetadataSpecificVersion, VersionsObj};

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageJson {
    pub name: String,
    #[serde(rename = "private")]
    pub is_private: Option<bool>,
    pub version: String,
    #[serde(rename = "type")]
    pub package_type: Option<String>,
    pub scripts: Option<IndexMap<String, String>>,
    pub dependencies: Option<IndexMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<IndexMap<String, String>>,
    #[serde(rename = "packageManager")]
    pub package_manager: Option<String>,
    #[serde(flatten)]
    pub extras: Option<IndexMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bugs {
    pub url: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageJsonParsedObject {
    pub package: String,
    pub version: String,
}
#[derive(Debug, Serialize, Deserialize)]

pub struct ParsedJsonDeps {
    pub deps: Vec<PackageJsonParsedObject>,
    pub dev_deps: Vec<PackageJsonParsedObject>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DepsWithMetadata {
    pub deps: Vec<PackageVersionInfo>,
    pub dev_deps: Vec<PackageVersionInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageVersionInfo {
    pub current: Option<VersionsObj>,
    pub latest: Option<VersionsObj>,
    pub latest_patch: Option<VersionsObj>,
    pub latest_minor: Option<VersionsObj>,
    pub latest_major: Option<VersionsObj>,
    pub error: Option<String>,
}

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageJson {
    pub name: String,
    pub description: Option<String>,
    pub license: Option<String>,
    pub version: String,
    #[serde(rename = "type")]
    pub package_type: Option<String>,
    pub types: Option<String>,
    pub engines: Option<HashMap<String, String>>,
    pub module: Option<String>,
    pub main: Option<String>,
    pub exports: Option<serde_json::Value>,
    pub imports: Option<serde_json::Value>,
    pub repository: Option<Repository>,
    pub bugs: Option<Bugs>,
    pub homepage: Option<String>,
    pub keywords: Option<Vec<String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<HashMap<String, String>>,
    pub dependencies: Option<HashMap<String, String>>,
    pub scripts: Option<HashMap<String, String>>,
    #[serde(rename = "_id")]
    pub id: Option<String>,
    #[serde(rename = "_integrity")]
    pub integrity: Option<String>,
    #[serde(rename = "_resolved")]
    pub resolved: Option<String>,
    #[serde(rename = "_from")]
    pub from: Option<String>,
    #[serde(rename = "_nodeVersion")]
    pub node_version: Option<String>,
    #[serde(rename = "_npmVersion")]
    pub npm_version: Option<String>,
    pub dist: Option<Dist>,
    #[serde(rename = "_npmUser")]
    pub npm_user: Option<NpmUser>,
    pub directories: Option<serde_json::Value>,
    pub maintainers: Option<Vec<Maintainer>>,
    #[serde(rename = "_npmOperationalInternal")]
    pub npm_operational_internal: Option<serde_json::Value>,
    #[serde(rename = "_hasShrinkwrap")]
    pub has_shrinkwrap: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Repository {
    #[serde(rename = "type")]
    pub repo_type: String,
    pub url: String,
    pub directory: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bugs {
    pub url: String,
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
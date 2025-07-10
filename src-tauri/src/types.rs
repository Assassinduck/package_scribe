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
    pub url: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageObject {
    pub(crate) package: String,
    pub(crate) version: String,
}

pub struct Deps {
    pub(crate) deps: Vec<PackageObject>,
    pub(crate) dev_deps: Vec<PackageObject>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DepsWithMetadata {
    pub(crate) deps: Vec<PackageVersionInfo>,
    pub(crate) dev_deps: Vec<PackageVersionInfo>,
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
    #[serde(rename = "npm-signature")] // Add this field
    pub npm_signature: Option<String>,
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MaintainerField {
    Single(Maintainer),
    Multiple(Vec<Maintainer>),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PackageVersionInfo {
    pub current: Option<NpmPackageResponse>,
    pub latest: Option<NpmPackageResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NpmPackageResponse {
    pub name: String,
    pub version: String,
    pub keywords: Option<Vec<String>>,
    pub license: Option<String>,
    #[serde(rename = "_id")]
    pub id: String,
    pub maintainers: Option<MaintainerField>,
    pub homepage: Option<String>,
    pub bugs: Option<Bugs>,
    pub dist: Option<Dist>, // Make optional,
    pub icon: Option<String>,
    #[serde(rename = "gitHead")]
    pub git_head: Option<String>, // Add gitHead field
    pub scripts: Option<IndexMap<String, String>>, // Add scripts field
    pub main: Option<String>,
    pub engines: Option<IndexMap<String, String>>,
    pub exports: Option<serde_json::Value>,
    #[serde(rename = "_npmUser")]
    pub npm_user: NpmUser,
    pub browserify: Option<serde_json::Value>,
    pub repository: Option<Repository>,
    #[serde(rename = "_npmVersion")]
    pub npm_version: Option<String>,
    pub description: Option<String>,
    pub directories: Option<serde_json::Value>,
    #[serde(rename = "_nodeVersion")]
    pub node_version: Option<String>,
    pub dependencies: Option<IndexMap<String, String>>,
    #[serde(rename = "_hasShrinkwrap")]
    pub has_shrinkwrap: Option<bool>,
    #[serde(rename = "_npmOperationalInternal")]
    pub npm_operational_internal: Option<NpmOperationalInternal>,
    pub contributors: Option<Vec<serde_json::Value>>,
    pub author: Option<AuthorField>, // Add author field
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Repository {
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub repo_type: String,
    pub directory: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NpmOperationalInternal {
    pub tmp: String,
    pub host: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NpmRegistryResponse {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_rev")]
    pub rev: String,
    pub name: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: DistTags,
    pub versions: IndexMap<String, PackageVersion>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DistTags {
    pub latest: String,
    pub beta: Option<String>,
    pub rc: Option<String>,
    pub experimental: Option<String>,
    pub next: Option<String>,
    pub canary: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageVersion {
    pub name: String,
    pub version: String,
    pub author: Option<AuthorField>,
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub maintainers: Option<MaintainerField>,
    pub contributors: Option<ContributorsField>,
    pub bugs: Option<Bugs>,
    pub dist: Dist,
    pub main: Option<String>,
    pub engines: Option<IndexMap<String, String>>,
    pub exports: Option<serde_json::Value>,
    #[serde(rename = "_npmUser")]
    pub npm_user: NpmUser,
    pub licenses: Option<Vec<License>>,
    pub license: Option<String>,
    pub repository: Option<Repository>,
    #[serde(rename = "_npmVersion")]
    pub npm_version: Option<String>,
    pub description: Option<String>,
    pub directories: Option<serde_json::Value>,
    #[serde(rename = "_nodeVersion")]
    pub node_version: Option<String>,
    pub dependencies: Option<IndexMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<IndexMap<String, String>>,
    #[serde(rename = "_defaultsLoaded")]
    pub defaults_loaded: Option<bool>,
    #[serde(rename = "_engineSupported")]
    pub engine_supported: Option<bool>,
    #[serde(rename = "_hasShrinkwrap")]
    pub has_shrinkwrap: Option<bool>,
    #[serde(rename = "_npmOperationalInternal")]
    pub npm_operational_internal: Option<NpmOperationalInternal>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
    pub url: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AuthorField {
    Object(Author),
    String(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct License {
    #[serde(rename = "type")]
    pub license_type: String,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LicenseField {
    Single(License),
    Multiple(Vec<License>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ContributorsField {
    Single(serde_json::Value),
    Multiple(Vec<serde_json::Value>),
}

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Maintainer {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MaintainerField {
    Single(Maintainer),
    Multiple(Vec<Maintainer>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bugs {
    pub url: Option<String>,
    #[serde(flatten)]
    pub extras: Option<IndexMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EnginesField {
    Object(IndexMap<String, String>),
    Array(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Engines {
    #[serde(flatten)]
    pub engines: Option<IndexMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Repository {
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub repo_type: String,
    pub directory: Option<String>,
    #[serde(flatten)]
    pub extras: Option<IndexMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ContributorsField {
    Single(Author),
    Multiple(Vec<Author>),
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
pub struct NpmPackageMetadataSpecificVersion {
    pub name: String,
    pub version: String,
    pub author: Option<AuthorField>,
    pub maintainers: Option<MaintainerField>,
    pub contributors: Option<ContributorsField>,
    pub bugs: Option<Bugs>,
    pub engines: Option<EnginesField>,
    pub keywords: Option<Vec<String>>,
    pub license: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<Repository>,
    pub description: Option<String>,
    #[serde(rename = "_nodeVersion")]
    pub node_version: Option<String>,
    pub dependencies: Option<IndexMap<String, serde_json::Value>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<IndexMap<String, serde_json::Value>>,
    #[serde(rename = "peerDependencies")]
    pub peer_dependencies: Option<IndexMap<String, serde_json::Value>>,
    #[serde(rename = "optionalDependencies")]
    pub optional_dependencies: Option<IndexMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub extras: Option<IndexMap<String, serde_json::Value>>,
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
pub struct VersionsObj {
    pub name: String,
    pub version: String,
    pub author: Option<AuthorField>,
    pub maintainers: Option<MaintainerField>,
    pub contributors: Option<ContributorsField>,
    pub bugs: Option<Bugs>,
    pub engines: Option<EnginesField>,
    pub licenses: Option<Vec<License>>,
    pub license: Option<String>,
    #[serde(rename = "_npmVersion")]
    pub npm_version: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "_nodeVersion")]
    pub node_version: Option<String>,
    pub dependencies: Option<IndexMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<IndexMap<String, String>>,
    #[serde(rename = "_engineSupported")]
    pub engine_supported: Option<bool>,
    #[serde(flatten)]
    pub extras: Option<IndexMap<String, serde_json::Value>>,
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
pub struct NpmPackageUnspecifiedVersion {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_rev")]
    pub rev: String,
    pub name: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: DistTags,
    pub versions: IndexMap<String, VersionsObj>,
    #[serde(flatten)]
    pub extras: Option<IndexMap<String, serde_json::Value>>,
}

use std::path::Path;
use std::str::FromStr;

use serde::Deserialize;

use crate::map_helper::MapKey;
use crate::{Class, Enumerator, Error, Function, Global, Map, Protocol, Tag, Typedef};

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[non_exhaustive]
pub struct ApiNotes {
    pub name: String,
    #[serde(default)]
    #[serde(with = "crate::map_helper")]
    pub swift_versions: Map<Version, Data>,
    #[serde(flatten)]
    pub data: Data,
}

impl ApiNotes {
    pub fn from_path(path: &Path) -> Result<Self, Error> {
        let contents = std::fs::read_to_string(path).expect("todo");

        Self::from_str(&contents)
    }
}

impl FromStr for ApiNotes {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::from_yaml)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
#[non_exhaustive]
pub enum Version {
    V3,
    V4,
    V4_2,
    V5,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct VersionHelper {
    version: f32,
}

impl<'de> MapKey<'de> for Version {
    type Inner = VersionHelper;
    fn from_inner(inner: Self::Inner) -> Self {
        if inner.version == 3.0 {
            Version::V3
        } else if inner.version == 4.0 {
            Version::V4
        } else if inner.version == 4.2 {
            Version::V4_2
        } else if inner.version == 5.0 {
            Version::V5
        } else {
            panic!("unknown version {}", inner.version)
        }
    }
    const CONTAIN_ERROR: &'static str = "a `Version` attribute";
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
#[non_exhaustive]
pub struct Data {
    #[serde(with = "crate::map_helper")]
    pub classes: Map<String, Class>,
    #[serde(with = "crate::map_helper")]
    pub protocols: Map<String, Protocol>,
    #[serde(with = "crate::map_helper")]
    pub tags: Map<String, Tag>,
    #[serde(with = "crate::map_helper")]
    pub typedefs: Map<String, Typedef>,
    #[serde(with = "crate::map_helper")]
    pub globals: Map<String, Global>,
    #[serde(with = "crate::map_helper")]
    pub enumerators: Map<String, Enumerator>,
    #[serde(with = "crate::map_helper")]
    pub functions: Map<String, Function>,
}

use serde::Deserialize;

use crate::map_helper::MapKey;
use crate::{General, Map, Nullability, Type};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum Kind {
    Instance,
    Class,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MethodKey {
    pub selector: String,
    #[serde(rename = "MethodKind")]
    pub kind: Kind,
}

impl<'de> MapKey<'de> for MethodKey {
    type Inner = Self;
    fn from_inner(inner: Self::Inner) -> Self {
        inner
    }
    const CONTAIN_ERROR: &'static str = "a `Selector` and a `MethodKind` attribute";
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropertyKey {
    pub name: String,
    /// `None` means that the property signifies both a class and an instance
    /// property.
    #[serde(rename = "PropertyKind")]
    #[serde(default)]
    pub kind: Option<Kind>,
}

impl<'de> MapKey<'de> for PropertyKey {
    type Inner = Self;
    fn from_inner(inner: Self::Inner) -> Self {
        inner
    }
    const CONTAIN_ERROR: &'static str = "a `Name` and a `PropertyKind` attribute";
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
#[non_exhaustive]
pub struct Method {
    #[serde(flatten)]
    pub general: General,
    /// Note: May not work due to compiler bugs
    pub nullability_of_ret: Option<Nullability>,
    pub nullability: Option<Vec<Nullability>>,
    pub result_type: Option<Type>,
    pub designated_init: bool,
    #[serde(with = "crate::map_helper")]
    pub parameters: Map<Position, Parameter>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
#[non_exhaustive]
pub struct Property {
    #[serde(flatten)]
    pub general: General,
    pub nullability: Option<Nullability>,
    // Default = false
    #[serde(default)]
    pub swift_import_as_accessors: bool,
    #[serde(rename = "Type")]
    pub type_: Option<Type>,
}

pub type Position = u8;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct PositionHelper {
    position: Position,
}

impl<'de> MapKey<'de> for Position {
    type Inner = PositionHelper;
    fn from_inner(inner: Self::Inner) -> Self {
        inner.position
    }
    const CONTAIN_ERROR: &'static str = "a `Position` attribute";
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
#[non_exhaustive]
pub struct Parameter {
    #[serde(flatten)]
    pub general: General,
    pub nullability: Option<Nullability>,
    #[serde(rename = "Type")]
    pub type_: Option<Type>,
    /// Only used on block parameters.
    pub no_escape: bool,
}

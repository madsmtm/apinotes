use serde::Deserialize;

use crate::{
    General, Map, Method, MethodKey, Nullability, Parameter, Position, Property, PropertyKey, Type,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
#[non_exhaustive]
pub struct Class {
    #[serde(flatten)]
    pub general: General,
    #[serde(with = "crate::map_helper")]
    pub methods: Map<MethodKey, Method>,
    #[serde(with = "crate::map_helper")]
    pub properties: Map<PropertyKey, Property>,
    pub swift_bridge: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
#[non_exhaustive]
pub struct Protocol {
    #[serde(flatten)]
    pub general: General,
    #[serde(with = "crate::map_helper")]
    pub methods: Map<MethodKey, Method>,
    #[serde(with = "crate::map_helper")]
    pub properties: Map<PropertyKey, Property>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum EnumKind {
    #[serde(alias = "CFEnum")]
    NSEnum,
    #[serde(alias = "CFClosedEnum")]
    NSClosedEnum,
    #[serde(alias = "CFOptions")]
    NSOptions,
    #[serde(rename = "none")]
    None,
}

/// Structs, enums, and unions.
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
#[non_exhaustive]
pub struct Tag {
    #[serde(flatten)]
    pub general: General,
    /// Only used on enums.
    #[serde(rename = "NSErrorDomain")]
    pub error_domain: Option<String>,
    /// Only used on enums.
    pub enum_kind: Option<EnumKind>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SwiftWrapper {
    Struct,
    Enum,
    None,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
#[non_exhaustive]
pub struct Typedef {
    #[serde(flatten)]
    pub general: General,
    pub swift_wrapper: Option<SwiftWrapper>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
#[non_exhaustive]
pub struct Global {
    #[serde(flatten)]
    pub general: General,
    pub nullability: Option<Nullability>,
    #[serde(rename = "Type")]
    pub type_: Option<Type>,
}

/// Enum cases.
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
#[non_exhaustive]
pub struct Enumerator {
    #[serde(flatten)]
    pub general: General,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
#[non_exhaustive]
pub struct Function {
    #[serde(flatten)]
    pub general: General,
    /// Note: May not work due to compiler bugs
    pub nullability_of_ret: Option<Nullability>,
    pub result_type: Option<Type>,
    #[serde(with = "crate::map_helper")]
    pub parameters: Map<Position, Parameter>,
    // pub retain_count_convention:
}

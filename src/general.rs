use serde::Deserialize;

pub type Availability = String;

/// General attributes.
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
#[non_exhaustive]
pub struct General {
    pub swift_name: Option<String>,
    // TODO: Handle availability better
    pub availability: Option<Availability>,
    pub availability_msg: Option<String>,
    pub swift_private: bool,
}

/// Note that this is overridden by `Type`, even in a `SwiftVersions` section.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum Nullability {
    #[serde(alias = "N")]
    Nonnull,
    #[serde(alias = "O")]
    Optional,
    #[serde(alias = "U")]
    Unspecified,
    #[serde(alias = "S")]
    Scalar,
}

pub type Type = String;

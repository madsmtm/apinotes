use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;

use serde::de::{self, Error};
use serde::Deserialize;

/// The map type used by this library.
pub type Map<V, K> = HashMap<V, K>;

pub(crate) trait MapKey<'de>: Hash + Eq {
    type Inner: Deserialize<'de>;
    fn from_inner(inner: Self::Inner) -> Self;
    const CONTAIN_ERROR: &'static str;
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Name {
    name: String,
}

#[derive(Deserialize)]
struct MapElement<K, V> {
    #[serde(flatten)]
    key: K,
    #[serde(flatten)]
    value: V,
}

impl<'de> MapKey<'de> for String {
    type Inner = Name;
    fn from_inner(inner: Self::Inner) -> Self {
        inner.name
    }
    const CONTAIN_ERROR: &'static str = "a `Name` attribute";
}

#[derive(Debug)]
struct Vis<K, V> {
    p: PhantomData<fn() -> Map<K, V>>,
}

impl<'de, K, V> de::Visitor<'de> for Vis<K, V>
where
    K: MapKey<'de>,
    V: Deserialize<'de>,
{
    type Value = Map<K, V>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "a sequence of maps, each of which contains {}",
            K::CONTAIN_ERROR
        )
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Map::new())
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut map = Map::with_capacity(seq.size_hint().unwrap_or(0));

        while let Some(MapElement { key, value }) = seq.next_element::<MapElement<K::Inner, V>>()? {
            match map.entry(K::from_inner(key)) {
                Entry::Occupied(_entry) => {
                    return Err(A::Error::custom("unexpected duplicate entry"));
                }
                Entry::Vacant(entry) => {
                    entry.insert(value);
                }
            }
        }

        Ok(map)
    }
}

pub(crate) fn deserialize<'de, D, K, V>(des: D) -> Result<Map<K, V>, D::Error>
where
    D: de::Deserializer<'de>,
    K: MapKey<'de>,
    V: Deserialize<'de>,
{
    des.deserialize_any(Vis { p: PhantomData })
}

// pub(crate) fn serialize<K, V, S>(map: &Map<K, V>, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: ser::Serializer,
// {
//     todo!()
// }

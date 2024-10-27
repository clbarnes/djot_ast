#[cfg(feature = "serde")]
use serde::{ser::SerializeStruct, Deserialize, Serialize};

use crate::{macros::impl_hasmeta, Block, Meta};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DefinitionList {
    pub children: Vec<DefinitionListItem>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(DefinitionList);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionListItem {
    pub term: Term,
    pub definition: Definition,

    meta: Meta,
}
impl_hasmeta!(DefinitionListItem);

#[cfg(feature = "serde")]
impl Serialize for DefinitionListItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("DefinitionListItem", 1)?;
        s.serialize_field("children", &(&self.term, &self.definition))?;
        s.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for DefinitionListItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let deser = DeserDefinitionListItem::deserialize(deserializer)?;
        Ok(deser.into())
    }
}

#[derive(Debug, Clone, Deserialize)]
#[cfg(feature = "serde")]
struct DeserDefinitionListItem {
    children: (Term, Definition),
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}

#[cfg(feature = "serde")]
impl From<DeserDefinitionListItem> for DefinitionListItem {
    fn from(value: DeserDefinitionListItem) -> Self {
        Self {
            term: value.children.0,
            definition: value.children.1,
            meta: value.meta,
        }
    }
}

crate::macros::inline_container!(Term);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Definition {
    pub children: Vec<Block>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(Definition);

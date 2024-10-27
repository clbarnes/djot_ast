#[cfg(feature = "serde")]
use serde::{ser::SerializeStruct, Deserialize, Serialize};

use crate::{macros::impl_hasmeta, Block, Meta, Node};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "definition_list")
)]
pub struct DefinitionList {
    pub children: Vec<DefinitionListItem>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl Node for DefinitionList {
    fn node_type(&self) -> crate::NodeType {
        crate::NodeType::Branch(self.children.len())
    }
}
impl_hasmeta!(DefinitionList);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionListItem {
    pub term: Term,
    pub definition: Definition,
    meta: Meta,
}
impl Node for DefinitionListItem {
    fn node_type(&self) -> crate::NodeType {
        crate::NodeType::Branch(2)
    }
}
impl_hasmeta!(DefinitionListItem);

#[cfg(feature = "serde")]
impl Serialize for DefinitionListItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("DefinitionListItem", 2)?;
        s.serialize_field("tag", "definition_list_item")?;
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
#[serde(tag = "tag", rename = "definition_list_item")]
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

crate::macros::inline_container!(Term, "term");

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "definition")
)]
pub struct Definition {
    pub children: Vec<Block>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl Node for Definition {
    fn node_type(&self) -> crate::NodeType {
        crate::NodeType::Branch(self.children.len())
    }
}
impl_hasmeta!(Definition);

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    #[test]
    fn test_tag_definition() {
        let s = r#"{"tag":"definition","children":[]}"#;
        let def: Definition = serde_json::from_str(s).unwrap();
        let s2 = serde_json::to_string(&def).unwrap();
        assert_eq!(s, s2);
    }
}

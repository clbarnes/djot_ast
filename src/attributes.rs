use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type Attributes = HashMap<String, String>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SourceLoc {
    pub line: u64,
    pub col: u64,
    pub offset: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pos {
    pub start: SourceLoc,
    pub end: SourceLoc,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Meta {
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Attributes::is_empty", default)
    )]
    attributes: Attributes,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Attributes::is_empty", default)
    )]
    auto_attributes: Attributes,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pos: Option<Pos>,
}

pub trait HasAttributes {
    /// Get a reference to the attributes.
    fn attributes(&self) -> &Attributes;

    /// Get a mutable reference to the attributes.
    fn attributes_mut(&mut self) -> &mut Attributes;

    /// Get a reference to the automatic attributes.
    fn auto_attributes(&self) -> &Attributes;

    /// Get an optional reference to the position.
    fn pos(&self) -> Option<&Pos>;

    /// Get a mutable reference to the optional position (so the position can be removed entirely).
    fn pos_mut(&mut self) -> &mut Option<Pos>;

    /// Look up an key in the explicit attributes, then the automatic attributes if not found.
    fn get_attribute(&self, key: &str) -> Option<&str> {
        self.attributes()
            .get(key)
            .or_else(|| self.auto_attributes().get(key))
            .map(|s| s.as_str())
    }
}

pub(crate) trait HasMeta {
    fn meta(&self) -> &Meta;
    fn meta_mut(&mut self) -> &mut Meta;
}

impl<T: HasMeta> HasAttributes for T {
    fn attributes(&self) -> &Attributes {
        self.meta().attributes()
    }

    fn attributes_mut(&mut self) -> &mut Attributes {
        self.meta_mut().attributes_mut()
    }

    fn auto_attributes(&self) -> &Attributes {
        self.meta().auto_attributes()
    }

    fn pos(&self) -> Option<&Pos> {
        self.meta().pos()
    }

    fn pos_mut(&mut self) -> &mut Option<Pos> {
        self.meta_mut().pos_mut()
    }
}

impl HasAttributes for Meta {
    fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    fn auto_attributes(&self) -> &Attributes {
        &self.auto_attributes
    }

    fn pos(&self) -> Option<&Pos> {
        self.pos.as_ref()
    }

    fn pos_mut(&mut self) -> &mut Option<Pos> {
        &mut self.pos
    }
}

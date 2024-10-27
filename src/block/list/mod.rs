#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{macros::impl_hasmeta, Block, Meta, Node};

mod ordered;
pub use ordered::{FenceStyle, NumberStyle, OrderedList, OrderedListStyle};

mod definition;
pub use definition::{Definition, DefinitionList, DefinitionListItem, Term};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "bullet_list", rename = "bullet_list")
)]
pub struct BulletList {
    tight: bool,
    style: BulletListStyle,
    children: Vec<ListItem>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl Node for BulletList {
    fn node_type(&self) -> crate::NodeType {
        crate::NodeType::Branch(self.children.len())
    }
}
impl_hasmeta!(BulletList);

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BulletListStyle {
    #[cfg_attr(feature = "serde", serde(rename = "+"))]
    Plus,
    #[cfg_attr(feature = "serde", serde(rename = "-"))]
    Dash,
    #[cfg_attr(feature = "serde", serde(rename = "*"))]
    Asterisk,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "list_item")
)]
pub struct ListItem {
    children: Vec<Block>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl Node for ListItem {
    fn node_type(&self) -> crate::NodeType {
        crate::NodeType::Branch(self.children.len())
    }
}
impl_hasmeta!(ListItem);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "task_list")
)]
pub struct TaskList {
    tight: bool,
    children: Vec<TaskListItem>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl Node for TaskList {
    fn node_type(&self) -> crate::NodeType {
        crate::NodeType::Branch(self.children.len())
    }
}
impl_hasmeta!(TaskList);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "task_list_item")
)]
pub struct TaskListItem {
    checkbox: CheckboxStatus,
    children: Vec<Block>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl Node for TaskListItem {
    fn node_type(&self) -> crate::NodeType {
        crate::NodeType::Branch(self.children.len())
    }
}
impl_hasmeta!(TaskListItem);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub enum CheckboxStatus {
    Checked,
    Unchecked,
}

impl CheckboxStatus {
    pub fn is_checked(&self) -> bool {
        match self {
            CheckboxStatus::Checked => true,
            CheckboxStatus::Unchecked => false,
        }
    }
}

use attributes::{HasMeta, Meta};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod error;
pub use error::{Error, Result};
mod macros;
use macros::{from_into_variants, impl_hasmeta};

pub mod block;
use block::{
    Block, Caption, Cell, Definition, DefinitionListItem, ListItem, Row, TaskListItem, Term,
};
pub mod inline;
use inline::Inline;

pub mod attributes;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(tag = "tag"))]
pub struct Reference {
    pub label: String,
    pub destination: String,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(Reference);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(tag = "tag"))]
pub struct Footnote {
    pub label: String,
    pub children: Vec<Block>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(Footnote);

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase", tag = "tag")
)]
pub struct Doc {
    pub references: HashMap<String, Reference>,
    pub auto_references: HashMap<String, Reference>,
    pub footnotes: HashMap<String, Footnote>,
    pub children: Vec<Block>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}

impl_hasmeta!(Doc);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename_all = "snake_case")
)]
pub enum AstNode {
    Doc(Doc),
    Block(Block),
    Inline(Inline),
    ListItem(ListItem),
    TaskListItem(TaskListItem),
    DefinitionListItem(DefinitionListItem),
    Term(Term),
    Definition(Definition),
    Row(Row),
    Cell(Cell),
    Caption(Caption),
    Footnote(Footnote),
    Reference(Reference),
}

impl HasMeta for AstNode {
    fn meta(&self) -> &Meta {
        use AstNode::*;
        match self {
            Doc(i) => i.meta(),
            Block(i) => i.meta(),
            Inline(i) => i.meta(),
            ListItem(i) => i.meta(),
            TaskListItem(i) => i.meta(),
            DefinitionListItem(i) => i.meta(),
            Term(i) => i.meta(),
            Definition(i) => i.meta(),
            Row(i) => i.meta(),
            Cell(i) => i.meta(),
            Caption(i) => i.meta(),
            Footnote(i) => i.meta(),
            Reference(i) => i.meta(),
        }
    }

    fn meta_mut(&mut self) -> &mut Meta {
        use AstNode::*;
        match self {
            Doc(i) => i.meta_mut(),
            Block(i) => i.meta_mut(),
            Inline(i) => i.meta_mut(),
            ListItem(i) => i.meta_mut(),
            TaskListItem(i) => i.meta_mut(),
            DefinitionListItem(i) => i.meta_mut(),
            Term(i) => i.meta_mut(),
            Definition(i) => i.meta_mut(),
            Row(i) => i.meta_mut(),
            Cell(i) => i.meta_mut(),
            Caption(i) => i.meta_mut(),
            Footnote(i) => i.meta_mut(),
            Reference(i) => i.meta_mut(),
        }
    }
}

from_into_variants!(
    AstNode,
    Doc,
    Block,
    Inline,
    ListItem,
    TaskListItem,
    DefinitionListItem,
    Term,
    Definition,
    Row,
    Cell,
    Caption,
    Footnote,
    Reference
);

/// Create a document root node.
pub fn new_document() -> AstNode {
    Doc::default().into()
}

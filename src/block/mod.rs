#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::attributes::{HasMeta, Meta};
use crate::{
    inline::Inline,
    macros::{atom, from_into_variants, impl_hasmeta},
};

mod list;
pub use list::{
    BulletList, BulletListStyle, CheckboxStatus, Definition, DefinitionList, DefinitionListItem,
    FenceStyle, ListItem, NumberStyle, OrderedList, OrderedListStyle, TaskList, TaskListItem, Term,
};
mod table;
pub use table::{Caption, Cell, Row, Table};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename_all = "snake_case")
)]
pub enum Block {
    Para(Para),
    Heading(Heading),
    ThematicBreak(ThematicBreak),
    Section(Section),
    Div(Div),
    CodeBlock(CodeBlock),
    RawBlock(RawBlock),
    BlockQuote(BlockQuote),
    OrderedList(OrderedList),
    BulletList(BulletList),
    TaskList(TaskList),
    DefinitionList(DefinitionList),
    Table(Table),
}

from_into_variants!(
    Block,
    Para,
    Heading,
    ThematicBreak,
    Section,
    Div,
    CodeBlock,
    RawBlock,
    BlockQuote,
    OrderedList,
    BulletList,
    TaskList,
    DefinitionList,
    Table
);

impl HasMeta for Block {
    fn meta(&self) -> &Meta {
        use Block::*;
        match self {
            Para(i) => i.meta(),
            Heading(i) => i.meta(),
            ThematicBreak(i) => i.meta(),
            Section(i) => i.meta(),
            Div(i) => i.meta(),
            CodeBlock(i) => i.meta(),
            RawBlock(i) => i.meta(),
            BlockQuote(i) => i.meta(),
            OrderedList(i) => i.meta(),
            BulletList(i) => i.meta(),
            TaskList(i) => i.meta(),
            DefinitionList(i) => i.meta(),
            Table(i) => i.meta(),
        }
    }

    fn meta_mut(&mut self) -> &mut Meta {
        use Block::*;
        match self {
            Para(i) => i.meta_mut(),
            Heading(i) => i.meta_mut(),
            ThematicBreak(i) => i.meta_mut(),
            Section(i) => i.meta_mut(),
            Div(i) => i.meta_mut(),
            CodeBlock(i) => i.meta_mut(),
            RawBlock(i) => i.meta_mut(),
            BlockQuote(i) => i.meta_mut(),
            OrderedList(i) => i.meta_mut(),
            BulletList(i) => i.meta_mut(),
            TaskList(i) => i.meta_mut(),
            DefinitionList(i) => i.meta_mut(),
            Table(i) => i.meta_mut(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "para")
)]
pub struct Para {
    pub children: Vec<Inline>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(Para);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "heading")
)]
pub struct Heading {
    pub level: u64,
    pub children: Vec<Inline>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(Heading);

atom!(ThematicBreak, "thematic_break");

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "section")
)]
pub struct Section {
    pub children: Vec<Block>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(Section);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "div")
)]
pub struct Div {
    pub children: Vec<Block>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(Div);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "block_quote")
)]
pub struct BlockQuote {
    pub children: Vec<Block>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(BlockQuote);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "code_block")
)]
pub struct CodeBlock {
    lang: Option<String>,
    text: String,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(CodeBlock);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "raw_block")
)]
pub struct RawBlock {
    format: String,
    text: String,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(RawBlock);

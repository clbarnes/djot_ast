#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    macros::{atom, from_into_variants, impl_hasmeta, inline_container, text_container},
    HasMeta, Meta, Node, NodeType,
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename_all = "snake_case")
)]
pub enum Inline {
    Str(Str),
    SoftBreak(SoftBreak),
    HardBreak(HardBreak),
    NonBreakingSpace(NonBreakingSpace),
    Symb(Symb),
    Verbatim(Verbatim),
    RawInline(RawInline),
    InlineMath(InlineMath),
    DisplayMath(DisplayMath),
    Url(Url),
    Email(Email),
    FootnoteReference(FootnoteReference),
    SmartPunctuation(SmartPunctuation),
    Emph(Emph),
    Strong(Strong),
    Link(Link),
    Image(Image),
    Span(Span),
    Mark(Mark),
    Superscript(Superscript),
    Subscript(Subscript),
    Insert(Insert),
    Delete(Delete),
    DoubleQuoted(DoubleQuoted),
    SingleQuoted(SingleQuoted),
}

impl HasMeta for Inline {
    fn meta(&self) -> &Meta {
        match self {
            Inline::Str(i) => i.meta(),
            Inline::SoftBreak(i) => i.meta(),
            Inline::HardBreak(i) => i.meta(),
            Inline::NonBreakingSpace(i) => i.meta(),
            Inline::Symb(i) => i.meta(),
            Inline::Verbatim(i) => i.meta(),
            Inline::RawInline(i) => i.meta(),
            Inline::InlineMath(i) => i.meta(),
            Inline::DisplayMath(i) => i.meta(),
            Inline::Url(i) => i.meta(),
            Inline::Email(i) => i.meta(),
            Inline::FootnoteReference(i) => i.meta(),
            Inline::SmartPunctuation(i) => i.meta(),
            Inline::Emph(i) => i.meta(),
            Inline::Strong(i) => i.meta(),
            Inline::Link(i) => i.meta(),
            Inline::Image(i) => i.meta(),
            Inline::Span(i) => i.meta(),
            Inline::Mark(i) => i.meta(),
            Inline::Superscript(i) => i.meta(),
            Inline::Subscript(i) => i.meta(),
            Inline::Insert(i) => i.meta(),
            Inline::Delete(i) => i.meta(),
            Inline::DoubleQuoted(i) => i.meta(),
            Inline::SingleQuoted(i) => i.meta(),
        }
    }

    fn meta_mut(&mut self) -> &mut Meta {
        match self {
            Inline::Str(i) => i.meta_mut(),
            Inline::SoftBreak(i) => i.meta_mut(),
            Inline::HardBreak(i) => i.meta_mut(),
            Inline::NonBreakingSpace(i) => i.meta_mut(),
            Inline::Symb(i) => i.meta_mut(),
            Inline::Verbatim(i) => i.meta_mut(),
            Inline::RawInline(i) => i.meta_mut(),
            Inline::InlineMath(i) => i.meta_mut(),
            Inline::DisplayMath(i) => i.meta_mut(),
            Inline::Url(i) => i.meta_mut(),
            Inline::Email(i) => i.meta_mut(),
            Inline::FootnoteReference(i) => i.meta_mut(),
            Inline::SmartPunctuation(i) => i.meta_mut(),
            Inline::Emph(i) => i.meta_mut(),
            Inline::Strong(i) => i.meta_mut(),
            Inline::Link(i) => i.meta_mut(),
            Inline::Image(i) => i.meta_mut(),
            Inline::Span(i) => i.meta_mut(),
            Inline::Mark(i) => i.meta_mut(),
            Inline::Superscript(i) => i.meta_mut(),
            Inline::Subscript(i) => i.meta_mut(),
            Inline::Insert(i) => i.meta_mut(),
            Inline::Delete(i) => i.meta_mut(),
            Inline::DoubleQuoted(i) => i.meta_mut(),
            Inline::SingleQuoted(i) => i.meta_mut(),
        }
    }
}

impl Node for Inline {
    fn node_type(&self) -> NodeType {
        match self {
            Inline::Str(i) => i.node_type(),
            Inline::SoftBreak(i) => i.node_type(),
            Inline::HardBreak(i) => i.node_type(),
            Inline::NonBreakingSpace(i) => i.node_type(),
            Inline::Symb(i) => i.node_type(),
            Inline::Verbatim(i) => i.node_type(),
            Inline::RawInline(i) => i.node_type(),
            Inline::InlineMath(i) => i.node_type(),
            Inline::DisplayMath(i) => i.node_type(),
            Inline::Url(i) => i.node_type(),
            Inline::Email(i) => i.node_type(),
            Inline::FootnoteReference(i) => i.node_type(),
            Inline::SmartPunctuation(i) => i.node_type(),
            Inline::Emph(i) => i.node_type(),
            Inline::Strong(i) => i.node_type(),
            Inline::Link(i) => i.node_type(),
            Inline::Image(i) => i.node_type(),
            Inline::Span(i) => i.node_type(),
            Inline::Mark(i) => i.node_type(),
            Inline::Superscript(i) => i.node_type(),
            Inline::Subscript(i) => i.node_type(),
            Inline::Insert(i) => i.node_type(),
            Inline::Delete(i) => i.node_type(),
            Inline::DoubleQuoted(i) => i.node_type(),
            Inline::SingleQuoted(i) => i.node_type(),
        }
    }
}

from_into_variants!(
    Inline,
    Str,
    SoftBreak,
    HardBreak,
    NonBreakingSpace,
    Symb,
    Verbatim,
    RawInline,
    InlineMath,
    DisplayMath,
    Url,
    Email,
    FootnoteReference,
    SmartPunctuation,
    Emph,
    Strong,
    Link,
    Image,
    Span,
    Mark,
    Superscript,
    Subscript,
    Insert,
    Delete,
    DoubleQuoted,
    SingleQuoted
);

atom!(SoftBreak, "soft_break");
atom!(HardBreak, "hard_break");
atom!(NonBreakingSpace, "non_breaking_space");

text_container!(Str, "str");
text_container!(FootnoteReference, "footnote_reference");
text_container!(Verbatim, "verbatim");
text_container!(InlineMath, "inline_math");
text_container!(DisplayMath, "display_math");
text_container!(Url, "url");
text_container!(Email, "email");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum SmartPunctuationType {
    LeftSingleQuote,
    RightSingleQuote,
    LeftDoubleQuote,
    RightDoubleQuote,
    Ellipses,
    EmDash,
    EnDash,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "smart_punctuation")
)]
pub struct SmartPunctuation {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub punc_type: SmartPunctuationType,
    pub text: String,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl Node for SmartPunctuation {
    fn node_type(&self) -> crate::NodeType {
        crate::NodeType::Leaf
    }
}
impl_hasmeta!(SmartPunctuation);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "symb")
)]
pub struct Symb {
    pub alias: String,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl Node for Symb {
    fn node_type(&self) -> crate::NodeType {
        crate::NodeType::Leaf
    }
}
impl_hasmeta!(Symb);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "raw_inline")
)]
pub struct RawInline {
    pub format: String,
    pub text: String,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl Node for RawInline {
    fn node_type(&self) -> crate::NodeType {
        crate::NodeType::Leaf
    }
}
impl_hasmeta!(RawInline);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "link")
)]
pub struct Link {
    destination: Option<String>,
    reference: Option<String>,
    children: Vec<Inline>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl Node for Link {
    fn node_type(&self) -> crate::NodeType {
        crate::NodeType::Branch(self.children.len())
    }
}
impl_hasmeta!(Link);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "image")
)]
pub struct Image {
    pub destination: Option<String>,
    pub reference: Option<String>,
    pub children: Vec<Inline>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl Node for Image {
    fn node_type(&self) -> crate::NodeType {
        crate::NodeType::Branch(self.children.len())
    }
}

impl_hasmeta!(Image);

inline_container!(Emph, "emph");
inline_container!(Strong, "strong");
inline_container!(Span, "span");
inline_container!(Mark, "mark");
inline_container!(Superscript, "superscript");
inline_container!(Subscript, "subscript");
inline_container!(Delete, "delete");
inline_container!(Insert, "insert");
inline_container!(DoubleQuoted, "double_quoted");
inline_container!(SingleQuoted, "single_quoted");

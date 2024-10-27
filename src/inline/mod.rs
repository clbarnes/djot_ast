#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    macros::{atom, from_into_variants, impl_hasmeta, inline_container, text_container},
    HasMeta, Meta,
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

atom!(SoftBreak);
atom!(HardBreak);
atom!(NonBreakingSpace);

text_container!(Str);
text_container!(FootnoteReference);
text_container!(Verbatim);
text_container!(InlineMath);
text_container!(DisplayMath);
text_container!(Url);
text_container!(Email);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename_all = "snake_case")
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SmartPunctuation {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub punc_type: SmartPunctuationType,
    pub text: String,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(SmartPunctuation);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Symb {
    pub alias: String,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(Symb);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RawInline {
    pub format: String,
    pub text: String,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(RawInline);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Link {
    destination: Option<String>,
    reference: Option<String>,
    children: Vec<Inline>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(Link);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Image {
    pub destination: Option<String>,
    pub reference: Option<String>,
    pub children: Vec<Inline>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}

impl_hasmeta!(Image);

inline_container!(Emph);
inline_container!(Strong);
inline_container!(Span);
inline_container!(Mark);
inline_container!(Superscript);
inline_container!(Subscript);
inline_container!(Delete);
inline_container!(Insert);
inline_container!(DoubleQuoted);
inline_container!(SingleQuoted);

use std::{
    fmt::{Display, Write},
    str::FromStr,
};

#[cfg(feature = "serde")]
use serde::{de, Deserialize, Serialize};

use crate::{macros::impl_hasmeta, Error, Meta, Result};

use super::ListItem;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OrderedList {
    pub style: OrderedListStyle,
    pub tight: bool,
    pub start: Option<u64>,
    pub children: Vec<ListItem>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(OrderedList);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderedListStyle {
    number: NumberStyle,
    fence: FenceStyle,
}

#[cfg(feature = "serde")]
impl Serialize for OrderedListStyle {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.to_string();
        s.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for OrderedListStyle {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(de::Error::custom)
    }
}

impl Display for OrderedListStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.fence == FenceStyle::BothParen {
            f.write_char('(')?;
        }
        let inner = match self.number {
            NumberStyle::Arabic => '1',
            NumberStyle::LowerAlpha => 'a',
            NumberStyle::UpperAlpha => 'A',
            NumberStyle::LowerRoman => 'i',
            NumberStyle::UpperRoman => 'I',
        };
        f.write_char(inner)?;
        let rfence = match self.fence {
            FenceStyle::Dot => '.',
            FenceStyle::RightParen | FenceStyle::BothParen => ')',
        };
        f.write_char(rfence)
    }
}

impl FromStr for OrderedListStyle {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self> {
        let invalid = || Error::General(format!("invalid ordered list style '{s}'"));
        let mut chars = s.chars();
        let mut c = chars.next().ok_or_else(invalid)?;
        let starts_paren = if c == '(' {
            c = chars.next().ok_or_else(invalid)?;
            true
        } else {
            false
        };

        let number = match c {
            '1' => NumberStyle::Arabic,
            'a' => NumberStyle::LowerAlpha,
            'A' => NumberStyle::UpperAlpha,
            'i' => NumberStyle::LowerRoman,
            'I' => NumberStyle::UpperRoman,
            _ => return Err(invalid()),
        };
        let fence = match chars.next().ok_or_else(invalid)? {
            ')' => {
                if starts_paren {
                    FenceStyle::BothParen
                } else {
                    FenceStyle::RightParen
                }
            }
            '.' => FenceStyle::Dot,
            _ => return Err(invalid()),
        };
        Ok(Self { number, fence })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberStyle {
    Arabic,
    LowerAlpha,
    UpperAlpha,
    LowerRoman,
    UpperRoman,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceStyle {
    Dot,
    RightParen,
    BothParen,
}

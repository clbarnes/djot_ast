use crate::{macros::impl_hasmeta, Error, Inline, Meta};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "caption")
)]
pub struct Caption {
    pub children: Vec<Inline>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(Caption);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "row")
)]
pub struct Row {
    pub head: bool,
    pub children: Vec<Cell>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(Row);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum Alignment {
    #[default]
    Default,
    Left,
    Right,
    Center,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "tag", rename = "cell")
)]
pub struct Cell {
    pub head: bool,
    pub align: Alignment,
    pub children: Vec<Inline>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}
impl_hasmeta!(Cell);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    caption: Caption,
    rows: Vec<Row>,
    meta: Meta,
}
impl_hasmeta!(Table);

#[cfg(feature = "serde")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "tag", rename = "table")]
struct DeserTable {
    children: Vec<DeserCapOrRow>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: Meta,
}

#[cfg(feature = "serde")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tag", rename_all = "snake_case")]
enum DeserCapOrRow {
    Caption(Caption),
    Row(Row),
}

#[cfg(feature = "serde")]
impl TryFrom<DeserTable> for Table {
    type Error = crate::Error;

    fn try_from(value: DeserTable) -> Result<Self, Self::Error> {
        let mut children = value.children.into_iter();
        let caption = match children.next() {
            Some(DeserCapOrRow::Caption(c)) => c,
            _ => return Err(Error::general("children must start with a caption")),
        };
        let r_rows = children
            .map(|c| match c {
                DeserCapOrRow::Row(row) => Ok(row),
                _ => Err(Error::general("children other than first must be rows")),
            })
            .collect::<crate::Result<Vec<Row>>>();
        Ok(Self {
            caption,
            rows: r_rows?,
            meta: value.meta,
        })
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Table {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let deser = DeserTable::deserialize(deserializer)?;
        deser.try_into().map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
#[derive(Debug, Serialize)]
#[serde(tag = "tag", rename = "table")]
struct SerTable<'a> {
    children: Vec<SerCapOrRow<'a>>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    meta: &'a Meta,
}

#[cfg(feature = "serde")]
#[derive(Debug, Serialize)]
#[serde(tag = "tag", rename_all = "snake_case")]
enum SerCapOrRow<'a> {
    Caption(&'a Caption),
    Row(&'a Row),
}

#[cfg(feature = "serde")]
impl<'a> From<&'a Table> for SerTable<'a> {
    fn from(value: &'a Table) -> Self {
        let mut children = Vec::with_capacity(1 + value.rows.len());
        children.push(SerCapOrRow::Caption(&value.caption));
        children.extend(value.rows.iter().map(SerCapOrRow::Row));

        Self {
            children,
            meta: &value.meta,
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for Table {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let sertable = SerTable::from(self);
        sertable.serialize(serializer)
    }
}

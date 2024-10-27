macro_rules! impl_hasmeta {
    ($name:ident) => {
        impl crate::HasMeta for $name {
            fn meta(&self) -> &crate::Meta {
                &self.meta
            }

            fn meta_mut(&mut self) -> &mut crate::Meta {
                &mut self.meta
            }
        }
    };
}

macro_rules! text_container {
    ($name:ident, $tag:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        #[cfg_attr(
                                                    feature = "serde",
                                                    derive(serde::Serialize, serde::Deserialize),
                                                    serde(tag = "tag", rename = $tag),
                                                )]
        pub struct $name {
            pub text: String,
            #[cfg_attr(feature = "serde", serde(flatten))]
            meta: crate::Meta,
        }

        impl crate::Node for $name {
            fn node_type(&self) -> crate::NodeType {
                crate::NodeType::Leaf
            }
        }

        crate::macros::impl_hasmeta!($name);
    };
}

macro_rules! inline_container {
    ($name:ident, $tag:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "tag", rename = $tag))]
        pub struct $name {
            pub children: Vec<crate::inline::Inline>,
            #[cfg_attr(feature = "serde", serde(flatten))]
            meta: crate::Meta,
        }

        crate::macros::impl_hasmeta!($name);

        impl crate::Node for $name {
            fn node_type(&self) -> crate::NodeType {
                crate::NodeType::Branch(self.children.len())
            }
        }
    };
}

macro_rules! atom {
    ($name:ident, $tag:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "tag", rename = $tag))]
        pub struct $name {
            #[cfg_attr(feature = "serde", serde(flatten))]
            meta: crate::Meta,
        }

        crate::macros::impl_hasmeta!($name);

        impl crate::Node for $name {
            fn node_type(&self) -> crate::NodeType {
                crate::NodeType::Leaf
            }
        }
    };
}

macro_rules! from_into_variants {
    ($enum:ident, $($variant:ident),+) => {
        $(

        impl From<$variant> for $enum {
            fn from(v: $variant) -> Self {
                Self::$variant(v)
            }
        }

        impl TryFrom<$enum> for $variant {
            type Error = crate::Error;

            fn try_from(v: $enum) -> crate::Result<Self> {
                match v {
                    $enum::$variant(inner) => Ok(inner),
                    _ => Err(crate::Error::general("wrong variant")),
                }
            }
        }
        )+
    };
}

pub(crate) use {atom, from_into_variants, impl_hasmeta, inline_container, text_container};

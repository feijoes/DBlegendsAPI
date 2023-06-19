use std::fmt;
use serde::{ Deserialize, Serialize };
use serde_with::skip_serializing_none;
use strum_macros::EnumString;
macro_rules! impl_enum_as_string {
    ($enum_name:ident { $($variant:ident),+ $(,)? }) => {
        impl $enum_name {
            #[allow(dead_code)]
            pub fn as_string(&self) -> &str {
                match self {
                    $( $enum_name::$variant => stringify!($variant) ),+
                }
            }
        }
    };
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, EnumString, Clone, Ord, PartialEq, PartialOrd, Eq)]
pub enum Rarity {
    HERO,
    EXTREME,
    SPARKING,
    ULTRA,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, EnumString, Clone, Ord, PartialEq, PartialOrd, Eq)]
pub enum Color {
    PUR,
    YEL,
    BLU,
    RED,
    GRN,
    LGT,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SortOptions {
    Name,
    Color,
    Rarity,
}
macro_rules! impl_display_enum {
    ($enum_name:ident { $($variant:ident),* }) => {
        impl fmt::Display for $enum_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let printable = match self {
                    $( $enum_name::$variant => stringify!($variant), )*
                };
                write!(f, "{}", printable)
            }
        }
    };
}

impl_display_enum!(Rarity { HERO, EXTREME, SPARKING, ULTRA });
impl_display_enum!(Color { PUR, YEL, BLU, RED, GRN, LGT });
impl_enum_as_string!(Color { PUR, YEL, BLU, RED, GRN, LGT });
impl_enum_as_string!(Rarity { HERO, EXTREME, SPARKING, ULTRA });

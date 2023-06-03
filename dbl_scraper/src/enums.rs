use std::fmt;

use serde::{ Deserialize, Serialize };
use serde_with::skip_serializing_none;
use strum_macros::EnumString;



#[skip_serializing_none]
#[derive( Debug, Deserialize, Serialize, EnumString )]
pub enum Rarity{
    HERO,
    EXTREME,
    SPAKING,
    ULTRA
}


#[skip_serializing_none]
#[derive( Debug, Deserialize, Serialize, EnumString)]
pub enum Color {
    PUR,
    YEL,
    BLU,
    RED,
    GRN,
    LGT
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

impl_display_enum!(Rarity { HERO, EXTREME, SPAKING, ULTRA });
impl_display_enum!(Color { PUR, YEL, BLU, RED, GRN, LGT });
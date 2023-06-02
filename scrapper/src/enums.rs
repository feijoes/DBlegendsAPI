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

impl fmt::Display for Rarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printable = match self {
            Rarity::HERO => "Hero",
            Rarity::EXTREME => "Extreme",
            Rarity::SPAKING => "Sparking",
            Rarity::ULTRA => "Ultra",
        };
        write!(f, "{}", printable)
    }
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
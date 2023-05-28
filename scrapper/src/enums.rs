use serde::{ Deserialize, Serialize };
use serde_with::skip_serializing_none;


#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub enum Rarity{
    Hero,
    Extreme,
    Sparking,
    Ultra
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub enum Color {
    PUR,
    YEL,
    BLU,
    RED,
    GRN,
    LGT
}
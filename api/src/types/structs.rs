use crate::types::enums::Rarity;
use serde::{ Deserialize, Serialize };
use serde_with::skip_serializing_none;
use std::{sync::{Arc, RwLock}};
use serde_with::{ StringWithSeparator, formats::CommaSeparator };

#[derive(Debug)]
pub struct AppState {
    pub characters: Arc<RwLock<Vec<Character>>>,
}

#[serde_with::serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[non_exhaustive]
pub struct ApiParams {
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    pub name: Option<Vec<String>>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    pub fname: Option<Vec<String>>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    pub title: Option<Vec<String>>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    pub categories: Option<Vec<String>>,
    pub sort_by: Option<SortOptions>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    pub id: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub rarity: Option<Rarity>,
    pub reverse: Option<bool>,
}


#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct MainAbility {
    pub name: String,
    pub effect: String,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct UniqueAbilities {
    pub unique_start_abilities: Vec<UniqueAbility>,
    pub unique_zenkai_abilities: Option<Vec<UniqueAbility>>,
}
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct UniqueAbility {
    pub ability_name: String,
    pub ability_effect: String,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct UltraAbility {
    pub name: String,
    pub effect: String,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Stats {
    pub power: i32,
    pub health: i32,
    pub strike_atk: i32,
    pub strike_def: i32,
    pub blast_atk: i32,
    pub blast_def: i32,
}
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ZAbility {
    pub tags: Vec<String>,
    pub effect: String,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ZAbilities {
    pub one: ZAbility,
    pub two: ZAbility,
    pub three: ZAbility,
    pub four: ZAbility,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct SpecialMove {
    pub name: String,
    pub effect: String,
}
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct UltimateSkill {
    pub name: String,
    pub effect: String,
}
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct SpecialSkill {
    pub name: String,
    pub effect: String,
}

use std::fmt;

use crate::character::Character;

use super::enums::SortOptions;

macro_rules! implement_display {
    ($struct_name:ident, $field_name:ident) => {
        impl fmt::Display for $struct_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.$field_name)
            }
        }
    };
}

implement_display!(MainAbility, name);
implement_display!(UltraAbility, name);

use serde::{ Deserialize, Serialize };
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct MainAbility {
    pub name: String,
    pub effect: String,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct UniqueAbility {
    pub ability1_name: String,
    pub ability1_effect: String,
    pub ability2_name: String,
    pub ability2_effect: String
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
pub struct ZAbility{
    pub tags: Vec<String>,
    pub effect: String
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
implement_display!(UniqueAbility, ability1_name);
implement_display!(UltraAbility, name);
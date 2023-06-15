use crate::types::enums::{ Rarity, Color };
use serde::{ Deserialize, Serialize };
use serde_with::skip_serializing_none;
use crate::types::structs::{
    MainAbility,
    UltraAbility,
    Stats,
    SpecialSkill,
    UltimateSkill,
    SpecialMove,
    ZAbilities,
    UniqueAbilities,
};


#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Character {
    pub name: String,
    pub id: String,
    pub color: Color,
    pub rarity: Rarity,
    pub tags: Vec<String>,
    pub main_ability: MainAbility,
    pub unique_ability: UniqueAbilities,
    pub ultra_ability: Option<UltraAbility>,
    pub base_stats: Stats,
    pub max_stats: Stats,
    pub strike: String,
    pub shot: String,
    pub image_url: String,
    pub special_move: SpecialMove,
    pub special_skill: SpecialSkill,
    pub ultimate_skill: Option<UltimateSkill>,
    pub z_ability: ZAbilities,
    pub is_lf: bool,
    pub is_tag: bool,
    pub has_zenkai: bool,
}


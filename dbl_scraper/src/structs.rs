use serde::{ Deserialize, Serialize };
use serde_with::skip_serializing_none;


#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct MainAbility{
    pub name: String,
    pub effect: String
}


#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct UniqueAbility{
    pub name: String,
    pub effect: String
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct UltraAbility{
    pub name: String,
    pub effect: String
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Stats {
    power: f32,
    health: f32,
    strike_atk : f32,
    strike_def: f32 ,
    blast_atk: f32,
    blast_def: f32
}
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ZAbility{
    pub one: String,
    pub two: String,
    pub three: String,
    pub four: String
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct SpecialMove{
    pub name: String,
    pub effect: String
}
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct UltimateSkill{
    pub name: String,
    pub effect: String
}
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct SpecialSkill{
    pub name: String,
    pub effect: String
}
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
pub struct UltraAbility{
    pub name: String,
    pub effect: String
}
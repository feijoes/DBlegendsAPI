mod structs;
mod enums;
use std::{error::Error};
use serde::{ Deserialize, Serialize };
use serde_with::skip_serializing_none;
use structs::{MainAbility, UltraAbility, Stats, UniqueAbility, ZAbility, SpecialSkill, UltimateSkill, SpecialMove};
use enums::{Rarity};
use scraper::{Selector};
use std::str::FromStr;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
struct Character{
    name: String,
    id: String,
    rarity: Rarity,
    tags: Vec<String>,
    main_ability: MainAbility,
    unique_ability: UniqueAbility,
    ultra_ability: Option<UltraAbility>,
    base_stats: Stats,
    max_stats: Stats,
    strike: String,
    shot: String,
    image_url: String,
    special_move: SpecialMove,
    special_skill: SpecialSkill,
    ultimate_skill: UltimateSkill,
    z_ability: ZAbility,
    is_lf : bool
}

async fn get_html(path:&str) -> Result<String, Box<dyn Error>> {
    let url = String::from("https://legends.dbz.space");
    let response = reqwest::Client::new()
        .get(url + path) 
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .send().await?
        .text().await?;
    Ok(response)
}
#[allow(unreachable_code,unused_variables)]
fn get_character_info( html: String ) -> Result<Option<Character>,Box<dyn Error>>{
    let document = scraper::Html::parse_document(&html);

    let name_sele = Selector::parse(".head.name.large.img_back>h1").unwrap();
    let name = document.select(&name_sele).next().unwrap().inner_html();

    let id_sele = Selector::parse(".head.name.id-right.small.img_back").unwrap();
    let id = document.select(&id_sele).next().unwrap().inner_html();

    let rarity_sele = Selector::parse(".rarity").unwrap();
    let rarity =Rarity::from_str(document.select(&rarity_sele).next().unwrap().inner_html().as_str()).unwrap();

    let tags_sele = Selector::parse(".ability.medium>a").unwrap();
    let tags: Vec<String> = document.select(&tags_sele).map(|element| { element.text().collect::<String>().replace("\n", "").replace("\t", "") }).collect();
    
    
    
    println!("{name} {rarity} {id} {tags:?}");
    //let character = Character {name: name, id: id, rarity: rarity, tags: tags, main_ability: todo!(), ultra_ability: todo!(), base_stats: todo!(), max_stats: todo!(), unique_ability: todo!(), strike: todo!(), shot: todo!(), special_move: todo!(), special_skill: todo!(), ultimate_skill: todo!(), z_ability: todo!(), image_url: todo!(), is_lf: todo!() } ;
    return Ok(None)
}
#[tokio::main]
async fn main() {
    color_eyre::install().unwrap();
    let url = "/characters/";
    
    
    let response = get_html(url).await.unwrap();
    
    let document = scraper::Html::parse_document(&response);
    let select_char =  scraper::Selector::parse(".chara.list> a").unwrap();

    let extracted_links: Vec<String> = document
        .select(&select_char)
        .map(|element| {
            let href = element.value().attr("href").unwrap().to_string();
            href
        })
        .collect();
    get_character_info(get_html(&extracted_links.first().unwrap()).await.unwrap());
    //for link in extracted_links{
    //    let _character = );
        
    //}
}


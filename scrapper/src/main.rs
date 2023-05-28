mod structs;
mod enums;
use std::{error::Error};
use serde::{ Deserialize, Serialize };
use serde_with::skip_serializing_none;
use structs::{MainAbility, UltraAbility, Stats};
use enums::{Rarity};

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
struct Character{
    name: String,
    id: String,
    rarity: Rarity,
    tags: Vec<String>,
    main_ability: MainAbility,
    ultra_ability: Option<UltraAbility>,
    base_stats: Stats,
    max_stats: Stats
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
#[allow(unreachable_code)]
fn get_character_info( html: String ) -> Result<Character,Box<dyn Error>>{
    let _document = scraper::Html::parse_document(&html);
    let character = Character {name: String::from(""), id: todo!(), rarity: Rarity::Extreme, tags: todo!(), main_ability: todo!(), ultra_ability: todo!(), base_stats: todo!(), max_stats: todo!() } ;
    return Ok(character)
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

    for link in extracted_links{
        let character = get_character_info(get_html(&link).await.unwrap());
        println!("{:?}",character)
    }
}


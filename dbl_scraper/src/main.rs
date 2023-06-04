mod structs;
mod enums;
use std::{ error::Error };
use serde::{ Deserialize, Serialize };
use serde_with::skip_serializing_none;
use structs::{
    MainAbility,
    UltraAbility,
    Stats,
    UniqueAbility,
    ZAbility,
    SpecialSkill,
    UltimateSkill,
    SpecialMove,
};
use enums::{ Rarity };
use scraper::{ Selector, Html };
use std::str::FromStr;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
struct Character {
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
    is_lf: bool,
    is_tag: bool,
    has_zenkai: bool,
}

async fn get_html(path: &str) -> Result<String, Box<dyn Error>> {
    let url = String::from("https://legends.dbz.space");
    let response = reqwest::Client
        ::new()
        .get(url + path)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"
        )
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .send().await?
        .text().await?;
    Ok(response)
}

fn remove_tabs_and_newlines(input: String) -> String {
    let modified_string = input.replace("\t", "").replace("\n", "");
    modified_string
}


fn get_inner_html(document: &Html, selector: &str) -> Option<String> {
    let sele = Selector::parse(selector).unwrap();
    Some(remove_tabs_and_newlines(document.select(&sele).next().unwrap().inner_html()))
}
fn get_text(document: &Html, selector: &str) -> Option<String> {
    let sele = Selector::parse(selector).unwrap();
    Some(
        remove_tabs_and_newlines(document.select(&sele).next().unwrap().text().collect::<String>())
    )
}
fn get_atribute(document: &Html, selector: &str, attr: &str) -> Option<String> {
    let sele = Selector::parse(selector).unwrap();
    Some(
        document.select(&sele).next().unwrap()
        .value().attr(attr).unwrap().to_string()
    )
}
#[allow(unreachable_code, unused_variables, unused_assignments)]
fn get_character_info(html: String) -> Result<Option<Character>, Box<dyn Error>> {
    let document = scraper::Html::parse_document(&html);

    let name = get_inner_html(&document, ".head.name.large.img_back>h1");

    let id = get_inner_html(&document, ".head.name.id-right.small.img_back").unwrap();

    let rarity = Rarity::from_str(get_inner_html(&document, ".rarity").unwrap().as_str()).unwrap();

    let tags_sele = Selector::parse(".ability.medium>a").unwrap();
    let tags: Vec<String> = document
        .select(&tags_sele)
        .map(|element| { remove_tabs_and_newlines(element.text().collect::<String>()) })
        .collect();

    let main_name = get_inner_html(&document, "div.frm.form0>.ability.medium").unwrap();
    let main_effect = get_text(&document, "div.frm.form0>.ability_text.small").unwrap();

    let main = MainAbility { name: main_name, effect: main_effect };

    let ultra;
    if
        let Some(ultra_name) = get_inner_html(
            &document,
            "#charaultra + div.ability_text > .frm.form0 > span"
        )
    {
        let ultra_effect = get_text(
            &document,
            "#charaultra + div.ability_text > .frm.form0 > div"
        ).unwrap();
        ultra = UltraAbility { name: ultra_name, effect: ultra_effect };
    }

    let unique_name1 = get_inner_html(
        &document,
        "#charaunique + div.ability_text > .frm.form0 > span"
    ).unwrap();
    let unique_effect1 = get_text(
        &document,
        "#charaunique + div.ability_text > .frm.form0> .ability_text"
    ).unwrap();

    let unique_name2 = get_inner_html(
        &document,
        "#charaunique + div.ability_text > .frm.form0:nth-of-type(2) > span"
    ).unwrap();
    let unique_effect2 = get_text(
        &document,
        "#charaunique + div.ability_text > .frm.form0:nth-of-type(2) > .ability_text"
    ).unwrap();

    let mut stats: Vec<String> = Vec::new();
    for i in 1..7 {
        let sele = format!(".row.lvlbreak.lvb1 > div:nth-of-type({}) > div.val", i );
        stats.push(get_atribute(&document,sele.as_str(),"raw").unwrap());
    }

    let base_stats = Stats {
        power: stats.get(0).unwrap().parse().unwrap(),
        health: stats.get(1).unwrap().parse().unwrap(),
        strike_atk: stats.get(2).unwrap().parse().unwrap(),
        strike_def: stats.get(3).unwrap().parse().unwrap(),
        blast_atk: stats.get(4).unwrap().parse().unwrap(),
        blast_def: stats.get(5).unwrap().parse().unwrap()
    };
    
    let mut stats2: Vec<String> = Vec::new();
    for i in 1..7 {
        let sele = format!(".row.lvlbreak.lvb5000 > div:nth-of-type({}) > div.val", i );
        stats2.push(get_atribute(&document,sele.as_str(),"raw").unwrap());
    }
    let max_stats = Stats {
        power: stats2.get(0).unwrap().parse().unwrap(),
        health: stats2.get(1).unwrap().parse().unwrap(),
        strike_atk: stats2.get(2).unwrap().parse().unwrap(),
        strike_def: stats2.get(3).unwrap().parse().unwrap(),
        blast_atk: stats2.get(4).unwrap().parse().unwrap(),
        blast_def: stats2.get(5).unwrap().parse().unwrap()
    };

    let image_url = get_atribute(&document, ".cutin.trs0.form0", "src");

    let strike = get_text(&document, ".ability_text.arts:nth-of-type(1) > .frm.form0 >.ability_text.small").unwrap();
    let shot = get_text(&document, ".ability_text.arts:nth-of-type(2) > .frm.form0 >.ability_text.small").unwrap();

    let special_move_name = get_text(&document, "selector");
    let special_move_effect = get_text(&document, "selector");
    // let character = Character {name: name, id: id, rarity: rarity, tags: tags, main_ability: main, ultra_ability: ultra, base_stats: base_stats, max_stats: todo!(), unique_ability: todo!(), strike: todo!(), shot: todo!(), special_move: todo!(), special_skill: todo!(), ultimate_skill: todo!(), z_ability: todo!(), image_url: todo!(), is_lf: todo!() } ;
    return Ok(None);
}
#[tokio::main]
async fn main() {
    color_eyre::install().unwrap();
    let url = "/characters/";

    let response = get_html(url).await.unwrap();

    let document = scraper::Html::parse_document(&response);
    let select_char = scraper::Selector::parse(".chara.list> a").unwrap();

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

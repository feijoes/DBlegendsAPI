fn main() {
    let response = reqwest::blocking::get(
        "https://legends.dbz.space/characters/",
    )
    .unwrap()
    .text()
    .unwrap();
    let document = scraper::Html::parse_document(&response);
    let select_char =  scraper::Selector::parse("chara list").unwrap();

    let list_char = document.select(&select_char);
    println!("{:?}",list_char)

}

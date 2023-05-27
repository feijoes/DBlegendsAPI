use std::{error::Error};
use tokio::runtime::Runtime;

async fn get_html(url:&str) -> Result<String, Box<dyn Error>> {
    color_eyre::install()?;
    let client = reqwest::Client::new();

    let request = client
        .get(url) 
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8");
    let response = request.send().await?;

    let body = response.text().await?;

    Ok(body)
}

fn main() {
    
    let url = "https://legends.dbz.space/characters/";
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        get_html(url).await.unwrap();
    });
}


use headless_chrome::browser::context::Context;
use headless_chrome::browser::tab::Tab;
use headless_chrome::protocol::page::ScreenshotFormat;
use std::error::Error;
use tokio::runtime::Runtime;

fn scrape_page_with_js(url: &str) -> Result<(), Box<dyn Error>> {
    // Create a new runtime
    let mut rt = Runtime::new().unwrap();

    // Create a new browser context
    let browser = rt.block_on(headless_chrome::Browser::new())?;
    let context = rt.block_on(browser.new_context())?;

    // Create a new tab in the context
    let tab = rt.block_on(context.new_tab())?;

    // Enable JavaScript on the tab
    rt.block_on(tab.enable_javascript())?;

    // Navigate to the desired URL
    rt.block_on(tab.navigate_to(url.into()))?;

    // Wait for JavaScript execution to complete (adjust the delay as needed)
    std::thread::sleep(std::time::Duration::from_secs(5));

    // Take a screenshot of the page
    let screenshot = rt.block_on(tab.capture_screenshot(ScreenshotFormat::PNG))?;

    // Save the screenshot to a file named "index.html"
    std::fs::write("index.html", screenshot)?;

    Ok(())
}

fn main() {
    let url = "https://example.com";

    match scrape_page_with_js(url) {
        Ok(()) => println!("Page scraped successfully!"),
        Err(err) => eprintln!("An error occurred: {:?}", err),
    }
}

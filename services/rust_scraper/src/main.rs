use std::env;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let server = env::var("SELENIUM_SERVER").unwrap_or("http://localhost:9515".to_string());
    let target = env::var("SELENIUM_TARGET").expect("TODO:");

    let chrome = DesiredCapabilities::chrome();
    let driver = WebDriver::new(&server, chrome).await?;

    driver.goto(&target).await?;

    // TODO:

    driver.quit().await?;

    Ok(())
}

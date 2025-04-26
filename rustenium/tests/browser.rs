use tokio::time::{Duration, sleep};
use rustenium::{Driver, ChromeDriver};
use rustenium_core::session::SessionConnectionType;

async fn create_browser() -> ChromeDriver<'static> {
    let mut browser = ChromeDriver::default();
    browser.driver.exe_path = "D:/Documents/m-workspace/rustenium/apps/drivers/chromedriver.exe";
    browser
}

#[tokio::test]
async fn open_browser() {
    let mut browser = create_browser().await;
    browser.launch(None, None).await;
    sleep(Duration::from_secs(13)).await;
}
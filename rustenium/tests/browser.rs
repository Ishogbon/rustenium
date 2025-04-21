use tokio::time::{Duration, sleep};
use rustenium::{Driver, ChromeDriver};
use rustenium_core::session::SessionConnectionType;

async fn create_browser() -> ChromeDriver {
    let mut browser = ChromeDriver::default();
    browser.driver.exe_path = "D:/Documents/m-workspace/rustenium/apps/browsers/chrome-win64/chrome.exe";
    browser.launch(None, None).await;
    return browser;
}
// #[tokio::test]
// async fn open_browser() {
//     create_browser().await;
// }
// 
#[tokio::test]
async fn new_session() {
    let mut browser = create_browser().await;
    browser.create_new_session(SessionConnectionType::WebSocket).await.unwrap();
    sleep(Duration::from_secs(3)).await;
}
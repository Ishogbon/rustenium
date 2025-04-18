use rustenium::{Browser, ChromeBrowser};
use rustenium_core::session::SessionConnectionType;

async fn create_browser() -> ChromeBrowser {
    let mut browser = ChromeBrowser::default();
    browser.browser.exe_path = "D:/Documents/m-workspace/rustenium/apps/browsers/chrome-win64/chrome.exe";
    browser.launch().await;
    return browser;
}
// #[tokio::test]
// async fn open_browser() {
//     create_browser().await;
// }

#[tokio::test]
async fn new_session() {
    let mut browser = create_browser().await;
    browser.create_new_session(SessionConnectionType::WebSocket).await.unwrap();
}
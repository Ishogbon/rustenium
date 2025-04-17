use rustenium::ChromeBrowser;

async fn create_browser() {
    let mut browser = ChromeBrowser::default();
    browser.browser.exe_path = "D:/Documents/m-workspace/rustenium/apps/browsers/chrome-win64/chrome.exe";
    browser.launch().await;
}
#[tokio::test]
async fn open_browser() {
    create_browser().await;
}

#[tokio::test]
async fn new_session() {
    let browser = create_browser().await;
    browser.
}
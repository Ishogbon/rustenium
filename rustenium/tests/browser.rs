use rustenium::ChromeBrowser;

#[tokio::test]
async fn open_browser() {
    let mut browser = ChromeBrowser::default();
    browser.browser.exe_path = "D:/Documents/m-workspace/rustenium/apps/browsers/chrome-win64/chrome.exe";
    browser.launch().await;
}

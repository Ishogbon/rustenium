use rustenium::ChromeBrowser;

#[tokio::test]
async fn open_browser() {
    let mut browser = ChromeBrowser::default();
    browser.browser.exe_path = "/Users/macbookpro/m-workspace/rustenium/apps/browsers/chrome/chromebrowser.app/Contents/MacOS/Google Chrome for Testing";
    browser.launch().await;
}

use rustenium::Browser;

#[tokio::test]
async fn open_browser() {
    let mut browser = Browser::default();
    browser.exe_path = "/Users/macbookpro/m-workspace/rustenium/apps/browsers/chrome/chromebrowser.app/Contents/MacOS/Google Chrome for Testing";
    browser.open().await;
}

use rustenium::Browser;

#[tokio::test]
async fn open_browser() {
    let mut browser = Browser::default();
    browser.open().await;
}

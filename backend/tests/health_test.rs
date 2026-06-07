mod helpers;

use helpers::TestApp;

#[tokio::test]
async fn health_check_returns_ok() {
    let app = TestApp::spawn().await;
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/health", app.address))
        .send()
        .await
        .expect("request failed");

    assert_eq!(res.status(), 200);
    let body: serde_json::Value = res.json().await.unwrap();
    assert_eq!(body["status"], "ok");
}

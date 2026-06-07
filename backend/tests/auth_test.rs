mod helpers;

use helpers::TestApp;

#[tokio::test]
async fn register_returns_201_with_token_and_user() {
    let app = TestApp::spawn().await;

    let res = app.register("newuser@example.com", "password123").await;
    assert_eq!(res.status(), 201);

    let body: serde_json::Value = res.json().await.unwrap();
    assert!(body["token"].as_str().is_some_and(|t| !t.is_empty()));
    assert_eq!(body["user"]["email"], "newuser@example.com");
    assert_eq!(body["user"]["plan"], "free");
    assert!(body["user"]["id"].as_str().is_some());
    // password_hash 는 절대 노출되지 않아야 한다.
    assert!(body["user"]["password_hash"].is_null());
}

#[tokio::test]
async fn duplicate_email_register_returns_400() {
    let app = TestApp::spawn().await;

    let first = app.register("dup@example.com", "password123").await;
    assert_eq!(first.status(), 201);

    let second = app.register("dup@example.com", "password456").await;
    assert_eq!(second.status(), 400);
}

#[tokio::test]
async fn login_returns_200_with_token() {
    let app = TestApp::spawn().await;
    app.register("login@example.com", "password123").await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/api/v1/auth/login", app.address))
        .json(&serde_json::json!({ "email": "login@example.com", "password": "password123" }))
        .send()
        .await
        .expect("login request failed");

    assert_eq!(res.status(), 200);
    let body: serde_json::Value = res.json().await.unwrap();
    assert!(body["token"].as_str().is_some_and(|t| !t.is_empty()));
    assert_eq!(body["user"]["email"], "login@example.com");
}

#[tokio::test]
async fn login_with_wrong_password_returns_401() {
    let app = TestApp::spawn().await;
    app.register("wrongpw@example.com", "password123").await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/api/v1/auth/login", app.address))
        .json(&serde_json::json!({ "email": "wrongpw@example.com", "password": "wrongpassword" }))
        .send()
        .await
        .expect("login request failed");

    assert_eq!(res.status(), 401);
}

#[tokio::test]
async fn login_with_unknown_email_returns_401() {
    let app = TestApp::spawn().await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/api/v1/auth/login", app.address))
        .json(&serde_json::json!({ "email": "ghost@example.com", "password": "password123" }))
        .send()
        .await
        .expect("login request failed");

    assert_eq!(res.status(), 401);
}

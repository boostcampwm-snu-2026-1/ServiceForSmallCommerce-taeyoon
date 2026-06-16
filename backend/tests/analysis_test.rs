mod helpers;

use helpers::TestApp;
use std::time::Duration;

const PRODUCT_URL: &str = "https://www.coupang.com/vp/products/12345";

/// completed/failed 가 될 때까지 유한 폴링하고 최종 body 를 반환한다.
async fn poll_until_terminal(app: &TestApp, token: &str, analysis_id: &str) -> serde_json::Value {
    let client = reqwest::Client::new();
    for _ in 0..50 {
        let res = client
            .get(format!("{}/api/v1/analyses/{}", app.address, analysis_id))
            .bearer_auth(token)
            .send()
            .await
            .expect("get analysis failed");
        assert_eq!(res.status(), 200);
        let body: serde_json::Value = res.json().await.unwrap();
        let status = body["status"].as_str().unwrap_or("");
        if status == "completed" || status == "failed" {
            return body;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    panic!("analysis did not reach terminal state in time");
}

#[tokio::test]
async fn create_poll_list_and_usage_flow() {
    let app = TestApp::spawn().await;
    let token = app
        .register_and_token("analyst@example.com", "password123")
        .await;
    let client = reqwest::Client::new();

    // ① POST → 202 + analysis_id
    let res = client
        .post(format!("{}/api/v1/analyses", app.address))
        .bearer_auth(&token)
        .json(&serde_json::json!({ "my_url": PRODUCT_URL, "competitor_urls": [PRODUCT_URL], "review_limit": 50 }))
        .send()
        .await
        .expect("create analysis failed");
    assert_eq!(res.status(), 202);
    let body: serde_json::Value = res.json().await.unwrap();
    let analysis_id = body["analysis_id"]
        .as_str()
        .expect("analysis_id should exist")
        .to_string();
    assert_eq!(body["status"], "pending");

    // ② 폴링 → completed + result.products / insights 존재
    let detail = poll_until_terminal(&app, &token, &analysis_id).await;
    assert_eq!(detail["status"], "completed", "detail={detail}");
    let products = detail["result"]["products"]
        .as_array()
        .expect("products should be an array");
    assert!(!products.is_empty());
    assert!(detail["result"]["insights"].is_object());
    // my_url 노출 + products[].is_mine 존재 확인
    assert!(detail["my_url"].is_string());
    assert!(products[0]["is_mine"].is_boolean());
    // 노출 금지 필드 확인
    assert!(detail["user_id"].is_null());
    assert!(detail["review_limit"].is_null());

    // ③ GET 목록 → 1건 이상
    let res = client
        .get(format!("{}/api/v1/analyses", app.address))
        .bearer_auth(&token)
        .send()
        .await
        .expect("list failed");
    assert_eq!(res.status(), 200);
    let list: serde_json::Value = res.json().await.unwrap();
    assert!(!list["analyses"].as_array().unwrap().is_empty());
    assert!(list["total"].as_i64().unwrap() >= 1);
    assert_eq!(list["page"], 1);
    assert_eq!(list["per_page"], 20);

    // ④ GET /users/me → 200, email 일치, usage 확인
    let res = client
        .get(format!("{}/api/v1/users/me", app.address))
        .bearer_auth(&token)
        .send()
        .await
        .expect("users/me failed");
    assert_eq!(res.status(), 200);
    let me: serde_json::Value = res.json().await.unwrap();
    assert_eq!(me["email"], "analyst@example.com");
    assert!(me["usage"]["analyses_this_month"].as_i64().unwrap() >= 1);
    assert!(me["usage"]["analyses_limit"].is_null());
}

#[tokio::test]
async fn create_without_token_returns_401() {
    let app = TestApp::spawn().await;
    let res = reqwest::Client::new()
        .post(format!("{}/api/v1/analyses", app.address))
        .json(&serde_json::json!({ "my_url": PRODUCT_URL, "competitor_urls": [PRODUCT_URL], "review_limit": 100 }))
        .send()
        .await
        .expect("request failed");
    assert_eq!(res.status(), 401);
}

#[tokio::test]
async fn create_with_invalid_urls_returns_400() {
    let app = TestApp::spawn().await;
    let token = app
        .register_and_token("badinput@example.com", "password123")
        .await;
    let client = reqwest::Client::new();

    // competitor_urls 0개
    let res = client
        .post(format!("{}/api/v1/analyses", app.address))
        .bearer_auth(&token)
        .json(&serde_json::json!({ "my_url": PRODUCT_URL, "competitor_urls": [], "review_limit": 100 }))
        .send()
        .await
        .expect("request failed");
    assert_eq!(res.status(), 400);

    // competitor_urls 4개
    let res = client
        .post(format!("{}/api/v1/analyses", app.address))
        .bearer_auth(&token)
        .json(&serde_json::json!({
            "my_url": PRODUCT_URL,
            "competitor_urls": [PRODUCT_URL, PRODUCT_URL, PRODUCT_URL, PRODUCT_URL],
            "review_limit": 100
        }))
        .send()
        .await
        .expect("request failed");
    assert_eq!(res.status(), 400);
}

#[tokio::test]
async fn other_user_cannot_access_analysis() {
    let app = TestApp::spawn().await;
    let owner_token = app
        .register_and_token("owner@example.com", "password123")
        .await;
    let client = reqwest::Client::new();

    let res = client
        .post(format!("{}/api/v1/analyses", app.address))
        .bearer_auth(&owner_token)
        .json(&serde_json::json!({ "my_url": PRODUCT_URL, "competitor_urls": [PRODUCT_URL], "review_limit": 100 }))
        .send()
        .await
        .expect("create failed");
    assert_eq!(res.status(), 202);
    let body: serde_json::Value = res.json().await.unwrap();
    let analysis_id = body["analysis_id"].as_str().unwrap().to_string();

    // 다른 유저로 조회 → 404
    let other_token = app
        .register_and_token("intruder@example.com", "password123")
        .await;
    let res = client
        .get(format!("{}/api/v1/analyses/{}", app.address, analysis_id))
        .bearer_auth(&other_token)
        .send()
        .await
        .expect("get failed");
    assert_eq!(res.status(), 404);
}

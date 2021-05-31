use {
    actix_web::{web, App, HttpResponse, HttpServer},
    std::{sync::mpsc, thread, time::Duration},
};

#[actix_rt::test]
async fn test_start() {
    use actix_http::client;
    let addr = "localhost:8443";
    let client = awc::Client::builder()
        .connector(
            client::Connector::new()
                .timeout(Duration::from_millis(100))
                .finish(),
        )
        .finish();

    let host = format!("https://{}", addr);
    let response = client.get(host.clone()).send().await.unwrap();

    assert!(response.status().is_success());
}

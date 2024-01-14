use super::*;
use actix_web::{body::to_bytes, http, test, web, App};
use cloud_storage::Client;
use std::fs::File;
use std::io::Write;

#[actix_rt::test]
async fn test_download_and_send_file() {
    env_logger::init();

    log::info!("test_download_and_send_file");

    let config_content =
        fs::read_to_string("/etc/storage-gateway/config.yaml").expect("Failed to read config file");

    let config: Config =
        serde_yaml::from_str(&config_content).expect("Failed to parse config file");

    // Initialize the test server with the configuration and routes
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(Client::default())) // Mock or actual GCS client
            .app_data(web::Data::new(config))
            .route("/{file_name:.*}", web::get().to(download_and_send_file)),
    )
    .await;

    // Create a request to the endpoint
    let req = test::TestRequest::get().uri("/comfyui.png").to_request();

    // Send the request
    let mut resp = test::call_service(&mut app, req).await;

    // Log the resp
    println!("resp: {:?}", resp);

    assert_eq!(resp.status(), http::StatusCode::OK);

    // Extract the body as Bytes
    let body = to_bytes(resp.into_body())
        .await
        .expect("Failed to extract body");

    // Write the response body to a file
    let mut file = File::create("downloaded_file.png").expect("Failed to create file");
    file.write_all(&body).expect("Failed to write to file");
}

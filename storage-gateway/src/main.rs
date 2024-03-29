extern crate diesel;

use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use cloud_storage::bucket::{Location, NALocation, SingleRegion};
use cloud_storage::{Client, ListRequest, NewBucket};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

use serde::Deserialize;
use std::fs;

mod api;

#[cfg(test)]
mod tests;

use futures::stream::StreamExt;
use thiserror::Error; // import StreamExt trait

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    //#[error("Cloud storage error")]
    //CloudStorage(#[from] cloud_storage::Error),
    #[error("Cloud storage error")]
    CloudStorage(#[from] cloud_storage::Error),

    // Create a custom error type with a string message
    #[error("Custom error: {0}")]
    Custom(String),
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    database_host: String,
    database_name: String,
    database_username: String,
    database_password: String,

    bind_address: String,
    bind_port: u16,

    app_env: String,
    gcs_bucket_name: String,
    gcs_bucket_subdirectory: String,
}

#[actix_web::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    // Manual error handling
    if let Err(e) = run_app().await {
        // Log the error and return an io::Error
        log::error!("Application error: {}", e);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        ));
    }

    Ok(())
}

async fn run_app() -> Result<(), MyError> {
    env_logger::init();
    log::info!("Starting server");

    let config_content =
        fs::read_to_string("/etc/storage-gateway/config.yaml").expect("Failed to read config file");

    let config: Config =
        serde_yaml::from_str(&config_content).expect("Failed to parse config file");

    let service_account_file = std::env::var("SERVICE_ACCOUNT").unwrap();
    let service_account_file_content =
        fs::read_to_string(service_account_file).expect("Failed to read service account file");

    log::info!("Reading from cloud storage");
    let bucket_name = &config.gcs_bucket_name;
    log::info!("bucket_name: {}", bucket_name);

    let client = Client::default();
    let all_objects_stream = client
        .object()
        .list(bucket_name, ListRequest::default())
        .await?;
    let all_objects: Vec<_> = all_objects_stream.collect().await;

    // Filter objects by gcs_bucket_subdirectory
    for object_result in all_objects {
        if let Ok(object_list) = object_result {
            for object in object_list.items {
                if object.name.starts_with(&config.gcs_bucket_subdirectory) {
                    log::info!("object: {:?}", object.name);
                    // Pretty print the object
                    // log::info!("{:#?}", object);
                }
            }
        }
    }

    let database_url = format!(
        "postgres://{}:{}@{}/{}",
        config.database_username,
        config.database_password,
        config.database_host,
        config.database_name
    );
    log::info!("database_url: {}", database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let config_clone = config.clone();
    HttpServer::new(move || {
        // This closure runs for every worker thread,
        // so if we log in here, we'll expect to see it appear
        // 24 times (assuming 24 workers).

        let cors = match config.app_env {
            ref x if x == "dev" => Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600),
            ref x if x == "prod" => Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600),
            _ => panic!("APP_ENV must be set to either 'dev' or 'prod'"),
        };

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .route("/{file_name:.*}", web::get().to(download_and_send_file))
        // .service(
        //     web::scope("/api")
        //         .service(
        //             web::resource("/weights/{weight_id}")
        //                 .route(web::patch().to(api::patch_weights))
        //                 .route(web::delete().to(api::delete_weights)),
        //         )
        //         .service(
        //             web::resource("/weights")
        //                 .route(web::get().to(api::get_weights))
        //                 .route(web::post().to(api::post_weights)),
        //         ),
        // )
        // .service(actix_files::Files::new("/", "/track/server/static/client/").index_file("index.html"))
    })
    .bind(format!(
        "{}:{}",
        config_clone.bind_address, config_clone.bind_port
    ))?
    .run()
    .await;

    Ok(())
}

use actix_web::{HttpResponse, Responder, Result};

pub async fn download_and_send_file(
    config: web::Data<Config>,
    file_name: web::Path<String>,
) -> Result<impl Responder> {
    let client = Client::default();

    let full_file_name = format!("{}/{}", config.gcs_bucket_subdirectory, file_name);

    log::info!("full_file_name: {}", full_file_name);

    let object_result = client
        .object()
        .download(&config.gcs_bucket_name, &full_file_name)
        .await;

    log::info!("object_result: {:?}", object_result);

    match object_result {
        Ok(bytes) => Ok(HttpResponse::Ok()
            .content_type("application/octet-stream")
            .body(bytes)),
        Err(e) => {
            log::error!("Error downloading file: {}", e);
            Ok(HttpResponse::InternalServerError().body("Internal Server Error"))
        }
    }
}

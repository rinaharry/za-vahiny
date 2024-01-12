use actix_web::{web, App, HttpServer, dev::Url};
use log::info;
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;


#[derive(serde::Deserialize)]
struct Info {
  name: String,
}

// Basic GET / endpoint
async fn index() -> &'static str {
    "Hello world!"
}

// POST /info endpoint to create info
async fn create_info(info: web::Json<Info>) -> String {
    format!("Welcome {}!", info.name) 
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init();
    dotenv().ok();

    info!("Starting actix-web server...");

    let port_url = env::var("PORT").expect("PORT_URL not found in .env");
    let url = format!("127.0.0.1:{PORT}", PORT = port_url);
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/info", web::post().to(create_info))
            
    })
    .bind(url)?;

    server.run().await?;

    info!("Server has shut down");

    Ok(())
}
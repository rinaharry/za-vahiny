
use actix_web::{get, http, web, App, HttpRequest, HttpResponse, HttpServer};
use log::info;
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;
use postgres::{ Client, NoTls };
use postgres::Error as PostgresError;


#[derive(serde::Deserialize)]
struct Info {
  name: String,
}

//DATABASE URL
const DB_URL: &str = env!("DATABASE_URL");



//db setup
fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )
    "
    )?;
    Ok(())
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
        let cors = Cors::default()
                             .allow_any_origin().send_wildcard();
        App::new()
            .wrap(cors)
            .route("/", web::get().to(index))
            .route("/info", web::post().to(create_info))
            
    })
    .bind(url)?;

    server.run().await?;

    info!("Server has shut down");

    Ok(())
}
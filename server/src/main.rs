

extern crate diesel;

use actix_cors::Cors;
use actix_web::{middleware, web, App ,post, HttpResponse, HttpServer};
use dotenv::dotenv;

use std::env;
use log::{debug, error, log_enabled, info, Level};
use env_logger;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
// We define a custom type for connection pool to use later.
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(serde::Deserialize)]
struct InfoPingPong {
  name: String,
}

// Basic GET / endpoint
async fn index() -> &'static str {
    "Server is ok!"
}

// POST /info endpoiDbPoolnt to create info
async fn create_info(res: web::Json<InfoPingPong>) -> String {
    format!("Welcome {}!", res.name) 
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    // Initialize logger
        env_logger::init();
        dotenv().ok(); 

      // set up database connection pool
      let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in .env");
      let manager = ConnectionManager::<PgConnection>::new(database_url);
      let pool: DbPool = r2d2::Pool::builder()
          .build(manager)
          .expect("Failed to create pool.");
        debug!("Starting actix-web server...");
        info!("Starting actix-web server...");

        let port_url: String = env::var("PORT").expect("PORT_URL not found in .env");
        let url: String = format!("0.0.0.0:{PORT}", PORT = port_url);
        
        let server = HttpServer::new(move || {
          App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap( Cors::permissive())
            .route("/", web::get().to(index))
            .route("/info", web::post().to(create_info))    
    })
    .bind(url)?;

    server.run().await?;
    debug!("Server has shut down");
    Ok(())

}
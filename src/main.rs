use std::env;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web::Data, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::setup::setup_tables;

mod routes;
mod utils;
mod setup;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info,audio=info");

    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    log::info!("Database URL: {}", database_url);

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            log::info!("Connected to database");
            setup_tables(&pool).await;
            pool
        }
        Err(err) => {
            log::error!("Failed to connect to the database: {:?}", err);
            panic!();
        }
    };

    HttpServer::new(move || {
        let logger = Logger::default();
        
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .wrap(logger)
            .wrap(cors)
            .service(routes::core::ping::ping)
            .service(routes::auth::user_signup::user_signup)
            .service(routes::auth::user_login::user_login)
            .service(routes::auth::session_refresh::session_refresh)
            .service(routes::auth::session_info::session_info)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}

use std::env;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};

mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();

        App::new().wrap(logger).service(routes::core::ping::ping)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}

mod api;
mod models;

use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting Git HTTP server at http://0.0.0.0:4545");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(api::git::info_refs)
            .service(api::git::service_rpc)
    })
    .bind(("0.0.0.0", 4545))?
    .run()
    .await
}

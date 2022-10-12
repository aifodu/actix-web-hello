mod api;

use dotenv::dotenv;

use api::task::get_task;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let port: String = std::env::var("PORT").expect("PORT not provided.");

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).service(get_task)
    })
    .bind(("127.0.0.1", port.parse::<u16>().unwrap()))?
    .run()
    .await
}

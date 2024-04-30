use actix_web::{App, HttpServer};
use std::env;
use dotenv::dotenv;

mod user;
use user::api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_url = env::var("SERVER_URL").expect("SERVER_URLS must be set");
    HttpServer::new(move || {
        App::new()
        .service(api::web_status)
        .service(api::web_login)
        .service(api::web_register)
        .service(api::web_announcement)
        .service(api::get_ip)
    })
        .bind(server_url)?
        .run()
        .await
}
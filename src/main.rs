use actix_web::{web, App, HttpServer};
use dotenv;

mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let port = dotenv::var("PORT").unwrap_or(String::from("3000"));

  HttpServer::new(|| {
    App::new()
      .route("/", web::get().to(controllers::happy_new_year))
      .route("/{name}", web::get().to(controllers::happy_new_year))
  })
  .bind(format!("127.0.0.1:{}", port))?
  .run()
  .await
}

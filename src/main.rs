use actix_cors::Cors;
use actix_web::{web, App, HttpServer, http};

mod api;
mod models;
mod repository;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
let todo_db = repository::database::Database::new();
let app_data = web::Data::new(todo_db);

HttpServer::new(move || {
    let cors = Cors::default()
              .allowed_origin("localhost:3001")
              .allowed_origin_fn(|origin, _req_head| {
                  origin.as_bytes().ends_with(b"3001")
              })
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);

    App::new()
        .wrap(cors)
        .app_data(app_data.clone())
        .configure(api::api::config)
        .wrap(actix_web::middleware::Logger::default())
}).bind(("127.0.0.1", 3000))?
.run()
.await
}

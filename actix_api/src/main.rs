mod controllers;
mod types;
mod utils;

use actix_cors::Cors;
use actix_web::{
    App,
    HttpServer,
    http::header,
    middleware::Logger,
    web
};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use crate::controllers::post::{ index, add_post };

fn setup_db() -> Pool<SqliteConnectionManager> {
  let manager = SqliteConnectionManager::file("actix_yew.db");
  let pool = Pool::new(manager).expect("Failed to initialize connection pool");
  let conn = pool
      .get()
      .expect("Failed to connect");

  conn.execute(
    "CREATE TABLE IF NOT EXISTS post(
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      title TEXT NOT NULL,
      body TEXT NOT NULL
    )",
    params![]
  ).expect("Failed to create table");
  pool
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error>{
  // set up logger
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  // setup database
  let pool = setup_db();
    
  HttpServer::new(move || {
    let cors = Cors::default()
        // for request from client side
        .allowed_origin("http://127.0.0.1:8080")
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600);

    App::new()
      .wrap(Logger::default())
      .wrap(cors)
      .service(index)
      .service(add_post)
      .app_data(web::Data::new(pool.clone()))
  })
  .bind("127.0.0.1:3000")?
  .run()
  .await?;
  Ok(())
}

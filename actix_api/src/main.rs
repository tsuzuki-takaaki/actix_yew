mod types;
mod utils;

use actix_cors::Cors;
use actix_web::{
    get,
    post,
    App,
    HttpServer,
    HttpResponse,
    http::header,
    middleware::Logger,
    web
};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use crate::types::post::{ Post, NewPost };
use crate::utils::errors::{ MyError };


#[post("/posts")]
async fn add_post(
  form: web::Json<NewPost>, // Deserialized
  db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, MyError> {
  let conn = db.get()?;

  // Todo [mkdir actions for model logic]
  conn.execute(
    "INSERT INTO post (title, body) VALUES (?1, ?2)",
    params![form.title, form.body]
  )?;
    
  Ok(HttpResponse::Accepted().into())
}

#[get("/")]
async fn index(
  db: web::Data<Pool<SqliteConnectionManager>>
) -> Result<web::Json<Vec<Post>>, MyError> {//Result<HttpResponse, MyError> {

  // Todo [mkdir actions for model logic]
  let conn = db.get()?;
  let mut stmt = conn.prepare("SELECT id, title, body FROM post")?;

  // execute SQL from prepared statement
  let rows = stmt.query_map(params![], |row| {
    let id = row.get(0)?;
    let title = row.get(1)?;
    let body = row.get(2)?;
    Ok(Post {id, title, body})
  })?;

  let mut entries = Vec::new();
  for row in rows {
      entries.push(row?);
  }
  Ok(web::Json(entries))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error>{
  // set up logger
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  // Todo [make function for database setup]
  // establish connection to db
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
    
  HttpServer::new(move || {
    let cors = Cors::default()
        .allowed_origin("http://127.0.0.1:8080") // Todo [define constant]
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

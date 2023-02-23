use actix_web::{
  get,
  post,
  web,
  HttpResponse,
};
use rusqlite::params;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::types::post::{ Post, NewPost };
use crate::utils::errors::{ MyError };

// GET /posts
#[get("/posts")]
pub async fn index(
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

// POST /posts
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

// GET /posts/:id

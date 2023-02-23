use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize)]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct NewPost {
  pub title: String,
  pub body: String,
}

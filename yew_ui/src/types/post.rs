// Todo [make unified directory for backend and client types]

use yew::prelude::*;
use serde::{ Deserialize };

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub body: String,
}

#[derive(Properties, PartialEq)]
pub struct PostsListProps {
  pub posts: Vec<Post>,
    pub on_click: Callback<Post>
}

#[derive(Properties, PartialEq)]
pub struct PostDetailProps {
  pub post: Post,
}

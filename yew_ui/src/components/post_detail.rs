use yew::prelude::*;

use crate::types::post::PostDetailProps;

// Todo [struct atomic design]
#[function_component(PostDetails)]
pub fn post_details(
  PostDetailProps{ post }: &PostDetailProps
) -> Html {
  html! {
    <div>
      <h3>{post.title.clone()}</h3>
      <p>{post.body.clone()}</p>
    </div>
  }
}

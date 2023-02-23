use yew::prelude::*;

use crate::types::post::PostsListProps;

// Todo [struct atomic design]
#[function_component(PostsList)]
pub fn posts_list(
  PostsListProps { posts, on_click }: &PostsListProps
) -> Html {
  let on_click = on_click.clone();

  posts.iter().map(|post| {
    let on_post_select = {
      let on_click = on_click.clone();
      let post = post.clone();
      Callback::from(move |_| {
        on_click.emit(post.clone())
      })
    };
    html! {
      <p key={post.id} onclick={on_post_select}>{format!("{}", post.title)}</p>
    }
  }).collect()
}

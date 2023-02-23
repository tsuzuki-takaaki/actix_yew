use gloo_net::http::Request;
use yew::prelude::*;

use crate::components::post_list::PostsList;
use crate::components::post_detail::PostDetails;
use crate::components::text::Text;
use crate::types::post::Post;


#[function_component(App)]
pub fn app() -> Html {
  let posts = use_state(|| vec![]);

  // Todo [I understand nothing]
  {
    let posts = posts.clone();

    use_effect_with_deps(move |_| {
      let posts = posts.clone();
      wasm_bindgen_futures::spawn_local(async move {
        let fetched_posts: Vec<Post> = Request::get("http://localhost:3000/posts") // Todo [define constant]
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        println!("{:?}", fetched_posts);
        posts.set(fetched_posts);
      });
      || ()
    }, ());
  }

  let selected_post = use_state(|| None);
  let on_post_select = {
    let selected_post = selected_post.clone();
    Callback::from(move |post: Post| {
      selected_post.set(Some(post))
    })
  };

  let details = selected_post.as_ref().map(|post| 
    html! {
      <PostDetails post={post.clone()} />
    }
  );

  html! {
    <>
      <h2>{"Hi, there!"}</h2>
      <Text />
      <PostsList posts={(*posts).clone()} on_click={on_post_select.clone()}/>
      {for details }
    </>
  }
}

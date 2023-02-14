#![allow(unused_imports)]
use gloo_net::http::Request;
use gloo_console::log;
use serde::{ Deserialize };
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Deserialize)]
struct Post {
  id: i32,
  title: String,
  body: String,
}
#[derive(Properties, PartialEq)]
struct PostsListProps {
  posts: Vec<Post>,
    on_click: Callback<Post>
}

#[derive(Properties, PartialEq)]
struct PostDetailProps {
  post: Post,
}

#[function_component(PostDetails)]
fn post_details(
  PostDetailProps{ post }: &PostDetailProps
) -> Html {
  html! {
    <div>
      <h3>{post.title.clone()}</h3>
      <p>{post.body.clone()}</p>
    </div>
  }
}

#[function_component(PostsList)]
fn posts_list(
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

#[function_component(App)]
fn app() -> Html {
  let posts = use_state(|| vec![]);

  // Todo [I understand nothing]
  {
    let posts = posts.clone();

    use_effect_with_deps(move |_| {
      let posts = posts.clone();
      wasm_bindgen_futures::spawn_local(async move {
        let fetched_posts: Vec<Post> = Request::get("http://localhost:3000") // Todo [define constant]
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
      <h1>{"Yew UI"}</h1>
      <Text />
      <PostsList posts={(*posts).clone()} on_click={on_post_select.clone()}/>
      {for details }
    </>
  }
}

// Todo [make file for component]
#[function_component(Text)]
fn text() -> Html {
  html!(
    <div class={classes!("markdown_container")}>
      <div class={classes!("preparse_area")}>
        <input class={classes!("title_input")}/>
        <textarea class={classes!("markdown_textarea")}/>
      </div>
      <div class={classes!("parsed_area")}>
      </div>
    </div>
  )
}

fn main() {
  yew::Renderer::<App>::new().render();
}

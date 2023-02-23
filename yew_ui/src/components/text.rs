use yew::prelude::*;
use web_sys::{ HtmlInputElement, HtmlTextAreaElement };
use crate::utils::parse_markdown::parse_markdown;

// Todo [struct atomic design]
#[function_component(Text)]
pub fn text() -> Html {

  let title_value = use_state(|| String::from(""));
  let content_value = use_state(|| String::from(""));

  let handle_title_input = {
    let title_value = title_value.clone();
    Callback::from(move |e: InputEvent| {
      let new_title_value: HtmlInputElement = e.target_unchecked_into();
      title_value.set(new_title_value.value());
    })
  };

  let handle_content_value = {
    let content_value = content_value.clone();
    Callback::from(move |e: InputEvent| {
      let new_content_value: HtmlTextAreaElement = e.target_unchecked_into();
      content_value.set(new_content_value.value());
    })
  };

  let parsed_content_value = parse_markdown(&content_value);

  html!(
    <div class={classes!("editor-container")}>
      <form>
        <div class={classes!("title-edit")}>
          <input
            class={classes!("title-input")}
            oninput={handle_title_input}
            value={(*title_value).clone()}
          />
        </div>
        <div class={classes!("realtime-content")}>
          <textarea
            class={classes!("preparsed_content")}
            oninput={handle_content_value}
            value={(*content_value).clone()}
          />
          <div class={classes!("parsed_content")}>
            { parsed_content_value }
          </div>
        </div>
      </form>
    </div>
  )
}

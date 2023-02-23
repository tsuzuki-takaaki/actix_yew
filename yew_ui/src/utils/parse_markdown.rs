use web_sys::{ Node };
use yew::prelude::*;
use yew::virtual_dom::VNode;

pub fn parse_markdown(target: &String) -> Html {
  let parser = pulldown_cmark::Parser::new(target);
  let mut html_output = String::new();
  pulldown_cmark::html::push_html(&mut html_output, parser);

  // creating node from result of markdown parse
  let div = web_sys::window()
      .unwrap()
      .document()
      .unwrap()
      .create_element("div")
      .unwrap();
  div.set_inner_html(&html_output);
  let node = Node::from(div);
  VNode::VRef(node)
}

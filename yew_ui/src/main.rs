mod components;
mod types;
mod utils;

use crate::components::app::App;

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::Renderer::<App>::new().render();
}

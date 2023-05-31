#![feature(iter_array_chunks)]
#![feature(try_blocks)]
#![feature(let_chains)]

pub mod app;
pub mod color_picker;
pub mod components;
pub mod routes;
pub mod settings;
pub mod string_utils;
pub mod toasts;
pub mod utils;

use cfg_if::cfg_if;
use leptos::*;
use wasm_bindgen::prelude::*;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {use app::*;
      use leptos::*;

      // initializes logging using the `log` crate
      _ = console_log::init_with_level(log::Level::Debug);
      console_error_panic_hook::set_once();

      leptos::mount_to_body(move |cx| {
          view! { cx, <App/> }
      });
    }
}
}

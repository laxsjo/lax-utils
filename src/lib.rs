#![feature(iter_array_chunks)]

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

#[component]
pub fn MultiplyWidget(cx: Scope, label: String) -> impl IntoView {
    let (value_str, set_value_str) = create_signal(cx, "0".to_owned());

    let computed_value = move || {
        let Ok(value) = value_str().parse::<i32>() else {
            return "invalid input".to_owned();
        };
        (value * 2 + 1).to_string()
    };

    view! { cx,
        <input type="number"
            on:input=move |ev| {
                set_value_str(event_target_value(&ev));
            }
            prop:value=value_str
        />

        <p>{value_str} " * 2 + 1 = " {computed_value} " (" {label} ")"</p>
    }
}

/// umm I should really document this...
/// The listener function should take any web-sys event type. (**I don't think
/// it's type checked in any way tough! D:**)
pub fn wrap_closure_as_event_listener<
    E: Into<web_sys::Event> + wasm_bindgen::convert::FromWasmAbi + 'static,
>(
    f: impl Fn(E) + 'static,
) -> JsValue {
    let handler = Box::new(f) as Box<dyn FnMut(E)>;

    Closure::wrap(handler).into_js_value() // TODO: does this leak memory?
}

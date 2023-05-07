pub mod app;
pub mod color_picker;
pub mod components;
pub mod routes;
use cfg_if::cfg_if;
use leptos::*;

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

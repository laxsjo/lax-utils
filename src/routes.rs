use crate::components::*;
use crate::*;
use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn RouteHome(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
pub fn RouteOther(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Welcome to the other page"</h1>
        <MultiplyWidget label={"hi".to_owned()}/>
    }
}

#[component]
pub fn RouteColorPicker(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Color Picker"</h1>
        <ColorPicker/>
    }
}
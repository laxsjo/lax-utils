use leptos::{ev::MouseEvent, *};
use leptos_router::*;

#[component]
pub fn RouteLink(cx: Scope, route_name: &'static str, children: Children) -> impl IntoView {
    let location = use_location(cx);
    let route_path = "/".to_string() + route_name;

    let is_open = move || {
        location.pathname.get() == "/".to_string() + route_name // TODO: This is very ugly...
    };

    view! {cx,
        <a href=route_path class:active={is_open}>{children(cx)}</a>
    }
}

#[component]
pub fn ColorPicker(cx: Scope) -> impl IntoView {
    let (pos_x, set_pos_x) = create_signal(cx, 0.);
    let (pos_y, set_pos_y) = create_signal(cx, 0.);

    let on_mouse_move = move |ev: MouseEvent| {
        // ev.client_x();
        let Some(element) = ev.target() else {
            return;
        };

        log!("move {}, {}", ev.offset_x(), ev.offset_x());
    };

    view! { cx,
        <div class="color-picker">

            <div
                class="color-picker__map"
                on:mousemove=on_mouse_move
            >
                <div class="color-picker__color">
                    <div class="color-picker__color__cursor"/>
                </div>
                <div class="color-picker__hue">
                    <div class="color-picker__hue__cursor"/>
                </div>
            </div>
        </div>
    }
}

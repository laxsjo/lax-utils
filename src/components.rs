use leptos::*;
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
    view! { cx,
        <div class="color-picker">
            <div class="color-picker__preview">
                <div></div>
            </div>
        </div>
    }
}

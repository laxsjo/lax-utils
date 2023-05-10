use leptos::{ev::MouseEvent, html::*, *};
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
    #[allow(unused)]
    let (col_pos_x, set_col_pos_x) = create_signal(cx, 0.);
    #[allow(unused)]
    let (col_pos_y, set_col_pos_y) = create_signal(cx, 0.);

    #[allow(unused)]
    let (hue_pos, set_hue_pos) = create_signal(cx, 0.);

    #[allow(unused)]
    let (hue, set_hue) = create_signal(cx, 20);

    let custom_properties = move || {
        format!(
            "--color-cursor-x: {}; --color-cursor-y: {}; --hue-cursor: {}; --current-hue: {};",
            col_pos_x(),
            col_pos_y(),
            hue_pos(),
            hue()
        )
    };

    let color_surface_ref = create_node_ref::<Div>(cx);
    #[allow(unused)]
    let hue_surface_ref = create_node_ref::<Div>(cx);

    let on_mouse_move_color = move |ev: MouseEvent| {
        let Some(target) = ev.target() else {
            log!(":(");
            return;
        };
        // let target_element: &HtmlElement<AnyElement> = target.as_ref();

        let Some(element) = color_surface_ref.get() else {
            log!{"Couldn't find element!"};
            return;
        };
        if !target.loose_eq(&element) {
            log!("not ours!");
            return;
        }

        let width = element.offset_width() as f64;
        let height = element.offset_height() as f64;
        let pixel_x = ev.offset_x() as f64;
        let pixel_y = ev.offset_y() as f64;
        let x = pixel_x / width;
        let y = pixel_y / height;

        set_col_pos_x(x);
        set_col_pos_y(y);

        log!(
            "move {}, {} => {}, {} ({}, {}) ({})",
            pixel_x,
            pixel_y,
            x,
            y,
            width,
            height,
            ev.event_phase()
        );
    };

    // let (x, set_x) = create_signal(cx, "0");

    view! { cx,
        <div
            class="color-picker"
            style=custom_properties
            // style=("--color-cursor-x", col_pos_x)
            // style=("--color-cursor-y", col_pos_y)
        >

            <div class="color-picker__map">
                <div
                    class="color-picker__color"
                    on:mousemove=on_mouse_move_color
                    _ref=color_surface_ref
                >
                    <div class="color-picker__color__cursor"/>
                </div>
                <div
                    class="color-picker__hue"
                    _ref=hue_surface_ref
                >
                    <div class="color-picker__hue__cursor"/>
                </div>
            </div>
        </div>
    }
}

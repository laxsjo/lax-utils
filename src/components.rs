use leptos::{ev::*, html::*, *};
use leptos_router::*;
use wasm_bindgen::prelude::*;

use crate::wrap_closure_as_event_listener;

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
    let (col_dragging, set_col_dragging) = create_signal(cx, false);

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

    let on_pointer_move_color = move |ev: Event| {
        // source: https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons
        const PRIMARY_BUTTON: u16 = 1;
        let ev = ev
            .dyn_ref::<PointerEvent>()
            .expect("event wasn't a pointer event");

        if (!col_dragging()) {
            return;
        }

        let Some(surface_element) = color_surface_ref.get() else {
            log!{"Couldn't find element '.color-picker__color'!"};
            return;
        };

        // log!("{}", ev.buttons());
        if (ev.buttons() & PRIMARY_BUTTON) == 0 {
            return;
        }

        let bounds = surface_element.get_bounding_client_rect();
        let element_x = bounds.left();
        let element_y = bounds.top();

        let width = surface_element.offset_width() as f64;
        let height = surface_element.offset_height() as f64;
        let global_x = ev.client_x() as f64;
        let global_y = ev.client_y() as f64;
        let x = ((global_x - element_x) / width).clamp(0., 1.);
        let y = ((global_y - element_y) / height).clamp(0., 1.);

        set_col_pos_x(x);
        set_col_pos_y(y);

        // log!(
        //     "move {}, {} ({}, {}) => {}, {} ({}, {})",
        //     global_x,
        //     global_y,
        //     element_x,
        //     element_y,
        //     x,
        //     y,
        //     width,
        //     height
        // );
    };
    // let on_pointer_move_color_closure = wrap_closure_as_event_listener(on_pointer_move_color);

    let on_pointer_down_color = move |ev: PointerEvent| {
        set_col_dragging(true);
        // window().add_event_listener_with_callback(
        //     "pointermove",
        //     on_pointer_move_color_closure.unchecked_ref(),
        // );
    };

    window_event_listener("pointermove", on_pointer_move_color);

    // document()
    // window().add_event_listener_with_callback("pointer_up", move |ev| {});
    window_event_listener("pointerup", move |ev| {
        set_col_dragging(false);
        // window().remove_event_listener_with_callback(
        //     "pointermove",
        //     &on_pointer_move_color_closure.unchecked_ref(),
        // );
    });

    // window().remove

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
                    // on:pointermove=on_pointer_move_color
                    on:pointerdown=on_pointer_down_color
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

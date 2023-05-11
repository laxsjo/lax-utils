use gloo_events::*;
use leptos::{ev::*, html::*, *};
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

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
    // let (col_dragging, set_col_dragging) = create_signal(cx, false);
    // let (hue_dragging, set_hue_dragging) = create_signal(cx, false);

    // #[allow(unused)]
    // let (col_pos_x, set_col_pos_x) = create_signal(cx, 0.);
    // #[allow(unused)]
    // let (col_pos_y, set_col_pos_y) = create_signal(cx, 0.);

    // #[allow(unused)]
    // let (hue_pos, set_hue_pos) = create_signal(cx, 0.);

    // #[allow(unused)]
    // let (hue, set_hue) = create_signal(cx, 20);

    // let custom_properties = move || {
    //     format!(
    //         "--color-cursor-x: {}; --color-cursor-y: {}; --hue-cursor: {}; --current-hue: {};",
    //         col_pos_x(),
    //         col_pos_y(),
    //         hue_pos(),
    //         hue()
    //     )
    // };

    // let color_surface_ref = create_node_ref::<Div>(cx);
    // #[allow(unused)]
    // let hue_surface_ref = create_node_ref::<Div>(cx);

    // let on_pointer_move_color = move |ev: &PointerEvent| {
    //     // source: https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons
    //     const PRIMARY_BUTTON: u16 = 1;

    //     let Some(surface_element) = color_surface_ref.get() else {
    //         log!{"Couldn't find element '.color-picker__color'!"};
    //         return;
    //     };

    //     // log!("{}", ev.buttons());
    //     if (ev.buttons() & PRIMARY_BUTTON) == 0 {
    //         return;
    //     }

    //     let bounds = surface_element.get_bounding_client_rect();
    //     let element_x = bounds.left();
    //     let element_y = bounds.top();

    //     let width = surface_element.offset_width() as f64;
    //     let height = surface_element.offset_height() as f64;
    //     let global_x = ev.client_x() as f64;
    //     let global_y = ev.client_y() as f64;
    //     let x = ((global_x - element_x) / width).clamp(0., 1.);
    //     let y = ((global_y - element_y) / height).clamp(0., 1.);

    //     set_col_pos_x(x);
    //     set_col_pos_y(y);
    // };

    // let on_pointer_move_hue = move |ev: &PointerEvent| {
    //     // source: https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons
    //     const PRIMARY_BUTTON: u16 = 1;

    //     let Some(surface_element) = hue_surface_ref.get() else {
    //         log!{"Couldn't find element '.color-picker__hue'!"};
    //         return;
    //     };

    //     if (ev.buttons() & PRIMARY_BUTTON) == 0 {
    //         return;
    //     }

    //     let bounds = surface_element.get_bounding_client_rect();
    //     let element_x = bounds.left();

    //     let width = surface_element.offset_width() as f64;
    //     let global_x = ev.client_x() as f64;
    //     let x = ((global_x - element_x) / width).clamp(0., 1.);

    //     set_hue_pos(x);
    // };

    // let on_pointer_move = move |ev: Event| {
    //     let ev = ev
    //         .dyn_ref::<PointerEvent>()
    //         .expect("event wasn't a pointer event");

    //     if col_dragging() {
    //         on_pointer_move_color(ev);
    //     }
    //     if hue_dragging() {
    //         on_pointer_move_hue(ev);
    //     }
    // };
    // // let on_pointer_move_color_closure = wrap_closure_as_event_listener(on_pointer_move_color);

    // let on_pointer_down_color = move |_: PointerEvent| {
    //     set_col_dragging(true);
    //     // window().add_event_listener_with_callback(
    //     //     "pointermove",
    //     //     on_pointer_move_color_closure.unchecked_ref(),
    //     // );
    // };

    // let on_pointer_down_hue = move |_| {
    //     set_hue_dragging(true);
    // };

    // window_event_listener("pointermove", on_pointer_move);

    // window_event_listener("pointerup", move |_| {
    //     set_col_dragging(false);
    //     set_hue_dragging(false);
    //     // window().remove_event_listener_with_callback(
    //     //     "pointermove",
    //     //     &on_pointer_move_color_closure.unchecked_ref(),
    //     // );
    // });

    let (hue, set_hue) = create_signal(cx, 0.);
    let (sat, set_sat) = create_signal(cx, 0.);
    let (value, set_value) = create_signal(cx, 0.);

    view! { cx,
        <div
            class="color-picker"
            // style=custom_properties
        >
            <div class="color-picker__map">
                <SatValueSurface
                    sat=sat
                    set_sat=set_sat
                    value=value
                    set_value=set_value
                    hue=hue
                />
                <HueSlider
                    hue=hue
                    set_hue=set_hue
                />
                // <div
                //     class="color-picker__hue"
                //     on:pointerdown=on_pointer_down_hue
                //     _ref=hue_surface_ref
                // >
                //     <div class="color-picker__hue__cursor"/>
                // </div>
            </div>
        </div>
    }
}

#[component]
pub fn SatValueSurface(
    cx: Scope,
    #[prop(into)] sat: Signal<f64>,
    set_sat: WriteSignal<f64>,
    #[prop(into)] value: Signal<f64>,
    set_value: WriteSignal<f64>,
    #[prop(into)] hue: Signal<f64>,
) -> impl IntoView {
    let (dragging, set_dragging) = create_signal(cx, false);

    let custom_properties = move || {
        format!(
            "--cursor-x: {}; --cursor-y: {}; --current-hue: {};",
            sat(),
            value(),
            hue(),
        )
    };

    let surface_ref = create_node_ref::<Div>(cx);

    let on_pointer_move_color = move |ev: &PointerEvent| {
        // source: https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons
        const PRIMARY_BUTTON: u16 = 1;

        let Some(surface_element) = surface_ref.get() else {
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

        set_sat(x);
        set_value(y);
    };

    let on_pointer_move = move |ev: Event| {
        let ev = ev
            .dyn_ref::<PointerEvent>()
            .expect("event wasn't a pointer event");

        if dragging() {
            on_pointer_move_color(ev);
        }
    };
    // let on_pointer_move_color_closure = wrap_closure_as_event_listener(on_pointer_move_color);

    let on_pointer_down = move |_: PointerEvent| {
        set_dragging(true);
        // window().add_event_listener_with_callback(
        //     "pointermove",
        //     on_pointer_move_color_closure.unchecked_ref(),
        // );
    };

    window_event_listener("pointermove", on_pointer_move);

    window_event_listener("pointerup", move |_| {
        // log!("up!");
        set_dragging(false);
        // window().remove_event_listener_with_callback(
        //     "pointermove",
        //     &on_pointer_move_color_closure.unchecked_ref(),
        // );
    });

    view! {cx,
        <div
            class="sat-value-surface"
            style=custom_properties
            on:pointerdown=on_pointer_down
            _ref=surface_ref
        >
            <div class="sat-value-surface__cursor"/>
        </div>
    }
}

#[component]
pub fn HueSlider(
    cx: Scope,
    #[prop(into)] hue: Signal<f64>,
    set_hue: WriteSignal<f64>,
) -> impl IntoView {
    let (dragging, set_dragging) = create_signal(cx, false);

    let custom_properties = move || format!("--hue: {}", hue());

    let surface_ref = create_node_ref::<Div>(cx);

    let on_pointer_down = move |_: PointerEvent| {
        // log!("down");
        set_dragging(true);
    };

    let on_pointer_move = move |ev: Event| {
        // source: https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons
        const PRIMARY_BUTTON: u16 = 1;

        // log!("moved {:?}", cx.id());

        if (!dragging()) {
            return;
        }

        let Some(ev) = ev.dyn_ref::<PointerEvent>() else {
            log!("incorrect event type");

            return;
        };

        if (ev.buttons() & PRIMARY_BUTTON) == 0 {
            return;
        }

        let Some(surface_element) = surface_ref.get() else {
            log!{"Couldn't find element '.hue-slider'!"};
            return;
        };

        let bounds = surface_element.get_bounding_client_rect();
        let element_x = bounds.left();

        let width = surface_element.offset_width() as f64;
        let global_x = ev.client_x() as f64;
        let x = ((global_x - element_x) / width).clamp(0., 1.);

        set_hue(x);
    };

    let on_pointer_up = move |_| {
        // log!("up");
        set_dragging(false);
    };

    // let listener = EventListener::new(&window(), "pointermove", move |ev| {
    //     on_pointer_move(ev);
    // });
    // listener.forget();

    // TODO: These event listeners are not destroyed on element cleanup.
    window_event_listener("pointerup", on_pointer_up);
    window_event_listener("pointermove", on_pointer_move);

    view! {cx,
        <div
            class="hue-slider"
            on:pointerdown=on_pointer_down
            _ref=surface_ref
            style=custom_properties
        >
            <div class="hue-slider__cursor"/>
        </div>
    }
}

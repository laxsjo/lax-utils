use gloo_events::*;
use leptos::{ev::*, html::*, *};
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

use crate::color_picker::*;

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
    let (hue_float, set_hue_float) = create_signal(cx, 0.);
    let (sat_float, set_sat_float) = create_signal(cx, 0.);
    let (value_float, set_value_float) = create_signal(cx, 0.);

    // let test: Box<dyn Color>;

    let (color_rgb, set_color_rgb) = create_signal(
        cx,
        Rgb {
            r: 255,
            g: 255,
            b: 255,
        },
    );

    let (color_space, set_color_space) = create_signal(cx, ColorSpace::Rgb);

    let component_1_ref = create_node_ref::<Input>(cx);
    let component_2_ref = create_node_ref::<Input>(cx);
    let component_3_ref = create_node_ref::<Input>(cx);
    let float_component_1_ref = create_node_ref::<Input>(cx);
    let float_component_2_ref = create_node_ref::<Input>(cx);
    let float_component_3_ref = create_node_ref::<Input>(cx);

    let update_with_inputs = move || {
        // let component_1
    };

    let on_input = move |_| {};

    view! { cx,
        <div
            class="color-picker"
            // style=custom_properties
        >
            <div class="color-picker__map">
                <SatValueSurface
                    sat=sat_float
                    set_sat=set_sat_float
                    value=value_float
                    set_value=set_value_float
                    hue=hue_float
                />
                <HueSlider
                    hue=hue_float
                    set_hue=set_hue_float
                />
                // <div
                //     class="color-picker__hue"
                //     on:pointerdown=on_pointer_down_hue
                //     _ref=hue_surface_ref
                // >
                //     <div class="color-picker__hue__cursor"/>
                // </div>
                <div class="controls">
                    <div class="integers">
                        <input
                            type="text"
                            inputmode="decimal"
                            placeholder="100"
                            on:input=on_input
                            _ref=component_1_ref
                        />
                        <input
                            type="text"
                            inputmode="decimal"
                            placeholder="100"
                            on:input=on_input
                            _ref=component_1_ref
                        />
                        <input
                            type="text"
                            inputmode="decimal"
                            placeholder="100"
                            on:input=on_input
                            _ref=component_3_ref
                        />
                    </div>
                    <div class="floats">
                        <input
                            type="text"
                            inputmode="decimal"
                            placeholder="1.0"
                            on:input=on_input
                            _ref=float_component_1_ref
                        />
                        <input
                            type="text"
                            inputmode="decimal"
                            placeholder="1.0"
                            on:input=on_input
                            _ref=float_component_2_ref
                        />
                        <input
                            type="text"
                            inputmode="decimal"
                            placeholder="1.0"
                            on:input=on_input
                            _ref=float_component_3_ref
                        />
                    </div>
                </div>
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

        if !dragging() {
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

use leptos::{ev::*, html::*, *};
use leptos_router::*;
use wasm_bindgen::prelude::*;

use crate::color_picker::*;
use crate::string_utils::*;
use crate::utils::*;

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
    const DECIMAL_PRECISION: usize = 2;

    let (color, set_color) =
        create_signal(cx, DynamicColor::from_color(Hsv::from_floats((1., 1., 1.))));

    let (color_hsv, set_color_hsv) = create_signal(cx, color().to_color::<Hsv>());

    let set_color_sync_hsv_color = move |color| {
        set_color(color);
        set_color_hsv(color.to_color::<Hsv>());
    };

    let component_1_ref = create_node_ref::<Input>(cx);
    let component_2_ref = create_node_ref::<Input>(cx);
    let component_3_ref = create_node_ref::<Input>(cx);
    let float_component_1_ref = create_node_ref::<Input>(cx);
    let float_component_2_ref = create_node_ref::<Input>(cx);
    let float_component_3_ref = create_node_ref::<Input>(cx);

    create_effect(cx, move |_| {
        let components = color().components();
        let floats = color().as_floats();

        let Some(component_1) = component_1_ref.get() else {
            // error!("couldn't find component_1");
            return;
        };
        let Some(component_2) = component_2_ref.get() else {
            // error!("couldn't find component_2");
            return;
        };
        let Some(component_3) = component_3_ref.get() else {
            // error!("couldn't find component_3");
            return;
        };

        let Some(float_1) = float_component_1_ref.get() else {
            // error!("couldn't find float_component_1");
            return;
        };
        let Some(float_2) = float_component_2_ref.get() else {
            // error!("couldn't find float_component_2");
            return;
        };
        let Some(float_3) = float_component_3_ref.get() else {
            // error!("couldn't find float_component_3");
            return;
        };

        log!("set components {:?}, floats {:?}", components, floats);

        let format_component = |value: f64| -> _ { naturally_format_float(value, 0, 2) };

        let format_float = |value: f64| -> _ { naturally_format_float(value, 1, 2) };

        sync_input_value_float(
            &component_1,
            components.0,
            DECIMAL_PRECISION,
            format_component,
        );
        sync_input_value_float(
            &component_2,
            components.1,
            DECIMAL_PRECISION,
            format_component,
        );
        sync_input_value_float(
            &component_3,
            components.2,
            DECIMAL_PRECISION,
            format_component,
        );

        sync_input_value_float(&float_1, floats.0, DECIMAL_PRECISION, format_float);
        sync_input_value_float(&float_2, floats.1, DECIMAL_PRECISION, format_float);
        sync_input_value_float(&float_3, floats.2, DECIMAL_PRECISION, format_float);
    });

    let update_with_components = move |_| {
        let Some(component_1) = component_1_ref.get() else {
            error!("couldn't find component_1");
            return;
        };
        let Some(component_2) = component_2_ref.get() else {
            error!("couldn't find component_2");
            return;
        };
        let Some(component_3) = component_3_ref.get() else {
            error!("couldn't find component_3");
            return;
        };

        let components = (
            component_1.value().parse_input::<f64>().unwrap_or(0.),
            component_2.value().parse_input::<f64>().unwrap_or(0.),
            component_3.value().parse_input::<f64>().unwrap_or(0.),
        );

        log!("got components {:?}", components);

        set_color_sync_hsv_color(color().set_components(components));
    };
    let update_with_floats = move |_| {
        let Some(float_1) = float_component_1_ref.get() else {
            error!("couldn't find float_component_1");
            return;
        };
        let Some(float_2) = float_component_2_ref.get() else {
            error!("couldn't find float_component_2");
            return;
        };
        let Some(float_3) = float_component_3_ref.get() else {
            error!("couldn't find float_component_3");
            return;
        };

        let floats = (
            float_1.value().parse_input::<f64>().unwrap_or(0.),
            float_2.value().parse_input::<f64>().unwrap_or(0.),
            float_3.value().parse_input::<f64>().unwrap_or(0.),
        );

        log!("got floats {:?}", floats);

        set_color_sync_hsv_color(color().set_floats(floats));
    };

    // let color_hsv = create_memo(cx, move |old_hsv| {
    //     let mut new_hsv = color().to_color::<Hsv>();

    //     let Some(old_hsv) = old_hsv else {
    //         return new_hsv;
    //     };

    //     if new_hsv.v == 0. {
    //         new_hsv.h = old_hsv.h;
    //         new_hsv.s = old_hsv.s;
    //     }
    //     if new_hsv.s == 0. {
    //         new_hsv.h = old_hsv.h
    //     }

    //     new_hsv
    // });

    let hue_float = Signal::derive(cx, move || color_hsv().as_floats().0);
    let sat_float = Signal::derive(cx, move || color_hsv().as_floats().1);
    let value_float = Signal::derive(cx, move || color_hsv().as_floats().2);

    // let (hue_float, set_hue_float) = create_signal(cx, 0.);
    // let (sat_float, set_sat_float) = create_signal(cx, 0.);
    // let (value_float, set_value_float) = create_signal(cx, 0.);

    // create_effect(move || {});

    let update_with_hsv_floats = move |floats: (f64, f64, f64)| {
        let hsv = DynamicColor::from_color(Hsv::from_floats((floats.0, floats.1, floats.2)));
        set_color(hsv.set_color_space(color().color_space()));
    };

    let on_hue_float_change = move |hue: f64| {
        // set_color_hsv(color_hsv);
        update_with_hsv_floats((hue, sat_float(), value_float()));
    };
    let on_sat_float_change = move |sat: f64| {
        update_with_hsv_floats((hue_float(), sat, value_float()));
    };
    let on_value_float_change = move |value: f64| {
        update_with_hsv_floats((hue_float(), sat_float(), value));
    };

    view! { cx,
        <div
            class="color-picker"
        >
            <div class="map">
                <SatValueSurface
                    sat=sat_float
                    set_sat=on_sat_float_change
                    value=value_float
                    set_value=on_value_float_change
                    hue=hue_float
                />
                <HueSlider
                    hue=hue_float
                    set_hue=on_hue_float_change
                />
            </div>
            <div class="controls">
                <div class="integers">
                    <input
                        type="text"
                        inputmode="decimal"
                        placeholder="100"
                        value="255"
                        on:input=update_with_components
                        _ref=component_1_ref
                    />
                    <input
                        type="text"
                        inputmode="decimal"
                        placeholder="100"
                        value="255"
                        on:input=update_with_components
                        _ref=component_2_ref
                    />
                    <input
                        type="text"
                        inputmode="decimal"
                        placeholder="100"
                        value="255"
                        on:input=update_with_components
                        _ref=component_3_ref
                    />
                </div>
                <div class="floats">
                    <input
                        type="text"
                        inputmode="decimal"
                        placeholder="1.0"
                        value="1.0"
                        on:input=update_with_floats
                        _ref=float_component_1_ref
                    />
                    <input
                        type="text"
                        inputmode="decimal"
                        placeholder="1.0"
                        value="1.0"
                        on:input=update_with_floats
                        _ref=float_component_2_ref
                    />
                    <input
                        type="text"
                        inputmode="decimal"
                        placeholder="1.0"
                        value="1.0"
                        on:input=update_with_floats
                        _ref=float_component_3_ref
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn SatValueSurface<S, V>(
    cx: Scope,
    #[prop(into)] sat: Signal<f64>,
    set_sat: S,
    #[prop(into)] value: Signal<f64>,
    set_value: V,
    #[prop(into)] hue: Signal<f64>,
) -> impl IntoView
where
    S: Fn(f64) + Copy + 'static,
    V: Fn(f64) + Copy + 'static,
{
    let (dragging, set_dragging) = create_signal(cx, false);

    let custom_properties = move || {
        format!(
            "--cursor-x: {}; --cursor-y: {}; --current-hue: {};",
            sat(),
            1. - value(),
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
        set_value(1. - y);
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
pub fn HueSlider<F>(cx: Scope, #[prop(into)] hue: Signal<f64>, set_hue: F) -> impl IntoView
where
    F: Fn(f64) + Copy + 'static,
{
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

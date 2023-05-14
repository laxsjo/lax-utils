use leptos::{ev::*, html::*, *};
use wasm_bindgen::prelude::*;

use crate::color_picker::*;
use crate::components::*;
use crate::string_utils::*;
use crate::utils::*;

#[component]
pub fn ColorPicker(cx: Scope) -> impl IntoView {
    const DECIMAL_PRECISION: usize = 2;

    let (color_space_options, _) = create_signal(
        cx,
        vec![ColorSpace::Rgb, ColorSpace::Hsl, ColorSpace::Hsv],
    );

    let (color_space, set_color_space) = create_signal(cx, ColorSpace::Rgb);

    let (color, set_color) = create_signal(
        cx,
        DynamicColor::from_floats((1., 1., 1.), color_space()),
    );

    let (color_hsv, set_color_hsv) =
        create_signal(cx, color().to_color::<Hsv>());

    let set_color_sync_hsv_color = move |color: DynamicColor| {
        set_color(color);
        set_color_hsv(color.to_color::<Hsv>());
    };

    let hex_code = create_memo(cx, move |_| {
        let rgb = color().to_color::<Rgb>();
        rgb.as_hex_code()
    });
    let hex_code_hashtag =
        Signal::derive(cx, move || format!("#{}", hex_code()));

    create_effect(cx, move |_| {
        set_color(color().set_color_space(color_space()));
    });

    let on_color_space_change = move |color_space: Option<_>| {
        if let Some(color_space) = color_space {
            set_color_space(color_space)
        }
    };

    let component_0_ref = create_node_ref::<Input>(cx);
    let component_1_ref = create_node_ref::<Input>(cx);
    let component_2_ref = create_node_ref::<Input>(cx);
    let float_component_0_ref = create_node_ref::<Input>(cx);
    let float_component_1_ref = create_node_ref::<Input>(cx);
    let float_component_2_ref = create_node_ref::<Input>(cx);

    create_effect(cx, move |_| {
        let components = color().components();
        let floats = color().as_floats();

        let Some(component_0) = component_0_ref.get() else {
            // error!("couldn't find component_1");
            return;
        };
        let Some(component_1) = component_1_ref.get() else {
            // error!("couldn't find component_2");
            return;
        };
        let Some(component_2) = component_2_ref.get() else {
            // error!("couldn't find component_3");
            return;
        };

        let Some(float_0) = float_component_0_ref.get() else {
            // error!("couldn't find float_component_1");
            return;
        };
        let Some(float_1) = float_component_1_ref.get() else {
            // error!("couldn't find float_component_2");
            return;
        };
        let Some(float_2) = float_component_2_ref.get() else {
            // error!("couldn't find float_component_3");
            return;
        };

        // log!("set components {:?}, floats {:?}", components, floats);

        let format_component =
            |value: f64| -> _ { naturally_format_float(value, 0, 2) };

        let format_float =
            |value: f64| -> _ { naturally_format_float(value, 1, 2) };

        sync_input_value_float(
            &component_0,
            components.0,
            DECIMAL_PRECISION,
            format_component,
        );
        sync_input_value_float(
            &component_1,
            components.1,
            DECIMAL_PRECISION,
            format_component,
        );
        sync_input_value_float(
            &component_2,
            components.2,
            DECIMAL_PRECISION,
            format_component,
        );

        sync_input_value_float(
            &float_0,
            floats.0,
            DECIMAL_PRECISION,
            format_float,
        );
        sync_input_value_float(
            &float_1,
            floats.1,
            DECIMAL_PRECISION,
            format_float,
        );
        sync_input_value_float(
            &float_2,
            floats.2,
            DECIMAL_PRECISION,
            format_float,
        );
    });

    let update_with_components = move |_| {
        let Some(component_0) = component_0_ref.get() else {
            error!("couldn't find component_1");
            return;
        };
        let Some(component_1) = component_1_ref.get() else {
            error!("couldn't find component_2");
            return;
        };
        let Some(component_2) = component_2_ref.get() else {
            error!("couldn't find component_3");
            return;
        };

        let components = (
            component_0.value().parse_input::<f64>().unwrap_or(0.),
            component_1.value().parse_input::<f64>().unwrap_or(0.),
            component_2.value().parse_input::<f64>().unwrap_or(0.),
        );

        // log!("got components {:?}", components);

        set_color_sync_hsv_color(color().set_components(components));
    };
    let update_with_floats = move |_| {
        let Some(float_0) = float_component_0_ref.get() else {
            error!("couldn't find float_component_1");
            return;
        };
        let Some(float_1) = float_component_1_ref.get() else {
            error!("couldn't find float_component_2");
            return;
        };
        let Some(float_2) = float_component_2_ref.get() else {
            error!("couldn't find float_component_3");
            return;
        };

        let floats = (
            float_0.value().parse_input::<f64>().unwrap_or(0.),
            float_1.value().parse_input::<f64>().unwrap_or(0.),
            float_2.value().parse_input::<f64>().unwrap_or(0.),
        );

        // log!("got floats {:?}", floats);

        set_color_sync_hsv_color(color().set_floats(floats));
    };

    let hue_float = Signal::derive(cx, move || color_hsv().as_floats().0);
    let sat_float = Signal::derive(cx, move || color_hsv().as_floats().1);
    let value_float = Signal::derive(cx, move || color_hsv().as_floats().2);

    let update_with_hsv_floats = move |floats: (f64, f64, f64)| {
        set_color_hsv(Hsv::from_floats((floats.0, floats.1, floats.2)));
        let hsv = DynamicColor::from_color(color_hsv());
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

    let color_space_info = create_memo(cx, move |_| color_space().info());

    let label_0 = Signal::derive(cx, move || {
        Some(color_space_info().labels.0.to_owned())
    });
    let label_1 = Signal::derive(cx, move || {
        Some(color_space_info().labels.1.to_owned())
    });
    let label_2 = Signal::derive(cx, move || {
        Some(color_space_info().labels.2.to_owned())
    });

    let unit_0 = Signal::derive(cx, move || {
        color_space_info().units.0.map(Into::<String>::into)
    });
    let unit_1 = Signal::derive(cx, move || {
        color_space_info().units.1.map(Into::<String>::into)
    });
    let unit_2 = Signal::derive(cx, move || {
        color_space_info().units.2.map(Into::<String>::into)
    });

    let color_display_style = move || {
        let rgb: Rgb = color().to_color();

        format!(
            "--r: {}; --g: {}; --b: {};",
            rgb.r as u8, rgb.g as u8, rgb.b as u8
        )
    };

    let id = unique_id();

    let select_id = Signal::derive(cx, move || {
        Some(format!("color-picker-color-space_{}", id))
    });

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
                <div class="color-space">
                    <label for=select_id>
                        "Color Space"
                    </label>
                    <FancySelect
                        items=color_space_options
                        on_select=on_color_space_change
                        select_id=select_id
                    />
                </div>
                <div class="integers">
                    <LabeledFloatInput
                        prefix=label_0
                        postfix=unit_0
                    >
                        <input
                            type="text"
                            inputmode="decimal"
                            placeholder="100"
                            value="255"
                            on:input=update_with_components
                            _ref=component_0_ref
                        />
                    </LabeledFloatInput>
                    <LabeledFloatInput
                        prefix=label_1
                        postfix=unit_1
                    >
                        <input
                            type="text"
                            inputmode="decimal"
                            placeholder="100"
                            value="255"
                            on:input=update_with_components
                            _ref=component_1_ref
                        />
                    </LabeledFloatInput>
                    <LabeledFloatInput
                        prefix=label_2
                        postfix=unit_2
                    >
                        <input
                            type="text"
                            inputmode="decimal"
                            placeholder="100"
                            value="255"
                            on:input=update_with_components
                            _ref=component_2_ref
                        />
                    </LabeledFloatInput>
                </div>
                <div class="floats">
                    <LabeledFloatInput
                        prefix=label_0
                        postfix=None
                    >
                        <input
                            type="text"
                            inputmode="decimal"
                            placeholder="1.0"
                            value="1.0"
                            on:input=update_with_floats
                            _ref=float_component_0_ref
                        />
                    </LabeledFloatInput>
                    <LabeledFloatInput
                        prefix=label_1
                        postfix=None
                    >
                        <input
                            type="text"
                            inputmode="decimal"
                            placeholder="1.0"
                            value="1.0"
                            on:input=update_with_floats
                            _ref=float_component_1_ref
                        />
                    </LabeledFloatInput>
                    <LabeledFloatInput
                        prefix=label_2
                        postfix=None
                    >
                        <input
                            type="text"
                            inputmode="decimal"
                            placeholder="1.0"
                            value="1.0"
                            on:input=update_with_floats
                            _ref=float_component_2_ref
                        />
                    </LabeledFloatInput>
                </div>
            </div>
            <div class="display">
                <div
                    class="color-display"
                    style=color_display_style
                />
                <CopyableLabel
                    content=hex_code_hashtag
                >
                    <span class="prefix">"#"</span>
                    <span class="code">
                        {hex_code}
                    </span>
                </CopyableLabel>
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
    // let on_pointer_move_color_closure =
    // wrap_closure_as_event_listener(on_pointer_move_color);

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
pub fn HueSlider<F>(
    cx: Scope,
    #[prop(into)] hue: Signal<f64>,
    set_hue: F,
) -> impl IntoView
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

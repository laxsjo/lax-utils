use std::marker::PhantomData;

use crate::{
    color_picker::*,
    components::*,
    settings::{StoredInput, StoredRadioGroup},
    string_utils::*,
    utils::*,
};
use leptos::{ev::*, html::*, *};
use wasm_bindgen::prelude::*;

#[component]
pub fn ColorPicker(cx: Scope) -> impl IntoView {
    const DECIMAL_PRECISION: usize = 2;

    // let (color_space_options_old, _) = create_signal(
    //     cx,
    //     vec![ColorSpace::Rgb, ColorSpace::Hsl, ColorSpace::Hsv],
    // );
    let color_space_options = vec![
        ("RGB".to_owned(), ColorSpace::Rgb),
        ("HSL".to_owned(), ColorSpace::Hsl),
        ("HSV".to_owned(), ColorSpace::Hsv),
    ];

    let (color_space, set_color_space) = create_signal(cx, ColorSpace::Rgb);

    let precise_inputs = create_rw_signal(cx, false);

    let (color, set_color) = create_signal(
        cx,
        DynamicColor::from_floats((1., 1., 1.), color_space.get_untracked()),
    );

    let (color_hsv, set_color_hsv) =
        create_signal(cx, color.get_untracked().to_color::<Hsv>());

    let (hex_code, set_hex_code) = create_signal(
        cx,
        color.get_untracked().to_color::<Rgb>().as_hex_code(),
    );
    let hex_code_hashtag =
        Signal::derive(cx, move || format!("#{}", hex_code()));

    let set_color_sync_other = move |color: DynamicColor| {
        set_color(color);
        set_color_hsv(color.to_color::<Hsv>());
        set_hex_code(color.to_color::<Rgb>().as_hex_code());
    };

    // let hex_code = create_memo(cx, move |_| {
    //     let rgb = color().to_color::<Rgb>();
    //     rgb.as_hex_code()
    // });

    create_effect(cx, move |_| {
        set_color(color().set_color_space(color_space()));
    });

    // let on_color_space_change_old = move |color_space: Option<_>| {
    //     if let Some(color_space) = color_space {
    //         set_color_space(color_space)
    //     }
    // };

    let on_color_space_change = set_color_space;

    let (force_update_inputs, set_force_update_inputs) =
        create_signal(cx, false);

    let component_0_ref = create_node_ref::<Input>(cx);
    let component_1_ref = create_node_ref::<Input>(cx);
    let component_2_ref = create_node_ref::<Input>(cx);
    let float_component_0_ref = create_node_ref::<Input>(cx);
    let float_component_1_ref = create_node_ref::<Input>(cx);
    let float_component_2_ref = create_node_ref::<Input>(cx);

    let format_component = move || match precise_inputs.get() {
        true => |value: f64| -> _ { naturally_format_float(value, 0, 2) },
        false => |value: f64| -> _ { naturally_format_float(value, 0, 0) },
    };

    let format_float =
        |value: f64| -> _ { naturally_format_float(value, 1, 2) };

    create_effect(cx, move |_| {
        precise_inputs.track();
        set_force_update_inputs.set_untracked(true);
    });

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

        let force_update = force_update_inputs.get_untracked();
        if force_update {
            set_force_update_inputs.set_untracked(false);
        }

        sync_input_value_float(
            &component_0,
            components.0,
            DECIMAL_PRECISION,
            force_update,
            format_component(),
        );
        sync_input_value_float(
            &component_1,
            components.1,
            DECIMAL_PRECISION,
            force_update,
            format_component(),
        );
        sync_input_value_float(
            &component_2,
            components.2,
            DECIMAL_PRECISION,
            force_update,
            format_component(),
        );

        sync_input_value_float(
            &float_0,
            floats.0,
            DECIMAL_PRECISION,
            force_update,
            format_float,
        );
        sync_input_value_float(
            &float_1,
            floats.1,
            DECIMAL_PRECISION,
            force_update,
            format_float,
        );
        sync_input_value_float(
            &float_2,
            floats.2,
            DECIMAL_PRECISION,
            force_update,
            format_float,
        );
    });

    // create_tri

    let update_with_components = move |ev: Event| {
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

        let format_component = format_component();

        if &ev.type_() == "change" {
            component_0.set_value(&format_component(components.0));
            component_1.set_value(&format_component(components.1));
            component_2.set_value(&format_component(components.2));
        }

        // log!("got components {:?}", components);

        set_color_sync_other(color.get_untracked().set_components(components));
    };
    let update_with_floats = move |ev: Event| {
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

        if &ev.type_() == "change" {
            float_0.set_value(&format_float(floats.0));
            float_1.set_value(&format_float(floats.1));
            float_2.set_value(&format_float(floats.2));
        }

        // log!("got floats {:?}", floats);

        set_color_sync_other(color().set_floats(floats));
    };

    let hue_float = Signal::derive(cx, move || color_hsv().as_floats().0);
    let sat_float = Signal::derive(cx, move || color_hsv().as_floats().1);
    let value_float = Signal::derive(cx, move || color_hsv().as_floats().2);

    let update_with_hsv_floats = move |floats: (f64, f64, f64)| {
        set_color_hsv(Hsv::from_floats((floats.0, floats.1, floats.2)));
        let hsv = DynamicColor::from_color(color_hsv.get_untracked());
        set_color(hsv.set_color_space(color.get_untracked().color_space()));
        set_hex_code(hsv.to_color::<Rgb>().as_hex_code());
    };

    let on_hue_float_change = move |hue: f64| {
        // set_color_hsv(color_hsv);
        update_with_hsv_floats((
            hue,
            sat_float.get_untracked(),
            value_float.get_untracked(),
        ));
    };
    let on_sat_float_change = move |sat: f64| {
        update_with_hsv_floats((
            hue_float.get_untracked(),
            sat,
            value_float.get_untracked(),
        ));
    };
    let on_value_float_change = move |value: f64| {
        update_with_hsv_floats((
            hue_float.get_untracked(),
            sat_float.get_untracked(),
            value,
        ));
    };

    let update_with_hex_code = move |hex: &str| {
        let Some(rgb) = Rgb::from_hex_code(hex) else {
            return;
        };

        let color = DynamicColor::new(
            rgb.as_components(),
            color.get_untracked().color_space(),
        );

        set_color(color);
        set_color_hsv(color.to_color::<Hsv>());
        set_hex_code(hex.to_owned());
    };

    let on_hex_code_change = move |ev: Event| {
        let value = event_target_value(&ev);
        let mut value_str = value.as_str();
        if value_str.starts_with('#') {
            // // ? is this unsafe?
            // let target = event_target::<web_sys::HtmlInputElement>(&ev);

            value_str = value_str.trim_start_matches('#');

            // target.set_value(value_str);
            // set_hex_code(value_str.to_owned());
        }

        update_with_hex_code(value_str);
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

    let components_copy_string = Signal::derive(cx, move || {
        let components = color().components();
        format!(
            "{}, {}, {}",
            format_component()(components.0),
            format_component()(components.1),
            format_component()(components.2)
        )
    });
    let floats_copy_string = Signal::derive(cx, move || {
        let floats = color().as_floats();
        format!(
            "{}, {}, {}",
            format_float(floats.0),
            format_float(floats.1),
            format_float(floats.2)
        )
    });

    // let on_precise_input_change = move |ev: Event| {
    //     let checked = event_target_checked(&ev);
    //     log!("precise input changed");

    //     set_force_update_inputs(true);
    //     set_precise_inputs(checked);
    // };

    let color_display_style = move || {
        let rgb: Rgb = color().to_color();

        format!(
            "--r: {}; --g: {}; --b: {};",
            rgb.r as u8, rgb.g as u8, rgb.b as u8
        )
    };

    // let id = unique_id();

    // let select_id = Signal::derive(cx, move || {
    //     Some(format!("color-picker-color-space_{}", id))
    // });

    let phantom_bool = PhantomData::<bool>;

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
                    <LabeledFloatInput
                        prefix=label_0
                        postfix=unit_0
                    >
                        <input
                            type="text"
                            inputmode="decimal"
                            placeholder="100"
                            value="255"
                            size=6
                            on:input=update_with_components
                            on:change=update_with_components
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
                            size=6
                            on:input=update_with_components
                            on:change=update_with_components
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
                            size=6
                            on:input=update_with_components
                            on:change=update_with_components
                            _ref=component_2_ref
                        />
                    </LabeledFloatInput>
                    <CopyButton
                        value=components_copy_string
                    >""</CopyButton>
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
                            size=6
                            on:input=update_with_floats
                            on:change=update_with_floats
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
                            size=6
                            on:input=update_with_floats
                            on:change=update_with_floats
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
                            size=6
                            value="1.0"
                            on:input=update_with_floats
                            on:change=update_with_floats
                            _ref=float_component_2_ref
                        />
                    </LabeledFloatInput>
                    <CopyButton
                        value=floats_copy_string
                    >""</CopyButton>
                </div>
            </div>
            <div class="display">
                <div
                    class="color-display"
                    style=color_display_style
                />
                // <CopyableLabel
                //     content=hex_code_hashtag
                // >
                //     <span class="prefix">"#"</span>
                //     <span class="code">
                //         {hex_code}
                //     </span>
                // </CopyableLabel>
                <div class="hex-code">
                    <span class="prefix">"#"</span>
                    <input
                        on:input=on_hex_code_change
                        prop:value=hex_code
                        placeholder="000000"
                        // value=hex_code.get_untracked()
                    />

                    <CopyButton
                        value=hex_code_hashtag
                    />
                </div>
            </div>
            <div class="color-space">
                // <label for=select_id>
                //     "Color Space"
                // </label>
                // <FancySelect
                //     items=color_space_options_old
                //     default_selected=color_space.get_untracked()
                //     on_select=on_color_space_change_old
                //     select_id=select_id
                // />
                <StoredRadioGroup
                    options=color_space_options
                    title="Color Space".to_owned()
                    name=Signal::derive(cx, || "color-space".to_owned())
                    on_change=on_color_space_change
                    key="s_color_space"
                />
            </div>
            <div class="options">
                <label>
                    "Precise Inputs"
                    <StoredInput
                        input=view! { cx,
                            <input
                                type="checkbox"
                                // on:input=on_precise_input_change
                            />
                        }
                        key="s_precise_inputs"
                        _type=phantom_bool
                        value=precise_inputs
                    />
                </label>
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

        let Some(surface_element) = surface_ref.get_untracked() else {
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

    let on_pointer_move = move |ev: &Event| {
        let ev = ev
            .dyn_ref::<PointerEvent>()
            .expect("event wasn't a pointer event");

        if dragging.get_untracked() {
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

    create_managed_window_event_listener(cx, "pointermove", on_pointer_move);

    create_managed_window_event_listener(cx, "pointerup", move |_| {
        set_dragging(false);
        // window().remove_event_listener_with_callback(
        //     "pointermove",
        //     &on_pointer_move_color_closure.unchecked_ref(),
        // );
    });

    // store_value(
    //     cx,
    //     EventListener::new(&window(), "pointermove", on_pointer_move),
    // );
    // store_value(
    //     cx,
    //     EventListener::new(&window(), "pointerup", move |_: &Event| {
    //         set_dragging(false);
    //     }),
    // );

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

    let on_pointer_move = move |ev: &Event| {
        // source: https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons
        const PRIMARY_BUTTON: u16 = 1;

        leptos_reactive::SpecialNonReactiveZone::enter();

        // log!("moved {:?}", cx.id());

        if !dragging.get_untracked() {
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

    let on_pointer_up = move |_: &_| {
        // log!("up");
        set_dragging(false);
    };

    // let listener = EventListener::new(&window(), "pointermove", move |ev| {
    //     on_pointer_move(ev);
    // });
    // listener.forget();

    // TODO: These event listeners are not destroyed on element cleanup.
    create_managed_window_event_listener(cx, "pointerup", on_pointer_up);
    create_managed_window_event_listener(cx, "pointermove", on_pointer_move);

    // create_managed_window_event_listener(cx, "pointerup", move |_| {
    //     log!("up");
    // });

    // create_managed_event_listener(cx, );

    // _ = window().focus();

    // if is_browser() {
    //     store_value(
    //         cx,
    //         EventListener::new(&window(), "pointerup", move |_| {
    //             log!("up");
    //             set_dragging(false);
    //         }),
    //     );
    // }
    // store_value(
    //     cx,
    //     EventListener::new(&window(), "pointermove", on_pointer_move),
    // );

    // window().event

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

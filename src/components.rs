use crate::{toasts, utils::*};
use gloo_events::EventListener;
use leptos::{html::*, leptos_dom::helpers::*, logging::error, window, *};
use leptos_router::*;
use std::{hash::*, time::Duration};

#[component]
pub fn RouteLink(
    route_path: &'static str,
    // #[prop(optional)] _ref: Option<NodeRef<A>>,
    // _ref: NodeRef<A>,
    children: Children,
) -> impl IntoView {
    let location = use_location();

    let is_open = move || location.pathname.get() == route_path;

    view! {
        <a href=route_path class:active={is_open}>{children()}</a>
    }
}

#[component]
pub fn FancySelect<T, F>(
    #[prop(into)] items: Signal<Vec<T>>,
    #[prop(optional)] default_selected: Option<T>,
    on_select: F,
    /// The [UiDisplay] environment.
    #[prop(optional)]
    env: T::Environment,
    #[prop(into)] select_id: MaybeSignal<Option<String>>,
) -> impl IntoView
where
    T: Copy + Eq + Hash + UiDisplay + 'static,
    F: Fn(Option<T>) + 'static,
{
    let select_ref = create_node_ref::<Select>();
    let selected_index = match default_selected {
        Some(selected) => {
            items().iter().position(|x| *x == selected).unwrap_or(0)
        }
        None => 0,
    };

    let on_change = move |_| {
        let Some(select) = select_ref.get() else {
        error!("Couldn't find select element");
        return;
    };

        let selected_index = select.selected_index();
        if selected_index < 0 {
            return on_select(None);
        }

        on_select(items().get(selected_index as usize).copied());
    };

    // let on_pointer_up = move |_| {
    //     log!("pointer up");
    //     set_expanded(false);
    // };

    // let on_focus = move |_| {
    //     log!("focus");
    //     set_expanded(true);
    // };

    // let on_blur = move |_| {
    //     log!("blur");
    //     set_expanded(false);
    // };

    select_ref.on_load(move |select| {
        // TODO: figure out what is changing the select value after loading
        set_timeout(
            move || {
                select.set_selected_index(selected_index as i32);
            },
            Duration::from_secs_f64(0.1),
        )
    });

    let generate_item = move |item: T| {
        let selected = match default_selected {
            Some(selected) => item == selected,
            None => false,
        };
        view! {
            <option
                selected=selected
            >
                {item.to_ui_string(env)}
            </option>
        }
    };

    view! {
        <div
            class="fancy-select"
        >
            <select
                id=select_id
                on:input=on_change
                _ref=select_ref
                // on:focus=on_focus
                // on:blur=on_blur
                // on:click=on_pointer_up
            >
                <For
                    each=items
                    key=|item: &T| {
                        *item
                    }
                    children=generate_item
                />
            </select>
            <Icon icon_id="chevron-down"/>
        </div>
    }
}

/// Display an svg icon.
///
/// ~~You can browse the available icons here: https://fonts.google.com/icons~~
/// You can browse the available icons here: https://feathericons.com/?query=git
/// The names you specify should be separated by dashes.
/// E.g. `arrow-left`
///
/// # Maintenance Note
/// The icons were generated using
/// [icomoon.io](https://icomoon.io/app/#/select), and exported as a single
/// svg symbol file called `symbol-defs.svg` on the page, which was then renamed
/// to `icon-symbol-defs.svg` and placed in the `assets/` folder.
#[component]
pub fn Icon(
    /// The icon to display. Should be of the form `<icon_name>`
    icon_id: &'static str,
) -> impl IntoView {
    let use_element_str = format!("<use href=\"#icon-{}\"></use>", icon_id);

    view! {
        <svg
            class="icon"
            viewBox="0 0 24 24"
            inner_html=use_element_str
        />
    }
}

#[component]
pub fn LabeledFloatInput(
    #[prop(into)] prefix: MaybeSignal<Option<String>>,
    #[prop(into)] postfix: MaybeSignal<Option<String>>,
    children: Children,
) -> impl IntoView
where
{
    let prefix = move || prefix().unwrap_or("".to_string());
    let postfix = move || postfix().unwrap_or("".to_string());
    view! {
        <div class="labeled-input">
            <span class="prefix">
                {prefix}
            </span>
            <span class="postfix">
                {postfix}
            </span>
            <span class="input">
                {children()}
            </span>
        </div>
    }
}

#[component]
pub fn CopyableLabel(
    #[prop(into)] content: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <CopyButton
            value=content
        >
            <span class="label">
                {children()}
            </span>
        </CopyButton>
    }
}

// #[component]
// pub fn CopyableInput(
//
//     /// The value that will be copied into the clipboard. It is the callers
//     /// responsibility to make sure that this signal is kept in sync with the
//     /// value of the input!
//     #[prop(into)]
//     content: Signal<String>,
//     children: Box<dyn FnOnce(Scope) -> HtmlElement<Input>>,
// ) -> impl IntoView {
//     view! {
//         <div class="copyable-input">
//             {children()}
//             <Icon icon_id="content_copy" />
//         </div>
//     }
// }

#[component]
pub fn CopyButton(
    /// The value that will be copied into the clipboard.
    #[prop(into)]
    value: Signal<String>,
    /// An optional aria-label and tooltip that will be applied to the button
    /// element.
    #[prop(into, optional)]
    label: MaybeSignal<Option<String>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    const DURATION_IN_SECONDS: f64 = 5.;
    const DURATION_OUT_SECONDS: f64 = 0.3;

    let copy_to_clipboard = move || -> bool {
        let Some(clipboard) = window().navigator().clipboard() else {
            // TODO: give user feedback that copy to clipboard failed.
            error!("Failed to get clipboard");
            return false;
        };

        let _ = clipboard.write_text(value().as_str());

        // activate_popup();

        true
    };

    let on_click = move |_| {
        copy_to_clipboard();
        toasts::add_toast(format!("Copied '{}' to clipboard", value()));
    };

    let style = format!(
        "--duration: {}s; --duration-out: {}s",
        DURATION_IN_SECONDS, DURATION_OUT_SECONDS
    );

    view! {
        <button
            class="copy-button"
            style=style
            aria-label=label.clone()
            title=label
            on:click=on_click
        >
            {children.map(|children| children())}
            <Icon icon_id="copy" />
        </button>
    }
}

#[component]
pub fn RadioGroup<T, F, W>(
    #[prop(into)] title: MaybeSignal<String>,
    #[prop(into)] name: Signal<String>,
    #[prop(into)] options: MaybeSignal<Vec<(String, T)>>,
    #[prop(optional)] on_change: Option<F>,
    // /// A reference to a boxed function that the caller can call to set the
    // /// value of the radio group. This reference will be set once this
    // /// component has been initalized.
    // #[prop(optional)]
    // set_value: Option<OptionalRcBox<dyn Fn(T) + 'static>>,
    /// A callback that will be called with a boxed setter function that sets
    /// the value of this radio group. It will be called once the component
    /// and it's elements have been initialized and loaded. (Unsure if this
    /// means that the elements have *also* been mounted...)
    #[prop(optional)]
    with_set_value: Option<W>,
    // value: RwSignal<T>,
) -> impl IntoView
where
    T: Copy + Hash + 'static + Eq + ToString,
    F: Fn(T) + 'static + Copy,
    W: Fn(Box<dyn Fn(T) + 'static>) + 'static + Copy,
{
    let any_changed = create_trigger();

    let (current_value, set_current_value) = create_signal::<Option<T>>(None);

    // log!("got set_value.is_none() {:?}", set_value.is_none());
    // if let Some(set_value) = set_value {
    //     let mut set_value = set_value.borrow_mut();

    //     *set_value = Some(Box::new(move |value: T| {
    //         log!("set value in callback");
    //         set_current_value(Some(value));
    //     }));
    //     log!("assigned set value callback");
    // }

    create_effect(move |_| {
        if let Some(on_change) = on_change {
            let Some(value) = current_value() else {
                return;
            };
            on_change(value);
        }
    });

    let create_input = move |(label, this_value): (String, T)| {
        let (checked, set_checked) = create_signal(false);
        let input = create_node_ref::<Input>();
        // let input

        create_effect(move |_| {
            any_changed();

            let Some(input) = input.get() else {
                set_checked(false);
                return;
            };

            let checked = input.checked();
            if checked {
                set_current_value(Some(this_value));
            }
        });

        create_effect(move |_| {
            set_checked(current_value() == Some(this_value));
        });

        // create_effect( move |_| {

        // })

        let on_change = move |_ev| {
            set_current_value(None);
            any_changed.notify();
            // let checked = event_target_checked(&ev);

            // if let Some(on_change) = on_change {
            //     // Failsafe in case change event is dispatched on deselect.
            //     if checked {
            //         on_change(this_value);
            //     }
            // }

            // log!("clicked for {:?}", cx);
            // let checked = event_target_checked(&ev);
            // set_checked(checked);

            // if let Some(on_change) = on_change {
            //     if checked {
            //         on_change(value);
            //     }
            // }
        };

        view! {
            <label data-checked=checked>
                <span>{label}</span>
                <input
                    type="radio"
                    name=name
                    value=this_value.to_string()
                    on:change=on_change
                    _ref=input
                />
            </label>
        }
    };

    let fieldset_ref = create_node_ref::<Fieldset>();

    let on_load = move || {
        if let Some(with_set_value) = with_set_value {
            with_set_value(Box::new(move |value: T| {
                set_current_value(Some(value));
            }));
        }
    };

    create_effect(move |_| {
        if fieldset_ref.get().is_some() {
            on_load();
        };
    });

    view! {
        <fieldset
            class="radio-group"
            // on:load=on_load
            _ref=fieldset_ref
        >
            <legend>{title}</legend>
            <div class="inputs">
                <For
                    each=options
                    key=move|(_, value)| *value
                    children=create_input
                />
            </div>
        </fieldset>
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnimationType {
    Animation,
    Transition,
}

#[component]
pub fn AnimatedReplacement<V, F>(
    view: F,
    animation_type: AnimationType,
) -> impl IntoView
where
    V: IntoView,
    F: Fn() -> Option<V> + 'static,
{
    let container = create_node_ref::<Div>();

    let current_element_ref = create_node_ref::<Div>();

    let pending_listener = store_value(None);
    let listener_function = store_value::<Option<Box<dyn Fn()>>>(None);

    create_effect(move |_| {
        let Some(container) = container.get() else {
            return;
        };

        listener_function.with_value(move |function| {
            if let Some(function) = function {
                function();
            }
        });
        pending_listener.set_value(None);
        listener_function.set_value(None);

        'current_element: {
            let Some(current_element) = current_element_ref.get_untracked() else {
                break 'current_element;
            };

            let _ = current_element.class_list().add_1("transition-out");

            let element_clone = current_element.clone();

            listener_function.set_value(Some(Box::new(move || {
                element_clone.remove();

                // remove the event listener
            })));

            let listener = EventListener::new(
                &current_element,
                match animation_type {
                    AnimationType::Animation => "animationend",
                    AnimationType::Transition => "transitionend",
                },
                move |_| {
                    listener_function.with_value(move |function| {
                        if let Some(function) = function {
                            function();
                        }
                    });
                    listener_function.set_value(None);
                    pending_listener.set_value(None);
                },
            );

            pending_listener.set_value(Some(listener));
        }

        if let Some(view) = view() {
            let next_element = view! {
                <div>
                    {view}
                </div>
            }
            .node_ref(current_element_ref);

            let _ = container.child(next_element);
        }
    });

    view! {
        <div
            class="animated-replacement"
            _ref=container
        >
        </div>
    }
}

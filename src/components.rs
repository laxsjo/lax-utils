use crate::{toasts, utils::*};
use leptos::{html::*, leptos_dom::helpers::*, window, *};
use leptos_router::*;
use std::{hash::*, time::Duration};

#[component]
pub fn RouteLink(
    cx: Scope,
    route_path: &'static str,
    // #[prop(optional)] _ref: Option<NodeRef<A>>,
    // _ref: NodeRef<A>,
    children: Children,
) -> impl IntoView {
    let location = use_location(cx);

    let is_open = move || location.pathname.get() == route_path;

    view! {cx,
        <a href=route_path class:active={is_open}>{children(cx)}</a>
    }
}

#[component]
pub fn FancySelect<T, F>(
    cx: Scope,
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
    let select_ref = create_node_ref::<Select>(cx);
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

    select_ref.on_load(cx, move |select| {
        // TODO: figure out what is changing the select value after loading
        set_timeout(
            move || {
                select.set_selected_index(selected_index as i32);
            },
            Duration::from_secs_f64(0.1),
        )
    });

    let generate_item = move |cx, item: T| {
        let selected = match default_selected {
            Some(selected) => item == selected,
            None => false,
        };
        view! { cx,
            <option
                selected=selected
            >
                {item.to_ui_string(env)}
            </option>
        }
    };

    view! { cx,
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
                    view=generate_item
                />
            </select>
            <Icon icon_id="expand_more"/>
        </div>
    }
}

/// Display an svg icon.
///
/// You can browse the available icons here: https://fonts.google.com/icons
/// The names you specify should be separated by underscores.
/// E.g. `border_color`
///
/// # Maintenance Note
/// The icons were generated using
/// [icomoon.io](https://icomoon.io/app/#/select), and exported as a single
/// svg symbol file called `symbol-defs.svg` on the page, which was then renamed
/// to `material-icons-defs.svg` and placed in the `assets/` folder.
#[component]
pub fn Icon(
    cx: Scope,
    /// The icon to display. Should be of the form `<icon_name>`
    icon_id: &'static str,
) -> impl IntoView {
    let use_element_str = format!("<use href=\"#icon-{}\"></use>", icon_id);

    view! { cx,
        <svg
            class="icon"
            viewBox="0 0 24 24"
            inner_html=use_element_str
        />
    }
}

#[component]
pub fn LabeledFloatInput(
    cx: Scope,
    #[prop(into)] prefix: MaybeSignal<Option<String>>,
    #[prop(into)] postfix: MaybeSignal<Option<String>>,
    children: Children,
) -> impl IntoView
where
{
    view! { cx,
        <div class="labeled-input">
            <span class="prefix">
                {prefix}
            </span>
            <span class="input">
                {children(cx)}
            </span>
            <span class="postfix">
                {postfix}
            </span>
        </div>
    }
}

#[component]
pub fn CopyableLabel(
    cx: Scope,
    #[prop(into)] content: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <CopyButton
            value=content
        >
            <span class="label">
                {children(cx)}
            </span>
        </CopyButton>
    }
}

// #[component]
// pub fn CopyableInput(
//     cx: Scope,
//     /// The value that will be copied into the clipboard. It is the callers
//     /// responsibility to make sure that this signal is kept in sync with the
//     /// value of the input!
//     #[prop(into)]
//     content: Signal<String>,
//     children: Box<dyn FnOnce(Scope) -> HtmlElement<Input>>,
// ) -> impl IntoView {
//     view! { cx,
//         <div class="copyable-input">
//             {children(cx)}
//             <Icon icon_id="content_copy" />
//         </div>
//     }
// }

#[component]
pub fn CopyButton(
    cx: Scope,
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
        toasts::add_toast(cx, format!("Copied '{}' to clipboard", value()));
    };

    let style = format!(
        "--duration: {}s; --duration-out: {}s",
        DURATION_IN_SECONDS, DURATION_OUT_SECONDS
    );

    view! { cx,
        <button
            class="copy-button"
            style=style
            aria-label=label.clone()
            title=label
            on:click=on_click
        >
            {children.map(|children| children(cx))}
            <Icon icon_id="content_copy" />
        </button>
    }
}

#[component]
pub fn RadioGroup<T, F, W>(
    cx: Scope,
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
) -> impl IntoView
where
    T: Copy + Hash + 'static + Eq + ToString,
    F: Fn(T) + 'static + Copy,
    W: Fn(Box<dyn Fn(T) + 'static>) + 'static + Copy,
{
    let any_changed = create_trigger(cx);

    let (current_value, set_current_value) =
        create_signal::<Option<T>>(cx, None);

    // log!("got set_value.is_none() {:?}", set_value.is_none());
    // if let Some(set_value) = set_value {
    //     let mut set_value = set_value.borrow_mut();

    //     *set_value = Some(Box::new(move |value: T| {
    //         log!("set value in callback");
    //         set_current_value(Some(value));
    //     }));
    //     log!("assigned set value callback");
    // }

    create_effect(cx, move |_| {
        if let Some(on_change) = on_change {
            let Some(value) = current_value() else {
                return;
            };
            on_change(value);
        }
    });

    let create_input = move |cx, (label, this_value): (String, T)| {
        let (checked, set_checked) = create_signal(cx, false);
        let input = create_node_ref::<Input>(cx);
        // let input

        create_effect(cx, move |_| {
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

        create_effect(cx, move |_| {
            set_checked(current_value() == Some(this_value));
        });

        // create_effect(cx, move |_| {

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

        view! {cx,
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

    let fieldset_ref = create_node_ref::<Fieldset>(cx);

    let on_load = move || {
        if let Some(with_set_value) = with_set_value {
            with_set_value(Box::new(move |value: T| {
                set_current_value(Some(value));
            }));
        }
    };

    create_effect(cx, move |_| {
        if let Some(_) = fieldset_ref.get() {
            on_load();
        };
    });

    view! { cx,
        <fieldset
            class="radio-group"
            // on:load=on_load
            _ref=fieldset_ref
        >
            <legend>{title}</legend>
            <For
                each=options
                key=move|(_, value)| *value
                view=create_input
            />
        </fieldset>
    }
}

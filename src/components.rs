use crate::utils::*;
use leptos::{html::*, leptos_dom::helpers::*, *};
use leptos_router::*;
use std::{hash::*, time::Duration};
use web_sys::Event;
// use web_sys::*;

#[component]
pub fn RouteLink(
    cx: Scope,
    route_name: &'static str,
    children: Children,
) -> impl IntoView {
    let location = use_location(cx);
    let route_path = "/".to_string() + route_name;

    let is_open = move || {
        location.pathname.get() == "/".to_string() + route_name
        // TODO: This is very ugly...
    };

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
    let (expanded, set_expanded) = create_signal(cx, false);

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
                log!("selected index {}", selected_index);
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
            class:expanded=expanded
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
        >
            {use_element_str}
        </svg>
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
    const DURATION_IN_SECONDS: f64 = 5.;
    const DURATION_OUT_SECONDS: f64 = 0.3;
    let (shown, set_shown) = create_signal(cx, false);
    let (hidden, set_hidden) = create_signal(cx, true);
    let (in_timeout_handle, set_in_timeout_handle) =
        create_signal::<Option<TimeoutHandle>>(cx, None);
    let (out_timeout_handle, set_out_timeout_handle) =
        create_signal::<Option<TimeoutHandle>>(cx, None);

    create_effect(cx, move |_| {
        if shown() {
            set_hidden(false);
        }
    });

    let activate_fade_out = move || {
        let Ok(handle) = set_timeout_with_handle(
            move || {
                set_out_timeout_handle(None);

                set_hidden(true);
            },
            Duration::from_secs_f64(DURATION_OUT_SECONDS),
        ) else {
            set_out_timeout_handle(None);
            return;
        };

        set_out_timeout_handle(Some(handle));
    };

    let activate_popup = move || {
        if let Some(handle) = out_timeout_handle() {
            handle.clear();
        }
        if let Some(handle) = in_timeout_handle() {
            handle.clear();
        }
        // log!("clicked!");
        set_shown(false);

        set_timeout(
            move || {
                set_shown(true);

                let Ok(handle) = set_timeout_with_handle(
                    move || {
                        // ? Does this cause a memory error if the component has been
                        // disposed of in the time between the timeout activation and
                        // this closure being ran?
                        set_shown(false);
                        activate_fade_out();
                    },
                    Duration::from_secs_f64(DURATION_IN_SECONDS),
                ) else {
                    set_in_timeout_handle(None);
                    return;
                };

                set_in_timeout_handle(Some(handle));
            },
            Duration::from_secs(0),
        );
    };

    let copy_to_clipboard = move || -> bool {
        let Some(clipboard) = window().navigator().clipboard() else {
            // TODO: give user feedback that copy to clipboard failed.
            error!("Failed to get clipboard");
            return false;
        };

        let _ = clipboard.write_text(content().as_str());

        activate_popup();

        true
    };

    let on_click = move |_| {
        copy_to_clipboard();
    };

    let style = format!(
        "--duration: {}s; --duration-out: {}s",
        DURATION_IN_SECONDS, DURATION_OUT_SECONDS
    );

    view! { cx,
        <button
            class="copyable-label"
            on:click=on_click
            style=style
        >
            // <Show
            //     when=popup_active
            //     fallback= move |_cx| view! {cx, <></> }
            // >
            //     <Transition/>
            // </Show>

            <span
                class="popup"
                aria-live="polite"
                class:shown=shown
                class:hidden=hidden
            >
                <Icon icon_id="check_circle"/>
                "Copied to clipboard"
            </span>
            <span class="label">
                {children(cx)}
            </span>
            <Icon icon_id="content_copy"/>
        </button>
    }
}

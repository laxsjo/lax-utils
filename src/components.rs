use crate::utils::*;
use leptos::{html::*, *};
use leptos_router::*;
use std::hash::*;

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

    view! { cx,
        <div class="fancy-select">
            <select
                id=select_id
                on:input=on_change
                _ref=select_ref
            >
                <For
                    each=items
                    key=|item: &T| {
                        *item
                    }
                    view=move |cx, item| view! { cx,
                        <option>{item.to_ui_string(env)}</option>
                    }
                />
            </select>
        </div>
    }
}

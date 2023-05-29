use crate::components::*;
use leptos::{
    leptos_dom::{helpers::TimeoutHandle, is_browser},
    *,
};
use std::time::Duration;

pub const TOAST_DURATION_SECONDS: f64 = 5.;
pub const TOAST_FADE_OUT_SECONDS: f64 = 0.3;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ToastHandle(u32);

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Toasts {
    toasts: Vec<(ToastHandle, String)>,
    next_id: u32,
}

/// Add a new toast and return a handle to that
pub fn add_toast(cx: Scope, message: String) {
    // TODO: this is bad as it uses the single global context.
    let toast = use_context::<RwSignal<String>>(cx)
        .expect("toasts context to be provided");

    toast.set(message);
}

pub fn use_toast(cx: Scope) -> Signal<String> {
    let toast = use_context::<RwSignal<String>>(cx)
        .expect("toasts context to be provided");

    toast.into()
}

/// Make sure that this is called **once** somewhere in the application.
pub fn provide_toast(cx: Scope) {
    let toast = create_rw_signal(cx, "".to_owned());
    provide_context(cx, toast);
}

/// Displays the currently active toasts.
#[component]
pub fn ToastsContainer(cx: Scope) -> impl IntoView {
    let new_toast = use_toast(cx);

    let next_index = store_value(cx, 0);

    let displayed_toasts = create_rw_signal::<Vec<(usize, String)>>(cx, vec![]);

    let timeout_handle: Option<TimeoutHandle> = None;

    create_effect(cx, move |last| {
        let toast = new_toast();
        if last.is_none() {
            return;
        }

        let index = next_index();

        displayed_toasts.update(move |toasts| {
            toasts.push((index, toast));
        });

        next_index.update_value(|i| *i += 1);

        set_timeout(
            move || {
                displayed_toasts.update(move |toasts| {
                    if let Some(index) =
                        toasts.iter().position(|(handle, _)| *handle == index)
                    {
                        toasts.remove(index);
                    }
                })
            },
            Duration::from_secs_f64(
                TOAST_DURATION_SECONDS + TOAST_FADE_OUT_SECONDS,
            ),
        );
    });

    view! { cx,
        <div
            class="toasts"
            style=("--duration-out", format!("{}s", TOAST_FADE_OUT_SECONDS))
        >
            <For
                each=displayed_toasts
                key=|(handle, _)| *handle
                view=move |cx, (_, message)| view! { cx,
                    <Toast
                        message=message
                        duration=Duration::from_secs_f64(TOAST_DURATION_SECONDS)
                    />
                }
            />
        </div>
    }
}

#[component]
pub fn Toast(cx: Scope, message: String, duration: Duration) -> impl IntoView {
    let (active, set_active) = create_signal(cx, true);

    if is_browser() {
        set_timeout(
            move || {
                set_active(false);
            },
            duration,
        )
    }
    view! { cx,
        <div
            class="toast"
            class:active=active
        >
        <Icon icon_id="check_circle"/>
        {message}
        </div>
    }
}

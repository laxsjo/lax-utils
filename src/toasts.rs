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
pub fn add_toast(message: String) {
    // TODO: this is bad as it uses the single global context.
    let toast = use_context::<RwSignal<String>>()
        .expect("toasts context to be provided");

    toast.set(message);
}

pub fn use_toast() -> Signal<String> {
    let toast = use_context::<RwSignal<String>>()
        .expect("toasts context to be provided");

    toast.into()
}

/// Make sure that this is called **once** somewhere in the application.
pub fn provide_toast() {
    let toast = create_rw_signal("".to_owned());
    provide_context(toast);
}

/// Displays the currently active toasts.
#[component]
pub fn ToastsContainer() -> impl IntoView {
    let new_toast = use_toast();

    // let next_index = store_value( 0);

    // let displayed_toasts = create_rw_signal::<Vec<(usize, String)>>(
    // vec![]);

    let (toast, set_toast) = create_signal(None);

    let timeout_handle = store_value::<Option<TimeoutHandle>>(None);

    create_effect(move |last| {
        let toast = new_toast();

        if last.is_none() {
            return;
        }

        if let Some(handle) = timeout_handle() {
            handle.clear();
        }

        set_toast(Some(toast));

        if let Ok(handle) = set_timeout_with_handle(
            move || {
                set_toast(None);
            },
            Duration::from_secs_f64(TOAST_DURATION_SECONDS),
        ) {
            timeout_handle.set_value(Some(handle));
        }

        // let index = next_index();

        // displayed_toasts.update(move |toasts| {
        //     toasts.push((index, toast));
        // });

        // next_index.update_value(|i| *i += 1);

        // set_timeout(
        //     move || {
        //         displayed_toasts.update(move |toasts| {
        //             if let Some(index) =
        //                 toasts.iter().position(|(handle, _)| *handle ==
        // index)             {
        //                 toasts.remove(index);
        //             }
        //         })
        //     },
        //     Duration::from_secs_f64(
        //         TOAST_DURATION_SECONDS + TOAST_FADE_OUT_SECONDS,
        //     ),
        // );
    });

    let toast_view = move || {
        toast().map(move |toast| {
            view! {
                <Toast
                    message=toast
                />
            }
        })
    };

    view! {
        <div
            class="toasts"
            style=("--duration-out", format!("{}s", TOAST_FADE_OUT_SECONDS))
        >
            <AnimatedReplacement
                view=toast_view
                animation_type=AnimationType::Animation
            />
            // <For
            //     each=displayed_toasts
            //     key=|(handle, _)| *handle
            //     view=move | (_, message)| view! {
            //         <Toast
            //             message=message
            //             duration=Duration::from_secs_f64(TOAST_DURATION_SECONDS)
            //         />
            //     }
            // />
        </div>
    }
}

#[component]
pub fn Toast(
    message: String,
    #[prop(optional)] duration: Option<Duration>,
) -> impl IntoView {
    let (active, set_active) = create_signal(true);

    if let Some(duration) = duration && is_browser() {
        set_timeout(
            move || {
                set_active(false);
            },
            duration,
        )
    }
    view! {
        <div
            class="toast"
            class:active=active
        >
        <Icon icon_id="check-circle"/>
        {message}
        </div>
    }
}

use std::{collections::HashSet, time::Duration};

use leptos::*;

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
pub fn add_toast(cx: Scope, message: String) -> ToastHandle {
    // TODO: this is bad as it uses the single global context.
    let toasts = use_context::<RwSignal<Toasts>>(cx)
        .expect("toasts context to be provided");
    let next_id = create_read_slice(cx, toasts, |toasts| toasts.next_id);
    let handle = ToastHandle(next_id());

    toasts.update(|toasts| {
        toasts.toasts.push((ToastHandle(toasts.next_id), message));
        toasts.next_id += 1;
    });

    set_timeout(
        move || remove_toast(cx, handle),
        Duration::from_secs_f64(TOAST_DURATION_SECONDS),
    );

    handle
}

pub fn remove_toast(cx: Scope, handle: ToastHandle) {
    let toasts = use_context::<RwSignal<Toasts>>(cx)
        .expect("toasts context to be provided");

    toasts.update(|toasts| {
        let Some(index) = toasts.toasts.iter().position(move |(this_handle,
    _)| *this_handle == handle) else {         return;
        };

        toasts.toasts.remove(index);
    });
}

pub fn use_toasts(cx: Scope) -> Signal<Vec<(ToastHandle, String)>> {
    let toasts = use_context::<RwSignal<Toasts>>(cx)
        .expect("toasts context to be provided");

    Signal::derive(cx, move || toasts().toasts)
}

/// Make sure that this is called **once** somewhere in the application.
pub fn provide_toasts(cx: Scope) {
    let toasts = create_rw_signal(cx, Toasts::default());
    provide_context(cx, toasts);
}

/// Displays the currently active toasts.
#[component]
pub fn ToastsContainer(cx: Scope) -> impl IntoView {
    let toasts = use_toasts(cx);

    let displayed_toasts = create_rw_signal(
        cx,
        toasts()
            .into_iter()
            .map(|toast| (toast.0, toast.1, true))
            .collect::<Vec<(_, _, _)>>(),
    );

    create_effect(cx, move |_| {
        let toasts_vec = toasts();
        let displayed_toasts_vec = displayed_toasts();

        let new_toasts = toasts_vec
            .iter()
            .map(|(handle, _)| *handle)
            .collect::<HashSet<ToastHandle>>();
        let old_toasts = displayed_toasts_vec
            .iter()
            .map(|(handle, _, _)| *handle)
            .collect::<HashSet<ToastHandle>>();

        let removed_toasts = &old_toasts - &new_toasts;
        let added_toasts = &new_toasts - &old_toasts;

        displayed_toasts.update(move |toasts| {
            toasts.append(
                &mut added_toasts
                    .into_iter()
                    .filter_map(|handle| {
                        toasts_vec.iter().find(|toast| toast.0 == handle).map(
                            |(handle, message)| {
                                (*handle, message.clone(), true)
                            },
                        )
                    })
                    .collect(),
            );

            for handle in removed_toasts {
                let Some(toast) = toasts.iter_mut().find(|(this_handle, _,_)| {
                    *this_handle == handle
                }) else {
                    log!("couldn't find toast handle {:?}", handle);
                    continue;

                };

                log!("hid toast handle {:?}", handle);

                toast.2 = false;

                // // ? One timeout per removed toast. This should be fine since
                // // only one toast will be removed at a time usually, right?
                // set_timeout(
                //     move || {
                //         displayed_toasts.update(move |toasts| {
                //             if let Some(index) =
                //                 toasts.iter().position(|(this_handle, _, _)|
                // {                     *this_handle == handle
                //                 })
                //             {
                //                 toasts.remove(index);
                //             };
                //         })
                //     },
                //     Duration::from_secs_f64(TOAST_FADE_OUT_SECONDS),
                // );
            }
        });
    });

    view! { cx,
        <div class="toasts">
            <For
                each=displayed_toasts
                key=|(handle, _, _)| *handle
                view=move |cx, (_, message, shown)| view! { cx,
                    <div
                        class="toast"
                        class:active=shown
                    >{message}</div>
                }
            />
        </div>
    }
}

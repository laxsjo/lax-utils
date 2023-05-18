use std::time::Duration;

use leptos::*;

pub const TOAST_DURATION_SECONDS: f64 = 5.;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
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

    set_timeout(move || {
        remove_toast(cx, handle)
    }, Duration::from_secs_f64(TOAST_DURATION_SECONDS));

    handle
}

pub fn remove_toast(cx: Scope, handle: ToastHandle) {
    let toasts = use_context::<RwSignal<Toasts>>(cx)
        .expect("toasts context to be provided");

    toasts.update(|toasts| {
        let Some(index) = toasts.toasts.iter().position(move |(this_handle, _)| *this_handle == handle) else {
            return;
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

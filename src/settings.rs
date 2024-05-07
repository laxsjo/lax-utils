use crate::{components::*, string_utils::StringUtils};
use gloo_events::EventListener;
use leptos::{
    html::*, leptos_dom::is_browser, logging::error, window, HtmlElement, *,
};
use std::{
    any::{Any, TypeId},
    // cell::RefCell,
    hash::Hash,
    marker::*,
    // rc::Rc,
    str::FromStr,
};
use wasm_bindgen::*;
use web_sys::*;

pub fn local_storage_value<T>(key: &'static str) -> Option<T>
where
    T: FromStr + ToString,
{
    let Ok(storage) = window().local_storage() else {
        return None;
    };

    let Ok(value_str) = storage?.get_item(key) else {
        return None;
    };
    let Ok(value) = value_str?.parse::<T>() else {
        return None
    };
    Some(value)
}

pub fn local_storage_set_value<T>(key: &'static str, value: T) -> Option<()>
where
    T: FromStr + ToString,
{
    let Ok(storage) = window().local_storage() else {
        return None;
    };

    let Ok(()) = storage?.set_item(key, &value.to_string()) else {
        return None
    };

    Some(())
}

#[component]
pub fn StoredInput<T>(
    input: HtmlElement<Input>,
    /// A unique key used to identify and store the setting in local storage.
    key: &'static str,
    #[prop(optional)] _type: PhantomData<T>,
    #[prop(optional)] value: Option<RwSignal<T>>,
) -> impl IntoView
where
    T: FromStr + ToString + Clone + 'static + Any,
{
    if !is_browser() {
        return input;
    }

    let input_ref = create_node_ref::<Input>();
    let input = input.node_ref(input_ref);

    fn update_storage<U: FromStr + ToString + 'static + Any>(
        key: &'static str,
        value: U,
    ) {
        if local_storage_set_value(key, value).is_none() {
            error!("Failed to store value.");
        }
    }

    let set_input_value = move |value: &T| {
        // leptos_reactive::SpecialNonReactiveZone::enter();
        let Some(input) = input_ref.get() else {
            error!("Couldn't find input element!");
            return;
        };

        let type_ = input.type_();
        if &type_ == "checkbox" || &type_ == "radio" {
            let value: &dyn Any = value;

            let Some(checked) = value.downcast_ref::<bool>() else {
                // this is ugly...
                panic!("stored input with type other than bool used for an input of type checkbox or radio");
            };

            input.set_checked(*checked);
        } else {
            input.set_value(&value.to_string());
        }
    };

    let listener = EventListener::new(&input, "change", move |ev| {
        // leptos_reactive::SpecialNonReactiveZone::enter();
        let Some(input) = input_ref.get() else {
            error!("Couldn't find input element!");
            return;
        };

        let type_ = input.type_();
        if &type_ == "checkbox" || &type_ == "radio" {
            if TypeId::of::<T>() != TypeId::of::<bool>() {
                panic!("stored input with type other than bool used for an input of type checkbox or radio");
            }

            let checked = event_target_checked(ev);

            update_storage(key, checked);
            let Some(value) = value else {
                return;
            };
            let value: &dyn Any = &value;
            let Some(value) = value.downcast_ref::<RwSignal<bool>>() else {
                panic!("stored input with type other than bool used for an input of type checkbox or radio");
            };
            value.set(checked);
            return;
        }

        let value = event_target_value(ev);
        let Some(value) = value.parse_input::<T>() else {
            // failed to parse input
            // ? Should I log this somehow?
            return;
        };
        update_storage(key, value);
    });
    store_value(listener);

    if let Some(value) = value {
        if let Some(value_local) = local_storage_value::<T>(key) {
            value.set(value_local);
        };
    };

    create_effect(move |_| {
        let Some(value) = value else {
            return;
        };

        let value = value.get();

        update_storage(key, value.clone());

        set_input_value(&value);

        // let Some(input) = input_ref.get() else {
        //     return;
        // };
        // input.set_value(&value.to_string());
    });

    input.on_mount(move |input| {
        let Some(value_local) = (match local_storage_value::<T>(key) {
            Some(value) => Some(value),
            None => value.map(|value| value.get_untracked())
        }) else {
            return;
        };

        set_input_value(&value_local);

        if let Some(value) = value {
            value.set(value_local);
        };

        let result: Result<(), JsValue> = try {
            input.dispatch_event(&Event::new("input")?)?;
            input.dispatch_event(&Event::new("change")?)?;
        };
        if result.is_err() {
            error!("failed to dispatch input or change event for StoredInput");
        }
    })
}

#[allow(clippy::diverging_sub_expression, unreachable_code)]
#[component]
pub fn StoredRadioGroup<T, F>(
    #[prop(into)] title: MaybeSignal<String>,
    #[prop(into)] name: Signal<String>,
    #[prop(into)] options: MaybeSignal<Vec<(String, T)>>,
    #[prop(optional)] on_change: Option<F>,
    key: &'static str,
) -> impl IntoView
where
    T: Copy + Hash + 'static + Eq + ToString + FromStr,
    F: Fn(T) + 'static + Copy,
{
    let on_change = move |value: T| {
        if local_storage_set_value(key, value).is_none() {
            error!("Failed to store value.");
        }

        if let Some(on_change) = on_change {
            on_change(value);
        }
    };

    let with_set_value = move |set_value: Box<dyn Fn(T)>| {
        let Some(value) = local_storage_value::<T>(key) else {
                    return;
        };
        // log!("set value");
        set_value(value);
        on_change(value);
    };

    view! {
        <RadioGroup
            title=title
            name=name
            options=options
            on_change=on_change
            with_set_value=with_set_value
        />
    }
}

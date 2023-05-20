use crate::{components::*, string_utils::StringUtils};
use gloo_events::*;
use leptos::{html::*, leptos_dom::is_browser, *};
use std::{
    any::{Any, TypeId},
    // cell::RefCell,
    hash::Hash,
    marker::*,
    // rc::Rc,
    str::FromStr,
};

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
    cx: Scope,
    input: HtmlElement<Input>,
    /// A unique key used to identify and store the setting in local storage.
    key: &'static str,
    #[prop(optional)] _type: PhantomData<T>,
) -> impl IntoView
where
    T: FromStr + ToString + 'static + Any,
{
    if !is_browser() {
        return input;
    }

    let input_ref = create_node_ref::<Input>(cx);
    let input = input.node_ref(input_ref);

    let listener = EventListener::new(&input, "change", move |ev| {
        leptos_reactive::SpecialNonReactiveZone::enter();
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

            if local_storage_set_value(key, checked).is_none() {
                error!("Failed to store value.");
            }
            return;
        }

        let value = event_target_value(ev);
        let Some(value) = value.parse_input::<T>() else {
            // failed to parse input
            // ? Should I log this somehow?
            return;
        };
        if local_storage_set_value(key, value).is_none() {
            error!("Failed to store value.");
        }
    });
    store_value(cx, listener);

    input.on_mount(move |input| {
        let type_ = input.type_();
        if &type_ == "checkbox" || &type_ == "radio" {
            // this is ugly...
            if TypeId::of::<T>() != TypeId::of::<bool>() {
                panic!("stored input with type other than bool used for an input of type checkbox or radio");
            }

            let Some(value) = local_storage_value::<bool>(key) else {
                return;
            };


            input.set_checked(value);
        } else {
            let Some(value) = local_storage_value::<T>(key) else {
            return;
            };
            input.set_value(&value.to_string());
        }
    })
}

#[allow(clippy::diverging_sub_expression, unreachable_code)]
#[component]
pub fn StoredRadioGroup<T, F>(
    cx: Scope,
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
    };

    view! { cx,
        <RadioGroup
            title=title
            name=name
            options=options
            on_change=on_change
            with_set_value=with_set_value
        />
    }

    // // let set_value_rc: OptionalRcBox<dyn Fn(T) + 'static> =
    // //     Rc::new(RefCell::new(None));

    // // let on_change = move |value: T| {
    // //     if local_storage_set_value(key, value).is_none() {
    // //         error!("Failed to store value.");
    // //     }

    // //     if let Some(on_change) = on_change {
    // //         on_change(value);
    // //     }
    // // };

    // let view = view! { cx,
    //     <RadioGroup
    //         title=title
    //         name=name
    //         options=options
    //         on_change=on_change
    //         set_value=set_value_rc.clone()
    //     />
    // };

    // log!("got RadioGroup, is_browser()={:?}", is_browser());

    // if is_browser() {
    //     // let set_value = set_value_rc.borrow();
    //     // let set_value =
    //     //     set_value.as_ref().expect("set_value should have been set");

    //     // let Some(value) = local_storage_value::<T>(key) else {
    //     //     return view;
    //     // };
    //     log!("set value");
    //     // set_value(value);

    //     // let listener =
    //     //     EventListener::new(&document(), "DOMContentLoaded", move |_ev|
    // {     //         log!("listener ran");
    //     //         let set_value = set_value_rc.borrow();
    //     //         let set_value =
    //     //             set_value.as_ref().expect("set_value to have been
    // set");

    //     //         let Some(value) = local_storage_value::<T>(key) else {
    //     //             return;
    //     //         };
    //     //         log!("set value");
    //     //         set_value(value);
    //     //     });
    //     // log!("created listener");
    //     // store_value(cx, listener);
    // }
    // // set_value(value);

    // view
}

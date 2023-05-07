use crate::{MultiplyWidget, MultiplyWidgetProps};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/lax-utils.css"/>

        // sets the document title
        <Title text="Welcome to Leptos :D"/>

        // content for this welcome page
        <Router>
            <aside>
                <ul>
                    <li><a href="/">"Homepage"</a></li>
                    <li><a href="/color-picker">"Color Picker"</a></li>
                    <li><a href="/base-converter">"Base Converter"</a></li>
                    <li><a href="/time-zones">"Time Zone Converter"</a></li>
                </ul>
            </aside>
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/about" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/color-picker" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/base-converter" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/time-zones" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
fn OtherPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Welcome to the other page"</h1>
        <MultiplyWidget label={"hi".to_owned()}/>
    }
}

use crate::{components::*, routes::*, toasts::*};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    provide_toasts(cx);

    // This is very cursed
    let icons_svg = include_str!("../assets/material-icons-defs.svg");

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/lax-utils.css"/>

        // sets the document title
        <Title text="lax-utils"/>

        <div class="hidden" inner_html=icons_svg />

        // content for this welcome page
        <Router>
            <ToastsContainer />
            <aside class="side-nav">
                <nav>
                    <li><RouteLink route_name="">"Home"</RouteLink></li>
                    <li><RouteLink route_name="color-picker">"Color Picker"</RouteLink></li>
                    <li><RouteLink route_name="base-converter">"Base Converter"</RouteLink></li>
                    <li><RouteLink route_name="time-zones">"Time Zone Converter"</RouteLink></li>
                </nav>
            </aside>
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <RouteHome/> }/>
                    <Route path="/about" view=|cx| view! { cx, <RouteHome/> }/>
                    <Route path="/color-picker" view=|cx| view! { cx, <RouteColorPicker/> }/>
                    <Route path="/base-converter" view=|cx| view! { cx, <RouteHome/> }/>
                    <Route path="/time-zones" view=|cx| view! { cx, <RouteHome/> }/>
                </Routes>
            </main>
            <footer>
                <A href="/about">"About"</A>
            </footer>
        </Router>
    }
}

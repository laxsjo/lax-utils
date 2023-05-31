use std::time::Duration;

use crate::{components::*, routes::*, toasts::*};
use leptos::{html::*, *};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    leptos_reactive::SpecialNonReactiveZone::enter(); // doesn't seem to work :(

    provide_meta_context(cx);

    provide_toast(cx);

    // This is very cursed
    let icons_svg = include_str!("../assets/material-icons-defs.svg");

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/lax-utils.css"/>

        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>
        // <Link href="https://fonts.googleapis.com/css2?family=Overpass+Mono&family=Quicksand:wght@300;400;500&display=swap" rel="stylesheet"/>
        <Link href="https://fonts.googleapis.com/css2?family=Quicksand:wght@300;400;500;600;700&display=swap" rel="stylesheet"/>
        // <Link href="https://fonts.googleapis.com/css2?family=Chivo+Mono&family=Overpass+Mono&family=Quicksand:wght@300;400;500&display=swap" rel="stylesheet"/>

        // sets the document title
        <Title text="lax-utils"/>

        <div class="hidden" inner_html=icons_svg />

        // content for this welcome page
        <Router>
            <ToastsContainer />
            <SideNav />
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <RouteHome/> }/>
                    <Route path="/about" view=|cx| view! { cx, <RouteAbout/> }/>
                    <Route path="/color-picker" view=|cx| view! { cx, <RouteColorPicker/> }/>
                    <Route path="/color-picker/test" view=|cx| view! { cx, <RouteComingSoon/> }/>
                    <Route path="/base-converter" view=|cx| view! { cx, <RouteComingSoon/> }/>
                    <Route path="/time-zones" view=|cx| view! { cx, <RouteComingSoon/> }/>
                </Routes>
            </main>
            <footer>
                <ul class="links">
                    <li>
                        <Icon icon_id="help"/>
                        <A href="/about">"About"</A>
                    </li>
                    <li>
                        <Icon icon_id="code"/>
                        <A href="https://github.com/laxsjo/lax-utils">"Repository"</A>
                    </li>
                </ul>
                <div class="right">
                    "Made with ❤️ by "
                    <A href="https://github.com/laxsjo">"Rasmus Söderhielm"</A>
                </div>
            </footer>
        </Router>
    }
}

#[component]
pub fn SideNav(cx: Scope) -> impl IntoView {
    let (js_enabled, set_js_enabled) = create_signal(cx, false);
    create_effect(cx, move |_| {
        set_interval(
            move || {
                set_js_enabled(true);
            },
            Duration::from_secs_f64(0.1),
        );
    });

    let paths = vec![
        ("/", "Home"),
        ("/color-picker", "Color Picker"),
        ("/base-converter", "Base Converter"),
        ("/time-zones", "Time Zone Converter"),
    ];
    let mut node_refs = vec![];
    for _ in 0..(paths.len()) {
        node_refs.push(create_node_ref::<Li>(cx));
    }

    let routes = store_value(
        cx,
        paths
            .into_iter()
            .zip(node_refs)
            .map(|((path, name), node_ref)| (path, name, node_ref))
            .collect::<Vec<_>>(),
    );

    let location = use_location(cx);

    let current_bounds = create_memo(cx, move |_| {
        let selected_path = location.pathname.get();

        let mut should_stop_next = true;
        routes.with_value(move |routes| {
            routes
                .iter()
                .take_while(move |(path, _, _)| {
                    let should_stop = should_stop_next;

                    should_stop_next = path != &selected_path; // this is so cursed...

                    should_stop
                })
                .fold(
                    (0, 0),
                    |(total_height, previous_height), (_, _, _ref)| {
                        let Some(element) = _ref.get() else {
                            return (total_height, 0);
                        };
                        let height = element.offset_height();
                        // log!("got height {}", height);
                        (total_height + previous_height, height)
                    },
                )
        })
    });
    let current_offset = move || current_bounds().0;
    let current_height = move || current_bounds().1;

    let generate_routes = || {
        Fragment::new(
            routes
                .get_value()
                .iter()
                .map(move |(path, name, _ref)| {
                    let path = *path;
                    let name = *name;
                    let _ref = *_ref;
                    view! { cx,
                        <li
                            _ref=_ref
                        >
                            <RouteLink route_path=path>{name}</RouteLink>
                        </li>
                    }
                    .into_view(cx)
                })
                .collect::<Vec<View>>(),
        )
    };

    view! { cx,
        <aside
            class="side-nav"
            data-js-enabled=js_enabled
            style=("--y-offset", move || format!("{}px", current_offset()))
            style=("--height", move || format!("{}px", current_height()))
        >
            <nav>
                {generate_routes()}
            </nav>
        </aside>
    }
}

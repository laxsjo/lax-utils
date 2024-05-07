use std::time::Duration;

use crate::{components::*, routes::*, toasts::*};
use leptos::{component, html::*, *};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    // leptos_reactive::SpecialNonReactiveZone::enter(); // doesn't seem to work
    // :(

    provide_meta_context();

    provide_toast();

    // This is very cursed
    let icons_svg = include_str!("../assets/icon-symbol-defs.svg");

    view! {
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
                    <Route path="/" view=|| view! {  <RouteHome/> }/>
                    <Route path="/about" view=|| view! {  <RouteAbout/> }/>
                    <Route path="/color-picker" view=|| view! {  <RouteColorPicker/> }/>
                    <Route path="/color-picker/test" view=|| view! {  <RouteComingSoon/> }/>
                    <Route path="/base-converter" view=|| view! {  <RouteComingSoon/> }/>
                    <Route path="/time-zones" view=|| view! {  <RouteComingSoon/> }/>
                </Routes>
            </main>
            <footer>
                <ul class="links">
                    <li>
                        <Icon icon_id="help-circle"/>
                        <A href="/about">"About"</A>
                    </li>
                    <li>
                        <Icon icon_id="github"/>
                        <a
                            href="https://github.com/laxsjo/lax-utils"
                            target="_blank"
                        >
                            "Repository"
                        </a>
                    </li>
                </ul>
                <div class="right">
                    "Made with ❤️ by "
                    <a
                        href="https://github.com/laxsjo"
                        target="_blank"
                    >"Rasmus Söderhielm"</a>
                </div>
            </footer>
        </Router>
    }
}

#[component]
pub fn SideNav() -> impl IntoView {
    let (js_enabled, set_js_enabled) = create_signal(false);
    create_effect(move |_| {
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
        node_refs.push(create_node_ref::<Li>());
    }

    let routes = store_value(
        paths
            .into_iter()
            .zip(node_refs)
            .map(|((path, name), node_ref)| (path, name, node_ref))
            .collect::<Vec<_>>(),
    );

    let location = use_location();

    let marker_hidden = create_memo(move |_| {
        let selected_path_owned = location.pathname.get();
        let selected_path = selected_path_owned.as_str();

        routes.with_value(move |routes| {
            routes
                .iter()
                .all(move |(path, _, _)| *path != selected_path)
        })
    });

    let current_bounds = create_memo(move |prev_value: Option<&(i32, i32)>| {
        let selected_path_owned = location.pathname.get();
        let selected_path = selected_path_owned.as_str();

        if marker_hidden() {
            let prev_value = prev_value.map_or((0, 0), move |value| *value);
            return prev_value;
        }

        let mut should_continue_next = true;
        routes.with_value(move |routes| {
            routes
                .iter()
                .take_while(move |(path, _, _)| {
                    let should_continue = should_continue_next;

                    should_continue_next = *path != selected_path; // this is so cursed...

                    should_continue
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
                    view! {
                        <li
                            _ref=_ref
                        >
                            <RouteLink route_path=path>{name}</RouteLink>
                        </li>
                    }
                    .into_view()
                })
                .collect::<Vec<View>>(),
        )
    };

    let (transition_pos, set_transition_pos) = create_signal(true);
    create_effect(move |_| {
        if marker_hidden() {
            set_transition_pos(false);
        }
    });

    let on_transition_start = move |_| {
        set_transition_pos(!marker_hidden());
    };

    view! {
        <aside
            class="side-nav"
            data-js-enabled=js_enabled
            style=("--y-offset", move || format!("{}px", current_offset()))
            style=("--height", move || format!("{}px", current_height()))
            data-marker-hidden=marker_hidden
            class=("transition-pos", transition_pos)
            on:transitionstart=on_transition_start
        >
            <nav>
                {generate_routes()}
            </nav>
        </aside>
    }
}

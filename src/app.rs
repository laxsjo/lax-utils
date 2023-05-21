use std::time::Duration;

use crate::{components::*, routes::*, toasts::*};
use leptos::{html::*, *};
use leptos_meta::*;
use leptos_router::*;
use web_sys::*;

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

        // <Test>
        //     <input />
        // </Test>

        // content for this welcome page
        <Router>
            <ToastsContainer />
            <SideNav />
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <RouteHome/> }/>
                    <Route path="/about" view=|cx| view! { cx, <RouteHome/> }/>
                    <Route path="/color-picker" view=|cx| view! { cx, <RouteColorPicker/> }/>
                    <Route path="/color-picker/test" view=|cx| view! { cx, <RouteHome/> }/>
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
        ("/time-zone-converter", "Time Zone Converter"),
    ];
    let mut node_refs = vec![];
    for _ in 0..(paths.len()) {
        node_refs.push(create_node_ref::<Li>(cx));
    }
    // let node_refs = store_value(cx, node_refs);

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
                    // log!("{:?} != {:?}", path, &selected_path);

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

    create_effect(cx, move |_| {
        let pathname = location.pathname.get();
    });

    let generate_routes = || {
        // let s = {
        //     leptos::Fragment::lazy(|| {
        //         vec![
        //             {
        //                 ::leptos::HtmlElement::from_chunks(
        //                     cx,
        //                     leptos::leptos_dom::html::Li::default(),
        //                     [leptos::leptos_dom::html::StringOrView::String(
        //                         format!(
        //                             "<li id=\"_{}\"></li>",
        //                             leptos::leptos_dom::HydrationCtx::peek()
        //                         )
        //                         .into(),
        //                     )],
        //                 )
        //             }
        //             .into_view(cx),
        //             {
        //                 ::leptos::HtmlElement::from_chunks(
        //                     cx,
        //                     leptos::leptos_dom::html::Li::default(),
        //                     [leptos::leptos_dom::html::StringOrView::String(
        //                         format!(
        //                             "<li id=\"_{}\"></li>",
        //                             leptos::leptos_dom::HydrationCtx::peek()
        //                         )
        //                         .into(),
        //                     )],
        //                 )
        //             }
        //             .into_view(cx),
        //             {
        //                 ::leptos::HtmlElement::from_chunks(
        //                     cx,
        //                     leptos::leptos_dom::html::Li::default(),
        //                     [leptos::leptos_dom::html::StringOrView::String(
        //                         format!(
        //                             "<li id=\"_{}\"></li>",
        //                             leptos::leptos_dom::HydrationCtx::peek()
        //                         )
        //                         .into(),
        //                     )],
        //                 )
        //             }
        //             .into_view(cx),
        //         ]
        //     })
        //     .with_view_marker("-0")
        // };
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
                // <For
                //     each=move|| routes
                //     key=move|(path, _, _)| path
                //     view=move|cx, (path, name, _ref)| view! { cx,
                //         <li _ref=_ref>
                //             <RouteLink route_path=path>{name}</RouteLink>
                //         </li>
                //     }
                // />

                // <li><RouteLink route_path="/color-picker">"Color Picker"</RouteLink></li>
                // <li><RouteLink route_path="/base-converter">"Base Converter"</RouteLink></li>
                // <li><RouteLink route_path="/time-zones">"Time Zone Converter"</RouteLink></li>
            </nav>
        </aside>
    }
}

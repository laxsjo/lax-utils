use crate::color_picker::components::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

/// Renders the home page of your application.
#[component]
pub fn RouteHome(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Welcome to lax-utils!"</h1>
        <p>
            "This websites host a collection of small utility programs. The
            color picker utility is the only one that's completed at this point
            though."
        </p>
    }
}

#[component]
pub fn RouteColorPicker(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Color Picker | lax-utils"/>

        <h1>"Color Picker"</h1>
        <ColorPicker/>
    }
}

#[component]
pub fn RouteComingSoon(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Coming (probably not so) soon!"</h1>
        <p>"Yea it might be a while before this get's added..."</p>
        <p>":)"</p>
    }
}

#[component]
pub fn RouteAbout(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"About"</h1>
        <p>
            "Lax-utils is a collection of small utility programs. The uniting
            property between these is that they're all programs that I've
            wanted and been frustrated by the lack of good options available."
        </p>
        <p>
            "As you've probably noticed by now, the "
            <A href="/color-picker">"color picker page"</A>
            " is the only page that currently exists. The rest are
            coming soon-ish™."
        </p>
        <p>
            "I don't wan't these to only be the perfect utilities for me. I
            also want them to be perfect for " <em>"you!"</em>
            <br/>
            "If you have any suggestions, requests, or general feedback, feel
            free to send me an email at "
            <a href="mailto:rasmus.soderhielm@gmail.com">
                "rasmus.soderhielm@gmail.com"
            </a>
            "."
        </p>
        <p>
            "You could also "
            <a
                href="https://github.com/laxsjo/lax-utils/issues"
                target="_blank"
            >"open an issue on github"</a>
            ", or even create a pull request if you're feeling adventurous."
        </p>

        <h2>"The Creator"</h2>
        <p>
            "This website was originally created as a school project to end the
            second year of gymnasium."
        </p>
        <p>
            "It was created by me, Rasmus Söderhielm. You can find "
            <a
                href="https://github.com/laxsjo"
                target="_blank"
            >"my github here"</a>
            "."
        </p>
        <p>
            <a
                href="https://github.com/laxsjo/lax-utils"
                target="_blank"
            >"The website repository"</a>
        </p>

    }
}

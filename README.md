<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Lax-Utils

Lax-Utils is a website holding a collection of small utility functions.

The only page that has been finished yet is the color picker page.
But I plan to add a numerical base converter and a time zone converter page.

The idea behind all these pages is that these are all things that I've needed,
but been disappointed in the lack of good polished options online.

This project was originally started as a school project during spring 2023 at Arena Academy.

## Executing a Server on a Remote Machine Without the Toolchain

After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:

```text
leptos_start
site/
```

Set the following environment variables (updating for your project as needed):

```text
LEPTOS_OUTPUT_NAME="lax-utils"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```

Finally, run the server binary.

potential logo? https://www.flaticon.com/free-icon/fish_3162099?term=salmon&related_id=3162099

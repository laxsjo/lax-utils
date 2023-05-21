<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Lax-Utils

Lax-Utils is a website holding a collection of small utility functions.

The only page that has been added yet is the color picker page.
But I plan to add a numerical base converter and a time zone converter page
at some point in the future (don't know when though...).

The idea behind all these pages is that these are all things that I've needed,
but been disappointed in the lack of good polished options online.

This project started as a school project (and still is ;) ) during spring 2023 at Arena Academy.

## Local Setup

This section explains how to create a local dev server.
I've tested this on windows, but the steps should theoretically work on any os.
Some shell syntax could still be different though.

I will also assume that you have a blank computer without rust installed, so you
can skip any irrelevant steps for your specific setup.

You first need to install rustup, a program to manage your installed rust
versions. You can find instructions for how to install it on their website: [rustup.rs](https://rustup.rs/)

Once rustup is installed the next step is to install nightly and the wasm
target. Run these in your terminal.

```powershell
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
```

You then need to install [cargo-leptos](https://github.com/leptos-rs/cargo-leptos).

```powershell
cargo install cargo-leptos
```

Then cd into the parent-folder where you want the folder containing your local copy of the project.
Then clone the project.

```powershell
git clone https://github.com/laxsjo/lax-utils.git
```

Then build, run, and watch for any changes to the project using the following
command.

```powershell
cargo leptos watch
```

If the build fails, you may need to install wasm a second time. That solved the
issue for me, I have no why though ¯\\\_(ツ)\_/¯.

```powershell
rustup target add wasm32-unknown-unknown
```

After it has finished compiling (it will take a time the first time you do it),
the local server will automatically be started under `localhost:3000`.

<!-- ## Executing a Server on a Remote Machine Without the Toolchain

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

Finally, run the server binary. -->

<!-- potential logo? https://www.flaticon.com/free-icon/fish_3162099?term=salmon&related_id=3162099 -->

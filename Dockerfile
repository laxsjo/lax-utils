FROM rustlang/rust:nightly AS prereq
RUN cargo install cargo-chef
RUN cargo install cargo-leptos
# target wasm32-unknown-unknown
RUN rustup target add wasm32-unknown-unknown
RUN curl -sL https://deb.nodesource.com/setup_12.x | bash -
RUN apt-get update && apt-get install nodejs
RUN npm install -g sass

FROM prereq AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM prereq as cacher
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --package=lax-utils --bin=lax-utils --target-dir=target/server --no-default-features --features=ssr  --recipe-path recipe.json
RUN cargo chef cook --release --package=lax-utils --target-dir=target/front --target=wasm32-unknown-unknown --no-default-features --features=hydrate  --recipe-path recipe.json

FROM prereq as builder
COPY . /app
WORKDIR /app
# copy dependecies
COPY --from=cacher /app/target /app/target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
# set env variables for build
# The source style file. If it ends with _.sass_ or _.scss_ then it will be compiled by `dart-sass`
# into CSS and processed by lightning css. When release is set, then it will also be minified.
ENV LEPTOS_STYLE_FILE "style/main.scss"
# The browserlist https://browsersl.ist query used for optimizing the CSS.
ENV LEPTOS_BROWSERQUERY "defaults"
# cursed step that fixes outdated project dependency to wasm-bindgen 0.2.84
# this will probably break once cargo-leptos updates their dependency...
RUN cargo update -p wasm-bindgen --precise 0.2.86
# build the app
RUN cargo leptos build --release

# use googles distroless as runtime image
FROM gcr.io/distroless/cc-debian11
# copy app form builder
COPY --from=builder /app/target/server/release/lax-utils /app/
COPY --from=builder /app/target/site /app/site
WORKDIR /app

# Site .env parameters cargo-leptos
ENV OUTPUT_NAME "lax-utils"
ENV LEPTOS_OUTPUT_NAME "lax-utils"
ENV LEPTOS_SITE_ROOT "site"
ENV LEPTOS_SITE_PKG_DIR "pkg"
ENV LEPTOS_ASSETS_DIR "assets"
ENV LEPTOS_SITE_ADDR "0.0.0.0:80"
EXPOSE "80"
# for local testing, I think?
# ENV LEPTOS_SITE_ADDR "0.0.0.0:3000"
# EXPOSE "3000"

# start the application
CMD ["./lax-utils"]
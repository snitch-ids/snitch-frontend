FROM rust:latest AS BUILDER

WORKDIR snitch-frontend

RUN cargo install --locked wasm-bindgen-cli
RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown
COPY . .
RUN mv tailwindcss-linux-arm64 tailwindcss
RUN trunk build

FROM nginx:latest AS RUNNER

COPY --from=BUILDER /snitch-frontend/dist/ /usr/share/nginx/html/

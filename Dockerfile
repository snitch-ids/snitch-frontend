FROM rust:latest AS BUILDER
ARG TARGETPLATFORM
WORKDIR snitch-frontend

RUN cargo install --locked wasm-bindgen-cli
RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown
COPY . .
RUN echo "I'm building for $TARGETPLATFORM"
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then mv tailwindcss-linux-arm64 tailwindcss; elif [ "$TARGETPLATFORM" = "linux/x86_64" ]; then mv tailwindcss-linux-x86 tailwindcss; fi
RUN trunk build

FROM nginx:latest AS RUNNER

COPY --from=BUILDER /snitch-frontend/dist/ /usr/share/nginx/html/

FROM rust:latest AS BUILDER
ARG TARGETPLATFORM="linux/x86_64"
ENV SNITCH_BACKEND_URL="https://api.snitch.cool"

WORKDIR snitch-frontend

RUN cargo install --locked wasm-bindgen-cli
RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown
COPY . .
RUN echo "I'm building for $TARGETPLATFORM"
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then ARCHITECTURE=linux-arm64; elif [ "$TARGETPLATFORM" = "linux/x86_64" ]; then ARCHITECTURE=linux-x64; fi \
  && curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.16/tailwindcss-${ARCHITECTURE} \
  && chmod +x tailwindcss-${ARCHITECTURE} \
  && mv tailwindcss-${ARCHITECTURE} tailwindcss

RUN trunk build --release

FROM nginx:latest AS RUNNER

COPY ./nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=BUILDER /snitch-frontend/dist/ /usr/share/nginx/html/

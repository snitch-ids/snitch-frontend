FROM rust:latest AS BUILDER
RUN rustup target add wasm32-unknown-unknown
RUN apt update && apt install npm -y
RUN npm install -D sass tailwindcss
RUN cargo install --locked wasm-bindgen-cli
RUN cargo install --locked trunk

WORKDIR snitch-frontend
COPY . .
RUN trunk build

FROM nginx:latest AS RUNNER
COPY --from=BUILDER /snitch-frontend/dist/ /usr/share/nginx/html/

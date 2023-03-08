FROM rust:latest AS BUILDER

WORKDIR snitch-frontend

# nvm environment variables
ENV NVM_DIR /usr/local/nvm
ENV NODE_VERSION 16.16.0

# install nvm
# https://github.com/creationix/nvm#install-script
RUN curl --silent -o- https://raw.githubusercontent.com/creationix/nvm/v0.31.2/install.sh | bash

# install node and npm
RUN bash -c "source $NVM_DIR/nvm.sh \
    && nvm install $NODE_VERSION \
    && nvm alias default $NODE_VERSION \
    && nvm use default"

# add node and npm to path so the commands are available
ENV NODE_PATH $NVM_DIR/v$NODE_VERSION/lib/node_modules
ENV PATH $NVM_DIR/versions/node/v$NODE_VERSION/bin:$PATH

# confirm installation
RUN node -v
RUN npm -v


RUN npm install -D sass
RUN rustup target add wasm32-unknown-unknown
RUN npm install -D tailwindcss
# RUN npm install -D sass tailwindcss
RUN cargo install --locked wasm-bindgen-cli
RUN cargo install --locked trunk

COPY . .
RUN trunk build

FROM nginx:latest AS RUNNER
COPY --from=BUILDER /snitch-frontend/dist/ /usr/share/nginx/html/

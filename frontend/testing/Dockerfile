# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bullseye AS builder

# If you’re using stable, use this instead
# FROM rust:1.74-bullseye as builder

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y
# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Setup tailwindcss 
# Install tailwindcss
RUN mkdir -p /config
WORKDIR /config
# Installs nvm (node version manager)
RUN export XDG_CONFIG_HOME="/config" \
    && PROFILE=/dev/null bash -c 'curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash'
# Loads nvm & install nodejs & install tailwindcss
RUN . /config/nvm/nvm.sh && nvm install 22 && npm install -D tailwindcss

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY cellreadr-frontend .

# Build the app
RUN . /config/nvm/nvm.sh \
    && cargo leptos build --release -vv 


FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get install -y zsh nodejs npm \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

RUN npm install -D tailwindcss

# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/app /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site

# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/


# Transfering over files for tailwindcss
COPY --from=builder /app/input.css /app/
COPY --from=builder /app/style /app/style

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:80"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 80

# Run the server
#RUN zsh -c 'npx tailwindcss -i /app/input.css -o /app/style/output.css --watch &!'
CMD ["/app/app"] 

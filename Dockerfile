FROM node

EXPOSE 8080

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add wasm32-unknown-unknown

RUN cargo install wasm-pack trunk

WORKDIR /usr/src/crabington

COPY . .

RUN npm i

RUN npm run build

RUN npm run tw

CMD npx cross-env WASM_PACK_PROFILE=dev parcel static/index.html -p 8080

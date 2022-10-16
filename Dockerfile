FROM node as builder

EXPOSE 3000

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl

# Update new packages
RUN apt-get update

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add wasm32-unknown-unknown

RUN cargo install wasm-pack

WORKDIR /usr/src/crabington

COPY . .

RUN npm i

RUN npm run build

RUN ls -lha dist/

CMD npx serve dist/

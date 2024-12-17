FROM rust:latest
WORKDIR /usr/src/rust-server
COPY . .
RUN cargo install --path .

ENV MONGODB_URI="mongodb://45.159.248.107:11052/"

CMD ["rust-server"]

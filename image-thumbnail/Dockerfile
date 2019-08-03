FROM rust:1.36

WORKDIR /usr/src/image-thumbnail
COPY . .

RUN cargo install --path .

CMD ["image-thumbnail"]
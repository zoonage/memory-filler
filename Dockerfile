FROM rust:alpine3.12 as builder

WORKDIR /opt/memory-tester/

ADD . .

RUN cargo build --release

FROM alpine

COPY --from=builder /opt/memory-tester/target/release/memory-filler /memory-filler

CMD [ "/memory-filler" ]

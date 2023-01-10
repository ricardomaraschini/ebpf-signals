FROM docker.io/library/rust:latest AS builder
RUN apt update -y && apt install -y build-essential
RUN rustup install stable
RUN rustup toolchain install nightly --component rust-src
RUN cargo install bpf-linker
WORKDIR /usr/src/ebpf-signals
COPY . .
RUN make release

FROM docker.io/library/debian:bullseye-slim
COPY --from=builder /usr/src/ebpf-signals/target/release/signals /usr/local/bin/signals
CMD /usr/local/bin/signals

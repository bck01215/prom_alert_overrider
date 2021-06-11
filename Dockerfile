FROM rust:1.52
WORKDIR /opt/prometheus_alert_overrider
COPY . .
RUN cargo build --release && cargo install --path .
ENTRYPOINT ["prometheus_alert_overrider"]
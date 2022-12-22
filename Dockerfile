FROM docker.io/library/rust:latest

# COPY target/release/weather-notif-scheduler /
COPY . /

CMD ["cargo", "run", "--release"]

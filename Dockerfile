FROM alpine:latest as builder
WORKDIR /home/app-dir
COPY ./target/x86_64-unknown-linux-musl/release/d_blog ./d_blog
COPY ./data ./data
COPY ./config ./config
COPY ./assets ./assets
COPY ./logs ./logs
ENV LANG en US.UTF-8
ENV LANGUAGE en US:en
ENV LC ALL en US.UTF-8
CMD ["/home/app-dir/d_blog"]
FROM alpine:latest
ENV TZ Asia/Shanghai
RUN apk add tzdata && cp /usr/share/zoneinfo/${TZ} /etc/localtime \
    && echo ${TZ} > /etc/timezone \
    && apk del tzdata
WORKDIR /home/app-dir
COPY ./target/x86_64-unknown-linux-musl/release/d_blog ./d_blog
COPY ./data ./data
COPY ./config ./config
COPY ./assets ./assets
COPY ./logs ./logs
ENV LANG en US.UTF-8
ENV LANGUAGE en US:en
ENV LC ALL en US.UTF-8
EXPOSE 5800
ENTRYPOINT ["/home/app-dir/d_blog"]
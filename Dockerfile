# 构建阶段
FROM rust:latest as build

# 在容器内创建一个新目录用于构建项目
WORKDIR /usr/src/rust_web

# 复制 Cargo.toml 和 Cargo.lock
COPY Cargo.toml Cargo.lock Rocket.toml ./

# 复制整个项目到容器中
COPY src ./src

# 构建最终的可执行文件
RUN cargo build --release

# 最终镜像
FROM alpine:latest

# 安装系统运行时所需的工具和库
RUN apk --no-cache add ca-certificates && update-ca-certificates

# 从构建阶段复制可执行文件到最终镜像中
COPY --from=build /usr/src/rust_web/target/release/rust_web /usr/local/bin/rust_web

# 设置容器启动时的默认命令
CMD ["rust_web"]
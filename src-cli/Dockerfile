# 多阶段构建 Dockerfile for jjj-cli

# 构建阶段
FROM rust:1.75-slim as builder

# 安装必要的构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 复制 Cargo 文件以利用 Docker 缓存
COPY jjj-core/Cargo.toml jjj-core/Cargo.lock ./jjj-core/
COPY src-cli/Cargo.toml src-cli/Cargo.lock ./src-cli/

# 创建虚拟项目以缓存依赖
RUN mkdir -p jjj-core/src src-cli/src && \
    echo "fn main() {}" > src-cli/src/main.rs && \
    echo "pub fn dummy() {}" > jjj-core/src/lib.rs && \
    cargo build --release --manifest-path src-cli/Cargo.toml && \
    rm -rf jjj-core/src src-cli/src

# 复制源代码
COPY jjj-core/ ./jjj-core/
COPY src-cli/ ./src-cli/

# 构建应用
RUN cargo build --release --manifest-path src-cli/Cargo.toml

# 运行阶段
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
RUN useradd -m -u 1000 jjj && \
    mkdir -p /app && \
    chown -R jjj:jjj /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/src-cli/target/release/jjj-cli /usr/local/bin/jjj-cli

# 设置权限
RUN chmod +x /usr/local/bin/jjj-cli

# 切换到非 root 用户
USER jjj

# 设置工作目录
WORKDIR /app

# 设置入口点
ENTRYPOINT ["jjj-cli"]

# 默认命令
CMD ["--help"]


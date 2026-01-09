# 多阶段构建 Dockerfile for jjj-cli

# 构建阶段
FROM rust:latest as builder

# 安装必要的构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 复制源代码
COPY jjj-core/ ./jjj-core/
COPY src-cli/ ./src-cli/

# 构建应用
RUN cargo build --release --manifest-path src-cli/Cargo.toml

# 运行阶段
FROM debian:bookworm-slim


# 安装运行时依赖 + 配置中国时区(Asia/Shanghai)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    tzdata \
    && echo "Asia/Shanghai" > /etc/timezone \
    && ln -snf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime \
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


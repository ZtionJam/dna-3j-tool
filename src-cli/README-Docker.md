# Docker 构建说明

## 构建 Docker 镜像

从项目根目录执行以下命令：

```bash
docker build -f src-cli/Dockerfile -t jjj-cli:latest .
```

## 运行容器

```bash
# 运行并查看帮助
docker run --rm jjj-cli:latest

# 运行并传递 token 参数
docker run --rm jjj-cli:latest --token YOUR_TOKEN
```

## 使用 Docker Compose（可选）

你也可以创建一个 `docker-compose.yml` 文件来简化使用。


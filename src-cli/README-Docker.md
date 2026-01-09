# Docker 构建说明

## 构建 Docker 镜像

从项目根目录执行以下命令：

```bash
docker build -f src-cli/Dockerfile -t ztionjam/jjj-cli:latest -t ztionjam/jjj-cli:0.0.4 .
```

## 运行容器

```bash
# 运行并查看帮助
docker run --rm ztionjam/jjj-cli:latest

# 运行并传递 token 参数
docker run --rm ztionjam/jjj-cli:latest --refresh-token YOUR_TOKEN

# 后台运行
docker run -d --rm ztionjam/jjj-cli:latest --refresh-token YOUR_TOKEN
```

# DNA-3J-Tool / 皎皎角签到工具

用于《二重螺旋》游戏社区《皎皎角》的自动签到工具，支持社区签到和角色签到功能。
<br/>
本文档由AI生成
## 📋 项目简介

本项目是一个自动化签到工具，帮助玩家自动完成《皎皎角》社区的每日签到任务，包括：
- 社区签到
- 角色签到

## 🎯 当前状态

- ✅ **CLI 控制台版本** - OK
- ✅ **桌面版本** - OK
- 🚧 **安卓版本** - 计划中 Tauri2

## 🚀 快速开始

### 下载预编译版本

在 [Releases](https://github.com/your-username/dna-3j-tool/releases) 页面下载对应平台的版本。

### CLI 使用方法


#### Docker（推荐）
```bash
docker pull ztionjam/jjj-cli:latest

#后台运行 （powershell环境用;替代&&） 
docker run -d --rm --name jjj ztionjam/jjj-cli:latest --token 你的Token && docker logs -f jjj

#docker-compose同理
```
#### Windows

```bash
jjj-cli.exe --token YOUR_TOKEN
```

#### Linux

```bash
./jjj-cli --token YOUR_TOKEN
```

#### 参数说明

- `-t, --token <TOKEN>`: 皎皎角的 token（必需）
  - 目前还没有找刷新 token 接口，直接使用 token
  - token通过ProxyPin等工具抓包获得，请自行获取

#### 运行示例

```bash
# Windows
jjj-cli.exe --token "your_token_here"

# Linux
./jjj-cli --token "your_token_here"
```

## 🔨 自行构建项目

### 前置要求

- **Rust** (最新稳定版) - [安装指南](https://www.rust-lang.org/tools/install)
- **Node.js** (v20+) 和 **pnpm** - 用于构建桌面版前端
- **系统依赖**（仅桌面版需要）：
  - Linux: `libwebkit2gtk-4.1-dev`, `libgtk-3-dev` 等
  - Windows: Visual Studio Build Tools

### 构建 CLI 版本

```bash
# 克隆项目
git clone https://github.com/your-username/dna-3j-tool.git
cd dna-3j-tool

# 构建 CLI
cd src-cli
cargo build --release

```

### 构建桌面版本


#### 构建

```bash
# 需要安装Rust + Node20+

# 构建
pnpm i
pnpm tauri build

#### 开发模式

```bash
# 启动开发服务器
pnpm tauri dev
```

## 📦 项目结构

```
dna-3j-tool/
├── jjj-core/          # 核心库，包含签到逻辑
├── src-cli/           # CLI 控制台版本
│   ├── Dockerfile     # Docker 构建文件
│   └── src/
├── src-tauri/         # 桌面版本（Tauri）
│   └── src/
├── src/               # 前端代码（Vue 3）
└── .github/
    └── workflows/     # GitHub Actions
```


## ⚠️ 注意事项

1. **Token 安全**：请妥善保管你的 token，不要分享给他人
2. **Token 有效期**：目前不支持自动刷新 token，如果 token 过期需要手动更新
3. **使用频率**：请合理使用，避免频繁请求导致账号异常

---

**免责声明**：本工具仅供学习交流使用，使用本工具产生的任何后果由使用者自行承担。

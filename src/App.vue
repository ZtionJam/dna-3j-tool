<script setup>
import { ref, onMounted, onUnmounted, nextTick, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { openUrl  } from "@tauri-apps/plugin-opener";

const TOKEN_STORAGE_KEY = "jjj_token";
const CURRENT_VERSION = "0.1.0"; // 当前版本号，需要与 tauri.conf.json 中的版本保持一致
const GITHUB_REPO = "ZtionJam/dna-3j-tool";
const GITHUB_API_URL = `https://api.github.com/repos/${GITHUB_REPO}/releases/latest`;
const GITHUB_RELEASE_URL = `https://github.com/${GITHUB_REPO}/releases/latest`;

// 从 localStorage 读取 token
const loadTokenFromStorage = () => {
  try {
    const savedToken = localStorage.getItem(TOKEN_STORAGE_KEY);
    return savedToken || "";
  } catch (error) {
    console.error("读取 refreshToken 失败:", error);
    return "";
  }
};

// 保存 refreshToken 到 localStorage
const saveTokenToStorage = (tokenValue) => {
  try {
    if (tokenValue && tokenValue.trim()) {
      localStorage.setItem(TOKEN_STORAGE_KEY, tokenValue);
    } else {
      localStorage.removeItem(TOKEN_STORAGE_KEY);
    }
  } catch (error) {
    console.error("保存 refreshToken 失败:", error);
  }
};

const token = ref(loadTokenFromStorage());
const isRunning = ref(false);
const showUpdateDialog = ref(false);
const latestVersion = ref("");
const logs = ref([
  "请输入 refreshToken 后点击启动按钮"
]);

// 监听 token 变化并保存到 localStorage
watch(token, (newValue) => {
  saveTokenToStorage(newValue);
});

// 添加日志函数
const addLog = (message) => {
  const now = new Date();
  const timeStr = now.toLocaleTimeString("zh-CN", { 
    hour12: false,
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit"
  });
  logs.value.push(`[${timeStr}] ${message}`);
  
  // 自动滚动到底部
  nextTick(() => {
    const consoleContent = document.querySelector(".console-content");
    if (consoleContent) {
      consoleContent.scrollTop = consoleContent.scrollHeight;
    }
  });
};

// 启动签到或退出程序
const startSignin = async () => {
  // 如果正在运行，则退出程序
  if (isRunning.value) {
    try {
      await invoke("exit_app");
    } catch (error) {
      addLog(`退出失败: ${error}`);
    }
    return;
  }

  if (!token.value.trim()) {
    addLog("错误: refreshToken 不能为空");
    return;
  }

  try {
    isRunning.value = true;
    addLog("正在启动签到服务...");
    
    await invoke("start_signin", { token: token.value });
    addLog("签到服务启动成功");
  } catch (error) {
    isRunning.value = false;
    addLog(`启动失败: ${error}`);
  }
};

// 版本号比较函数
const compareVersions = (version1, version2) => {
  // 移除 'v' 前缀并分割版本号
  const v1 = version1.replace(/^v/, "").split(".").map(Number);
  const v2 = version2.replace(/^v/, "").split(".").map(Number);
  
  // 补齐长度
  const maxLength = Math.max(v1.length, v2.length);
  while (v1.length < maxLength) v1.push(0);
  while (v2.length < maxLength) v2.push(0);
  
  // 比较版本号
  for (let i = 0; i < maxLength; i++) {
    if (v1[i] > v2[i]) return 1;
    if (v1[i] < v2[i]) return -1;
  }
  return 0;
};

// 检查更新
const checkForUpdates = async () => {
  try {
    const response = await fetch(GITHUB_API_URL);
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    
    const data = await response.json();
    const latestTag = data.tag_name; // 格式: v1.0.0
    
    // 比较版本号
    if (compareVersions(latestTag, CURRENT_VERSION) > 0) {
      latestVersion.value = latestTag;
      showUpdateDialog.value = true;
    }
  } catch (error) {
    // 静默失败，不打扰用户
    console.error("检查更新失败:", error);
  }
};

// 跳转到 GitHub 发布页面
const goToGitHub = async () => {
  try {
    await openUrl (GITHUB_RELEASE_URL);
    showUpdateDialog.value = false;
  } catch (error) {
    console.error("打开浏览器失败:", error);
  }
};

// 关闭更新对话框
const closeUpdateDialog = () => {
  showUpdateDialog.value = false;
};

// 监听日志事件
let unlistenLog = null;

onMounted(async () => {
  // 监听来自后端的日志事件
  unlistenLog = await listen("log", (event) => {
    addLog(event.payload);
  });
  
  // 启动时检查更新
  checkForUpdates();
});

onUnmounted(() => {
  // 清理事件监听
  if (unlistenLog) {
    unlistenLog();
  }
});
</script>

<template>
  <main class="app-container">
    <!-- Token 输入区域 -->
    <div class="token-section">
      <label class="token-label">RefreshToken</label>
      <input 
        v-model="token" 
        type="password"
        class="token-input" 
        placeholder="请输入您的 refreshToken"
      />
    </div>

    <!-- 控制台区域 -->
    <div class="console-section">
      <div class="console-header">
        <span class="console-title">控制台</span>
      </div>
      <div class="console-content" ref="consoleContent">
        <div 
          v-for="(log, index) in logs" 
          :key="index" 
          class="log-line"
        >
          <span class="log-text">{{ log }}</span>
        </div>
      </div>
    </div>

    <!-- 启动按钮 -->
    <div class="action-section">
      <button 
        class="start-button" 
        :class="{ 'running': isRunning }"
        @click="startSignin"
      >
        {{ isRunning ? '运行中，点击退出' : '启动' }}
      </button>
    </div>

    <!-- 更新提示对话框 -->
    <div v-if="showUpdateDialog" class="update-dialog-overlay" @click="closeUpdateDialog">
      <div class="update-dialog" @click.stop>
        <div class="update-dialog-header">
          <h3 class="update-dialog-title">发现新版本</h3>
        </div>
        <div class="update-dialog-content">
          <p class="update-dialog-text">
            当前版本: <strong>v{{ CURRENT_VERSION }}</strong>
          </p>
          <p class="update-dialog-text">
            最新版本: <strong>{{ latestVersion }}</strong>
          </p>
          <p class="update-dialog-hint">
            点击下方按钮前往 GitHub 下载最新版本
          </p>
        </div>
        <div class="update-dialog-actions">
          <button class="update-button-cancel" @click="closeUpdateDialog">
            稍后提醒
          </button>
          <button class="update-button-confirm" @click="goToGitHub">
            前往下载
          </button>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
.app-container {
  width: 100%;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: linear-gradient(135deg, #f5f7fa 0%, #e8ecf1 100%);
  overflow: hidden;
}

/* Token 输入区域 */
.token-section {
  padding: 20px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  border-bottom: 1px solid rgba(0, 0, 0, 0.05);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.03);
}

.token-label {
  display: block;
  font-size: 13px;
  font-weight: 600;
  color: #4a5568;
  margin-bottom: 8px;
  letter-spacing: 0.3px;
}

.token-input {
  width: 100%;
  padding: 12px 16px;
  font-size: 14px;
  color: #2d3748;
  background: #ffffff;
  border: 1.5px solid #e2e8f0;
  border-radius: 8px;
  outline: none;
  transition: all 0.2s ease;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.token-input::placeholder {
  color: #a0aec0;
}

.token-input:focus {
  border-color: #4299e1;
  box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.1);
}

/* 控制台区域 */
.console-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  margin: 12px;
  margin-bottom: 8px;
  background: #1a202c;
  border-radius: 10px;
  overflow: hidden;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15), inset 0 1px 0 rgba(255, 255, 255, 0.05);
}

.console-header {
  padding: 10px 14px;
  background: rgba(0, 0, 0, 0.3);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.console-title {
  font-size: 12px;
  font-weight: 600;
  color: #cbd5e0;
  letter-spacing: 0.5px;
  text-transform: uppercase;
}

.console-content {
  flex: 1;
  padding: 12px;
  overflow-y: auto;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 12px;
  line-height: 1.6;
}

.console-content::-webkit-scrollbar {
  width: 6px;
}

.console-content::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.2);
}

.console-content::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.2);
  border-radius: 3px;
}

.console-content::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.3);
}

.log-line {
  margin-bottom: 4px;
  color: #e2e8f0;
  word-break: break-word;
}

.log-text {
  color: #cbd5e0;
}

/* 启动按钮区域 */
.action-section {
  padding: 16px 20px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  border-top: 1px solid rgba(0, 0, 0, 0.05);
  box-shadow: 0 -2px 8px rgba(0, 0, 0, 0.03);
}

.start-button {
  width: 100%;
  padding: 14px 24px;
  font-size: 15px;
  font-weight: 600;
  color: #ffffff;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
  letter-spacing: 0.5px;
}

.start-button:hover {
  transform: translateY(-1px);
  box-shadow: 0 6px 16px rgba(102, 126, 234, 0.5);
}

.start-button:active {
  transform: translateY(0);
  box-shadow: 0 2px 8px rgba(102, 126, 234, 0.4);
}

.start-button:focus {
  outline: none;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4), 0 0 0 3px rgba(102, 126, 234, 0.2);
}

.start-button.running {
  background: linear-gradient(135deg, #e53e3e 0%, #c53030 100%);
  box-shadow: 0 4px 12px rgba(229, 62, 62, 0.4);
}

.start-button.running:hover {
  background: linear-gradient(135deg, #c53030 0%, #9b2c2c 100%);
  box-shadow: 0 6px 16px rgba(229, 62, 62, 0.5);
}

.start-button.running:active {
  background: linear-gradient(135deg, #c53030 0%, #9b2c2c 100%);
  box-shadow: 0 2px 8px rgba(229, 62, 62, 0.4);
}

/* 更新对话框样式 */
.update-dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

.update-dialog {
  background: #ffffff;
  border-radius: 12px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
  width: 90%;
  max-width: 400px;
  overflow: hidden;
  animation: dialogSlideIn 0.3s ease-out;
}

@keyframes dialogSlideIn {
  from {
    opacity: 0;
    transform: translateY(-20px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.update-dialog-header {
  padding: 20px 24px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #ffffff;
}

.update-dialog-title {
  font-size: 18px;
  font-weight: 600;
  margin: 0;
  letter-spacing: 0.3px;
}

.update-dialog-content {
  padding: 24px;
}

.update-dialog-text {
  font-size: 14px;
  color: #4a5568;
  margin-bottom: 12px;
  line-height: 1.6;
}

.update-dialog-text strong {
  color: #2d3748;
  font-weight: 600;
}

.update-dialog-hint {
  font-size: 13px;
  color: #718096;
  margin-top: 16px;
  margin-bottom: 0;
}

.update-dialog-actions {
  padding: 16px 24px;
  background: #f7fafc;
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}

.update-button-cancel,
.update-button-confirm {
  padding: 10px 20px;
  font-size: 14px;
  font-weight: 600;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.update-button-cancel {
  background: #e2e8f0;
  color: #4a5568;
}

.update-button-cancel:hover {
  background: #cbd5e0;
}

.update-button-confirm {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #ffffff;
  box-shadow: 0 2px 8px rgba(102, 126, 234, 0.3);
}

.update-button-confirm:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.update-button-confirm:active {
  transform: translateY(0);
}
</style>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  overflow: hidden;
}

#app {
  width: 100%;
  height: 100vh;
}
</style>

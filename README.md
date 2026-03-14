# Night && Day Toggle ☀️/🌙

![tauri](https://img.shields.io/badge/Tauri-2.10.3-FFC131?style=flat&logo=tauri&logoColor=white) ![rust](https://img.shields.io/badge/Rust-1.77.2-orange?style=flat&logo=rust&logoColor=white) ![LICENSE](https://img.shields.io/badge/License-MIT-blue?style=flat) ![windows](https://img.shields.io/badge/Platform-Windows-green?style=flat) [![B站灵感](https://img.shields.io/badge/Bilibili-梗视频灵感-FB7299?style=flat&logo=bilibili&logoColor=white)](https://www.bilibili.com/video/BV1Pm4y127ui)

一个桌面悬浮开关，可一键切换系统日间/黑夜模式，灵感来源于[B站](https://www.bilibili.com/video/BV1Pm4y127ui)，[原前端代码](https://codepen.io/jh3y/pen/LYgjpYZ)在此。

因为经常用电脑💻，为了爱护眼睛👁👁，要根据光线和场景选择开浅色还是深色模式，所以做了一个小工具。

## 🚀 快速开始

### 前提条件

- https://nodejs.org/ (v18 或更高)
- https://www.rust-lang.org/tools/install (v1.70 或更高)
- https://tauri.app/v1/guides/getting-started/prerequisites

### 安装

```bash
# 克隆项目
git clone https://github.com/yourusername/sun-moon-toggle.git
cd sun-moon-toggle

# 安装依赖
npm install

# 运行开发模式
npm run tauri dev
```

### 构建

```bash
# 构建应用
npm run tauri build
```

## 🏗️ 项目结构

```
sun-moon-toggle/
├── node_modules/
├── src/
│   ├── index.html
│   ├── style.css
│   └── script.js
└── src-tauri/
    ├── capabilities/
    ├── gen/
    │   └── schemas/
    ├── icons/
    ├── src/
    │   ├── lib.rs
    │   └── main.rs
    ├── target/
    ├── .gitignore
    ├── build.rs
    ├── Cargo.lock
    ├── Cargo.toml
    └── tauri.conf.json
```

## 🎮 使用方法

1. **启动应用**：运行应用后，会在桌面上看到一个悬浮开关
2. **切换主题**：左键点击悬浮球切换日间/黑夜模式
3. **拖动窗口**：
   - 右键点击任意位置拖动
   - 左键在非按钮区域拖动

（目前只是一个demo，问题还很多）

## 🛠️ 技术栈

| 组件 | 技术 | 说明 |
|------|------|------|
| **前端** | HTML5 + CSS3 + JavaScript | 基于原 CodePen 动画改造 |
| **后端** | Rust | 系统调用和逻辑处理 |
| **框架** | Tauri 2.x | 桌面应用框架 |
| **构建工具** | Cargo + npm | Rust 和 Node.js 工具链 |

## 📁 文件说明

### 前端 (`src/`)
- `index.html` - 应用主页面，包含动画 SVG
- `style.css` - 样式文件，实现透明背景和动画效果
- `script.js` - 前端逻辑，处理用户交互和 Tauri API 调用

### 后端 (`src-tauri/src/`)
- `main.rs` - Rust 后端，包含主题切换逻辑和窗口控制
- `Cargo.toml` - Rust 依赖配置

## 📄 许可证

本项目采用[MIT LICENSE](LICENSE)。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 🙏 致谢

- 感谢 https://codepen.io/jh3y 的原始动画设计
- 感谢 https://tauri.app/ 提供的优秀框架
- 感谢所有开源贡献者

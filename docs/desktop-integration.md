# Kitty Desktop Integration Guide

> 目标：`kitty_desktop` 作为**独立仓库**，引用 `kitty` 作为浏览器内核能力。

## 1. 仓库边界

- `kitty` 仓库：内核能力（core/ai/render/script/3d/compat/webapp）与 `kitty-shell` 示例。
- `kitty_desktop` 仓库：桌面窗口、GUI/WebView、生命周期、打包发布。

这意味着 `kitty_desktop` 不应作为 `kitty` workspace 成员直接提交在本仓库中。

## 2. 依赖方式（推荐）

在 `kitty_desktop/Cargo.toml` 中通过 git 依赖（或本地 path 依赖）引入内核 crate：

```toml
[dependencies]
kitty-core = { git = "https://github.com/<org>/kitty.git" }
kitty-ai = { git = "https://github.com/<org>/kitty.git" }
kitty-render = { git = "https://github.com/<org>/kitty.git" }
kitty-script = { git = "https://github.com/<org>/kitty.git" }
kitty-3d = { git = "https://github.com/<org>/kitty.git" }
kitty-compat = { git = "https://github.com/<org>/kitty.git" }
kitty-webapp = { git = "https://github.com/<org>/kitty.git" }
```

开发联调阶段，也可以用 path：

```toml
[dependencies]
kitty-core = { path = "../kitty/crates/kitty-core" }
```

## 3. 最小实现计划（建议）

### Phase 1：内核接入
- 建立 `kitty_desktop` 独立 workspace。
- 接入 `kitty-core` 与至少一个子系统（如 `kitty-ai`）。
- 跑通桌面启动 + 内核自检输出。

### Phase 2：窗口与渲染容器
- 选择 GUI/WebView 技术栈（Tauri/wry/winit/egui）。
- 完成窗口管理、事件循环、导航基础能力。
- 建立渲染容器与内核通信通道。

### Phase 3：能力桌面化
- 将 AI、脚本、3D、兼容性结果接入桌面 UI。
- 增加标签页、地址栏、历史与设置。

### Phase 4：工程化发布
- 跨平台打包（Windows/macOS/Linux）。
- 崩溃恢复、自动更新、日志与诊断。
- CI/CD 与版本发布策略。

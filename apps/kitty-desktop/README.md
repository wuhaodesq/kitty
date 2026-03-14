# kitty_desktop

`kitty_desktop` 是 `kitty` 项目的桌面封装层：

- **内核复用**：直接复用 `kitty-shell` 的 `run_demo`（其背后连接 `kitty-core`、`kitty-ai`、`kitty-render`、`kitty-script`、`kitty-3d`、`kitty-compat`、`kitty-webapp` 等能力）。
- **桌面职责**：管理桌面窗口配置、桌面启动流程与桌面运行摘要输出。
- **定位**：作为后续接入真实 GUI/WebView 框架（如 Tauri/winit/wry/egui）的过渡层。

## 当前实现

- `WindowConfig`：桌面窗口参数（标题、尺寸、是否可拉伸）。
- `DesktopApp`：桌面入口对象，负责调用 kitty 内核并生成 `DesktopSummary`。
- `format_summary`：将桌面层 + 内核层关键信息统一输出。

运行方式：

```bash
cargo run -p kitty-desktop
```

## 实现计划（Roadmap）

### Phase 1：基础封装（已完成）
- [x] 建立 `apps/kitty-desktop` crate。
- [x] 复用 `kitty` 内核能力作为桌面启动内核。
- [x] 抽象窗口配置与桌面启动摘要。
- [x] 添加基础单元测试。

### Phase 2：真实桌面运行时（计划中）
- [ ] 选型并接入 GUI/WebView 容器（Tauri/wry/winit）。
- [ ] 将 `WindowConfig` 映射到真实窗口管理。
- [ ] 提供事件循环、生命周期管理与崩溃恢复策略。

### Phase 3：内核能力桌面化（计划中）
- [ ] 在桌面端暴露内核状态面板（AI、渲染、脚本、3D、兼容性）。
- [ ] 增加地址栏、标签页、导航历史等桌面浏览器基础交互。
- [ ] 提供桌面配置中心（性能、隐私、安全策略）。

### Phase 4：工程化与发布（计划中）
- [ ] 增加跨平台打包（Windows/macOS/Linux）。
- [ ] 引入自动更新、崩溃报告、遥测开关。
- [ ] 完善 CI（测试、lint、打包检查）与版本发布流程。

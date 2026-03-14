# Kitty Browser

> 一个由 **Rust** 全栈驱动、原生融合 **AI** 与 **3D** 能力的下一代浏览器。

Kitty Browser 致力于构建一个兼顾 **性能、安全、可扩展性与兼容性** 的现代浏览器平台：

- 浏览器核心完全采用 Rust 编写；
- 原生支持 AI 推理与智能交互；
- 原生强化 3D 图形与沉浸式 Web 体验；
- 支持开发者使用 Rust 构建网站应用；
- 兼容当前主流浏览器可访问的网站与主流 Web 标准。

---

## ✨ 项目目标

### 1) Rust Native, Security First
- 通过 Rust 的内存安全与并发模型提升稳定性。
- 以高性能渲染与可维护架构为核心设计原则。

### 2) AI-Native Browser
- 提供统一 AI 运行时接口（本地/远程模型接入）。
- 支持页面语义理解、内容总结、智能检索、自动化代理等能力。

### 3) 3D-First Web Experience
- 强化 3D 图形渲染与交互能力。
- 面向可视化、数字孪生、游戏化界面与沉浸式应用场景。

### 4) Rust for Web Apps
- 提供 Rust 应用开发友好能力，支持使用 Rust 构建网站与 Web 应用。
- 降低多语言栈成本，提升端到端工程一致性。

### 5) Mainstream Web Compatibility
- 对齐主流浏览器生态与 Web 标准。
- 确保现有网站可访问，降低迁移成本。

---

## 📦 仓库结构（当前）

```text
.
├── apps/
│   └── kitty-shell/      # 启动入口（CLI 原型）
├── crates/
│   ├── kitty-core/       # 浏览器核心抽象（配置/能力）
│   ├── kitty-ai/         # AI 运行时抽象（provider 原型）
│   └── kitty-render/     # 页面/DOM/布局最小渲染抽象
└── Cargo.toml            # Rust workspace
```

---

## 🚀 快速开始

### 环境要求
- Rust stable（建议通过 `rustup` 安装）

### 构建与运行

```bash
cargo run -p kitty-shell
```

### 运行测试

```bash
cargo test
```

---

## 🧭 典型应用场景

- AI 助手驱动的浏览与办公自动化
- 3D 可视化平台与交互式数据展示
- 面向 Rust 团队的一体化 Web 技术栈
- 对安全性、性能和可控性要求更高的浏览器环境

---

## 🗺️ 路线图（草案）

- [x] 初始化 Rust workspace 与基础模块骨架
- [x] 页面渲染基础能力（DOM / Layout 最小抽象）
- [ ] 架构设计与核心模块细分
- [ ] 脚本运行基础能力
- [ ] AI 运行时与模型接入抽象（扩展）
- [ ] 3D 渲染管线增强
- [ ] 开发者工具链与 Rust Web 应用支持
- [ ] 兼容性测试基线与稳定性优化

---

## 📌 当前状态

项目目前处于早期规划/建设阶段，已完成 workspace 初始化与渲染基础模块原型。

---

## 🤝 贡献

欢迎通过 Issue / PR 提出建议与改进方向。

---

## 📄 License

暂未发布，后续补充。

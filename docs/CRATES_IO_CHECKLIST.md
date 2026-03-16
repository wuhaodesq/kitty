# crates.io 发布清单（kitty）

> 目标：让其他项目可以通过 `cargo add kitty` 直接集成。

## 0. 前提

- 已有 crates.io 账号并在本机完成登录。
- 仓库代码与测试状态正常。

## 1. 本地质量检查

```bash
cargo test
cargo package -p kitty --allow-dirty
```

检查 `target/package/kitty-*.crate` 中是否包含：
- `LICENSE`
- `crates/kitty/README.md`
- 必要源代码文件

## 2. 版本策略

- 首次发布使用 `0.1.0`。
- 后续遵循 semver：
  - 兼容新增：`0.1.x -> 0.2.0`（在 0.y 阶段可按团队约定）
  - 破坏性变更：提升 minor（或 1.0 后提升 major）

## 3. 发布命令

```bash
cargo login <your-crates-io-token>
cargo publish -p kitty
```

## 4. 发布后验证

在任意项目中执行：

```bash
cargo add kitty
cargo tree | rg kitty
```

最小可运行示例：

```rust
use kitty::KittySdk;

fn main() {
    let mut sdk = KittySdk::new();
    sdk.register_echo_model();
    println!("{}", sdk.infer_echo("hello").unwrap());
}
```

## 5. 建议后续动作

- 为 `kitty` 增加 changelog。
- 增加 API 稳定性说明与 deprecation 策略。
- 在 README 中补充 crates.io 与 docs.rs 徽章（发布后）。

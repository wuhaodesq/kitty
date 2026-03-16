# kitty

`kitty` 是 Kitty Browser 原型能力的统一封装层，供其他 Rust 项目直接依赖。

## 能力

- AI：`register_echo_model` / `infer_echo`
- 脚本：`run_script`
- 渲染：`build_layout_box_count`
- 3D：`render_triangle_frame`
- 兼容性：`check_site`
- WebApp：`create_webapp_home_component_name`

## 示例

```rust
use kitty::KittySdk;

fn main() {
    let mut sdk = KittySdk::new();
    sdk.register_echo_model();

    let out = sdk.infer_echo("hello").unwrap();
    println!("{out}");
}
```

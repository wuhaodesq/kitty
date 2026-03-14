use kitty_ai::AiRuntime;
use kitty_core::{Browser, BrowserConfig};
use kitty_render::{DomNode, LayoutTree, Page};
use kitty_script::{ScriptRuntime, ScriptValue};

fn main() {
    let browser = Browser::new(BrowserConfig::default());
    let runtime = AiRuntime::new("local");

    let page = Page::new(
        "Kitty Home",
        DomNode::element("html")
            .with_child(DomNode::element("body").with_child(DomNode::text("h1", "Hello Kitty"))),
    );
    let layout = LayoutTree::from_dom(&page.root);

    let mut script = ScriptRuntime::new();
    let script_out = script
        .execute("set mode dev\nadd visits 1\nget mode")
        .expect("script should execute successfully");

    println!("Starting {}", browser.config().name);
    println!("AI provider: {}", runtime.provider());
    println!("Capabilities: {:?}", browser.capabilities());
    println!("Render page: {}", page.title);
    println!("Layout boxes: {}", layout.boxes.len());
    match script_out {
        Some(ScriptValue::Str(value)) => println!("Script mode: {}", value),
        Some(ScriptValue::Number(value)) => println!("Script output number: {}", value),
        None => println!("Script output: <none>"),
    }
}

use kitty_ai::AiRuntime;
use kitty_core::{Browser, BrowserConfig};
use kitty_render::{DomNode, LayoutTree, Page};

fn main() {
    let browser = Browser::new(BrowserConfig::default());
    let runtime = AiRuntime::new("local");

    let page = Page::new(
        "Kitty Home",
        DomNode::element("html")
            .with_child(DomNode::element("body").with_child(DomNode::text("h1", "Hello Kitty"))),
    );
    let layout = LayoutTree::from_dom(&page.root);

    println!("Starting {}", browser.config().name);
    println!("AI provider: {}", runtime.provider());
    println!("Capabilities: {:?}", browser.capabilities());
    println!("Render page: {}", page.title);
    println!("Layout boxes: {}", layout.boxes.len());
}

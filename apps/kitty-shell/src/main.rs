use kitty_ai::AiRuntime;
use kitty_core::{Browser, BrowserConfig};

fn main() {
    let browser = Browser::new(BrowserConfig::default());
    let runtime = AiRuntime::new("local");

    println!("Starting {}", browser.config().name);
    println!("AI provider: {}", runtime.provider());
    println!("Capabilities: {:?}", browser.capabilities());
}

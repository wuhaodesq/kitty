use kitty_shell::{print_summary, run_demo, run_demo_with_config, DemoConfig};

fn main() {
    let prompt = std::env::var("KITTY_PROMPT").ok();
    let domain = std::env::var("KITTY_DOMAIN").ok();

    let result = match (prompt, domain) {
        (None, None) => run_demo(),
        (prompt, domain) => {
            let mut config = DemoConfig::default();
            if let Some(prompt) = prompt {
                config.prompt = prompt;
            }
            if let Some(domain) = domain {
                config.domain = domain;
            }
            run_demo_with_config(&config)
        }
    };

    match result {
        Ok(summary) => print_summary(&summary),
        Err(err) => {
            eprintln!("kitty-shell demo failed: {err:?}");
            std::process::exit(1);
        }
    }
}

use std::collections::HashMap;

use kitty_shell::{apply_env_overrides, print_summary, run_demo, run_demo_with_config, DemoConfig};

fn main() {
    let env: HashMap<String, String> = std::env::vars().collect();

    let result = if env.contains_key("KITTY_PROMPT")
        || env.contains_key("KITTY_DOMAIN")
        || env.contains_key("KITTY_SCRIPT")
        || env.contains_key("KITTY_DISABLE_ECHO_MODEL")
    {
        let mut config = DemoConfig::default();
        apply_env_overrides(&mut config, &env);
        run_demo_with_config(&config)
    } else {
        run_demo()
    };

    match result {
        Ok(summary) => print_summary(&summary),
        Err(err) => {
            eprintln!("kitty-shell demo failed: {err:?}");
            std::process::exit(1);
        }
    }
}

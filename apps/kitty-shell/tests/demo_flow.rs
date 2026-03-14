use kitty_shell::{run_demo, run_demo_with_config, DemoConfig};

#[test]
fn run_demo_integration_smoke() {
    let summary = run_demo();
    assert!(summary.compat_score >= 4);
    assert_eq!(summary.webapp_home_component, "home");
}

#[test]
fn run_demo_with_config_integration_smoke() {
    let config = DemoConfig {
        prompt: "integration".to_string(),
        domain: "integration.test".to_string(),
        requires_webgl2: false,
        requires_webassembly: false,
        requires_service_worker: false,
    };

    let summary = run_demo_with_config(&config);
    assert_eq!(summary.ai_output, "echo:integration");
    assert_eq!(summary.compat_domain, "integration.test");
    assert_eq!(summary.compat_score, 6);
}

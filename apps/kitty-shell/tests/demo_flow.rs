use kitty_shell::run_demo;

#[test]
fn run_demo_integration_smoke() {
    let summary = run_demo();
    assert!(summary.compat_score >= 4);
    assert_eq!(summary.webapp_home_component, "home");
}

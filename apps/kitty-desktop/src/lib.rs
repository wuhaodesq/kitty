use kitty_shell::{run_demo, DemoSummary};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Kitty Desktop".to_string(),
            width: 1280,
            height: 800,
            resizable: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopSummary {
    pub window: WindowConfig,
    pub kernel: DemoSummary,
    pub shell: &'static str,
}

pub struct DesktopApp {
    window: WindowConfig,
}

impl DesktopApp {
    pub fn new(window: WindowConfig) -> Self {
        Self { window }
    }

    pub fn bootstrap(&self) -> DesktopSummary {
        DesktopSummary {
            window: self.window.clone(),
            kernel: run_demo(),
            shell: "kitty-desktop-shell-v0",
        }
    }
}

pub fn format_summary(summary: &DesktopSummary) -> String {
    format!(
        "Launching {title} ({width}x{height}, resizable={resizable})\nKernel browser: {browser}\nAI provider: {provider}\nDesktop shell: {shell}",
        title = summary.window.title,
        width = summary.window.width,
        height = summary.window.height,
        resizable = summary.window.resizable,
        browser = summary.kernel.browser_name,
        provider = summary.kernel.ai_provider,
        shell = summary.shell,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desktop_bootstrap_uses_kitty_kernel() {
        let app = DesktopApp::new(WindowConfig::default());
        let summary = app.bootstrap();

        assert_eq!(summary.window.title, "Kitty Desktop");
        assert_eq!(summary.kernel.browser_name, "Kitty Browser");
        assert_eq!(summary.kernel.ai_provider, "local");
        assert_eq!(summary.shell, "kitty-desktop-shell-v0");
    }

    #[test]
    fn desktop_summary_formatter_contains_window_and_kernel_data() {
        let app = DesktopApp::new(WindowConfig::default());
        let summary = app.bootstrap();
        let text = format_summary(&summary);

        assert!(text.contains("Kitty Desktop"));
        assert!(text.contains("Kernel browser: Kitty Browser"));
        assert!(text.contains("Desktop shell: kitty-desktop-shell-v0"));
    }
}

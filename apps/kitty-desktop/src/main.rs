use kitty_desktop::{format_summary, DesktopApp, WindowConfig};

fn main() {
    let app = DesktopApp::new(WindowConfig::default());
    let summary = app.bootstrap();
    println!("{}", format_summary(&summary));
}

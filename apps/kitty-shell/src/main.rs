use kitty_shell::{print_summary, run_demo};

fn main() {
    let summary = run_demo();
    print_summary(&summary);
}

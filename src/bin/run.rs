use dsa::modules::{discovery, fzf, runner};

fn main() {
    let problems = discovery::discover_problems();

    if problems.is_empty() {
        println!("⚠️  No LeetCode solutions found!");
        println!("💡 Create directories like: 01-array/, 02-linked-list/, etc.");
        return;
    }

    if let Some(selected) = fzf::run_fzf(&problems) {
        runner::run_problem(&selected);
    }
}

use crate::modules::discovery::Problem;
use std::io::Write;
use std::process::{Command, Stdio};

/// Launches FZF with all problems, returns selected problem
pub fn run_fzf(problems: &[Problem]) -> Option<Problem> {
    println!("🎯 Starting FZF fuzzy finder...\n");

    let fzf_args = get_rust_theme();

    let mut child = Command::new("fzf")
        .args(&fzf_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok()?;

    // Write problem list to FZF stdin
    if let Some(mut stdin) = child.stdin.take() {
        for problem in problems {
            writeln!(stdin, "{}", problem.display_name()).ok()?;
        }
    }

    // Get FZF output
    let output = child.wait_with_output().ok()?;

    if !output.status.success() {
        println!("👋 Exiting...");
        return None;
    }

    let selection = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if selection.is_empty() {
        println!("👋 No selection made.");
        return None;
    }

    // Parse selection: "Two Sum                        📁 01-array"
    let selected_name = selection.split("📁").next()?.trim();

    // Find matching problem by display name
    problems.iter().find(|p| p.name == selected_name).cloned()
}

/// Returns FZF theme styled for Rust
fn get_rust_theme() -> Vec<String> {
    vec![
        "--height".to_string(),
        "40%".to_string(),
        "--reverse".to_string(),
        "--prompt".to_string(),
        "🦀 Search Rust Problems > ".to_string(),
        "--pointer".to_string(),
        "→".to_string(),
        "--marker".to_string(),
        "✓".to_string(),
        "--header".to_string(),
        "Use ↑↓ to navigate • Enter to run • Ctrl+C to exit".to_string(),
        "--color".to_string(),
        "fg:#CE412B".to_string(), // Rust orange
        "--color".to_string(),
        "fg+:#CE412B".to_string(),
        "--color".to_string(),
        "bg:#1E1E1E".to_string(), // Dark background
        "--color".to_string(),
        "bg+:#0D1117".to_string(),
        "--color".to_string(),
        "hl:#CE412B".to_string(),
        "--color".to_string(),
        "hl+:#CE412B".to_string(),
        "--color".to_string(),
        "gutter:#1E1E1E".to_string(),
        "--color".to_string(),
        "border:#CE412B".to_string(),
        "--color".to_string(),
        "header:#CE412B".to_string(),
        "--color".to_string(),
        "info:#E1E4E8".to_string(),
        "--color".to_string(),
        "pointer:#CE412B".to_string(),
        "--color".to_string(),
        "marker:#CE412B".to_string(),
        "--color".to_string(),
        "spinner:#CE412B".to_string(),
        "--color".to_string(),
        "prompt:#CE412B".to_string(),
        "--border".to_string(),
        "double".to_string(),
    ]
}

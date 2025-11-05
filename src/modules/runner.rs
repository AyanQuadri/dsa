use crate::modules::discovery::Problem;
use std::process::Command;

/// Runs a specific problem using cargo run --bin
pub fn run_problem(problem: &Problem) {
    println!("\n🚀 Running: {} from {}\n", problem.name, problem.category);
    println!("{}", "=".repeat(50));

    let status = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(&problem.bin_name)
        .status();

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("\n✅ Execution completed successfully!");
            } else {
                println!("\n❌ Execution failed!");
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to run cargo: {}", e);
        }
    }
}

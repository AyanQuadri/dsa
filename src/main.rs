use clap::{Parser, Subcommand};
use dsa::modules::{discovery, runner, updater};

#[derive(Parser)]
#[command(name = "leetcode")]
#[command(about = "LeetCode Problem Manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Update Cargo.toml with all discovered problems
    Update,

    /// List all available problems
    List {
        /// Optional: Filter by category (e.g., "array", "linked-list")
        category: Option<String>,
    },

    /// Run a specific problem by name
    Run {
        /// Problem name (e.g., "two_sum")
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Update) => {
            handle_update();
        }
        Some(Commands::List { category }) => {
            handle_list(category);
        }
        Some(Commands::Run { name }) => {
            handle_run(name);
        }
        None => {
            // Default: Update Cargo.toml
            handle_update();
        }
    }
}

fn handle_update() {
    let problems = discovery::discover_problems();

    if problems.is_empty() {
        println!("⚠️  No LeetCode solutions found!");
        println!("💡 Create directories like: 01-array/, 02-linked-list/, etc.");
        return;
    }

    match updater::update_cargo_toml(&problems) {
        Ok(_) => {
            println!("\n🚀 Usage:");
            println!("   cargo run --bin <solution_name>");
            println!("\n📋 Available solutions:");
            for problem in problems {
                println!("   cargo run --bin {}", problem.bin_name);
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}

fn handle_list(category: Option<String>) {
    let problems = match category {
        Some(cat) => discovery::list_by_category(&cat),
        None => discovery::discover_problems(),
    };

    discovery::print_problems(&problems);
}

fn handle_run(name: String) {
    let problems = discovery::discover_problems();

    if let Some(problem) = problems.iter().find(|p| p.bin_name == name) {
        runner::run_problem(problem);
    } else {
        println!("❌ Problem '{}' not found.", name);
        println!("💡 Use 'cargo run list' to see available problems");
    }
}

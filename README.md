# 🦀 DSA 

A modular Rust-based LeetCode problem manager with interactive fuzzy finding.

## Quick Start

```bash
# Update Cargo.toml with discovered problems
cargo run

# Interactive fuzzy finder (FZF)
cargo run --bin run

# List all problems
cargo run list

# Run specific problem
cargo run --bin <problem_name>
```

## Requirements

- Rust (latest stable)
- [FZF](https://github.com/junegunn/fzf) for interactive mode

**Install FZF:**
```bash
# macOS
brew install fzf

# Linux
sudo apt install fzf  # Ubuntu/Debian
sudo pacman -S fzf    # Arch
```

## Commands

| Command | Description |
|---------|-------------|
| `cargo run` | Update Cargo.toml with all problems |
| `cargo run list` | List all available problems |
| `cargo run list <category>` | Filter by category (e.g., `arrays`) |
| `cargo run run <name>` | Run specific problem by name |
| `cargo run --bin run` | 🔥 Interactive FZF mode |
| `cargo run --bin <name>` | Direct problem execution |

## Built With

- [Clap](https://github.com/clap-rs/clap) - CLI argument parsing
- [FZF](https://github.com/junegunn/fzf) - Fuzzy finder

---

_Expanding as I solve more problems_ 

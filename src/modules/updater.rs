use crate::modules::discovery::Problem;
use std::fs;
use std::io::Write;

/// Updates Cargo.toml with all discovered problems
pub fn update_cargo_toml(problems: &[Problem]) -> Result<(), String> {
    if problems.is_empty() {
        return Err("⚠️  No problems found to add to Cargo.toml".to_string());
    }

    println!("📝 Updating Cargo.toml with {} binaries...", problems.len());

    let cargo_toml_path = "Cargo.toml";
    let cargo_content = fs::read_to_string(cargo_toml_path)
        .map_err(|e| format!("❌ Failed to read Cargo.toml: {}", e))?;

    // Remove existing [[bin]] sections
    let lines: Vec<&str> = cargo_content.lines().collect();
    let mut new_lines = Vec::new();
    let mut skip_bin_section = false;

    for line in lines {
        if line.trim().starts_with("[[bin]]") {
            skip_bin_section = true;
            continue;
        }

        if skip_bin_section {
            if line.trim().starts_with("name =") || line.trim().starts_with("path =") {
                continue;
            }
            if line.trim().is_empty() {
                skip_bin_section = false;
                continue;
            }
        }

        new_lines.push(line);
    }

    // Remove trailing empty lines
    while new_lines.last() == Some(&"") {
        new_lines.pop();
    }

    // Build new Cargo.toml content
    let mut new_content = new_lines.join("\n");
    new_content.push_str("\n\n# Auto-generated binary entries\n");

    for problem in problems {
        new_content.push_str(&format!("[[bin]]\n"));
        new_content.push_str(&format!("name = \"{}\"\n", problem.bin_name));
        new_content.push_str(&format!("path = \"{}\"\n", problem.path));
        new_content.push_str("\n");
    }

    // Write back to Cargo.toml
    let mut file = fs::File::create(cargo_toml_path)
        .map_err(|e| format!("❌ Failed to write Cargo.toml: {}", e))?;

    file.write_all(new_content.as_bytes())
        .map_err(|e| format!("❌ Failed to write content: {}", e))?;

    println!("✅ Successfully updated Cargo.toml!");
    Ok(())
}

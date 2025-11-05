use std::fs;
use std::io::Write;

fn main() {
    println!("🔍 Scanning for LeetCode solutions...\n");

    let mut binaries = Vec::new();

    // Read all entries in the root directory
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            let path = entry.path();

            // Look for directories that start with digits (like 01-array, 02-linked-list)
            if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|s| s.to_str()) {
                    // Check if directory name starts with a digit
                    if dir_name
                        .chars()
                        .next()
                        .map_or(false, |c| c.is_ascii_digit())
                    {
                        println!("📁 Found category: {}", dir_name);

                        // Scan for .rs files inside this directory
                        if let Ok(rs_files) = fs::read_dir(&path) {
                            for file_entry in rs_files.flatten() {
                                let file_path = file_entry.path();

                                if file_path.extension().and_then(|s| s.to_str()) == Some("rs") {
                                    if let Some(file_name) =
                                        file_path.file_stem().and_then(|s| s.to_str())
                                    {
                                        let relative_path =
                                            format!("{}/{}.rs", dir_name, file_name);
                                        binaries.push((file_name.to_string(), relative_path));
                                        println!("  ✓ {}", file_name);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if binaries.is_empty() {
        println!("⚠️  No LeetCode solutions found in numbered directories (e.g., 01-array/)");
        println!("💡 Create directories like: 01-array/, 02-linked-list/, etc.");
        return;
    }

    // Sort binaries alphabetically for consistency
    binaries.sort_by(|a, b| a.0.cmp(&b.0));

    println!("\n📝 Updating Cargo.toml...");

    // Read existing Cargo.toml
    let cargo_toml_path = "Cargo.toml";
    let cargo_content = fs::read_to_string(cargo_toml_path).expect("❌ Failed to read Cargo.toml");

    // Split content and remove existing [[bin]] sections
    let lines: Vec<&str> = cargo_content.lines().collect();
    let mut new_lines = Vec::new();
    let mut skip_bin_section = false;

    for line in lines {
        if line.trim().starts_with("[[bin]]") {
            skip_bin_section = true;
            continue;
        }

        if skip_bin_section {
            // Skip name and path lines in bin sections
            if line.trim().starts_with("name =") || line.trim().starts_with("path =") {
                continue;
            }
            // Empty line after bin section
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

    // Add all binary declarations
    for (name, path) in &binaries {
        new_content.push_str(&format!("[[bin]]\n"));
        new_content.push_str(&format!("name = \"{}\"\n", name));
        new_content.push_str(&format!("path = \"{}\"\n", path));
        new_content.push_str("\n");
    }

    // Write back to Cargo.toml
    let mut file = fs::File::create(cargo_toml_path).expect("❌ Failed to write Cargo.toml");

    file.write_all(new_content.as_bytes())
        .expect("❌ Failed to write content");

    println!(
        "\n✅ Successfully updated Cargo.toml with {} binaries!",
        binaries.len()
    );
    println!("\n🚀 Usage:");
    println!("   cargo run --bin <solution_name>");
    println!("\n📋 Available solutions:");
    for (name, _) in binaries {
        println!("   cargo run --bin {}", name);
    }
}

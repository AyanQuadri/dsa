use std::fs;

#[derive(Debug, Clone)]
pub struct Problem {
    pub name: String,
    pub category: String,
    pub bin_name: String,
    pub path: String,
}

impl Problem {
    pub fn display_name(&self) -> String {
        format!("{:<30} 📁 {}", self.name, self.category)
    }
}

/// Discovers all LeetCode problems in numbered directories
pub fn discover_problems() -> Vec<Problem> {
    let mut problems = Vec::new();

    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|s| s.to_str()) {
                    // Only process directories starting with digits
                    if dir_name
                        .chars()
                        .next()
                        .map_or(false, |c| c.is_ascii_digit())
                    {
                        if let Ok(rs_files) = fs::read_dir(&path) {
                            for file_entry in rs_files.flatten() {
                                let file_path = file_entry.path();

                                if file_path.extension().and_then(|s| s.to_str()) == Some("rs") {
                                    if let Some(file_name) =
                                        file_path.file_stem().and_then(|s| s.to_str())
                                    {
                                        let relative_path =
                                            format!("{}/{}.rs", dir_name, file_name);

                                        problems.push(Problem {
                                            name: format_display_name(file_name),
                                            category: dir_name.to_string(),
                                            bin_name: file_name.to_string(),
                                            path: relative_path,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    problems.sort_by(|a, b| a.bin_name.cmp(&b.bin_name));
    problems
}

/// Lists problems by category (e.g., "array", "linked-list")
pub fn list_by_category(category: &str) -> Vec<Problem> {
    discover_problems()
        .into_iter()
        .filter(|p| p.category.to_lowercase().contains(&category.to_lowercase()))
        .collect()
}

/// Converts "two_sum" to "Two Sum"
fn format_display_name(file_name: &str) -> String {
    file_name
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Prints all problems in a formatted list
pub fn print_problems(problems: &[Problem]) {
    if problems.is_empty() {
        println!("⚠️  No problems found!");
        return;
    }

    println!("\n📚 Found {} problems:\n", problems.len());
    for (i, problem) in problems.iter().enumerate() {
        println!("  {}. {}", i + 1, problem.display_name());
    }
    println!();
}

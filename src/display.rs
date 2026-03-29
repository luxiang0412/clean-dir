use colored::Colorize;

use crate::detector::ProjectArtifact;
use crate::scanner::DetectedDir;

pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    match bytes {
        b if b >= GB => format!("{:.1} GB", b as f64 / GB as f64),
        b if b >= MB => format!("{:.1} MB", b as f64 / MB as f64),
        b if b >= KB => format!("{:.1} KB", b as f64 / KB as f64),
        b => format!("{} B", b),
    }
}

fn type_label(artifact: ProjectArtifact) -> String {
    match artifact {
        ProjectArtifact::PythonVenv => format!("[{}]", "Python".green()),
        ProjectArtifact::NodeModules => format!("[{}]", "Node".yellow()),
        ProjectArtifact::JavaTarget => format!("[{}]", "Java".red()),
    }
}

pub fn print_results(results: &[DetectedDir]) {
    println!(
        "\nFound {} cleanable {}:\n",
        results.len().to_string().bold(),
        if results.len() == 1 {
            "directory"
        } else {
            "directories"
        }
    );

    let max_path_len = results
        .iter()
        .map(|d| d.path.display().to_string().len())
        .max()
        .unwrap_or(0);

    for d in results {
        let path_str = d.path.display().to_string();
        println!(
            "  {:10} {:<width$}  {}",
            type_label(d.artifact_type),
            path_str,
            format_size(d.size_bytes).bold(),
            width = max_path_len
        );
    }

    let total: u64 = results.iter().map(|d| d.size_bytes).sum();
    println!(
        "\nTotal: {} across {} directories",
        format_size(total).bold(),
        results.len()
    );
}

pub fn print_clean_summary(deleted: u64, freed: u64, failed: u64) {
    println!(
        "\n{} Deleted {} directories, freed {}",
        "Done!".green().bold(),
        deleted,
        format_size(freed).bold()
    );
    if failed > 0 {
        println!(
            "  {} Failed to delete {} directories",
            "Warning:".yellow(),
            failed
        );
    }
}

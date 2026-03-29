mod cleaner;
mod cli;
mod detector;
mod display;
mod scanner;

use std::io::{self, Write};
use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use indicatif::ProgressBar;

use cli::Cli;
use scanner::ScanConfig;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = cli.path.canonicalize().map_err(|e| {
        anyhow::anyhow!("Cannot access '{}': {}", cli.path.display(), e)
    })?;

    // Scan with spinner
    let spinner = ProgressBar::new_spinner();
    spinner.set_message(format!("Scanning {}...", root.display()));
    spinner.enable_steady_tick(Duration::from_millis(100));

    let config = ScanConfig::from(&cli);
    let results = scanner::scan(&root, &config);
    spinner.finish_and_clear();

    if results.is_empty() {
        println!("No cleanable directories found.");
        return Ok(());
    }

    display::print_results(&results);

    if cli.dry_run {
        println!("\nDry run -- no files were deleted.");
        return Ok(());
    }

    if !cli.yes {
        print!("\nDelete all? [y/N]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
    }

    let result = cleaner::clean(&results);
    display::print_clean_summary(result.deleted, result.freed, result.failed);

    Ok(())
}

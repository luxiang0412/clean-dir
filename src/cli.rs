use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "clean-dir",
    about = "Find and remove project build/dependency directories to reclaim disk space",
    version
)]
pub struct Cli {
    /// Root directory to scan (defaults to current directory)
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Show what would be deleted without actually deleting
    #[arg(short = 'n', long)]
    pub dry_run: bool,

    /// Only scan for specific types: python, node, java (comma-separated)
    #[arg(short = 't', long, value_delimiter = ',')]
    pub types: Option<Vec<String>>,

    /// Maximum depth to scan
    #[arg(short = 'd', long)]
    pub max_depth: Option<usize>,

    /// Number of threads for scanning (0 = auto)
    #[arg(short = 'j', long, default_value = "0")]
    pub threads: usize,

    /// Skip confirmation prompt (auto-yes)
    #[arg(short = 'y', long)]
    pub yes: bool,
}

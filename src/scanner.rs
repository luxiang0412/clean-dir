use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use jwalk::WalkDirGeneric;
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::cli::Cli;
use crate::detector::ProjectArtifact;

#[derive(Debug)]
pub struct DetectedDir {
    pub path: PathBuf,
    pub artifact_type: ProjectArtifact,
    pub size_bytes: u64,
}

pub struct ScanConfig {
    pub threads: usize,
    pub max_depth: Option<usize>,
    pub type_filter: Option<Vec<String>>,
}

impl From<&Cli> for ScanConfig {
    fn from(cli: &Cli) -> Self {
        ScanConfig {
            threads: if cli.threads == 0 {
                num_cpus()
            } else {
                cli.threads
            },
            max_depth: cli.max_depth,
            type_filter: cli.types.clone(),
        }
    }
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

pub fn scan(root: &Path, config: &ScanConfig) -> Vec<DetectedDir> {
    let results: Arc<Mutex<Vec<DetectedDir>>> = Arc::new(Mutex::new(Vec::new()));
    let results_clone = results.clone();
    let type_filter = config.type_filter.clone();

    let parallelism = jwalk::Parallelism::RayonNewPool(config.threads);
    let mut walker = WalkDirGeneric::<((), ())>::new(root)
        .follow_links(false)
        .skip_hidden(false)
        .parallelism(parallelism)
        .process_read_dir(move |_depth, _path, _state, children| {
            for child_result in children.iter_mut() {
                if let Ok(ref mut child) = child_result {
                    if !child.file_type.is_dir() {
                        continue;
                    }

                    let name = child.file_name.to_string_lossy().to_string();
                    let child_path = child.path();
                    let parent = match child_path.parent() {
                        Some(p) => p,
                        None => continue,
                    };

                    if let Some(artifact) = ProjectArtifact::from_name(&name, parent) {
                        if let Some(ref filter) = type_filter {
                            if !artifact.matches_filter(filter) {
                                continue;
                            }
                        }

                        // Skip descending into this directory
                        child.read_children_path = None;

                        results_clone.lock().unwrap().push(DetectedDir {
                            path: child_path,
                            artifact_type: artifact,
                            size_bytes: 0,
                        });
                    }
                }
            }
        });

    if let Some(depth) = config.max_depth {
        walker = walker.max_depth(depth);
    }

    // Drive the iterator to completion
    for _ in walker {}

    // Drop is implicit after loop, but the closure inside walker still holds
    // a clone of the Arc. Use lock instead of try_unwrap.
    let mut found = std::mem::take(&mut *results.lock().unwrap());

    // Compute sizes in parallel
    found.par_iter_mut().for_each(|d| {
        d.size_bytes = compute_dir_size(&d.path);
    });

    // Sort by size descending
    found.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));

    found
}

fn compute_dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}

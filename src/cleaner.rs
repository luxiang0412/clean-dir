use rayon::prelude::*;

use crate::scanner::DetectedDir;

pub struct CleanResult {
    pub deleted: u64,
    pub freed: u64,
    pub failed: u64,
}

pub fn clean(dirs: &[DetectedDir]) -> CleanResult {
    let (deleted, freed, failed) = dirs
        .par_iter()
        .fold(
            || (0u64, 0u64, 0u64),
            |(del, freed, fail), d| match std::fs::remove_dir_all(&d.path) {
                Ok(()) => (del + 1, freed + d.size_bytes, fail),
                Err(e) => {
                    eprintln!("Failed to remove {}: {}", d.path.display(), e);
                    (del, freed, fail + 1)
                }
            },
        )
        .reduce(
            || (0, 0, 0),
            |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2),
        );

    CleanResult {
        deleted,
        freed,
        failed,
    }
}

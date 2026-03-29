# clean-dir

A high-performance CLI tool for finding and removing project build/dependency directories to reclaim disk space.

## Features

- **Fast** — Parallel directory scanning via [jwalk](https://github.com/Byron/jwalk) + [rayon](https://github.com/rayon-rs/rayon), automatically skips descent into matched directories
- **Smart detection** — Correctly identifies Java `target/` directories by checking for `pom.xml` / `build.gradle` / `build.gradle.kts` in the parent, avoiding false positives
- **Safe** — Dry-run mode, interactive confirmation before deletion, colored output for easy review
- **Cross-platform** — Supports Linux, macOS, and Windows

## Supported Directory Types

| Type | Directory | Detection Rule |
|------|-----------|----------------|
| Python | `.venv` | Directory named `.venv` |
| Node.js | `node_modules` | Directory named `node_modules` |
| Java | `target` | Directory named `target` with `pom.xml`, `build.gradle`, or `build.gradle.kts` in parent |

## Installation

### From source

```bash
cargo install --path .
```

### From pre-built binaries

Download the latest release for your platform from the [Releases](https://github.com/yourname/clean-dir/releases) page.

## Usage

```
clean-dir [OPTIONS] [PATH]
```

### Arguments

| Argument | Description | Default |
|----------|-------------|---------|
| `PATH` | Root directory to scan | `.` (current directory) |

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--dry-run` | `-n` | Show what would be deleted without deleting |
| `--types <TYPES>` | `-t` | Filter by type: `python`, `node`, `java` (comma-separated) |
| `--max-depth <N>` | `-d` | Maximum scan depth |
| `--threads <N>` | `-j` | Number of threads (0 = auto, default) |
| `--yes` | `-y` | Skip confirmation prompt |
| `--help` | `-h` | Print help |
| `--version` | `-V` | Print version |

### Examples

```bash
# Preview what would be cleaned in ~/projects
clean-dir ~/projects --dry-run

# Only clean node_modules and .venv
clean-dir ~/projects -t node,python -n

# Clean everything, skip confirmation
clean-dir ~/projects -y

# Limit scan depth to 3 levels
clean-dir ~/projects -d 3 -n

# Use 8 threads
clean-dir ~/projects -j 8 -n
```

### Sample Output

```
Scanning /home/user/projects...

Found 3 cleanable directories:

  [Node]     /home/user/projects/frontend/node_modules   891.2 MB
  [Python]   /home/user/projects/api/.venv               312.4 MB
  [Java]     /home/user/projects/backend/target            78.3 MB

Total: 1.3 GB across 3 directories

Delete all? [y/N]:
```

## Building

```bash
# Debug build
make build

# Release build
make release

# Run clippy lints
make lint

# Run tests
make test
```

## Cross-compilation

Requires [cross](https://github.com/cross-rs/cross) and Docker.

```bash
# Install cross
cargo install cross

# Build Linux + Windows targets
make all-cross

# Build macOS targets (only on macOS host)
make all-macos

# Build all + create archives
make dist
```

### Build Targets

| Target | Build Command | Notes |
|--------|---------------|-------|
| `x86_64-unknown-linux-gnu` | `make all-cross` | via Docker |
| `aarch64-unknown-linux-gnu` | `make all-cross` | via Docker |
| `x86_64-pc-windows-gnu` | `make all-cross` | via Docker |
| `x86_64-apple-darwin` | `make all-macos` | macOS host only |
| `aarch64-apple-darwin` | `make all-macos` | macOS host only |

## License

MIT

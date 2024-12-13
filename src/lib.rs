//! # Barexp
//!
//! `barexp` is a library that automatically generates `mod.rs` files for your Rust project's subdirectories.
//!
//! ## Quick Start
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [build-dependencies]
//! barexp = "1.1.0"
//! ```
//!
//! Then create a `build.rs` in your project root:
//!
//! ```rust
//! fn main() {
//!     barexp::build();
//! }
//! ```
//!
//! ## Examples
//!
//! Basic usage:
//!
//! ```rust
//! // Your project's build.rs
//! fn main() {
//!     barexp::build();
//! }
//! ```
//!
//! ## Features
//!
//! - Recursively scans project subdirectories
//! - Automatically generates `mod.rs` files
//! - Re-exports all modules
//! - Ignores hidden files and `target` directory


use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Represents a Rust module in the file system
#[derive(Debug)]
struct Module {
    name: String,
    path: PathBuf,
    is_file: bool,
}
/// Optional function to generate mod.rs files for a given directory
///
/// This function is useful if you want to generate mod.rs files for a directory
/// other than the default "src" directory.
///
/// # Arguments
///
/// * `src_dir` - The source directory to scan
///
/// # Examples
///
/// ```rust
/// fn main() {
///   barexp::generate_mod_files("src/services");
/// }
/// ```
pub fn generate_mod_files(src_dir: &str) {
    println!("cargo:rerun-if-changed={}", src_dir);

    let src_path = Path::new(src_dir);

    // Tüm Rust dosyalarını ve dizinlerini bul
    for entry in WalkDir::new(src_dir)
        .into_iter()
        .filter_entry(|e| !is_hidden(e.path()))
        .filter_map(|e| e.ok()) {

        let path = entry.path();

        // Root src dizini hariç, dizin içindeki mod.rs'i oluştur
        if path.is_dir() && path != src_path && should_create_mod_rs(path) {
            generate_mod_rs(path);
        }
    }
}

/// Internal function to check if a path is hidden
///
/// # Arguments
///
/// * `path` - The path to check
///
/// # Returns
///
/// * `true` if the path is hidden or is "target" directory
/// * `false` otherwise
fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.starts_with('.') || s == "target")
        .unwrap_or(false)
}

fn should_create_mod_rs(dir: &Path) -> bool {
    // Dizinde en az bir .rs dosyası veya alt dizin varsa
    fs::read_dir(dir).map_or(false, |entries| {
        entries
            .filter_map(Result::ok)
            .any(|e| {
                let p = e.path();
                (p.is_file() && p.extension().map_or(false, |ext| ext == "rs") &&
                    p.file_name().map_or(false, |name| name != "mod.rs")) ||
                    (p.is_dir() && !is_hidden(&p))
            })
    })
}

fn collect_modules(dir: &Path) -> Vec<Module> {
    let mut modules = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();

            // Gizli dosyaları ve mod.rs'i atla
            if is_hidden(&path) || path.file_name().map_or(false, |n| n == "mod.rs") {
                continue;
            }

            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                // foo.rs -> mod foo
                if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                    modules.push(Module {
                        name: name.to_string(),
                        path,
                        is_file: true,
                    });
                }
            } else if path.is_dir() {
                // foo/mod.rs veya foo/lib.rs varsa -> mod foo
                if path.join("mod.rs").exists() || path.join("lib.rs").exists() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        modules.push(Module {
                            name: name.to_string(),
                            path,
                            is_file: false,
                        });
                    }
                }
            }
        }
    }

    modules
}

fn generate_mod_rs(dir: &Path) {
    let modules = collect_modules(dir);

    if modules.is_empty() {
        return;
    }

    let mut content = String::new();

    // Önce mod declarations
    for module in &modules {
        content.push_str(&format!("pub mod {};\n", module.name));
    }

    content.push('\n');

    // Sonra re-exports
    content.push_str("pub use self::{\n");
    for module in &modules {
        content.push_str(&format!("    {}::*,\n", module.name));
    }
    content.push_str("};\n");

    // mod.rs dosyasını oluştur veya güncelle
    let mod_path = dir.join("mod.rs");
    fs::write(mod_path, content).unwrap();
}

/// Main function to generate mod.rs files
///
/// This function scans the given directory and its subdirectories
/// to automatically generate mod.rs files.
///
/// # Arguments
///
/// * `src_dir` - The source directory to scan, typically "src"
///
/// # Examples
///
/// ```rust
/// fn main() {
///     barexp::build();
/// }
/// ```
///
/// # Panics
///
/// Will panic if:
/// * The source directory doesn't exist
/// * File system operations fail
/// * Permission errors occur
pub fn build() {
    generate_mod_files("src");
}
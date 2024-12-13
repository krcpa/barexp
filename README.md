# Auto Mods

Automatically generates `mod.rs` files for your Rust project's subdirectories. This crate simplifies module management by automatically creating and maintaining `mod.rs` files with proper module declarations and re-exports.

## Features

- 🔍 Recursively scans your project's subdirectories
- 🚀 Automatically generates/updates `mod.rs` files
- ♻️ Re-exports all modules with `pub use`
- 🎯 Ignores hidden files and `target` directory
- 🛡️ Preserves your project's root `lib.rs` or `main.rs`

## Installation

Add this to your `Cargo.toml`:

```toml
[build-dependencies]
auto-mods = "0.1.0"
```

## Usage

1. Create a `build.rs` file in your project root:

```rust
fn main() {
    auto_mods::build();
}
```

That's it! The crate will automatically:
- Scan your `src` directory recursively
- Generate `mod.rs` files in subdirectories
- Update existing `mod.rs` files when needed

### Example Project Structure

Before:
```
src/
  ├── lib.rs
  ├── services/
  │   ├── crypto.rs
  │   └── user.rs
  └── models/
      ├── account.rs
      └── transaction.rs
```

After:
```
src/
  ├── lib.rs
  ├── services/
  │   ├── mod.rs          // Auto-generated
  │   ├── crypto.rs
  │   └── user.rs
  └── models/
      ├── mod.rs          // Auto-generated
      ├── account.rs
      └── transaction.rs
```

Generated `mod.rs` content example:
```rust
pub mod crypto;
pub mod user;

pub use self::{
    crypto::*,
    user::*,
};
```

## How It Works

1. During the build process, the crate:
    - Scans all subdirectories in your `src` folder
    - Identifies Rust source files (`.rs`)
    - Creates or updates `mod.rs` files
    - Adds proper module declarations and re-exports

2. The crate preserves:
    - Your root `src/lib.rs` or `src/main.rs`
    - Hidden files and directories
    - The `target` directory

## Configuration

Currently, the crate works with default settings. Future versions will include:
- Custom ignore patterns
- Export strategies configuration
- Export attribute support
- Logging configuration

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Notes

- The crate operates during the build phase
- Changes to source files trigger automatic updates
- IDE integration works as expected since the generated files are regular Rust modules

## Known Limitations

- Assumes all modules should be public (`pub mod`)
- Re-exports all items from modules (`pub use`)
- No custom configuration options yet

## Future Plans

- [ ] Custom ignore patterns
- [ ] Different export strategies
- [ ] Export attribute support
- [ ] Logging and error handling
- [ ] Test coverage
- [ ] Custom module visibility options
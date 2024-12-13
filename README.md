# barexp

A Rust library providing elegant barrel exports with compile-time safety and collision detection.

## Overview

barexp simplifies module exports in Rust by providing a macro-based approach similar to TypeScript's barrel exports. It offers compile-time collision detection, full path exports, and a clean API for managing module exports.

## Features

- Attribute-based exports with `#[export]` and `#[export_fullpath]`
- Compile-time collision detection
- Module-level export aggregation
- Zero runtime overhead
- Clean and intuitive API
- Workspace-friendly design

## Installation

Add barexp to your `Cargo.toml`:

```toml
[dependencies]
barexp = "0.1.0"
```

## Usage

### Basic Export

Use the `#[export]` attribute to mark items for export and the `export!()` macro to expose them at the module level:

```rust
// services/crypto.rs
use barexp::export;

#[export]
pub struct CryptoService {
    // Your implementation
}

// services/mod.rs
use barexp::export;
export!();

// main.rs
use crate::services::CryptoService; // Direct access!
```

### Handling Name Collisions

When you have multiple items with the same name in different modules, you can use `#[export_fullpath]`:

```rust
// services/user/auth.rs
use barexp::export_fullpath;

#[export_fullpath]
pub struct User { }  // Will be exported as services::user::auth::User

// services/admin/auth.rs
use barexp::export_fullpath;

#[export_fullpath]
pub struct User { }  // Will be exported as services::admin::auth::User
```

### Compile-time Safety

barexp provides compile-time safety by detecting name collisions during compilation:

```rust
// This will cause a compile error:
#[export]
pub struct User { }  // in module A

#[export]
pub struct User { }  // in module B

// Error: Name collision detected: 'User' is already exported in 'moduleA'
```

## How It Works

barexp uses Rust's procedural macros and the inventory pattern to:

1. Register exported items at compile time
2. Detect naming collisions early
3. Generate efficient re-exports
4. Maintain type safety throughout

The library has zero runtime overhead as all the work is done during compilation.

## Best Practices

1. Use `#[export]` for most cases when you want clean imports
2. Switch to `#[export_fullpath]` when you need to disambiguate between items with the same name
3. Place `export!()` in your module's root (mod.rs)
4. Consider using `#[export_fullpath]` by default in large projects to prevent future collisions

## Examples

### Organizing Services

```rust
// services/crypto.rs
use barexp::export;

#[export]
pub struct CryptoService {
    // implementation
}

// services/user.rs
use barexp::export;

#[export]
pub struct UserService {
    // implementation
}

// services/mod.rs
use barexp::export;
export!();

// main.rs
use crate::services::{CryptoService, UserService};

fn main() {
    let crypto = CryptoService {};
    let user = UserService {};
}
```

### Using Full Paths

```rust
// features/auth/user.rs
use barexp::export_fullpath;

#[export_fullpath]
pub struct AuthService {
    // implementation
}

// features/payment/user.rs
use barexp::export_fullpath;

#[export_fullpath]
pub struct AuthService {
    // implementation
}

// Usage:
use crate::features::auth::AuthService as AuthAuthService;
use crate::features::payment::AuthService as PaymentAuthService;
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## Testing

Run the test suite:

```bash
cargo test               # Run all tests
cargo test -- --nocapture  # Run tests with output
```

The test suite includes:
- Unit tests
- Integration tests
- Compile-time tests (using trybuild)
- Documentation tests

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

This project was inspired by TypeScript's barrel exports pattern and built using Rust's powerful macro system.
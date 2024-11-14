# vtid - Volatile Type ID

A Rust library for generating volatile type IDs that change when a crate is recompiled.

## 🚀 Features

- **Extended Type IDs**: Generate extended type IDs that change with each crate recompilation.
- **Derive Macro Support**: Easily derive the `HasVtid` trait for your types.
- **`no_std` Compatible**: Use in embedded and other `no_std` environments.
- **Minimal Dependencies**: Zero dependencies, except for the derive macro.

## 📦 Installation

Add `vtid` to your `Cargo.toml`:

```toml
[dependencies]
vtid = { version = "0.1.0", features = ["derive"] }
```

## 🛠️ Usage

Here's how to use `vtid` in your project:

```rust
use vtid::{Vtid, HasVtid};

// Derive HasVtid for your types
#[derive(HasVtid)]
struct MyType;

// Get the volatile type ID
let type_id = Vtid::of::<MyType>();
println!("Type ID: {:?}", type_id);

// IDs change when crate is recompiled
let id1 = Vtid::of::<MyType>();

// Restart the program.
let id2 = Vtid::of::<MyType>(); // Same as id1

// Recompile program, but this crate and deps are not changed, so rlib is reused.
let id3 = Vtid::of::<MyType>(); // Should be the same as id1

// After this crate recompilation...
let id4 = Vtid::of::<MyType>(); // Different from id1
```

## 📜 License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

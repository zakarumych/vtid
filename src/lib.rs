//! # vtid - Volatile Type ID
//!
//! A Rust library for generating volatile type IDs that change when a crate is recompiled.
//!
//! ## 🚀 Features
//!
//! - **Extended Type IDs**: Generate extended type IDs that change with each crate recompilation.
//! - **Derive Macro Support**: Easily derive the `HasVtid` trait for your types.
//! - **`no_std` Compatible**: Use in embedded and other `no_std` environments.
//! - **Minimal Dependencies**: Zero dependencies, except for the derive macro.
//!
//! ## 📦 Installation
//!
//! Add `vtid` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! vtid = { version = "0.1.0", features = ["derive"] }
//! ```
//!
//! ## 🛠️ Usage
//!
//! Here's how to use `vtid` in your project:
//!
//! ```rust
//! use vtid::{Vtid, HasVtid};
//!
//! // Derive HasVtid for your types
//! #[derive(HasVtid)]
//! struct MyType;
//!
//! // Get the volatile type ID
//! let type_id = Vtid::of::<MyType>();
//! println!("Type ID: {:?}", type_id);
//!
//! // IDs change when crate is recompiled
//! let id1 = Vtid::of::<MyType>();
//!
//! // Restart the program.
//! let id2 = Vtid::of::<MyType>(); // Same as id1
//!
//! // Recompile program, but this crate and deps are not changed, so rlib is reused.
//! let id3 = Vtid::of::<MyType>(); // Should be the same as id1
//!
//! // After this crate recompilation...
//! let id4 = Vtid::of::<MyType>(); // Different from id1
//! ```

#![no_std]

use core::{any::TypeId, fmt};

/// A trait that provides type identification that can change between crate compilations.
///
/// This trait is typically implemented via the derive macro when the "derive" feature is enabled.

pub trait HasVtid {
    #[doc(hidden)]
    fn vtid() -> Vtid
    where
        Self: Sized;
}

/// A type identifier that changes between compilations of the containing crate.
///
/// This struct combines a stable TypeId with a base identifier that changes whenever
/// the crate containing the type is recompiled.
///
/// This allows reusing existing instances of a type from cdylib crate when new version is linked
/// if Vtid does not change, since crate is not recompiled and thus memory layout of the type is unchanged.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vtid {
    tid: TypeId,
    base_id: u64,
}

impl Vtid {
    /// Creates a volatile type identifier for a type that implements `HasVtid`.
    ///
    /// # Type Parameters
    /// * `T` - The type to create a Vtid for. Must implement `HasVtid`.
    pub fn of<T>() -> Self
    where
        T: HasVtid,
    {
        T::vtid()
    }
}

impl fmt::Debug for Vtid {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vtid({:#016x}, {:?})", self.base_id, self.tid)
    }
}

#[cfg(feature = "derive")]
#[doc(hidden)]
pub mod private {
    use core::any::TypeId;

    /// Creates a new Vtid instance with the given base_id for type T.
    ///
    /// # Type Parameters
    /// * `T` - The type to create a Vtid for. Must implement 'static.
    pub fn vtid<T: 'static>(base_id: u64) -> super::Vtid {
        super::Vtid {
            tid: TypeId::of::<T>(),
            base_id,
        }
    }
}

#[cfg(feature = "derive")]
pub use vtid_proc::HasVtid;

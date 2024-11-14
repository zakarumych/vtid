#![no_std]

use core::{any::TypeId, fmt};

/// A trait that provides type identification that can change between crate compilations.
///
/// This trait is typically implemented via the derive macro when the "derive" feature is enabled.
#[doc(hidden)]
pub trait HasVtid {
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

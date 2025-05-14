#![stable(feature = "rust1", since = "1.0.0")]

#[stable(feature = "rust1", since = "1.0.0")]
#[doc(inline)]
pub use self::hash_map::HashMap;

// #[stable(feature = "rust1", since = "1.0.0")]
// FIXME(#82080) The deprecation here is only theoretical, and does not actually produce a warning.
// #[deprecated(note = "moved to `std::ops::Bound`", since = "1.26.0")]
// #[doc(hidden)]
// pub use crate::ops::Bound;

mod hash;

#[stable(feature = "rust1", since = "1.0.0")]
pub mod hash_map {
    //! A hash map implemented with quadratic probing and SIMD lookup.
    #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::hash::map::*;
    #[stable(feature = "hashmap_build_hasher", since = "1.7.0")]
    pub use crate::hash::random::DefaultHasher;
    #[stable(feature = "hashmap_build_hasher", since = "1.7.0")]
    pub use crate::hash::random::RandomState;
}

#![feature(hashmap_internals)]
#![feature(extend_one)]
#![feature(hasher_prefixfree_extras)]
#![feature(error_in_core)]
#![feature(try_reserve_kind)]
#![feature(thread_local)]
#![feature(const_hash)]

extern crate alloc;
pub use alloc::collections::*;
pub use hashbrown::HashMap;

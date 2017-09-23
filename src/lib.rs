#[cfg(feature="with_mmap")]
extern crate memmap;
extern crate owning_ref;
extern crate num_traits;

mod types;
mod read;
mod write;
mod byteviews;

pub use types::*;
pub use read::*;
pub use write::*;
pub use byteviews::*;

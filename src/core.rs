#[macro_use]
pub mod macros;

mod array;
mod base;
mod storage;
mod traits;

pub use array::Array;
pub use base::ArrayBase;
pub use storage::{ArrayStorage, ArrayStorageVariant};
pub use traits::NdArrayBase;
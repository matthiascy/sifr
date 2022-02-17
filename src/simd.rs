pub(crate) mod avx2;
pub(crate) mod avx512;
pub(crate) mod sse42;

use avx2::*;
use avx512::*;
use sse42::*;

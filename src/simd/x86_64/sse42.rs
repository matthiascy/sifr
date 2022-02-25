use crate::impl_nd_array_base;
use crate::core::NdArrayBase;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use std::ops::{Add, Deref, DerefMut, Index, IndexMut};
use crate::simd::{slice_to_array, slice_to_array_unchecked};

#[repr(C, align(16))]
pub struct Align16<T: Copy, const N: usize>(pub(crate) [T; N]);

impl<T: Copy, const N: usize> Deref for Align16<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Copy, const N: usize> DerefMut for Align16<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Copy, const N: usize> Align16<T, N> {
    pub const fn from_array(array: [T; N]) -> Self {
        Self(array)
    }

    pub const fn from_slice(slice: &[T]) -> Self {
        Self(slice_to_array(slice))
    }

    pub const fn from_slice_unchecked(slice: &[T]) -> Self {
        Self(slice_to_array_unchecked(slice))
    }
}

#[derive(Copy, Clone)]
pub struct F32x4(__m128);

#[derive(Copy, Clone)]
pub struct F64x2(__m128d);

#[derive(Copy, Clone)]
pub struct I8x16(__m128i);

#[derive(Copy, Clone)]
pub struct I16x8(__m128i);

#[derive(Copy, Clone)]
pub struct I32x4(__m128i);

#[derive(Copy, Clone)]
pub struct I64x2(__m128i);

#[test]
fn test_sse_alignment() {
    use core::mem::{align_of, align_of_val};
    assert_eq!(align_of::<Align16<f32, 4>>(), 16);
    let arr = [0f32; 4];
    let a = Align16::<f32, 4>::from_array(arr);
    assert_eq!(align_of_val(&a), 16);
    assert_ne!(align_of_val(&arr), align_of_val(&a));
}

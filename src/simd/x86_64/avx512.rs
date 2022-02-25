use std::arch::x86_64;
use std::ops::{Deref, DerefMut};
use crate::simd::{slice_to_array, slice_to_array_unchecked};

#[repr(C, align(64))]
pub struct Align64<T: Copy, const N: usize>(pub(crate) [T; N]);

impl<T: Copy, const N: usize> Deref for Align64<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Copy, const N: usize> DerefMut for Align64<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Copy, const N: usize> Align64<T, N> {
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

pub struct F32x16(x86_64::__m512);

pub struct F64x8(x86_64::__m512d);

pub struct I8x64(x86_64::__m512i);

pub struct I16x32(x86_64::__m512bh);

pub struct I32x16(x86_64::__m512i);

pub struct I64x8(x86_64::__m512i);

// impl_nd_array_base!(F32x16, F64x8, I8x64, I16x32, I32x16, I64x8);

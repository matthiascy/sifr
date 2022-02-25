use crate::impl_nd_array_base;
use std::arch::x86_64::{_mm_add_ps, _mm_load_ps, _mm_store_ps};
use std::ops::{Add, Deref, DerefMut};
use crate::simd::{slice_to_array, slice_to_array_unchecked};

// 32x8, 64x4, 16x16, 8x32

#[repr(C, align(32))]
pub struct Align32<T: Copy, const N: usize>(pub(crate) [T; N]);

impl<T: Copy, const N: usize> Deref for Align32<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Copy, const N: usize> DerefMut for Align32<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Copy, const N: usize> Align32<T, N> {
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

impl Add for Align32<f32, 8> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Align32::from_array(self.0);
        unsafe {
            _mm_store_ps(
                output.as_mut_ptr(),
                _mm_add_ps(
                    _mm_load_ps(self.0.as_ptr()),
                    _mm_load_ps(rhs.0.as_ptr()),
                ),
            );
        }
        output
    }
}

impl<T: Copy, const N: usize> Align32<T, N> {
    pub const fn into_array<const M: usize>(self) -> [T; M] {
        assert!(M < N,
        "cannot convert into an array with greater size");
        slice_to_array_unchecked(&self.0)
    }
}
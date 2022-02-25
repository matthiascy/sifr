use crate::core::traits::{NdArrayBase, NdArrayDepth};
use crate::core::ScalarType;
use core::marker::PhantomData;
use core::ops::Deref;
use std::ops::{Add, DerefMut};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use std::arch::x86_64::*;

use crate::simd::{slice_to_array, slice_to_array_unchecked};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx2")]
use crate::simd::avx2::Align32;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::simd::avx512::Align64;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "sse4.2")]
use crate::simd::sse42::Align16;

pub struct Array<T, const N: usize> {
    data: [T; N],
    _marker: PhantomData<T>,
}

impl<T, const N: usize> Deref for Array<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, const N: usize> DerefMut for Array<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T, const N: usize> IntoIterator for Array<T, N> {
    type Item = T;
    type IntoIter = core::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Array<T, N> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut Array<T, N> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl<T, const N: usize> NdArrayDepth for Array<T, N> {
    const ARRAY_DEPTH: usize = <T as NdArrayDepth>::ARRAY_DEPTH + 1;
}

impl<T, const N: usize> ScalarType for Array<T, N> {
    type Scalar = <T as ScalarType>::Scalar;
}

impl<T: Add<Output = T> + Copy, const N: usize> Add for Array<T, N> {
    type Output = Array<T, N>;

    default fn add(self, rhs: Self) -> Self::Output {
        println!("default add");
        let mut data = unsafe { core::mem::zeroed::<[T; N]>() };
        let mut i = 0;
        while i < N {
            data[i] = self.data[i] + rhs.data[i];
            i += 1;
        }
        Array {
            data,
            _marker: Default::default(),
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "sse4.2")]
impl Add for Array<f32, 4> {
    fn add(self, rhs: Self) -> Self::Output {
        println!("add f32x4");
        Array {
            data: unsafe {
                let mut output: Align16<f32, 4> = Align16::from_array(self.data);
                _mm_store_ps(
                    output.as_mut_ptr(),
                    _mm_add_ps(
                        _mm_load_ps(self.data.as_ptr()),
                        _mm_load_ps(rhs.data.as_ptr()),
                    ),
                );
                output.0
            },
            _marker: Default::default(),
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "sse4.2")]
impl Add for Array<f32, 3> {
    fn add(self, rhs: Self) -> Self::Output {
        println!("add f32x3");
        Array {
            data: unsafe {
                let mut output: Align16<f32, 4> = Align16::from_slice_unchecked(&self.data);
                _mm_store_ps(
                    output.as_mut_ptr(),
                    _mm_add_ps(
                        _mm_load_ps(self.data.as_ptr()),
                        _mm_load_ps(rhs.data.as_ptr()),
                    ),
                );
                slice_to_array(&output.0)
            },
            _marker: Default::default(),
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "sse4.2")]
impl Add for Array<f32, 2> {
    fn add(self, rhs: Self) -> Self::Output {
        println!("add f32x2");
        Array {
            data: unsafe {
                let mut output: Align16<f32, 4> = Align16::from_slice_unchecked(&self.data);
                _mm_store_ps(
                    output.as_mut_ptr(),
                    _mm_add_ps(
                        _mm_load_ps(self.data.as_ptr()),
                        _mm_load_ps(rhs.data.as_ptr()),
                    ),
                );
                slice_to_array(&output.0)
            },
            _marker: Default::default(),
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx")]
impl Add for Array<f32, 8> {
    fn add(self, rhs: Self) -> Self::Output {
        println!("add f32x8");
        Array {
            data: (Align32::from_array(self.data) + Align32::from_array(rhs.data)).into_array(),
            _marker: Default::default(),
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx")]
impl Add for Array<f32, 7> {
    fn add(self, rhs: Self) -> Self::Output {
        println!("add f32x7");
        Array {
            data: unsafe {
                let mut output: Align32<f32, 8> = Align32::from_slice_unchecked(&self.data);
                _mm256_store_ps(
                    output.as_mut_ptr(),
                    _mm256_add_ps(
                        _mm256_load_ps(self.data.as_ptr()),
                        _mm256_load_ps(rhs.data.as_ptr()),
                    ),
                );
                slice_to_array(&output.0)
            },
            _marker: Default::default(),
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx")]
impl Add for Array<f32, 6> {
    fn add(self, rhs: Self) -> Self::Output {
        println!("add f32x6");
        Array {
            data: unsafe {
                let mut output: Align32<f32, 8> = Align32::from_slice_unchecked(&self.data);
                _mm256_store_ps(
                    output.as_mut_ptr(),
                    _mm256_add_ps(
                        _mm256_load_ps(self.data.as_ptr()),
                        _mm256_load_ps(rhs.data.as_ptr()),
                    ),
                );
                slice_to_array(&output.0)
            },
            _marker: Default::default(),
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx")]
impl Add for Array<f32, 5> {
    fn add(self, rhs: Self) -> Self::Output {
        println!("add f32x5");
        Array {
            data: unsafe {
                let mut output: Align32<f32, 8> = Align32::from_slice_unchecked(&self.data);
                _mm256_store_ps(
                    output.as_mut_ptr(),
                    _mm256_add_ps(
                        _mm256_load_ps(self.data.as_ptr()),
                        _mm256_load_ps(rhs.data.as_ptr()),
                    ),
                );
                slice_to_array(&output.0)
            },
            _marker: Default::default(),
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx512f")]
impl Add for Array<f32, 16> {
    fn add(self, rhs: Self) -> Self::Output {
        println!("add f32x16");
        Array {
            data: unsafe {
                let mut output: Align64<f32, 16> = Align64::from_array(self.data);
                _mm512_store_ps(
                    output.as_mut_ptr(),
                    _mm512_add_ps(
                        _mm512_load_ps(self.data.as_ptr()),
                        _mm512_load_ps(rhs.data.as_ptr()),
                    ),
                );
                output.0
            },
            _marker: Default::default(),
        }
    }
}

impl<T, const N: usize> Array<T, N> {
    pub fn new() -> Self {
        Self {
            data: unsafe { core::mem::zeroed::<[T; N]>() },
            _marker: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Array;

    #[test]
    fn array_add_f32x4() {
        {
            let mut a: Array<f32, 4> = Array::new();
            let mut b: Array<f32, 4> = Array::new();
            for i in 0..4 {
                a[i] = i as f32;
                b[i] = (4 - i) as f32;
            }

            let c = a + b;
            for i in 0..4 {
                assert_eq!(c[i], 4.0);
            }
        }

        {
            let mut a: Array<f32, 3> = Array::new();
            let mut b: Array<f32, 3> = Array::new();
            for i in 0..3 {
                a[i] = i as f32;
                b[i] = (4 - i) as f32;
            }

            let c = a + b;
            for i in 0..3 {
                assert_eq!(c[i], 4.0);
            }
        }
    }

    #[test]
    fn array_add_f32x8() {
        let mut a: Array<f32, 8> = Array::new();
        let mut b: Array<f32, 8> = Array::new();
        for i in 0..8 {
            a[i] = i as f32;
            b[i] = (8 - i) as f32;
        }

        let c = a + b;
        for i in 0..8 {
            assert_eq!(c[i], 8.0);
        }
    }

    #[test]
    fn array_add_f32x7() {
        let mut a: Array<f32, 7> = Array::new();
        let mut b: Array<f32, 7> = Array::new();
        for i in 0..7 {
            a[i] = i as f32;
            b[i] = (8 - i) as f32;
        }

        let c = a + b;
        for i in 0..7 {
            assert_eq!(c[i], 8.0);
        }
    }

    #[test]
    fn array_add_f32x16() {
        let mut a: Array<f32, 16> = Array::new();
        let mut b: Array<f32, 16> = Array::new();
        for i in 0..16 {
            a[i] = i as f32;
            b[i] = (16 - i) as f32;
        }

        let c = a + b;
        for i in 0..16 {
            assert_eq!(c[i], 16.0);
        }
    }
}

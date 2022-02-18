use crate::core::traits::{ArrayStorage, NdArrayBase, NdArrayDepth};
use crate::core::{DynamicStorage, StaticStorage, UnreachableStorage};
use core::marker::PhantomData;
use core::ops::Deref;
use std::ops::DerefMut;

pub struct ArrayBase<T, const N: usize, const D: bool>
where
    Self: ArrayStorage<T>,
{
    pub(crate) inner: <Self as ArrayStorage<T>>::StorageVariant,
    _marker: PhantomData<T>,
}

impl<T, const N: usize, const D: bool> ArrayStorage<T> for ArrayBase<T, N, D> {
    default type StorageVariant = UnreachableStorage<T>;
}

impl<T, const N: usize> ArrayStorage<T> for ArrayBase<T, N, true> {
    default type StorageVariant = DynamicStorage<T>;
}

impl<T, const N: usize> ArrayStorage<T> for ArrayBase<T, N, false> {
    default type StorageVariant = StaticStorage<T, N>;
}

impl<T, const N: usize, const D: bool> ArrayBase<T, N, D> {
    pub fn new() -> Self {
        Self {
            inner: <<Self as ArrayStorage<T>>::StorageVariant as NdArrayBase>::new(),
            _marker: Default::default(),
        }
    }
}

impl<T, const N: usize, const D: bool> Deref for ArrayBase<T, N, D> {
    type Target = <ArrayBase<T, N, D> as ArrayStorage<T>>::StorageVariant;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, const N: usize, const D: bool> DerefMut for ArrayBase<T, N, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T, const N: usize, const D: bool> IntoIterator for ArrayBase<T, N, D> {
    type Item = T;
    type IntoIter =
        <<ArrayBase<T, N, D> as ArrayStorage<T>>::StorageVariant as NdArrayBase>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, T, const N: usize, const D: bool> IntoIterator for &'a ArrayBase<T, N, D> {
    type Item = &'a T;
    type IntoIter =
        <<ArrayBase<T, N, D> as ArrayStorage<T>>::StorageVariant as NdArrayBase>::Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a, T, const N: usize, const D: bool> IntoIterator for &'a mut ArrayBase<T, N, D> {
    type Item = &'a mut T;
    type IntoIter =
        <<ArrayBase<T, N, D> as ArrayStorage<T>>::StorageVariant as NdArrayBase>::IterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

impl<T, const N: usize, const D: bool> NdArrayDepth for ArrayBase<T, N, D> {
    const ARRAY_DEPTH: usize = <T as NdArrayDepth>::ARRAY_DEPTH + 1;
}

// // 128-bit single/double-precision floating numbers
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "sse4.2")]
// impl ArrayStorageVariant for ArrayStorage<f32, 4> {
//     type Variant = simd::sse42::F32x4;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "sse4.2")]
// impl ArrayStorageVariant for ArrayStorage<f32, 3> {
//     type Variant = simd::sse42::F32x4;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "sse4.2")]
// impl ArrayStorageVariant for ArrayStorage<f32, 2> {
//     type Variant = simd::sse42::F32x4;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "sse4.2")]
// impl ArrayStorageVariant for ArrayStorage<f32, 1> {
//     type Variant = simd::sse42::F32x4;
// }

// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "sse4.2")]
// impl ArrayStorageVariant for ArrayStorage<f64, 2> {
//     type Variant = simd::sse42::F64x2;
// }

// // 256-bit single/double-precision floating numbers
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "avx")]
// impl ArrayStorageVariant for ArrayStorage<f32, 8> {
//     type Variant = simd::avx2::F32x8;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "avx")]
// impl ArrayStorageVariant for ArrayStorage<f32, 7> {
//     type Variant = simd::avx2::F32x8;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "avx")]
// impl ArrayStorageVariant for ArrayStorage<f32, 6> {
//     type Variant = simd::avx2::F32x8;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "avx")]
// impl ArrayStorageVariant for ArrayStorage<f32, 5> {
//     type Variant = simd::avx2::F32x8;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "avx")]
// impl ArrayStorageVariant for ArrayStorage<f64, 4> {
//     type Variant = simd::avx2::F64x4;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "avx")]
// impl ArrayStorageVariant for ArrayStorage<f64, 3> {
//     type Variant = simd::avx2::F64x4;
// }
//
// // 512-bit single/double-precision floating numbers
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "avx512f")]
// impl ArrayStorageVariant for ArrayStorage<f32, 16> {
//     type Variant = simd::avx512::F32x16;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "avx512f")]
// impl ArrayStorageVariant for ArrayStorage<f32, 15> {
//     type Variant = simd::avx512::F32x16;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "avx512f")]
// impl ArrayStorageVariant for ArrayStorage<f32, 14> {
//     type Variant = simd::avx512::F32x16;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "avx512f")]
// impl ArrayStorageVariant for ArrayStorage<f32, 13> {
//     type Variant = simd::avx512::F32x16;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "avx512f")]
// impl ArrayStorageVariant for ArrayStorage<f32, 12> {
//     type Variant = simd::avx512::F32x16;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "avx512f")]
// impl ArrayStorageVariant for ArrayStorage<f32, 11> {
//     type Variant = simd::avx512::F32x16;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "avx512f")]
// impl ArrayStorageVariant for ArrayStorage<f32, 10> {
//     type Variant = simd::avx512::F32x16;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "avx512f")]
// impl ArrayStorageVariant for ArrayStorage<f32, 9> {
//     type Variant = simd::avx512::F32x16;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "avx512f")]
// impl ArrayStorageVariant for ArrayStorage<f64, 8> {
//     type Variant = simd::avx512::F64x8;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "avx512f")]
// impl ArrayStorageVariant for ArrayStorage<f64, 7> {
//     type Variant = simd::avx512::F64x8;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "avx512f")]
// impl ArrayStorageVariant for ArrayStorage<f64, 6> {
//     type Variant = simd::avx512::F64x8;
// }
//
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// #[cfg(target_feature = "avx512f")]
// impl ArrayStorageVariant for ArrayStorage<f64, 5> {
//     type Variant = simd::avx512::F64x8;
// }

#[cfg(test)]
mod tests {
    use crate::core::{ArrayBase, ArrayStorage, DynamicStorage, StaticStorage};

    #[test]
    fn array_base_new() {
        use core::any::{type_name, type_name_of_val};

        let a: ArrayBase<u32, 4, false> = ArrayBase::new();
        let b: ArrayBase<f32, 10, true> = ArrayBase::new();

        assert_eq!(
            type_name::<StaticStorage<u32, 4>>(),
            type_name_of_val(&a.inner)
        );
        assert_eq!(
            type_name::<DynamicStorage<f32>>(),
            type_name_of_val(&b.inner)
        );
    }

    fn array_base_indexing() {}

    fn array_base_iter() {
    }

    fn array_base_into_iter() {}

    fn array_base_iter_mut() {}
}

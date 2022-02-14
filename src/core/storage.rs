use core::marker::PhantomData;
use crate::core::traits::NdArrayBase;
use crate::simd;

pub trait ArrayStorageVariant {
    type Variant: NdArrayBase;
}

pub struct ArrayStorage<T, const N: usize> {
    _marker: PhantomData<T>,
}

impl<T, const N: usize> ArrayStorageVariant for ArrayStorage<T, N> {
    default type Variant = StaticArray<T, N>;
}

// 128-bit single/double-precision floating numbers
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "sse4_2")]
impl ArrayStorageVariant for ArrayStorage<f32, 4> {
    type Variant = simd::sse42::F32x4;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "sse4_2")]
impl ArrayStorageVariant for ArrayStorage<f32, 3> {
    type Variant = simd::sse42::F32x4;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "sse4_2")]
impl ArrayStorageVariant for ArrayStorage<f32, 2> {
    type Variant = simd::sse42::F32x4;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "sse4_2")]
impl ArrayStorageVariant for ArrayStorage<f32, 1> {
    type Variant = simd::sse42::F32x4;
}


#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "sse4_2")]
impl ArrayStorageVariant for ArrayStorage<f64, 2> {
    type Variant = simd::sse42::F64x2;
}

// 256-bit single/double-precision floating numbers
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx")]
impl ArrayStorageVariant for ArrayStorage<f32, 8> {
    type Variant = simd::avx2::F32x8;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx")]
impl ArrayStorageVariant for ArrayStorage<f32, 7> {
    type Variant = simd::avx2::F32x8;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx")]
impl ArrayStorageVariant for ArrayStorage<f32, 6> {
    type Variant = simd::avx2::F32x8;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx")]
impl ArrayStorageVariant for ArrayStorage<f32, 5> {
    type Variant = simd::avx2::F32x8;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx")]
impl ArrayStorageVariant for ArrayStorage<f64, 4> {
    type Variant = simd::avx2::F64x4;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx")]
impl ArrayStorageVariant for ArrayStorage<f64, 3> {
    type Variant = simd::avx2::F64x4;
}

// 512-bit single/double-precision floating numbers
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx512f")]
impl ArrayStorageVariant for ArrayStorage<f32, 16> {
    type Variant = simd::avx512::F32x16;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx512f")]
impl ArrayStorageVariant for ArrayStorage<f32, 15> {
    type Variant = simd::avx512::F32x16;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx512f")]
impl ArrayStorageVariant for ArrayStorage<f32, 14> {
    type Variant = simd::avx512::F32x16;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx512f")]
impl ArrayStorageVariant for ArrayStorage<f32, 13> {
    type Variant = simd::avx512::F32x16;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx512f")]
impl ArrayStorageVariant for ArrayStorage<f32, 12> {
    type Variant = simd::avx512::F32x16;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx512f")]
impl ArrayStorageVariant for ArrayStorage<f32, 11> {
    type Variant = simd::avx512::F32x16;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx512f")]
impl ArrayStorageVariant for ArrayStorage<f32, 10> {
    type Variant = simd::avx512::F32x16;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx512f")]
impl ArrayStorageVariant for ArrayStorage<f32, 9> {
    type Variant = simd::avx512::F32x16;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx512f")]
impl ArrayStorageVariant for ArrayStorage<f64, 8> {
    type Variant = simd::avx512::F64x8;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx512f")]
impl ArrayStorageVariant for ArrayStorage<f64, 7> {
    type Variant = simd::avx512::F64x8;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx512f")]
impl ArrayStorageVariant for ArrayStorage<f64, 6> {
    type Variant = simd::avx512::F64x8;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_feature = "avx512f")]
impl ArrayStorageVariant for ArrayStorage<f64, 5> {
    type Variant = simd::avx512::F64x8;
}


pub struct StaticArray<T, const N: usize> {
    data: [T; N],
    _marker: PhantomData<T>
}

impl<T, const N: usize> NdArrayBase for StaticArray<T, N> {
    fn new() -> Self {
        println!("static array new");
        Self {
            data: unsafe { ::core::mem::zeroed::<[T; N]>() },
            _marker: Default::default()
        }
    }
}
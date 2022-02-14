use std::ops::Deref;
use crate::core::{ArrayBase, ArrayStorage, ArrayStorageVariant};
use crate::core::traits::NdArrayDepth;

pub struct Array<T, const N: usize>(ArrayBase<T, N, ArrayStorage<T, N>>);

impl<T, const N: usize> Array<T, N> {
    pub const LEN: usize = N;
    pub const REAL_LEN: usize = N;
    pub const DEPTH: usize = ArrayBase::<T, N, ArrayStorage<T, N>>::ARRAY_DEPTH;

    pub const fn depth() -> usize {
        Self::DEPTH
    }

    pub fn new() -> Self {
        Self(ArrayBase::new())
    }
}

impl<T, const N: usize> Deref for Array<T, N> {
    type Target = <ArrayStorage<T, N> as ArrayStorageVariant>::Variant;

    fn deref(&self) -> &Self::Target {
        &self.0.inner
    }
}

pub struct DynArray<T> {
    data: Box<[T]>,
}

impl<T> NdArrayDepth for DynArray<T> {
    const ARRAY_DEPTH: usize = <T as NdArrayDepth>::ARRAY_DEPTH + 1;
}
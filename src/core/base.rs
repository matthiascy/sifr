use std::marker::PhantomData;
use crate::core::{Array, ArrayStorage, ArrayStorageVariant, NdArrayBase};
use crate::core::traits::NdArrayDepth;

pub struct ArrayBase<T, const N: usize, S>
    where S: ArrayStorageVariant
{
    pub(crate) inner: S::Variant,
    _marker: PhantomData<T>,
}

impl<T, const N: usize> NdArrayDepth for ArrayBase<T, N, ArrayStorage<T, N>> {
    const ARRAY_DEPTH: usize = <T as NdArrayDepth>::ARRAY_DEPTH + 1;
}

impl<T, const N: usize> ArrayBase<T, N, ArrayStorage<T, N>> {
    pub fn new() -> Self {
        Self {
            inner: <<ArrayStorage<T, N> as ArrayStorageVariant>::Variant as NdArrayBase>::new(),
            _marker: Default::default()
        }
    }
}
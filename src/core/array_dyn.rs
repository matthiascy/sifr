use crate::core::traits::NdArrayDepth;
use crate::core::ArrayBase;
use std::ops::{Deref, DerefMut};

pub struct DynArray<T>(ArrayBase<T, 0, true>);

impl<T> NdArrayDepth for DynArray<T> {
    const ARRAY_DEPTH: usize = <T as NdArrayDepth>::ARRAY_DEPTH + 1;
}

impl<T> DynArray<T> {
    pub const DEPTH: usize = ArrayBase::<T, 0, true>::ARRAY_DEPTH;

    pub const fn depth(&self) -> usize {
        Self::DEPTH
    }

    pub fn new() -> Self {
        Self(ArrayBase::new())
    }
}

impl<T> Deref for DynArray<T> {
    type Target = ArrayBase<T, 0, true>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for DynArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

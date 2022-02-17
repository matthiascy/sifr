use crate::core::traits::NdArrayDepth;
use crate::core::ArrayBase;
use std::ops::{Deref, DerefMut};

pub struct Array<T, const N: usize>(pub(crate) ArrayBase<T, N, false>);

impl<T, const N: usize> NdArrayDepth for Array<T, N> {
    const ARRAY_DEPTH: usize = <T as NdArrayDepth>::ARRAY_DEPTH + 1;
}

impl<T, const N: usize> Array<T, N> {
    pub const LEN: usize = N;
    pub const REAL_LEN: usize = N;
    pub const DEPTH: usize = ArrayBase::<T, N, false>::ARRAY_DEPTH;

    pub const fn depth(&self) -> usize {
        Self::DEPTH
    }

    pub const fn len(&self) -> usize {
        Self::LEN
    }

    pub fn new() -> Self {
        Self(ArrayBase::new())
    }
}

impl<T, const N: usize> Deref for Array<T, N> {
    type Target = ArrayBase<T, N, false>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for Array<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Array;
    use crate::core::NdArrayBase;

    #[test]
    fn array_new_constructor() {
        let a: Array<u32, 8> = Array::new();
        for e in a.iter() {
            assert_eq!(e, &0);
        }
    }

    #[test]
    fn array_underlying_storage() {
        let a: Array<u32, 4> = Array::new();
    }
}

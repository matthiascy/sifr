use crate::core::traits::NdArrayDepth;
use crate::core::{ArrayBase, ArrayStorage, DesugaredArrayType, NdArray, NdArrayBase, ScalarType};
use std::ops::{Add, Deref, DerefMut};

pub struct Array<T, const N: usize>(pub(crate) ArrayBase<T, N, false>);

impl<T, const N: usize> DesugaredArrayType for Array<T, N> {
    type Desugared = ArrayBase<T, N, false>;
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

impl<T, const N: usize> IntoIterator for Array<T, N> {
    type Item = T;
    // type IntoIter = <<ArrayBase<T, N, false> as ArrayStorage<T>>::StorageVariant as NdArrayBase>::IntoIter;
    type IntoIter = <<Self as DesugaredArrayType>::Desugared as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.inner.into_iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Array<T, N> {
    type Item = &'a T;
    // type IntoIter = <<ArrayBase<T, N, false> as ArrayStorage<T>>::StorageVariant as NdArrayBase>::Iter<'a>;
    type IntoIter = <&'a ArrayBase<T, N, false> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.inner.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut Array<T, N> {
    type Item = &'a mut T;
    // type IntoIter = <<ArrayBase<T, N, false> as ArrayStorage<T>>::StorageVariant as NdArrayBase>::IterMut<'a>;
    type IntoIter = <&'a mut ArrayBase<T, N, false> as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.inner.iter_mut()
    }
}

impl<T, const N: usize> NdArray for Array<T, N> {
    type Value = T;
    type Scalar = <<T as DesugaredArrayType>::Desugared as ScalarType>::Scalar;
}

impl<T: Add<Output = T> + Copy, const N: usize> Add for Array<T, N> {
    type Output = Array<T, N>;

    fn add(self, rhs: Self) -> Self::Output {
        Array(self.0 + rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Array;
    use crate::core::{ArrayBase, NdArray};
    use core::any::type_name;

    #[test]
    fn array_new() {
        let a: Array<u32, 8> = Array::new();
        for e in &a {
            assert_eq!(e, &0);
        }
    }

    #[test]
    fn array_for_iter() {
        let mut a = Array::<u32, 12>::new();
        for e in &mut a {
            *e = 1;
        }

        for e in &a {
            assert_eq!(e, &1);
        }
    }

    #[test]
    fn array_scalar_type() {
        assert_eq!(
            type_name::<<Array<u32, 12> as NdArray>::Scalar>(),
            type_name::<<Array<Array<u32, 4>, 4> as NdArray>::Scalar>()
        );
        assert_ne!(
            type_name::<<Array<Array<f32, 4>, 12> as NdArray>::Scalar>(),
            type_name::<<Array<Array<u32, 4>, 4> as NdArray>::Scalar>()
        );
    }

    #[test]
    fn array_add() {
        let mut a: Array<u32, 4> = Array::new();
        let mut b: Array<u32, 4> = Array::new();
        for e in &mut a {
            *e = 1;
        }
        for e in &mut b {
            *e = 10;
        }

        let c = a + b;
        for e in &c {
            assert_eq!(e, &11);
        }
    }
}

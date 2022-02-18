use crate::core::traits::NdArrayDepth;
use crate::core::{ArrayBase, DesugaredArrayType, NdArray, ArrayScalarType, ArrayStorage, NdArrayBase};
use std::ops::{Deref, DerefMut};

pub struct DynArray<T>(pub(crate) ArrayBase<T, 0, true>);

impl<T> DesugaredArrayType for DynArray<T> {
    type Desugared = ArrayBase<T, 0, true>;
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

impl<T> IntoIterator for DynArray<T> {
    type Item = T;
    type IntoIter = <<ArrayBase<T, 0, true> as ArrayStorage<T>>::StorageVariant as NdArrayBase>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.inner.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a DynArray<T> {
    type Item = &'a T;
    type IntoIter = <<ArrayBase<T, 0, true> as ArrayStorage<T>>::StorageVariant as NdArrayBase>::Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.inner.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut DynArray<T> {
    type Item = &'a mut T;
    type IntoIter = <<ArrayBase<T, 0, true> as ArrayStorage<T>>::StorageVariant as NdArrayBase>::IterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.inner.iter_mut()
    }
}

impl<T> NdArray for DynArray<T> {
    type Value = T;
    type Scalar = <<T as DesugaredArrayType>::Desugared as ArrayScalarType>::Scalar;
}

#[cfg(test)]
mod tests {
    use core::any::type_name;
    use crate::core::{Array, DynArray, NdArray};

    #[test]
    fn dyn_array_new() {
        let a: DynArray<u32> = DynArray::new();
        for e in &a {
            assert_eq!(e, &0);
        }
    }

    #[test]
    fn dyn_array_for_iter() {
        let mut a = DynArray::<f32>::new();
        for e in &mut a {
            *e = 1.0;
        }

        for e in &a {
            assert_eq!(e, &1.0);
        }
    }

    #[test]
    fn dyn_array_scalar_type() {
        assert_eq!(type_name::<<DynArray<u32> as NdArray>::Scalar>(),
                   type_name::<<DynArray<Array<u32, 4>> as NdArray>::Scalar>());
        assert_ne!(type_name::<<DynArray<Array<f32, 4>> as NdArray>::Scalar>(),
                   type_name::<<DynArray<Array<u32, 4>> as NdArray>::Scalar>());
    }
}

use crate::core::traits::NdArrayBase;
use std::marker::PhantomData;
use std::ops::{Add, Index, IndexMut};
use std::process::Output;
use crate::core::{NdArrayBaseOps, ScalarType};

pub struct DynamicStorage<T> {
    data: Vec<T>,
    _marker: PhantomData<T>,
}

impl<T> ScalarType for DynamicStorage<T> {
    type Scalar = <T as ScalarType>::Scalar;
}

impl<T> Index<usize> for DynamicStorage<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for DynamicStorage<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Add<Output = T>> NdArrayBase for DynamicStorage<T> {
    type Value = T;
    type Scalar = <Self as ScalarType>::Scalar;
    type Mask = ();
    type Iter<'a>
    where
        T: 'a,
    = core::slice::Iter<'a, T>;
    type IterMut<'a>
    where
        T: 'a,
    = core::slice::IterMut<'a, T>;
    type IntoIter = std::vec::IntoIter<T>;

    fn new() -> Self {
        Self {
            data: vec![],
            _marker: Default::default(),
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.data.iter()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.data.iter_mut()
    }

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }

    fn add(self, other: Self) -> Self where Self::Value: Copy + Add<Output=Self::Value> {
        Self {
            data: self.data.into_iter().zip(other.into_iter()).map(|(a, b)| {
                a + b
            }).collect(),
            _marker: Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{DynamicStorage, NdArrayBase};

    #[test]
    fn dynamic_storage_add() {
        let mut a: DynamicStorage<f32> = DynamicStorage::new();
        let mut b: DynamicStorage<f32> = DynamicStorage::new();
    }
}
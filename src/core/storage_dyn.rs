use crate::core::traits::NdArrayBase;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

pub struct DynamicStorage<T> {
    data: Vec<T>,
    _marker: PhantomData<T>,
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

impl<T> NdArrayBase for DynamicStorage<T> {
    type Value = T;
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

    fn iter(&self) -> Self::Iter<'_> {
        self.data.iter()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.data.iter_mut()
    }

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

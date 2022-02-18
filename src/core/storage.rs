use crate::core::traits::NdArrayBase;
use core::marker::PhantomData;
use std::ops::{Index, IndexMut};

pub struct StaticStorage<T, const N: usize> {
    data: [T; N],
    _marker: PhantomData<T>,
}

impl<T, const N: usize> Index<usize> for StaticStorage<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for StaticStorage<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T, const N: usize> NdArrayBase for StaticStorage<T, N> {
    type Value = T;
    type Iter<'a>
    where
        T: 'a,
    = core::slice::Iter<'a, T>;
    type IterMut<'a>
    where
        T: 'a,
    = core::slice::IterMut<'a, T>;
    type IntoIter = core::array::IntoIter<Self::Value, N>;

    fn new() -> Self {
        Self {
            data: unsafe { ::core::mem::zeroed::<[T; N]>() },
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

pub struct UnreachableStorage<T>(PhantomData<T>);

impl<T> Index<usize> for UnreachableStorage<T> {
    type Output = T;

    fn index(&self, _index: usize) -> &Self::Output {
        unreachable!();
    }
}

impl<T> IndexMut<usize> for UnreachableStorage<T> {
    fn index_mut(&mut self, _index: usize) -> &mut Self::Output {
        unreachable!();
    }
}

impl<T> NdArrayBase for UnreachableStorage<T> {
    type Value = T;
    type Iter<'a>
    where
        T: 'a,
    = core::slice::Iter<'a, T>;
    type IterMut<'a>
    where
        T: 'a,
    = core::slice::IterMut<'a, T>;
    type IntoIter = core::array::IntoIter<T, 0>;

    fn new() -> Self {
        unreachable!();
    }

    fn iter(&self) -> Self::Iter<'_> {
        unreachable!();
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        unreachable!();
    }

    fn into_iter(self) -> Self::IntoIter {
        unreachable!();
    }
}

use std::ops::{Index, IndexMut};

pub trait ArrayStorage<T> {
    type StorageVariant: NdArrayBase<T>;
}

pub trait NdArrayBase<T>: Index<usize, Output = T> + IndexMut<usize, Output = T> {
    type IntoIter: Iterator<Item = T>;
    type Iter<'a>: Iterator<Item = &'a T>
    where
        Self: 'a,
        T: 'a;
    type IterMut<'a>: Iterator<Item = &'a mut T>
    where
        Self: 'a,
        T: 'a;

    fn new() -> Self;

    fn iter(&self) -> Self::Iter<'_>;

    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    fn into_iter(self) -> Self::IntoIter;
}

pub trait NdArray {
    type Item;
}

pub trait NdArrayDepth {
    const ARRAY_DEPTH: usize;
}

impl<T> NdArrayDepth for T {
    default const ARRAY_DEPTH: usize = 0;
}

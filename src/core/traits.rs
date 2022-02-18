use std::ops::{Index, IndexMut};

pub trait ArrayStorage<T> {
    type StorageVariant: NdArrayBase<ElemType = T>;
}

pub trait NdArrayBase:
    Index<usize, Output = Self::ElemType> + IndexMut<usize, Output = Self::ElemType>
{
    type ElemType;
    type Iter<'a>: Iterator<Item = &'a Self::ElemType>
    where
        Self: 'a,
        Self::ElemType: 'a;
    type IterMut<'a>: Iterator<Item = &'a mut Self::ElemType>
    where
        Self: 'a,
        Self::ElemType: 'a;
    type IntoIter: Iterator<Item = Self::ElemType>;

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

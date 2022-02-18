use std::ops::{Index, IndexMut};

pub trait ArrayStorage<T> {
    type StorageVariant: NdArrayBase<Value= T>;
}

pub trait NdArrayBase:
    Index<usize, Output = Self::Value> + IndexMut<usize, Output = Self::Value>
{
    /// Value data type stored by the array.
    type Value;

    /// Iterator type created by `iter` method.
    type Iter<'a>: Iterator<Item = &'a Self::Value>
    where
        Self: 'a,
        Self::Value: 'a;

    /// Mutable iterator type created by [`iter_mut`] method.
    type IterMut<'a>: Iterator<Item = &'a mut Self::Value>
    where
        Self: 'a,
        Self::Value: 'a;

    /// Consumable iterator type created by [`into_iter`] method.
    type IntoIter: Iterator<Item = Self::Value>;

    fn new() -> Self;

    fn iter(&self) -> Self::Iter<'_>;

    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    fn into_iter(self) -> Self::IntoIter;
}

pub trait NdArray: IntoIterator {
    /// Value data type stored by the array.
    type Value;

    /// Scalar data type all the way at the lowest level.
    type Scalar;
}

pub trait NdArrayDepth {
    const ARRAY_DEPTH: usize;
}

impl<T> NdArrayDepth for T {
    default const ARRAY_DEPTH: usize = 0;
}


/// Get the underlying value type of the reference.
pub trait Decay {
    type Type;
}

impl<T> Decay for T {
    default type Type = T;
}

impl<'a, T> Decay for &'a T {
    type Type = <T as Decay>::Type;
}

impl<'a, T> Decay for &'a mut T {
    type Type = <T as Decay>::Type;
}

/// Get the scalar type all the way at the lowest level.
pub trait ArrayScalarType {
    type Scalar;
}

impl<T> ArrayScalarType for T {
    default type Scalar = <T as Decay>::Type;
}

/// Get the array type (ArrayBase) encapsulated inside of tuple struct.
pub trait DesugaredArrayType {
    type Desugared;
}

impl<T> DesugaredArrayType for T {
    default type Desugared = T;
}
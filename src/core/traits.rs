use std::ops::{Add, Index, IndexMut};

pub trait ArrayStorage<T> {
    type StorageVariant: NdArrayBase<Value = T>;
}

pub trait NdArrayAdd<Rhs = Self> {
    type Value;
    type Output;

    fn add(self, other: Rhs) -> Self where Self::Value: Copy + Add<Output = Self::Value>;
    // fn add_ref(&self, other: &Self) -> Self where Self::Value: Copy + Add<Output = Self::Value>;
    // fn add_val_ref(self, other: Self) -> Self where Self::Value: Copy + Add<Output = Self::Value>;
    // fn add_ref_val(&self, other: Self) -> Self where Self::Value: Copy + Add<Output = Self::Value>;
}

pub trait NdArrayBaseOps {
    // fn splat(s: Value) -> Self;

    // fn cmp_eq(self, other: Self) -> Self::Mask;
    // fn cmp_ne(self, other: Self) -> Self::Mask;
    // fn cmp_gt(self, other: Self) -> Self::Mask;
    // fn cmp_ge(self, other: Self) -> Self::Mask;
    // fn cmp_lt(self, other: Self) -> Self::Mask;
    // fn cmp_le(self, other: Self) -> Self::Mask;

    // fn add(self, other: Self) -> Self where ;
    // fn add_ref(&self, other: &Self) -> Self;
    // fn add_ref_val(&self, other: Self) -> Self;

    // fn add_ref_ref(&self, other: &Self) -> Self;
    // fn add_ref_val(&self, other: Self) -> Self;
    // fn add_val_ref(self, other: &Self) -> Self;

    // fn sub(self, other: Self) -> Self;
    // fn div(self, other: Self) -> Self;
    // fn mul(self, other: Self) -> Self;
    // fn mul_add(self, a: Self, b: Self) -> Self;
    // fn rem(self, rhs: Self) -> Self;
    //
    // fn add_value(self, other: Self::Value) -> Self;
    // fn sub_value(self, other: Self::Value) -> Self;
    // fn mul_value(self, other: Self::Value) -> Self;
    // fn div_value(self, other: Self::Value) -> Self;
    // fn rem_value(self, other: Self::Value) -> Self;
    //
    // fn add_scalar(self, other: Self::Scalar) -> Self;
    // fn sub_scalar(self, other: Self::Scalar) -> Self;
    // fn mul_scalar(self, other: Self::Scalar) -> Self;
    // fn div_scalar(self, other: Self::Scalar) -> Self;
    // fn rem_scalar(self, other: Self::Scalar) -> Self;
    //
    // fn min(self, other: Self) -> Self;
    // fn max(self, other: Self) -> Self;
}

pub trait NdArrayBase:
    Index<usize, Output = Self::Value> + IndexMut<usize, Output = Self::Value>
{
    /// Value data type stored by the array.
    type Value;

    /// Scalar data type all the way at the lowest level.
    type Scalar;

    /// Mask vector type.
    type Mask;

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

    fn len(&self) -> usize;

    fn iter(&self) -> Self::Iter<'_>;

    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    fn into_iter(self) -> Self::IntoIter;

    fn add(self, other: Self) -> Self where Self::Value: Copy + Add<Output = Self::Value>;
    // fn add_ref(&self, other: &Self) -> Self where Self::Value: Copy + Add<Output = Self::Value>;
    // fn add_val_ref(self, other: Self) -> Self where Self::Value: Copy + Add<Output = Self::Value>;
    // fn add_ref_val(&self, other: Self) -> Self where Self::Value: Copy + Add<Output = Self::Value>;
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
pub trait ScalarType {
    type Scalar;
}

impl<T> ScalarType for T {
    default type Scalar = <T as Decay>::Type;
}

/// Get the array type (ArrayBase) encapsulated inside of tuple struct.
pub trait DesugaredArrayType {
    type Desugared;
}

impl<T> DesugaredArrayType for T {
    default type Desugared = T;
}

pub mod num;

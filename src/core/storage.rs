use crate::core::num::{Num, Zero};
use crate::core::traits::NdArrayBase;
use crate::core::{NdArrayBaseOps, ScalarType};
use core::marker::PhantomData;
use std::cmp::Ordering;
use std::ops::{Add, Index, IndexMut};

pub struct StaticStorage<T, const N: usize> {
    data: [T; N],
    _marker: PhantomData<T>,
}

impl<T: Copy, const N: usize> Clone for StaticStorage<T, N> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            _marker: Default::default()
        }
    }
}

impl<T: Copy, const N: usize> Copy for StaticStorage<T, N> { }

impl<T, const N: usize> ScalarType for StaticStorage<T, N> {
    type Scalar = <T as ScalarType>::Scalar;
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

impl<T: Add<Output = T>, const N: usize> NdArrayBase for StaticStorage<T, N> {
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
    type IntoIter = core::array::IntoIter<Self::Value, N>;

    fn new() -> Self {
        Self {
            data: unsafe { ::core::mem::zeroed::<[T; N]>() },
            _marker: Default::default(),
        }
    }

    fn len(&self) -> usize {
        N
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
        let mut data = unsafe { core::mem::zeroed::<[Self::Value; N]>() };
        for i in 0..N {
            data[i] = self.data[i] + other.data[i];
        }
        Self {
            data,
            _marker: Default::default()
        }
    }
}

// // val + val
// impl<T: Add<Output = T>, const N: usize> Add for StaticStorage<T, N> {
//     type Output = StaticStorage<T, N>;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         <Self as NdArrayBaseOps>::add(&self, &rhs)
//     }
// }
//
// // val + ref
// impl<'a, T: Add<Output = T>, const N: usize> Add<&'a StaticStorage<T, N>> for StaticStorage<T, N> {
//     type Output = StaticStorage<T, N>;
//
//     fn add(self, rhs: &'a StaticStorage<T, N>) -> Self::Output {
//         <Self as NdArrayBaseOps>::add(&self, &rhs)
//     }
// }
//
// // ref + ref
// impl<'a, T: Add<Output = T>, const N: usize> Add for &'a StaticStorage<T, N> {
//     type Output = StaticStorage<T, N>;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         <Self as NdArrayBaseOps>::add(&self, &rhs)
//     }
// }
//
// // ref + val
// impl<'a, T: Add<Output = T>, const N: usize> Add<StaticStorage<T, N>> for &'a StaticStorage<T, N> {
//     type Output = StaticStorage<T, N>;
//
//     fn add(self, rhs: StaticStorage<T, N>) -> Self::Output {
//         <Self as NdArrayBaseOps>::add(&self, rhs)
//     }
// }

/// Default unreachable storage.
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
    type Scalar = T;
    type Mask = UnreachableStorage<T>;
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

    fn len(&self) -> usize {
        0
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

    fn add(self, other: Self) -> Self where Self::Value: Copy + Add<Output=Self::Value> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;
    use crate::core::{NdArrayBase, StaticStorage};

    #[test]
    fn static_storage_add() {
        let mut a: StaticStorage<u32, 10> = StaticStorage::new();
        let mut b: StaticStorage<u32, 10> = StaticStorage::new();
        let ref_b = &mut b;
        for i in 0..10 {
            a[i] = (i * i + i % 3) as u32;
            b[i] = (i * 2 + i % 5) as u32;
        }
    }
}
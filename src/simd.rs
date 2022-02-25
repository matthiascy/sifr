#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) mod x86_64;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use x86_64::*;

pub const fn slice_to_array<T: Copy, const N: usize>(slice: &[T]) -> [T; N] {
    assert!(
        slice.len() >= N,
        "slice length must be at least the number of lanes"
    );
    let mut array = [slice[0]; N];

    let mut i = 0;
    while i < N {
        array[i] = slice[i];
        i += 1;
    }
    array
}

pub const fn slice_to_array_unchecked<T: Copy, const N: usize>(slice: &[T]) -> [T; N] {
    let mut array = [slice[0]; N];
    if slice.len() >= N {
        let mut i = 0;
        while i < N {
            array[i] = slice[i];
            i += 1;
        }
    } else {
        let mut i = 0;
        while i < slice.len() {
            array[i] = slice[i];
            i += 1;
        }
    }
    array
}
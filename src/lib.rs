#![feature(specialization)]
#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]
#![feature(decl_macro)]
#![feature(stdsimd)]
#![feature(type_name_of_val)]

mod core;
mod simd;

pub mod prelude {
    use crate::core::*;
}

#[cfg(test)]
mod tests {
    use crate::core::ArrayBase;
    use std::ops::{Deref, Index};

    #[test]
    fn it_works() {
        let a: ArrayBase<f32, 32, false> = ArrayBase::new();

        // println!("{}", std::any::type_name::<<<ArrayStorage<f64, 6_usize> as ArrayStorageVariant>::Variant as Index<usize>>::Output>());
        println!("{}", a.inner[10]);
    }
}

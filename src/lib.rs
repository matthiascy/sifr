#![feature(const_maybe_uninit_zeroed)]
#![feature(specialization)]
#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]
#![feature(decl_macro)]
#![feature(stdsimd)]
#![feature(type_name_of_val)]
#![feature(const_fn_trait_bound)]

mod core;
mod simd;

pub mod prelude {
    use crate::core::*;
}

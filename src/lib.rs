#![feature(specialization)]
#![feature(associated_type_defaults)]
#![feature(decl_macro)]
#![feature(stdsimd)]

mod core;
mod simd;

pub mod prelude {
    use crate::core::*;
}

#[cfg(test)]
mod tests {
    use crate::core::Array;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        let _a: Array<f32, 10> = Array::new();
        let _a: Array<f32, 9> = Array::new();
        let _b: Array<f32, 8> = Array::new();
        let _c: Array<f64, 7> = Array::new();
        let _c: Array<f64, 6> = Array::new();
    }
}

#[macro_export]
macro_rules! impl_nd_array_base {
    ($($t:ty),+ $(,)?) => {
        $(
            impl $crate::core::NdArrayBase for $t {
                fn new() -> Self {
                    println!("{}", stringify!($t));
                    Self {}
                }
            }
        )+
    };
}

macro_rules! impl_array_storage_variant_simd {
    ($t:ty; $($n:expr),+; $variant:ty) => {
        $(
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            #[cfg(target_feature = "sse4_2")]
            impl ArrayStorageVariant for ArrayStorage<$t, $n> {
                type Variant = $variant;
            }
        )+
    };
}

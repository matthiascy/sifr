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

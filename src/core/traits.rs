pub trait NdArrayBase {
    fn new() -> Self;
}

pub trait NdArray {
}

pub trait NdArrayDepth {
    const ARRAY_DEPTH: usize;
}

impl<T> NdArrayDepth for T {
    default const ARRAY_DEPTH: usize = 0;
}
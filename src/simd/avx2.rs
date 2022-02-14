use crate::impl_nd_array_base;
use std::arch::x86_64;

pub struct F32x8(x86_64::__m256);

pub struct F64x4(x86_64::__m256d);

pub struct I8x32(x86_64::__m256i);

pub struct I16x16(x86_64::__m256bh);

pub struct I32x8(x86_64::__m256i);

pub struct I64x4(x86_64::__m256i);

impl_nd_array_base!(F32x8, F64x4, I8x32, I16x16, I32x8, I64x4);
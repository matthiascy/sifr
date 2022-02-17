use crate::impl_nd_array_base;
use std::arch::x86_64;

pub struct F32x16(x86_64::__m512);

pub struct F64x8(x86_64::__m512d);

pub struct I8x64(x86_64::__m512i);

pub struct I16x32(x86_64::__m512bh);

pub struct I32x16(x86_64::__m512i);

pub struct I64x8(x86_64::__m512i);

// impl_nd_array_base!(F32x16, F64x8, I8x64, I16x32, I32x16, I64x8);

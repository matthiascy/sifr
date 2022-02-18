use crate::impl_nd_array_base;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

pub struct F32x4(__m128);

pub struct F64x2(__m128d);

pub struct I8x16(__m128i);

pub struct I16x8(__m128bh);

pub struct I32x4(__m128i);

pub struct I64x2(__m128i);

// impl_nd_array_base!(F32x4, F64x2, I8x16, I32x4, I64x2);

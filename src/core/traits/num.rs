use core::num::{FpCategory, Wrapping};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

/// Definition of additive identity.
/// ```text
/// x + 0 = x for all x ∈ 𝑭ⁿ
/// ```
pub trait Zero: Sized {
    /// Returns additive identity of type `Self`.
    fn zero() -> Self;

    /// Returns `true` if `self` is equal to the additive identity.
    fn is_zero(&self) -> bool;

    /// Sets `self` to the additive identity of type `Self`.
    fn set_to_zero(&mut self) {
        *self = Zero::zero();
    }
}

macro_rules! impl_zero {
    ($($t:ty)*, $val:expr) => {$(
        impl Zero for $t {
            #[inline]
            fn zero() -> $t {
                $val
            }

            #[inline]
            fn is_zero(&self) -> bool
                where Self: PartialEq
            {
                *self == $val
            }
        }
    )*};
}

impl_zero!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize, 0);
impl_zero!(f32 f64, 0.0);

#[cfg(has_i128)]
impl_zero!(u128 i128, 0);

impl<T: Zero> Zero for Wrapping<T> {
    fn zero() -> Self {
        Wrapping(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    fn set_to_zero(&mut self) {
        self.0.set_to_zero();
    }
}

/// Definition of multiplicative identity.
/// ```text
/// x * 1 = x for all x ∈ 𝑭ⁿ
/// ```
pub trait One: Sized {
    /// Returns multiplicative identity of type `Self`.
    fn one() -> Self;

    /// Sets `self` to the multiplicative identity.
    fn set_to_one(&mut self) {
        *self = One::one();
    }

    /// Returns `true` if `self` equals to the multiplicative identity.
    #[inline]
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        *self == Self::one()
    }
}

macro_rules! impl_one {
    ($($t:ty)+, $val:expr) => {$(
        impl One for $t {
            #[inline]
            fn one() -> $t {
                $val
            }

            #[inline]
            fn is_one(&self) -> bool {
                *self == $val
            }
        }
    )+};
}

impl_one!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize, 1);
impl_one!(f32 f64, 1.0);

#[cfg(has_i128)]
impl_one!(u128 i128, 1);

#[inline(always)]
pub fn zero<T: Zero>() -> T {
    Zero::zero()
}

#[inline(always)]
pub fn one<T: One>() -> T {
    One::one()
}

impl<T: One> One for Wrapping<T> {
    fn one() -> Self {
        Wrapping(T::one())
    }

    fn set_to_one(&mut self) {
        self.0.set_to_one();
    }
}

pub trait Num<Rhs = Self, Output = Self>:
    Sized
    + Zero
    + One
    + PartialEq
    + PartialOrd
    + Copy
    + Clone
    + Add<Rhs, Output = Output>
    + AddAssign<Rhs>
    + Sub<Rhs, Output = Output>
    + SubAssign<Rhs>
    + Mul<Rhs, Output = Output>
    + MulAssign<Rhs>
    + Div<Rhs, Output = Output>
    + DivAssign<Rhs>
    + Rem<Rhs, Output = Output>
    + RemAssign<Rhs>
{
    fn negate(self) -> Self;
    fn signum(self) -> Self;
}

macro_rules! impl_num_signed {
    ($($t:ty)*) => {$(
        impl Num for $t {
            #[inline]
            fn negate(self) -> Self {
                -self
            }

            #[inline]
            fn signum(self) -> Self {
                if self == 0 as $t {
                    0 as $t
                } else if self > 0 as $t {
                    1 as $t
                } else {
                    -1 as $t
                }
            }
        }
    )*}
}

macro_rules! impl_num_float {
    ($($t:ty)*) => {$(
        impl Num for $t {
            #[inline]
            fn negate(self) -> Self {
                -self
            }

            #[inline]
            fn signum(self) -> Self {
                if self.is_nan() {
                    <$t>::NAN
                } else if (self - 0.0 as $t).abs() <= <$t>::EPSILON {
                    0.0 as $t
                } else if self > 0.0 as $t {
                    1 as $t
                } else {
                    -1 as $t
                }
            }
        }
    )*}
}

macro_rules! impl_num_unsigned {
    ($($t:ty)*) => {$(
        impl Num for $t {
            #[inline]
            fn negate(self) -> Self {
                panic!("Trying to negate unsigned number!");
            }

            #[inline]
            fn signum(self) -> Self {
                if self == 0 as $t {
                    0 as $t
                } else {
                    1 as $t
                }
            }
        }
    )*}
}

pub trait Fractional<Rhs = Self, Output = Self>: Num<Rhs, Output> {
    fn recip(self) -> Self;
}

macro_rules! impl_fractional {
    ($($t:ty)*) => {$(
        impl Fractional for $t {
            fn recip(self) -> Self {
                1.0 as $t / self
            }
        }
    )*}
}

impl_fractional!(f32 f64);

/// Floating number supports trigonometric and hyperbolic functions and related functions.
pub trait Floating<Rhs = Self, Output = Self>: Fractional {
    fn nan() -> Self;
    fn inf() -> Self;
    fn inf_neg() -> Self;
    fn pi() -> Self;
    fn epsilon() -> Self;
    fn e() -> Self;
    fn frac_1_pi() -> Self;
    fn frac_1_sqrt_2() -> Self;
    fn frac_2_pi() -> Self;
    fn frac_2_sqrt_pi() -> Self;
    fn frac_pi_2() -> Self;
    fn frac_pi_3() -> Self;
    fn frac_pi_4() -> Self;
    fn frac_pi_6() -> Self;
    fn frac_pi_8() -> Self;
    fn ln_2() -> Self;
    fn ln_10() -> Self;
    fn log2_10() -> Self;
    fn log2_e() -> Self;
    fn log10_2() -> Self;
    fn log10_e() -> Self;
    fn sqrt2() -> Self;
    fn tau() -> Self;

    fn is_nan(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_finite(self) -> bool;
    fn is_normal(self) -> bool;
    fn is_sub_normal(self) -> bool;
    fn is_negative_zero(self) -> bool;
    fn is_sign_negative(self) -> bool;
    fn is_sign_positive(self) -> bool;

    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn trunc(self) -> Self;
    fn fract(self) -> Self;
    fn abs(self) -> Self;
    fn signum(self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
    fn classify(self) -> FpCategory;

    fn recip(self) -> Self;

    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    // TODO: fn sin_cos(self) -> (Self, Self);
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;
    fn asinh(self) -> Self;
    fn acosh(self) -> Self;
    fn atanh(self) -> Self;

    fn hypot(self, other: Self) -> Self;

    fn mul_add(self, a: Self, b: Self) -> Self;
    fn div_euclid(self, rhs: Self) -> Self;
    fn rem_euclid(self, rhs: Self) -> Self;

    fn powi(self, n: i32) -> Self;
    fn powf(self, n: Self) -> Self;
    fn sqrt(self) -> Self;
    fn cbrt(self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn ln(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn log2(self) -> Self;
    fn log10(self) -> Self;

    fn exp_m1(self) -> Self;
    fn ln_1p(self) -> Self;
}

macro_rules! forward_floating_constant {
    ($($constant:ident () -> $ret:expr;)+) => {$(
        #[inline]
        fn $constant() -> Self {
            $ret
        }
    )+};
}

macro_rules! forward_floating_method {
    ($method:ident ($($args:ident : $argt:ty),*) -> $t:ty) => {
        fn $method(self, $($args: $argt),*) -> $t {
            self.$method($($args),*)
        }
    };
}

macro_rules! impl_floating {
    ($($t:ty, $p:path),*) => {$(
        impl Floating for $t {
            forward_floating_constant! {
                nan() -> <$t>::NAN;
                inf() -> <$t>::INFINITY;
                inf_neg() -> <$t>::NEG_INFINITY;
                epsilon() -> <$t>::EPSILON;
                pi() -> { use $p as base; base::PI };
                e() -> { use $p as base; base::E };
                frac_1_pi() -> { use $p as base; base::FRAC_1_PI };
                frac_1_sqrt_2() -> { use $p as base; base::FRAC_1_SQRT_2 };
                frac_2_pi() -> { use $p as base; base::FRAC_2_PI };
                frac_2_sqrt_pi() -> {use $p as base; base::FRAC_2_SQRT_PI };
                frac_pi_2() -> { use $p as base; base::FRAC_PI_2 };
                frac_pi_3() -> { use $p as base; base::FRAC_PI_3 };
                frac_pi_4() -> { use $p as base; base::FRAC_PI_4 };
                frac_pi_6() -> { use $p as base; base::FRAC_PI_6 };
                frac_pi_8() -> { use $p as base; base::FRAC_PI_8 };
                ln_2() -> { use $p as base; base::LN_2 };
                ln_10() -> { use $p as base; base::LN_10 };
                log2_10() -> { use $p as base; base::LOG2_10 };
                log2_e() -> { use $p as base; base::LOG2_E };
                log10_2() -> { use $p as base; base::LOG10_2 };
                log10_e() -> { use $p as base; base::LOG10_E };
                sqrt2() -> { use $p as base; base::SQRT_2 };
                tau() -> { use $p as base; base::TAU };
           }

            forward_floating_method!(is_nan() -> bool);
            forward_floating_method!(is_infinite() -> bool);
            forward_floating_method!(is_finite() -> bool);
            forward_floating_method!(is_normal() -> bool);
            forward_floating_method!(is_sign_negative() -> bool);
            forward_floating_method!(is_sign_positive() -> bool);
            forward_floating_method!(floor() -> $t);
            forward_floating_method!(ceil() -> $t);
            forward_floating_method!(round() -> $t);
            forward_floating_method!(trunc() -> $t);
            forward_floating_method!(fract() -> $t);
            forward_floating_method!(abs() -> $t);
            forward_floating_method!(signum() -> $t);
            forward_floating_method!(clamp(min: $t, max: $t) -> $t);
            forward_floating_method!(classify() -> FpCategory);
            forward_floating_method!(sin() -> $t);
            forward_floating_method!(cos() -> $t);
            forward_floating_method!(tan() -> $t);
            forward_floating_method!(asin() -> $t);
            forward_floating_method!(acos() -> $t);
            forward_floating_method!(atan() -> $t);
            forward_floating_method!(atan2(other: $t) -> $t);
            forward_floating_method!(sinh() -> $t);
            forward_floating_method!(cosh() -> $t);
            forward_floating_method!(tanh() -> $t);
            forward_floating_method!(asinh() -> $t);
            forward_floating_method!(acosh() -> $t);
            forward_floating_method!(atanh() -> $t);
            forward_floating_method!(sqrt() -> $t);
            forward_floating_method!(cbrt() -> $t);
            forward_floating_method!(exp() -> $t);
            forward_floating_method!(exp2() -> $t);
            forward_floating_method!(ln() -> $t);
            forward_floating_method!(log2() -> $t);
            forward_floating_method!(log10() -> $t);
            forward_floating_method!(exp_m1() -> $t);
            forward_floating_method!(ln_1p() -> $t);
            forward_floating_method!(recip() -> $t);

            fn is_sub_normal(self) -> bool {
                self >= 0.0 && self <= Self::MIN_POSITIVE
            }

            fn is_negative_zero(self) -> bool {
                self == -0.0
            }

            forward_floating_method!(hypot(other: $t) -> $t);
            forward_floating_method!(mul_add(a: $t, b: $t) -> $t);
            forward_floating_method!(div_euclid(rhs: $t) -> $t);
            forward_floating_method!(rem_euclid(rhs: $t) -> $t);
            forward_floating_method!(powi(n: i32) -> $t);
            forward_floating_method!(powf(n: $t) -> $t);
            forward_floating_method!(log(base: $t) -> $t);
        }
    )*}
}

impl_floating! { f32, std::f32::consts, f64, std::f64::consts }

impl_num_signed!(i8 i16 i32 i64 isize);
impl_num_unsigned!(u8 u16 u32 u64 usize);
impl_num_float!(f32 f64);

#[cfg(has_i128)]
impl_num_signed! { i128 }

#[cfg(has_i128)]
impl_num_unsigned! { u128 }

#[cfg(test)]
mod tests {
    use super::Num;

    macro_rules! negate_signed_number_tests {
        ($($func: ident $t:ty)*) => {$(
            #[test]
            fn $func() {
                assert_eq!(0 as $t, (123 as $t).negate() + (-123 as $t).negate());
            }
        )*}
    }

    macro_rules! sign_of_signed_number_tests {
        ($($func: ident $t:ty)*) => {$(
            #[test]
            fn $func() {
                assert_eq!(0 as $t, <$t as Num>::signum(0 as $t));
                assert_eq!(1 as $t, <$t as Num>::signum(123 as $t));
                assert_eq!(-1 as $t, <$t as Num>::signum(-123 as $t));
            }
        )*}
    }

    negate_signed_number_tests!(negate_i8 i8 negate_i16 i16 negate_i32 i32 negate_i64 i64 negate_f32 f32 negate_f64 f64 negate_isize isize);
    sign_of_signed_number_tests!(sign_of_i8 i8 sign_of_i16 i16 sign_of_i32 i32 sign_of_i64 i64 sign_of_f32 f32 sign_of_f64 f64 sign_of_isize isize);

    #[test]
    fn sign_of_nan() {
        assert!(<f32 as Num>::signum(f32::NAN).is_nan());
        assert!(<f64 as Num>::signum(f64::NAN).is_nan());
    }

    #[cfg(has_i128)]
    negate_signed_number_tests!(negate_i128 i128);

    #[cfg(has_i128)]
    sign_of_signed_number_tests!(sign_of_i128 i128);

    macro_rules! negate_unsigned_number_tests {
        ($($func: ident $t:ty)*) => {$(
            #[test]
            #[should_panic]
            fn $func() {
                (123 as $t).negate();
            }
        )*}
    }

    macro_rules! sign_of_unsigned_number_tests {
        ($($func: ident $t:ty)*) => {$(
            #[test]
            fn $func() {
                assert_eq!(0 as $t, (0 as $t).signum());
                assert_eq!(1 as $t, (123 as $t).signum());
            }
        )*}
    }

    #[test]
    fn sign_of_zero_float() {
        println!("sign of 0.0 is {}", 0.0_f32.signum());
    }

    negate_unsigned_number_tests!(negate_u8 u8 negate_u16 u16 negate_u32 u32 negate_u64 u64 negate_usize usize);
    sign_of_unsigned_number_tests!(sign_of_u8 u8 sign_of_u16 u16 sign_of_u32 u32 sign_of_u64 u64 sign_of_usize usize);

    #[cfg(has_i128)]
    negate_unsigned_number_tests!(negate_u128 u128);

    #[cfg(has_i128)]
    sign_of_signed_number_tests!(sign_of_u128 u128);
}

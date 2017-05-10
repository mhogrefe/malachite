#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
extern crate gmp_mpfr_sys;
extern crate rand;

pub mod integer {
    use gmp_mpfr_sys::gmp::{self, mpz_t};
    use rand::distributions::{IndependentSample, Range};
    use rand::Rng;
    use std::{i32, u32};
    use std::cmp::Ordering;
    use std::error::Error;
    use std::ffi::CString;
    use std::fmt::{self, Binary, Debug, Display, Formatter, LowerHex, Octal, UpperHex};
    use std::hash::Hash;
    use std::hash::Hasher;
    use std::mem;
    use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign,
                   Div, DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr,
                   ShrAssign, Sub, SubAssign};
    use std::os::raw::{c_char, c_int, c_long, c_ulong};
    use std::slice;
    use std::str::FromStr;
    /// Assigns to a number from another value.
    pub trait Assign<Rhs = Self> {
        /// Peforms the assignement.
        fn assign(&mut self, rhs: Rhs);
    }
    /// Negates the value inside `self`.
    pub trait NegAssign {
        /// Peforms the negation.
        fn neg_assign(&mut self);
    }
    /// Peforms a bitwise complement of the value inside `self`.
    pub trait NotAssign {
        /// Peforms the complement.
        fn not_assign(&mut self);
    }
    /// Subtract and assigns the result to the rhs operand.
    ///
    /// `rhs.sub_from_assign(lhs)` has the same effect as
    /// `rhs = lhs - rhs`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gmp_to_flint_adaptor_lib::integer::{Integer, SubFromAssign};
    /// let mut rhs = Integer::from(10);
    /// rhs.sub_from_assign(100);
    /// // rhs = 100 - 10
    /// assert!(rhs == 90);
    /// ```
    pub trait SubFromAssign<Lhs = Self> {
        /// Peforms the subtraction.
        fn sub_from_assign(&mut self, lhs: Lhs);
    }
    /// Divide and assign the result to the rhs operand.
    ///
    /// `rhs.div_from_assign(lhs)` has the same effect as
    /// `rhs = lhs / rhs`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gmp_to_flint_adaptor_lib::integer::{DivFromAssign, Integer};
    /// let lhs = Integer::from(50);
    /// let mut rhs = Integer::from(5);
    /// rhs.div_from_assign(lhs);
    /// // rhs = 50 / 5
    /// assert!(rhs == 10);
    /// ```
    pub trait DivFromAssign<Lhs = Self> {
        /// Peforms the division.
        fn div_from_assign(&mut self, lhs: Lhs);
    }
    /// Compute the remainder and assign the result to the rhs operand.
    ///
    /// `rhs.rem_from_assign(lhs)` has the same effect as
    /// `rhs = lhs % rhs`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gmp_to_flint_adaptor_lib::integer::{Integer, RemFromAssign};
    /// let lhs = Integer::from(17);
    /// let mut rhs = Integer::from(2);
    /// rhs.rem_from_assign(&lhs);
    /// // rhs = 17 / 2
    /// assert!(rhs == 1);
    /// ```
    pub trait RemFromAssign<Lhs = Self> {
        /// Peforms the remainder operation.
        fn rem_from_assign(&mut self, lhs: Lhs);
    }
    /// Provides the power operation.
    pub trait Pow<Rhs> {
        /// The resulting type after the power operation.
        type Output;
        /// Performs the power operation.
        fn pow(self, rhs: Rhs) -> Self::Output;
    }
    /// Provides the power operation inside `self`.
    pub trait PowAssign<Rhs> {
        /// Peforms the power operation.
        fn pow_assign(&mut self, rhs: Rhs);
    }
    /// An arbitrary-precision integer.
    ///
    /// Standard arithmetic operations, bitwise operations and comparisons
    /// are supported. In standard arithmetic operations such as addition,
    /// you can mix `Integer` and primitive integer types; the result will
    /// be an `Integer`.
    ///
    /// Internally the integer is not stored using two's-complement
    /// representation, however, for bitwise operations and shifts, the
    /// functionality is the same as if the representation was using two's
    /// complement.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gmp_to_flint_adaptor_lib::integer::Integer;
    ///
    /// let mut i = Integer::from(1);
    /// i = i << 1000;
    /// // i is now 1000000... (1000 zeros)
    /// assert!(i.significant_bits() == 1001);
    /// assert!(i.find_one(0) == Some(1000));
    /// i -= 1;
    /// // i is now 111111... (1000 ones)
    /// assert!(i.count_ones() == Some(1000));
    ///
    /// let a = Integer::from(0xf00d);
    /// let all_ones_xor_a = Integer::from(-1) ^ &a;
    /// // a is unchanged as we borrowed it
    /// let complement_a = !a;
    /// // now a has been moved, so this would cause an error:
    /// // assert!(a > 0);
    /// assert!(all_ones_xor_a == complement_a);
    /// assert!(complement_a == -0xf00e);
    /// assert!(format!("{:x}", complement_a) == "-f00e");
    /// ```
    pub struct Integer {
        inner: mpz_t,
    }
    pub struct IntegerU32s<'a> {
        x: &'a Integer,
        i: u32,
        mask: u32,
        length: u32,
    }
    impl<'a> IntegerU32s<'a> {
        pub fn new(x: &'a Integer) -> IntegerU32s {
            IntegerU32s {
                x: x,
                i: 0,
                mask: 1,
                length: x.significant_bits(),
            }
        }
    }
    impl<'a> Iterator for IntegerU32s<'a> {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            let mut val: u32 = 0;
            while self.i < self.length {
                if self.x.get_bit(self.i) {
                    val |= self.mask;
                }
                self.mask <<= 1;
                self.i += 1;
                if self.mask == 0 {
                    val = 0;
                    self.mask = 1;
                    return Some(val);
                }
            }
            if val == 0 { None } else { Some(val) }
        }
    }
    impl Integer {
        /// Converts to an `i32` if the value fits.
        ///
        /// # Examples
        /// ```rust
        /// use gmp_to_flint_adaptor_lib::integer::Integer;
        /// use std::str::FromStr;
        ///
        /// assert_eq!(Integer::from(1000000).to_i32(), Some(1000000));
        /// assert_eq!(Integer::from(-1000000).to_i32(), Some(-1000000));
        /// assert_eq!(Integer::from_str("1000000000000").unwrap().to_i32(), None);
        /// ```
        pub fn to_i32(&self) -> Option<i32> {
            if *self >= i32::MIN && *self <= i32::MAX {
                Some(self.to_i32_wrapping())
            } else {
                None
            }
        }
        /// Converts to an `i32`, wrapping if the value is too large.
        /// # Examples
        /// ```rust
        /// use gmp_to_flint_adaptor_lib::integer::Integer;
        /// use std::str::FromStr;
        ///
        /// assert_eq!(Integer::from(1000000).to_i32_wrapping(), 1000000);
        /// assert_eq!(Integer::from(-1000000).to_i32_wrapping(), -1000000);
        /// assert_eq!(Integer::from_str("1000000000000").unwrap().to_i32_wrapping(), -727379968);
        /// ```
        pub fn to_i32_wrapping(&self) -> i32 {
            self.to_u32_wrapping() as i32
        }
        /// Converts to an `f64`, rounding towards zero.
        pub fn to_f64(&self) -> f64 {
            unsafe { gmp::mpz_get_d(&self.inner) }
        }
        /// Converts to an `f32`, rounding towards zero.
        pub fn to_f32(&self) -> f32 {
            self.to_f64() as f32
        }
        /// Computes the quotient and remainder of `self` divided by
        /// `divisor.
        ///
        /// # Panics
        ///
        /// Panics if `divisor` is zero.
        pub fn div_rem(&mut self, divisor: &mut Integer) {
            if !(divisor.sign() != Ordering::Equal) {
                {
                    ::rt::begin_panic("division by zero", {
                        static _FILE_LINE: (&'static str, u32) = ("src/integer_old.rs", 295u32);
                        &_FILE_LINE
                    })
                }
            };
            unsafe {
                gmp::mpz_tdiv_qr(&mut self.inner,
                                 &mut divisor.inner,
                                 &self.inner,
                                 &divisor.inner)
            };
        }
        /// Divides `self` by `other`. This is much faster than normal
        /// division, but produces correct results only when the division
        /// is exact.
        ///
        /// # Panics
        ///
        /// Panics if `other` is zero.
        pub fn div_exact(&mut self, other: &Integer) -> &mut Integer {
            if !(other.sign() != Ordering::Equal) {
                {
                    ::rt::begin_panic("division by zero", {
                        static _FILE_LINE: (&'static str, u32) = ("src/integer_old.rs", 320u32);
                        &_FILE_LINE
                    })
                }
            };
            unsafe {
                gmp::mpz_divexact(&mut self.inner, &self.inner, &other.inner);
            }
            self
        }
        /// Returns `true` if `self` is divisible by `other`.
        pub fn is_divisible(&self, other: &Integer) -> bool {
            unsafe { gmp::mpz_divisible_p(&self.inner, &other.inner) != 0 }
        }
        /// Returns `true` if `self` is congruent to `c` modulo `d`, that
        /// is, if there exists a `q` such that `self == c + q * d`.
        /// Unlike other division functions, `d` can be zero.
        pub fn is_congruent(&self, c: &Integer, d: &Integer) -> bool {
            unsafe { gmp::mpz_congruent_p(&self.inner, &c.inner, &d.inner) != 0 }
        }
        /// Computes the `n`th root of `self` and truncates the result.
        pub fn root(&mut self, n: u32) -> &mut Integer {
            unsafe {
                gmp::mpz_root(&mut self.inner, &self.inner, n.into());
            }
            self
        }
        /// Computes the `n`th root of `self` and returns the truncated
        /// root and the remainder.  The remainder is `self` minus the
        /// truncated root raised to the power of `n`.
        /// The remainder is stored in `buf`.
        pub fn root_rem(&mut self, buf: &mut Integer, n: u32) {
            unsafe {
                gmp::mpz_rootrem(&mut self.inner, &mut buf.inner, &self.inner, n.into());
            }
        }
        /// Computes the square root of `self` and truncates the result.
        pub fn sqrt(&mut self) -> &mut Integer {
            unsafe {
                gmp::mpz_sqrt(&mut self.inner, &self.inner);
            }
            self
        }
        /// Computes the square root of `self` and returns the truncated
        /// root and the remainder.  The remainder is `self` minus the
        /// truncated root squared.
        /// The remainder is stored in `buf`.
        pub fn sqrt_rem(&mut self, buf: &mut Integer) {
            unsafe {
                gmp::mpz_sqrtrem(&mut self.inner, &mut buf.inner, &self.inner);
            }
        }
        /// Returns `true` if `self` is a perfect power.
        pub fn is_perfect_power(&self) -> bool {
            unsafe { gmp::mpz_perfect_power_p(&self.inner) != 0 }
        }
        /// Returns `true` if `self` is a perfect square.
        pub fn is_perfect_square(&self) -> bool {
            unsafe { gmp::mpz_perfect_square_p(&self.inner) != 0 }
        }
        /// Finds the greatest common divisor. The result is always
        /// positive except when both inputs are zero.
        pub fn gcd(&mut self, other: &Integer) -> &mut Integer {
            unsafe {
                gmp::mpz_gcd(&mut self.inner, &self.inner, &other.inner);
            }
            self
        }
        /// Finds the least common multiple. The result is always positive
        /// except when one or both inputs are zero.
        pub fn lcm(&mut self, other: &Integer) -> &mut Integer {
            unsafe {
                gmp::mpz_lcm(&mut self.inner, &self.inner, &other.inner);
            }
            self
        }
        /// Finds the inverse of `self` modulo `m` if an inverse exists.
        ///
        /// # Panics
        ///
        /// Panics if `m` is zero.
        pub fn invert(&mut self, m: &Integer) -> Option<&mut Integer> {
            if !(m.sign() != Ordering::Equal) {
                {
                    ::rt::begin_panic("division by zero", {
                        static _FILE_LINE: (&'static str, u32) = ("src/integer_old.rs", 409u32);
                        &_FILE_LINE
                    })
                }
            };
            let exists = unsafe { gmp::mpz_invert(&mut self.inner, &self.inner, &m.inner) != 0 };
            if exists { Some(self) } else { None }
        }
        /// Computes the factorial of `n`.
        /// The value of `self` is ignored.
        pub fn set_factorial(&mut self, n: u32) -> &mut Integer {
            unsafe {
                gmp::mpz_fac_ui(&mut self.inner, n.into());
            }
            self
        }
        /// Computes the double factorial of `n`.
        /// The value of `self` is ignored.
        pub fn set_factorial_2(&mut self, n: u32) -> &mut Integer {
            unsafe {
                gmp::mpz_2fac_ui(&mut self.inner, n.into());
            }
            self
        }
        /// Computes the `m`-multi factorial of `n`.
        /// The value of `self` is ignored.
        pub fn set_factorial_m(&mut self, n: u32, m: u32) -> &mut Integer {
            unsafe {
                gmp::mpz_mfac_uiui(&mut self.inner, n.into(), m.into());
            }
            self
        }
        /// Computes the primorial of `n`.
        /// The value of `self` is ignored.
        pub fn set_primorial(&mut self, n: u32) -> &mut Integer {
            unsafe {
                gmp::mpz_primorial_ui(&mut self.inner, n.into());
            }
            self
        }
        /// Computes the binomial coefficient `self` over `k`.
        pub fn binomial(&mut self, k: u32) -> &mut Integer {
            unsafe {
                gmp::mpz_bin_ui(&mut self.inner, &self.inner, k.into());
            }
            self
        }
        /// Computes the binomial coefficient `n` over `k`.
        /// The value of `self` is ignored.
        pub fn set_binomial(&mut self, n: u32, k: u32) -> &mut Integer {
            unsafe {
                gmp::mpz_bin_uiui(&mut self.inner, n.into(), k.into());
            }
            self
        }
        /// Compares the absolute values of `self` and `other`.
        pub fn cmp_abs(&self, other: &Integer) -> Ordering {
            unsafe { gmp::mpz_cmpabs(&self.inner, &other.inner).cmp(&0) }
        }

        /// Returns the number of ones in `self` if the value >= 0.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use gmp_to_flint_adaptor_lib::integer::Integer;
        /// assert!(Integer::from(0).count_ones() == Some(0));
        /// assert!(Integer::from(15).count_ones() == Some(4));
        /// assert!(Integer::from(-1).count_ones() == None);
        /// ```
        pub fn count_ones(&self) -> Option<u32> {
            bitcount_to_u32(unsafe { gmp::mpz_popcount(&self.inner) })
        }
        /// Retuns the Hamming distance between `self` and `other` if they
        /// have the same sign.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use gmp_to_flint_adaptor_lib::integer::Integer;
        /// let i = Integer::from(-1);
        /// assert!(Integer::from(0).ham_dist(&i) == None);
        /// assert!(Integer::from(-1).ham_dist(&i) == Some(0));
        /// assert!(Integer::from(-13).ham_dist(&i) == Some(2));
        /// ```
        pub fn ham_dist(&self, other: &Integer) -> Option<u32> {
            bitcount_to_u32(unsafe { gmp::mpz_hamdist(&self.inner, &other.inner) })
        }
        /// Returns the location of the first zero, starting at `start`.
        /// If the bit at location `start` is zero, returns `start`.
        ///
        /// ```rust
        /// use gmp_to_flint_adaptor_lib::integer::Integer;
        /// assert!(Integer::from(-2).find_zero(0) == Some(0));
        /// assert!(Integer::from(-2).find_zero(1) == None);
        /// assert!(Integer::from(15).find_zero(0) == Some(4));
        /// assert!(Integer::from(15).find_zero(20) == Some(20));
        pub fn find_zero(&self, start: u32) -> Option<u32> {
            bitcount_to_u32(unsafe { gmp::mpz_scan0(&self.inner, start.into()) })
        }
        /// Returns the location of the first one, starting at `start`.
        /// If the bit at location `start` is one, returns `start`.
        ///
        /// ```rust
        /// use gmp_to_flint_adaptor_lib::integer::Integer;
        /// assert!(Integer::from(1).find_one(0) == Some(0));
        /// assert!(Integer::from(1).find_one(1) == None);
        /// assert!(Integer::from(-16).find_one(0) == Some(4));
        /// assert!(Integer::from(-16).find_one(20) == Some(20));
        pub fn find_one(&self, start: u32) -> Option<u32> {
            bitcount_to_u32(unsafe { gmp::mpz_scan1(&self.inner, start.into()) })
        }
        /// Sets the bit at location `index` to 1 if `val` is `true` or 0
        /// if `val` is `false`.
        pub fn set_bit(&mut self, index: u32, val: bool) -> &mut Integer {
            unsafe {
                if val {
                    gmp::mpz_setbit(&mut self.inner, index.into());
                } else {
                    gmp::mpz_clrbit(&mut self.inner, index.into());
                }
            }
            self
        }
        /// Toggles the bit at location `index`.
        pub fn invert_bit(&mut self, index: u32) -> &mut Integer {
            unsafe {
                gmp::mpz_combit(&mut self.inner, index.into());
            }
            self
        }
        pub fn to_u32s<'a>(&'a self) -> IntegerU32s<'a> {
            IntegerU32s::new(self)
        }
        pub fn assign_bits_unsigned(&mut self, bits: &[bool]) {
            self.assign(0);
            if bits.is_empty() {
                return;
            }
            let bit_length = bits.len();
            let limb_bits = gmp::LIMB_BITS as usize;
            let whole_limbs = bit_length / limb_bits;
            let extra_bits = bit_length % limb_bits;
            let total_limbs = whole_limbs + (((extra_bits + limb_bits - 1) / limb_bits) as usize);
            let limbs = unsafe {
                if (self.inner.alloc as usize) < total_limbs {
                    gmp::_mpz_realloc(&mut self.inner, total_limbs as c_long);
                }
                slice::from_raw_parts_mut(self.inner.d, total_limbs)
            };
            let mut limbs_used: c_int = 0;
            let mut j = 0;
            let mut mask = 1;
            for (i, limb) in limbs.iter_mut().enumerate() {
                let mut val: gmp::limb_t = 0;
                while j < bit_length && mask != 0 {
                    if bits[j] {
                        val |= mask;
                    }
                    j += 1;
                    mask <<= 1;
                }
                if val != 0 {
                    limbs_used = (i as c_int) + 1;
                }
                *limb = val;
                if j == bit_length {
                    break;
                }
            }
            self.inner.size = limbs_used;
        }
        /// Generates a random number with a specified number of bits.
        ///
        /// # Examples
        ///
        /// ```rust
        /// extern crate gmp_to_flint_adaptor_lib;
        /// extern crate rand;
        /// use gmp_to_flint_adaptor_lib::integer::Integer;
        ///
        /// fn main() {
        ///     let mut rng = rand::thread_rng();
        ///     let mut i = Integer::new();
        ///     i.assign_random_bits_unsigned(0, &mut rng);
        ///     assert!(i == 0);
        ///     i.assign_random_bits_unsigned(80, &mut rng);
        ///     assert_eq!(i.significant_bits(), 80);
        /// }
        /// ```
        pub fn assign_random_bits_unsigned<R: Rng>(&mut self, bits: u32, rng: &mut R) {
            self.assign(0);
            if bits == 0 {
                return;
            }
            let limb_bits = gmp::LIMB_BITS as u32;
            let whole_limbs = (bits / limb_bits) as usize;
            let extra_bits = bits % limb_bits;
            let total_limbs = whole_limbs + (((extra_bits + limb_bits - 1) / limb_bits) as usize);
            let limbs = unsafe {
                if (self.inner.alloc as usize) < total_limbs {
                    gmp::_mpz_realloc(&mut self.inner, total_limbs as c_long);
                }
                slice::from_raw_parts_mut(self.inner.d, total_limbs)
            };
            let mut limbs_used: c_int = 0;
            for (i, limb) in limbs.iter_mut().enumerate() {
                let mut val: gmp::limb_t = rng.gen();
                if i == whole_limbs {
                    val &= ((1 as gmp::limb_t) << extra_bits) - 1;
                    val |= (1 as gmp::limb_t) << (extra_bits - 1);
                }
                limbs_used = (i as c_int) + 1;
                *limb = val;
            }
            self.inner.size = limbs_used;
        }
        pub fn assign_random_bits<R: Rng>(&mut self, bits: u32, rng: &mut R) {
            self.assign_random_bits_unsigned(bits, rng);
            let sign: bool = rng.gen();
            if !sign {
                self.neg_assign();
            }
        }
        pub fn assign_random_bits_unsigned_variable<R: Rng>(&mut self,
                                                            max_bits: u32,
                                                            rng: &mut R) {
            let bits = Range::new(0, max_bits + 1).ind_sample(rng);
            self.assign_random_bits_unsigned(bits, rng);
        }
        pub fn assign_random_bits_variable<R: Rng>(&mut self, max_bits: u32, rng: &mut R) {
            let bits = Range::new(0, max_bits + 1).ind_sample(rng);
            self.assign_random_bits(bits, rng);
        }
        pub fn assign_random_bits_nonzero_variable<R: Rng>(&mut self, max_bits: u32, rng: &mut R) {
            if max_bits == 0 {
                {
                    ::rt::begin_panic("max_bits must be positive", {
                        static _FILE_LINE: (&'static str, u32) = ("src/integer_old.rs", 702u32);
                        &_FILE_LINE
                    })
                };
            }
            loop {
                self.assign_random_bits_variable(max_bits, rng);
                if self.sign() != Ordering::Equal {
                    break;
                }
            }
        }
        /// Generates a non-negative random number below the given
        /// boundary value.
        ///
        /// # Examples
        ///
        /// ```rust
        /// extern crate gmp_to_flint_adaptor_lib;
        /// extern crate rand;
        /// use gmp_to_flint_adaptor_lib::integer::Integer;
        ///
        /// fn main() {
        ///     let mut rng = rand::thread_rng();
        ///     let bound = Integer::from(15);
        ///     let mut random = bound.clone();
        ///     random.random_below(&mut rng);
        ///     println!("0 <= {} < {}", random, bound);
        ///     assert!(random < bound);
        /// }
        /// ```
        ///
        /// # Panics
        ///
        /// Panics if the boundary value is less than or equal to zero.
        pub fn random_below<R: Rng>(&mut self, rng: &mut R) -> &mut Integer {
            if !(self.sign() == Ordering::Greater) {
                {
                    ::rt::begin_panic("cannot be below zero", {
                        static _FILE_LINE: (&'static str, u32) = ("src/integer_old.rs", 736u32);
                        &_FILE_LINE
                    })
                }
            };
            let bits = self.significant_bits();
            let limb_bits = gmp::LIMB_BITS as u32;
            let whole_limbs = (bits / limb_bits) as usize;
            let extra_bits = bits % limb_bits;
            let total_limbs = whole_limbs + (((extra_bits + limb_bits - 1) / limb_bits) as usize);
            let limbs = unsafe { slice::from_raw_parts_mut(self.inner.d, total_limbs) };
            'restart: loop {
                let mut limbs_used: c_int = 0;
                let mut still_equal = true;
                'next_limb: for i in (0..total_limbs).rev() {
                    let mut val: gmp::limb_t = rng.gen();
                    if i == whole_limbs {
                        val &= ((1 as gmp::limb_t) << extra_bits) - 1;
                    }
                    if limbs_used == 0 && val != 0 {
                        limbs_used = (i as c_int) + 1;
                    }
                    if still_equal {
                        if val > limbs[i] {
                            continue 'restart;
                        }
                        if val == limbs[i] {
                            continue 'next_limb;
                        }
                        still_equal = false;
                    }
                    limbs[i] = val;
                }
                if !still_equal {
                    self.inner.size = limbs_used;
                    return self;
                }
            }
        }
        /// Returns a string representation of `self` for the specified
        /// `radix`.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use gmp_to_flint_adaptor_lib::integer::{Assign, Integer};
        /// let mut i = Integer::new();
        /// assert!(i.to_string_radix(10) == "0");
        /// i.assign(-10);
        /// assert!(i.to_string_radix(16) == "-a");
        /// i.assign(0x1234cdef);
        /// assert!(i.to_string_radix(4) == "102031030313233");
        /// i.assign_str_radix("1234567890aAbBcCdDeEfF", 16).unwrap();
        /// assert!(i.to_string_radix(16) == "1234567890aabbccddeeff");
        /// ```
        ///
        /// # Panics
        ///
        /// Panics if `radix` is less than 2 or greater than 36.
        pub fn to_string_radix(&self, radix: i32) -> String {
            make_string(self, radix, false)
        }
        /// Parses an `Integer`.
        ///
        /// See the [corresponding assignment](#method.assign_str_radix).
        ///
        /// # Panics
        ///
        /// Panics if `radix` is less than 2 or greater than 36.
        pub fn from_str_radix(src: &str, radix: i32) -> Result<Integer, ParseIntegerError> {
            let mut i = Integer::new();
            i.assign_str_radix(src, radix)?;
            Ok(i)
        }
        /// Parses an `Integer` from a string in decimal.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use gmp_to_flint_adaptor_lib::integer::Integer;
        /// let mut i = Integer::new();
        /// i.assign_str("123").unwrap();
        /// assert!(i == 123);
        /// let ret = i.assign_str("bad");
        /// assert!(ret.is_err());
        /// ```
        pub fn assign_str(&mut self, src: &str) -> Result<(), ParseIntegerError> {
            self.assign_str_radix(src, 10)
        }
        /// Parses an `Integer` from a string with the specified radix.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use gmp_to_flint_adaptor_lib::integer::Integer;
        /// let mut i = Integer::new();
        /// i.assign_str_radix("ff", 16).unwrap();
        /// assert!(i == 255);
        /// ```
        ///
        /// # Panics
        ///
        /// Panics if `radix` is less than 2 or greater than 36.
        pub fn assign_str_radix(&mut self, src: &str, radix: i32) -> Result<(), ParseIntegerError> {
            let s = check_str_radix(src, radix)?;
            let c_str = CString::new(s).unwrap();
            let err = unsafe { gmp::mpz_set_str(&mut self.inner, c_str.as_ptr(), radix.into()) };
            if !(err == 0) {
                {
                    ::rt::begin_panic("assertion failed: err == 0", {
                        static _FILE_LINE: (&'static str, u32) = ("src/integer_old.rs", 846u32);
                        &_FILE_LINE
                    })
                }
            };
            Ok(())
        }
        /// Checks if an `Integer` can be parsed.
        ///
        /// If this method does not return an error, neither will any
        /// other function that parses an `Integer`. If this method
        /// returns an error, the other functions will return the same
        /// error.
        ///
        /// # Panics
        ///
        /// Panics if `radix` is less than 2 or greater than 36.
        pub fn valid_str_radix(src: &str, radix: i32) -> Result<(), ParseIntegerError> {
            check_str_radix(src, radix).map(|_| ())
        }
    }
    fn check_str_radix(src: &str, radix: i32) -> Result<&str, ParseIntegerError> {
        use self::ParseIntegerError as Error;
        use self::ParseErrorKind as Kind;
        if !(radix >= 2 && radix <= 36) {
            {
                ::rt::begin_panic("radix out of range", {
                    static _FILE_LINE: (&'static str, u32) = ("src/integer_old.rs", 869u32);
                    &_FILE_LINE
                })
            }
        };
        let (skip_plus, chars) = if src.starts_with('+') {
            (&src[1..], src[1..].chars())
        } else if src.starts_with('-') {
            (src, src[1..].chars())
        } else {
            (src, src.chars())
        };
        let mut got_digit = false;
        for c in chars {
            let digit_value = match c {
                '0'...'9' => (c as i32) - ('0' as i32),
                'a'...'z' => (c as i32) - ('a' as i32) + 10,
                'A'...'Z' => (c as i32) - ('A' as i32) + 10,
                _ => return Err(Error { kind: Kind::InvalidDigit }),
            };
            if digit_value >= radix {
                return Err(Error { kind: Kind::InvalidDigit });
            }
            got_digit = true;
        }
        if !got_digit {
            return Err(Error { kind: Kind::NoDigits });
        }
        Ok(skip_plus)
    }
    impl FromStr for Integer {
        type Err = ParseIntegerError;
        /// Parses an `Integer`.
        ///
        /// See the [corresponding assignment](#method.assign_str).
        fn from_str(src: &str) -> Result<Integer, ParseIntegerError> {
            let mut i = Integer::new();
            i.assign_str(src)?;
            Ok(i)
        }
    }
    impl<'a> From<&'a Integer> for Integer {
        #[doc = r" Constructs an `Integer` from"]
        #[doc = "another `Integer`."]
        fn from(t: &Integer) -> Integer {
            let mut ret = Integer::new();
            ret.assign(t);
            ret
        }
    }
    impl From<i32> for Integer {
        #[doc = r" Constructs an `Integer` from"]
        #[doc = "an `i32`."]
        fn from(t: i32) -> Integer {
            let mut ret = Integer::new();
            ret.assign(t);
            ret
        }
    }
    impl Assign<f64> for Integer {
        /// Assigns from an `f64`, rounding towards zero.
        fn assign(&mut self, val: f64) {
            unsafe {
                gmp::mpz_set_d(&mut self.inner, val);
            }
        }
    }
    impl Assign<f32> for Integer {
        /// Assigns from an `f32`, rounding towards zero.
        fn assign(&mut self, val: f32) {
            self.assign(val as f64);
        }
    }
    impl<'a> Add<&'a Integer> for Integer {
        type Output = Integer;
        fn add(mut self, op: &'a Integer) -> Integer {
            AddAssign::<&'a Integer>::add_assign(&mut self, op);
            self
        }
    }
    impl Add<Integer> for Integer {
        type Output = Integer;
        fn add(self, op: Integer) -> Integer {
            self.add(&op)
        }
    }
    impl<'a> AddAssign<&'a Integer> for Integer {
        fn add_assign(&mut self, op: &'a Integer) {
            unsafe {
                gmp::mpz_add(&mut self.inner, &self.inner, &op.inner);
            }
        }
    }
    impl AddAssign<Integer> for Integer {
        fn add_assign(&mut self, op: Integer) {
            self.add_assign(&op);
        }
    }
    impl<'a> Sub<&'a Integer> for Integer {
        type Output = Integer;
        fn sub(mut self, op: &'a Integer) -> Integer {
            SubAssign::<&'a Integer>::sub_assign(&mut self, op);
            self
        }
    }
    impl Sub<Integer> for Integer {
        type Output = Integer;
        fn sub(self, op: Integer) -> Integer {
            self.sub(&op)
        }
    }
    impl<'a> SubAssign<&'a Integer> for Integer {
        fn sub_assign(&mut self, op: &'a Integer) {
            unsafe {
                gmp::mpz_sub(&mut self.inner, &self.inner, &op.inner);
            }
        }
    }
    impl SubAssign<Integer> for Integer {
        fn sub_assign(&mut self, op: Integer) {
            self.sub_assign(&op);
        }
    }
    impl<'a> SubFromAssign<&'a Integer> for Integer {
        fn sub_from_assign(&mut self, lhs: &'a Integer) {
            unsafe {
                gmp::mpz_sub(&mut self.inner, &lhs.inner, &self.inner);
            }
        }
    }
    impl SubFromAssign<Integer> for Integer {
        fn sub_from_assign(&mut self, lhs: Integer) {
            self.sub_from_assign(&lhs);
        }
    }
    impl<'a> Mul<&'a Integer> for Integer {
        type Output = Integer;
        fn mul(mut self, op: &'a Integer) -> Integer {
            MulAssign::<&'a Integer>::mul_assign(&mut self, op);
            self
        }
    }
    impl Mul<Integer> for Integer {
        type Output = Integer;
        fn mul(self, op: Integer) -> Integer {
            self.mul(&op)
        }
    }
    impl<'a> MulAssign<&'a Integer> for Integer {
        fn mul_assign(&mut self, op: &'a Integer) {
            unsafe {
                gmp::mpz_mul(&mut self.inner, &self.inner, &op.inner);
            }
        }
    }
    impl MulAssign<Integer> for Integer {
        fn mul_assign(&mut self, op: Integer) {
            self.mul_assign(&op);
        }
    }
    impl<'a> Div<&'a Integer> for Integer {
        type Output = Integer;
        fn div(mut self, op: &'a Integer) -> Integer {
            DivAssign::<&'a Integer>::div_assign(&mut self, op);
            self
        }
    }
    impl Div<Integer> for Integer {
        type Output = Integer;
        fn div(self, op: Integer) -> Integer {
            self.div(&op)
        }
    }
    impl<'a> DivAssign<&'a Integer> for Integer {
        fn div_assign(&mut self, op: &'a Integer) {
            unsafe {
                mpz_tdiv_q(&mut self.inner, &self.inner, &op.inner);
            }
        }
    }
    impl DivAssign<Integer> for Integer {
        fn div_assign(&mut self, op: Integer) {
            self.div_assign(&op);
        }
    }
    impl<'a> DivFromAssign<&'a Integer> for Integer {
        fn div_from_assign(&mut self, lhs: &'a Integer) {
            unsafe {
                mpz_tdiv_q(&mut self.inner, &lhs.inner, &self.inner);
            }
        }
    }
    impl DivFromAssign<Integer> for Integer {
        fn div_from_assign(&mut self, lhs: Integer) {
            self.div_from_assign(&lhs);
        }
    }
    impl<'a> Rem<&'a Integer> for Integer {
        type Output = Integer;
        fn rem(mut self, op: &'a Integer) -> Integer {
            RemAssign::<&'a Integer>::rem_assign(&mut self, op);
            self
        }
    }
    impl Rem<Integer> for Integer {
        type Output = Integer;
        fn rem(self, op: Integer) -> Integer {
            self.rem(&op)
        }
    }
    impl<'a> RemAssign<&'a Integer> for Integer {
        fn rem_assign(&mut self, op: &'a Integer) {
            unsafe {
                mpz_tdiv_r(&mut self.inner, &self.inner, &op.inner);
            }
        }
    }
    impl RemAssign<Integer> for Integer {
        fn rem_assign(&mut self, op: Integer) {
            self.rem_assign(&op);
        }
    }
    impl<'a> RemFromAssign<&'a Integer> for Integer {
        fn rem_from_assign(&mut self, lhs: &'a Integer) {
            unsafe {
                mpz_tdiv_r(&mut self.inner, &lhs.inner, &self.inner);
            }
        }
    }
    impl RemFromAssign<Integer> for Integer {
        fn rem_from_assign(&mut self, lhs: Integer) {
            self.rem_from_assign(&lhs);
        }
    }
    impl<'a> BitAnd<&'a Integer> for Integer {
        type Output = Integer;
        fn bitand(mut self, op: &'a Integer) -> Integer {
            BitAndAssign::<&'a Integer>::bitand_assign(&mut self, op);
            self
        }
    }
    impl BitAnd<Integer> for Integer {
        type Output = Integer;
        fn bitand(self, op: Integer) -> Integer {
            self.bitand(&op)
        }
    }
    impl<'a> BitAndAssign<&'a Integer> for Integer {
        fn bitand_assign(&mut self, op: &'a Integer) {
            unsafe {
                gmp::mpz_and(&mut self.inner, &self.inner, &op.inner);
            }
        }
    }
    impl BitAndAssign<Integer> for Integer {
        fn bitand_assign(&mut self, op: Integer) {
            self.bitand_assign(&op);
        }
    }
    impl<'a> BitOr<&'a Integer> for Integer {
        type Output = Integer;
        fn bitor(mut self, op: &'a Integer) -> Integer {
            BitOrAssign::<&'a Integer>::bitor_assign(&mut self, op);
            self
        }
    }
    impl BitOr<Integer> for Integer {
        type Output = Integer;
        fn bitor(self, op: Integer) -> Integer {
            self.bitor(&op)
        }
    }
    impl<'a> BitOrAssign<&'a Integer> for Integer {
        fn bitor_assign(&mut self, op: &'a Integer) {
            unsafe {
                gmp::mpz_ior(&mut self.inner, &self.inner, &op.inner);
            }
        }
    }
    impl BitOrAssign<Integer> for Integer {
        fn bitor_assign(&mut self, op: Integer) {
            self.bitor_assign(&op);
        }
    }
    impl<'a> BitXor<&'a Integer> for Integer {
        type Output = Integer;
        fn bitxor(mut self, op: &'a Integer) -> Integer {
            BitXorAssign::<&'a Integer>::bitxor_assign(&mut self, op);
            self
        }
    }
    impl BitXor<Integer> for Integer {
        type Output = Integer;
        fn bitxor(self, op: Integer) -> Integer {
            self.bitxor(&op)
        }
    }
    impl<'a> BitXorAssign<&'a Integer> for Integer {
        fn bitxor_assign(&mut self, op: &'a Integer) {
            unsafe {
                gmp::mpz_xor(&mut self.inner, &self.inner, &op.inner);
            }
        }
    }
    impl BitXorAssign<Integer> for Integer {
        fn bitxor_assign(&mut self, op: Integer) {
            self.bitxor_assign(&op);
        }
    }
    impl Not for Integer {
        type Output = Integer;
        fn not(mut self) -> Integer {
            self.not_assign();
            self
        }
    }
    impl NotAssign for Integer {
        fn not_assign(&mut self) {
            unsafe {
                gmp::mpz_com(&mut self.inner, &self.inner);
            }
        }
    }
    unsafe fn mpz_tdiv_q(q: *mut mpz_t, n: *const mpz_t, d: *const mpz_t) {
        if !(gmp::mpz_sgn(d) != 0) {
            {
                ::rt::begin_panic("division by zero", {
                    static _FILE_LINE: (&'static str, u32) = ("src/integer_old.rs", 1095u32);
                    &_FILE_LINE
                })
            }
        };
        gmp::mpz_tdiv_q(q, n, d);
    }
    unsafe fn mpz_tdiv_r(q: *mut mpz_t, n: *const mpz_t, d: *const mpz_t) {
        if !(gmp::mpz_sgn(d) != 0) {
            {
                ::rt::begin_panic("division by zero", {
                    static _FILE_LINE: (&'static str, u32) = ("src/integer_old.rs", 1100u32);
                    &_FILE_LINE
                })
            }
        };
        gmp::mpz_tdiv_r(q, n, d);
    }
    impl Add<u32> for Integer {
        type Output = Integer;
        fn add(mut self, op: u32) -> Integer {
            self.add_assign(op);
            self
        }
    }
    impl AddAssign<u32> for Integer {
        fn add_assign(&mut self, op: u32) {
            unsafe {
                gmp::mpz_add_ui(&mut self.inner, &self.inner, op.into());
            }
        }
    }
    impl Add<Integer> for u32 {
        type Output = Integer;
        fn add(self, op: Integer) -> Integer {
            op.add(self)
        }
    }
    impl<'a> Add<&'a Integer> for u32 {
        type Output = Integer;
        fn add(self, op: &'a Integer) -> Integer {
            self.add(op.clone())
        }
    }
    impl Sub<u32> for Integer {
        type Output = Integer;
        fn sub(mut self, op: u32) -> Integer {
            self.sub_assign(op);
            self
        }
    }
    impl SubAssign<u32> for Integer {
        fn sub_assign(&mut self, op: u32) {
            unsafe {
                gmp::mpz_sub_ui(&mut self.inner, &self.inner, op.into());
            }
        }
    }
    impl Sub<Integer> for u32 {
        type Output = Integer;
        fn sub(self, mut op: Integer) -> Integer {
            op.sub_from_assign(self);
            op
        }
    }
    impl<'a> Sub<&'a Integer> for u32 {
        type Output = Integer;
        fn sub(self, op: &'a Integer) -> Integer {
            self.sub(op.clone())
        }
    }
    impl SubFromAssign<u32> for Integer {
        fn sub_from_assign(&mut self, lhs: u32) {
            unsafe {
                gmp::mpz_ui_sub(&mut self.inner, lhs.into(), &self.inner);
            }
        }
    }
    impl Mul<u32> for Integer {
        type Output = Integer;
        fn mul(mut self, op: u32) -> Integer {
            self.mul_assign(op);
            self
        }
    }
    impl MulAssign<u32> for Integer {
        fn mul_assign(&mut self, op: u32) {
            unsafe {
                gmp::mpz_mul_ui(&mut self.inner, &self.inner, op.into());
            }
        }
    }
    impl Mul<Integer> for u32 {
        type Output = Integer;
        fn mul(self, op: Integer) -> Integer {
            op.mul(self)
        }
    }
    impl<'a> Mul<&'a Integer> for u32 {
        type Output = Integer;
        fn mul(self, op: &'a Integer) -> Integer {
            self.mul(op.clone())
        }
    }
    impl Div<u32> for Integer {
        type Output = Integer;
        fn div(mut self, op: u32) -> Integer {
            self.div_assign(op);
            self
        }
    }
    impl DivAssign<u32> for Integer {
        fn div_assign(&mut self, op: u32) {
            unsafe {
                mpz_tdiv_q_ui(&mut self.inner, &self.inner, op.into());
            }
        }
    }
    impl Div<Integer> for u32 {
        type Output = Integer;
        fn div(self, mut op: Integer) -> Integer {
            op.div_from_assign(self);
            op
        }
    }
    impl<'a> Div<&'a Integer> for u32 {
        type Output = Integer;
        fn div(self, op: &'a Integer) -> Integer {
            self.div(op.clone())
        }
    }
    impl DivFromAssign<u32> for Integer {
        fn div_from_assign(&mut self, lhs: u32) {
            unsafe {
                mpz_ui_tdiv_q(&mut self.inner, lhs.into(), &self.inner);
            }
        }
    }
    impl Rem<u32> for Integer {
        type Output = Integer;
        fn rem(mut self, op: u32) -> Integer {
            self.rem_assign(op);
            self
        }
    }
    impl RemAssign<u32> for Integer {
        fn rem_assign(&mut self, op: u32) {
            unsafe {
                mpz_tdiv_r_ui(&mut self.inner, &self.inner, op.into());
            }
        }
    }
    impl Rem<Integer> for u32 {
        type Output = Integer;
        fn rem(self, mut op: Integer) -> Integer {
            op.rem_from_assign(self);
            op
        }
    }
    impl<'a> Rem<&'a Integer> for u32 {
        type Output = Integer;
        fn rem(self, op: &'a Integer) -> Integer {
            self.rem(op.clone())
        }
    }
    impl RemFromAssign<u32> for Integer {
        fn rem_from_assign(&mut self, lhs: u32) {
            unsafe {
                mpz_ui_tdiv_r(&mut self.inner, lhs.into(), &self.inner);
            }
        }
    }
    impl Shl<u32> for Integer {
        type Output = Integer;
        fn shl(mut self, op: u32) -> Integer {
            self.shl_assign(op);
            self
        }
    }
    impl ShlAssign<u32> for Integer {
        fn shl_assign(&mut self, op: u32) {
            unsafe {
                gmp::mpz_mul_2exp(&mut self.inner, &self.inner, op.into());
            }
        }
    }
    impl Shr<u32> for Integer {
        type Output = Integer;
        fn shr(mut self, op: u32) -> Integer {
            self.shr_assign(op);
            self
        }
    }
    impl ShrAssign<u32> for Integer {
        fn shr_assign(&mut self, op: u32) {
            unsafe {
                gmp::mpz_fdiv_q_2exp(&mut self.inner, &self.inner, op.into());
            }
        }
    }
    impl Pow<u32> for Integer {
        type Output = Integer;
        fn pow(mut self, op: u32) -> Integer {
            self.pow_assign(op);
            self
        }
    }
    impl PowAssign<u32> for Integer {
        fn pow_assign(&mut self, op: u32) {
            unsafe {
                gmp::mpz_pow_ui(&mut self.inner, &self.inner, op.into());
            }
        }
    }
    impl BitAnd<u32> for Integer {
        type Output = Integer;
        fn bitand(mut self, op: u32) -> Integer {
            self.bitand_assign(op);
            self
        }
    }
    impl BitAndAssign<u32> for Integer {
        fn bitand_assign(&mut self, op: u32) {
            unsafe {
                bitand_ui(&mut self.inner, &self.inner, op.into());
            }
        }
    }
    impl BitAnd<Integer> for u32 {
        type Output = Integer;
        fn bitand(self, op: Integer) -> Integer {
            op.bitand(self)
        }
    }
    impl<'a> BitAnd<&'a Integer> for u32 {
        type Output = Integer;
        fn bitand(self, op: &'a Integer) -> Integer {
            self.bitand(op.clone())
        }
    }
    impl BitOr<u32> for Integer {
        type Output = Integer;
        fn bitor(mut self, op: u32) -> Integer {
            self.bitor_assign(op);
            self
        }
    }
    impl BitOrAssign<u32> for Integer {
        fn bitor_assign(&mut self, op: u32) {
            unsafe {
                bitor_ui(&mut self.inner, &self.inner, op.into());
            }
        }
    }
    impl BitOr<Integer> for u32 {
        type Output = Integer;
        fn bitor(self, op: Integer) -> Integer {
            op.bitor(self)
        }
    }
    impl<'a> BitOr<&'a Integer> for u32 {
        type Output = Integer;
        fn bitor(self, op: &'a Integer) -> Integer {
            self.bitor(op.clone())
        }
    }
    impl BitXor<u32> for Integer {
        type Output = Integer;
        fn bitxor(mut self, op: u32) -> Integer {
            self.bitxor_assign(op);
            self
        }
    }
    impl BitXorAssign<u32> for Integer {
        fn bitxor_assign(&mut self, op: u32) {
            unsafe {
                bitxor_ui(&mut self.inner, &self.inner, op.into());
            }
        }
    }
    impl BitXor<Integer> for u32 {
        type Output = Integer;
        fn bitxor(self, op: Integer) -> Integer {
            op.bitxor(self)
        }
    }
    impl<'a> BitXor<&'a Integer> for u32 {
        type Output = Integer;
        fn bitxor(self, op: &'a Integer) -> Integer {
            self.bitxor(op.clone())
        }
    }
    impl Add<i32> for Integer {
        type Output = Integer;
        fn add(mut self, op: i32) -> Integer {
            self.add_assign(op);
            self
        }
    }
    impl AddAssign<i32> for Integer {
        fn add_assign(&mut self, op: i32) {
            unsafe {
                mpz_add_si(&mut self.inner, &self.inner, op.into());
            }
        }
    }
    impl Add<Integer> for i32 {
        type Output = Integer;
        fn add(self, op: Integer) -> Integer {
            op.add(self)
        }
    }
    impl<'a> Add<&'a Integer> for i32 {
        type Output = Integer;
        fn add(self, op: &'a Integer) -> Integer {
            self.add(op.clone())
        }
    }
    impl Sub<i32> for Integer {
        type Output = Integer;
        fn sub(mut self, op: i32) -> Integer {
            self.sub_assign(op);
            self
        }
    }
    impl SubAssign<i32> for Integer {
        fn sub_assign(&mut self, op: i32) {
            unsafe {
                mpz_sub_si(&mut self.inner, &self.inner, op.into());
            }
        }
    }
    impl Sub<Integer> for i32 {
        type Output = Integer;
        fn sub(self, mut op: Integer) -> Integer {
            op.sub_from_assign(self);
            op
        }
    }
    impl<'a> Sub<&'a Integer> for i32 {
        type Output = Integer;
        fn sub(self, op: &'a Integer) -> Integer {
            self.sub(op.clone())
        }
    }
    impl SubFromAssign<i32> for Integer {
        fn sub_from_assign(&mut self, lhs: i32) {
            unsafe {
                mpz_si_sub(&mut self.inner, lhs.into(), &self.inner);
            }
        }
    }
    impl Mul<i32> for Integer {
        type Output = Integer;
        fn mul(mut self, op: i32) -> Integer {
            self.mul_assign(op);
            self
        }
    }
    impl MulAssign<i32> for Integer {
        fn mul_assign(&mut self, op: i32) {
            unsafe {
                gmp::mpz_mul_si(&mut self.inner, &self.inner, op.into());
            }
        }
    }
    impl Mul<Integer> for i32 {
        type Output = Integer;
        fn mul(self, op: Integer) -> Integer {
            op.mul(self)
        }
    }
    impl<'a> Mul<&'a Integer> for i32 {
        type Output = Integer;
        fn mul(self, op: &'a Integer) -> Integer {
            self.mul(op.clone())
        }
    }
    impl Div<i32> for Integer {
        type Output = Integer;
        fn div(mut self, op: i32) -> Integer {
            self.div_assign(op);
            self
        }
    }
    impl DivAssign<i32> for Integer {
        fn div_assign(&mut self, op: i32) {
            unsafe {
                mpz_tdiv_q_si(&mut self.inner, &self.inner, op.into());
            }
        }
    }
    impl Div<Integer> for i32 {
        type Output = Integer;
        fn div(self, mut op: Integer) -> Integer {
            op.div_from_assign(self);
            op
        }
    }
    impl<'a> Div<&'a Integer> for i32 {
        type Output = Integer;
        fn div(self, op: &'a Integer) -> Integer {
            self.div(op.clone())
        }
    }
    impl DivFromAssign<i32> for Integer {
        fn div_from_assign(&mut self, lhs: i32) {
            unsafe {
                mpz_si_tdiv_q(&mut self.inner, lhs.into(), &self.inner);
            }
        }
    }
    impl Rem<i32> for Integer {
        type Output = Integer;
        fn rem(mut self, op: i32) -> Integer {
            self.rem_assign(op);
            self
        }
    }
    impl RemAssign<i32> for Integer {
        fn rem_assign(&mut self, op: i32) {
            unsafe {
                mpz_tdiv_r_si(&mut self.inner, &self.inner, op.into());
            }
        }
    }
    impl Rem<Integer> for i32 {
        type Output = Integer;
        fn rem(self, mut op: Integer) -> Integer {
            op.rem_from_assign(self);
            op
        }
    }
    impl<'a> Rem<&'a Integer> for i32 {
        type Output = Integer;
        fn rem(self, op: &'a Integer) -> Integer {
            self.rem(op.clone())
        }
    }
    impl RemFromAssign<i32> for Integer {
        fn rem_from_assign(&mut self, lhs: i32) {
            unsafe {
                mpz_si_tdiv_r(&mut self.inner, lhs.into(), &self.inner);
            }
        }
    }
    impl Shl<i32> for Integer {
        type Output = Integer;
        fn shl(mut self, op: i32) -> Integer {
            self.shl_assign(op);
            self
        }
    }
    impl ShlAssign<i32> for Integer {
        fn shl_assign(&mut self, op: i32) {
            unsafe {
                mpz_lshift_si(&mut self.inner, &self.inner, op.into());
            }
        }
    }
    impl Shr<i32> for Integer {
        type Output = Integer;
        fn shr(mut self, op: i32) -> Integer {
            self.shr_assign(op);
            self
        }
    }
    impl ShrAssign<i32> for Integer {
        fn shr_assign(&mut self, op: i32) {
            unsafe {
                mpz_rshift_si(&mut self.inner, &self.inner, op.into());
            }
        }
    }
    unsafe fn mpz_tdiv_q_ui(q: *mut mpz_t, n: *const mpz_t, d: c_ulong) {
        if !(d != 0) {
            {
                ::rt::begin_panic("division by zero", {
                    static _FILE_LINE: (&'static str, u32) = ("src/integer_old.rs", 1232u32);
                    &_FILE_LINE
                })
            }
        };
        gmp::mpz_tdiv_q_ui(q, n, d);
    }
    unsafe fn mpz_ui_tdiv_q(q: *mut mpz_t, n: c_ulong, d: *const mpz_t) {
        let sgn_d = gmp::mpz_sgn(d);
        if !(sgn_d != 0) {
            {
                ::rt::begin_panic("division by zero", {
                    static _FILE_LINE: (&'static str, u32) = ("src/integer_old.rs", 1238u32);
                    &_FILE_LINE
                })
            }
        };
        if gmp::mpz_cmpabs_ui(d, n) > 0 {
            gmp::mpz_set_ui(q, 0);
        } else {
            let ui = n / gmp::mpz_get_ui(d);
            gmp::mpz_set_ui(q, ui);
            if sgn_d < 0 {
                gmp::mpz_neg(q, q);
            }
        }
    }
    unsafe fn mpz_tdiv_r_ui(q: *mut mpz_t, n: *const mpz_t, d: c_ulong) {
        if !(d != 0) {
            {
                ::rt::begin_panic("division by zero", {
                    static _FILE_LINE: (&'static str, u32) = ("src/integer_old.rs", 1251u32);
                    &_FILE_LINE
                })
            }
        };
        gmp::mpz_tdiv_r_ui(q, n, d);
    }
    unsafe fn mpz_ui_tdiv_r(q: *mut mpz_t, n: c_ulong, d: *const mpz_t) {
        if !(gmp::mpz_sgn(d) != 0) {
            {
                ::rt::begin_panic("division by zero", {
                    static _FILE_LINE: (&'static str, u32) = ("src/integer_old.rs", 1256u32);
                    &_FILE_LINE
                })
            }
        };
        if gmp::mpz_cmpabs_ui(d, n) > 0 {
            gmp::mpz_set_ui(q, n);
        } else {
            let ui = n % gmp::mpz_get_ui(d);
            gmp::mpz_set_ui(q, ui);
        }
    }
    unsafe fn bitand_ui(rop: *mut mpz_t, op1: *const mpz_t, op2: c_ulong) {
        if !(mem::size_of::<c_long>() <= mem::size_of::<gmp::limb_t>()) {
            {
                ::rt::begin_panic("assertion failed: mem::size_of::<c_long>() <= mem::size_of::<gmp::limb_t>()",
                                  {
                                      static _FILE_LINE: (&'static str, u32) = ("src/integer_old.rs", 1266u32);
                                      &_FILE_LINE
                                  })
            }
        };
        let lop2 = op2 as gmp::limb_t;
        match (*op1).size.cmp(&0) {
            Ordering::Equal => {
                (*rop).size = 0;
            }
            Ordering::Greater => {
                *(*rop).d = *(*op1).d & lop2;
                (*rop).size = if *(*rop).d == 0 { 0 } else { 1 }
            }
            Ordering::Less => {
                *(*rop).d = (*(*op1).d).wrapping_neg() & lop2;
                (*rop).size = if *(*rop).d == 0 { 0 } else { 1 }
            }
        }
    }
    unsafe fn bitor_ui(rop: *mut mpz_t, op1: *const mpz_t, op2: c_ulong) {
        if !(mem::size_of::<c_long>() <= mem::size_of::<gmp::limb_t>()) {
            {
                ::rt::begin_panic("assertion failed: mem::size_of::<c_long>() <= mem::size_of::<gmp::limb_t>()",
                                  {
                                      static _FILE_LINE: (&'static str, u32) = ("src/integer_old.rs", 1284u32);
                                      &_FILE_LINE
                                  })
            }
        };
        let lop2 = op2 as gmp::limb_t;
        match (*op1).size.cmp(&0) {
            Ordering::Equal => {
                if op2 == 0 {
                    (*rop).size = 0;
                } else {
                    *(*rop).d = lop2;
                    (*rop).size = 1;
                }
            }
            Ordering::Greater => {
                gmp::mpz_set(rop, op1);
                *(*rop).d |= lop2;
            }
            Ordering::Less => {
                gmp::mpz_com(rop, op1);
                *(*rop).d &= !lop2;
                if (*rop).size == 1 && *(*rop).d == 0 {
                    (*rop).size = 0;
                } else if (*rop).size == 0 && *(*rop).d != 0 {
                    (*rop).size = 1;
                }
                gmp::mpz_com(rop, rop);
            }
        }
    }
    unsafe fn bitxor_ui(rop: *mut mpz_t, op1: *const mpz_t, op2: c_ulong) {
        if !(mem::size_of::<c_long>() <= mem::size_of::<gmp::limb_t>()) {
            {
                ::rt::begin_panic("assertion failed: mem::size_of::<c_long>() <= mem::size_of::<gmp::limb_t>()",
                                  {
                                      static _FILE_LINE: (&'static str, u32) = ("src/integer_old.rs", 1313u32);
                                      &_FILE_LINE
                                  })
            }
        };
        let lop2 = op2 as gmp::limb_t;
        match (*op1).size.cmp(&0) {
            Ordering::Equal => {
                if op2 == 0 {
                    (*rop).size = 0;
                } else {
                    *(*rop).d = lop2;
                    (*rop).size = 1;
                }
            }
            Ordering::Greater => {
                gmp::mpz_set(rop, op1);
                *(*rop).d ^= lop2;
                if (*rop).size == 1 && *(*rop).d == 0 {
                    (*rop).size = 0;
                }
            }
            Ordering::Less => {
                gmp::mpz_com(rop, op1);
                *(*rop).d ^= lop2;
                if (*rop).size == 1 && *(*rop).d == 0 {
                    (*rop).size = 0;
                } else if (*rop).size == 0 && *(*rop).d != 0 {
                    (*rop).size = 1;
                }
                gmp::mpz_com(rop, rop);
            }
        }
    }
    unsafe fn mpz_add_si(rop: *mut mpz_t, op1: *const mpz_t, op2: c_long) {
        if op2 >= 0 {
            gmp::mpz_add_ui(rop, op1, op2 as c_ulong);
        } else {
            gmp::mpz_sub_ui(rop, op1, op2.wrapping_neg() as c_ulong);
        }
    }
    unsafe fn mpz_sub_si(rop: *mut mpz_t, op1: *const mpz_t, op2: c_long) {
        if op2 >= 0 {
            gmp::mpz_sub_ui(rop, op1, op2 as c_ulong);
        } else {
            gmp::mpz_add_ui(rop, op1, op2.wrapping_neg() as c_ulong);
        }
    }
    unsafe fn mpz_si_sub(rop: *mut mpz_t, op1: c_long, op2: *const mpz_t) {
        if op1 >= 0 {
            gmp::mpz_ui_sub(rop, op1 as c_ulong, op2);
        } else {
            gmp::mpz_neg(rop, op2);
            gmp::mpz_sub_ui(rop, rop, op1.wrapping_neg() as c_ulong);
        }
    }
    unsafe fn mpz_tdiv_q_si(q: *mut mpz_t, n: *const mpz_t, d: c_long) {
        let neg = d < 0;
        mpz_tdiv_q_ui(q, n, d.wrapping_abs() as c_ulong);
        if neg {
            gmp::mpz_neg(q, q);
        }
    }
    unsafe fn mpz_si_tdiv_q(q: *mut mpz_t, n: c_long, d: *const mpz_t) {
        let neg = n < 0;
        mpz_ui_tdiv_q(q, n.wrapping_abs() as c_ulong, d);
        if neg {
            gmp::mpz_neg(q, q);
        }
    }
    unsafe fn mpz_tdiv_r_si(q: *mut mpz_t, n: *const mpz_t, d: c_long) {
        mpz_tdiv_r_ui(q, n, d.wrapping_abs() as c_ulong);
    }
    unsafe fn mpz_si_tdiv_r(q: *mut mpz_t, n: c_long, d: *const mpz_t) {
        let neg = n < 0;
        mpz_ui_tdiv_r(q, n.wrapping_abs() as c_ulong, d);
        if neg {
            gmp::mpz_neg(q, q);
        }
    }
    unsafe fn mpz_lshift_si(rop: *mut mpz_t, op1: *const mpz_t, op2: c_long) {
        if op2 >= 0 {
            gmp::mpz_mul_2exp(rop, op1, op2 as c_ulong);
        } else {
            gmp::mpz_fdiv_q_2exp(rop, op1, op2.wrapping_neg() as c_ulong);
        }
    }
    unsafe fn mpz_rshift_si(rop: *mut mpz_t, op1: *const mpz_t, op2: c_long) {
        if op2 >= 0 {
            gmp::mpz_fdiv_q_2exp(rop, op1, op2 as c_ulong);
        } else {
            gmp::mpz_mul_2exp(rop, op1, op2.wrapping_neg() as c_ulong);
        }
    }
    impl PartialOrd<f64> for Integer {
        fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
            if other.is_nan() {
                None
            } else {
                let ord = unsafe { gmp::mpz_cmp_d(&self.inner, *other) };
                Some(ord.cmp(&0))
            }
        }
    }
    impl PartialEq<f64> for Integer {
        fn eq(&self, other: &f64) -> bool {
            self.partial_cmp(other) == Some(Ordering::Equal)
        }
    }
    impl PartialOrd<Integer> for f64 {
        fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
            match other.partial_cmp(self) {
                None => None,
                Some(x) => Some(x.reverse()),
            }
        }
    }
    impl PartialEq<Integer> for f64 {
        fn eq(&self, other: &Integer) -> bool {
            other.eq(self)
        }
    }
    impl PartialOrd<f32> for Integer {
        fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
            let o = *other as f64;
            self.partial_cmp(&o)
        }
    }
    impl PartialEq<f32> for Integer {
        fn eq(&self, other: &f32) -> bool {
            let o = *other as f64;
            self.eq(&o)
        }
    }
    impl PartialEq<Integer> for f32 {
        fn eq(&self, other: &Integer) -> bool {
            other.eq(self)
        }
    }
    impl PartialOrd<Integer> for f32 {
        fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
            match other.partial_cmp(self) {
                None => None,
                Some(x) => Some(x.reverse()),
            }
        }
    }
    impl Hash for Integer {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.sign().hash(state);
            for i in self.to_u32s() {
                i.hash(state);
            }
        }
    }
    fn make_string(i: &Integer, radix: i32, to_upper: bool) -> String {
        if !(radix >= 2 && radix <= 36) {
            {
                ::rt::begin_panic("radix out of range", {
                    static _FILE_LINE: (&'static str, u32) = ("src/integer_old.rs", 1541u32);
                    &_FILE_LINE
                })
            }
        };
        let i_size = unsafe { gmp::mpz_sizeinbase(&i.inner, radix) };
        let size = i_size.checked_add(2).unwrap();
        let mut buf = Vec::<u8>::with_capacity(size);
        let case_radix = if to_upper { -radix } else { radix };
        unsafe {
            buf.set_len(size);
            gmp::mpz_get_str(buf.as_mut_ptr() as *mut c_char,
                             case_radix as c_int,
                             &i.inner);
            let nul_index = buf.iter().position(|&x| x == 0).unwrap();
            buf.set_len(nul_index);
            String::from_utf8_unchecked(buf)
        }
    }
    fn fmt_radix(i: &Integer,
                 f: &mut Formatter,
                 radix: i32,
                 to_upper: bool,
                 prefix: &str)
                 -> fmt::Result {
        let s = make_string(i, radix, to_upper);
        let (neg, buf) = if s.starts_with('-') {
            (true, &s[1..])
        } else {
            (false, &s[..])
        };
        f.pad_integral(!neg, prefix, buf)
    }
    impl Display for Integer {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            fmt_radix(self, f, 10, false, "")
        }
    }
    impl Debug for Integer {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            fmt_radix(self, f, 10, false, "")
        }
    }
    impl Binary for Integer {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            fmt_radix(self, f, 2, false, "0b")
        }
    }
    impl Octal for Integer {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            fmt_radix(self, f, 8, false, "0o")
        }
    }
    impl LowerHex for Integer {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            fmt_radix(self, f, 16, false, "0x")
        }
    }
    impl UpperHex for Integer {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            fmt_radix(self, f, 16, true, "0x")
        }
    }
    /// An error which can be returned when parsing an `Integer`.
    #[structural_match]
    pub struct ParseIntegerError {
        kind: ParseErrorKind,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ParseIntegerError {
        #[inline]
        fn clone(&self) -> ParseIntegerError {
            match *self {
                ParseIntegerError { kind: ref __self_0_0 } => {
                    ParseIntegerError { kind: ::std::clone::Clone::clone(&(*__self_0_0)) }
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ParseIntegerError {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ParseIntegerError { kind: ref __self_0_0 } => {
                    let mut builder = __arg_0.debug_struct("ParseIntegerError");
                    let _ = builder.field("kind", &&(*__self_0_0));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for ParseIntegerError {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<ParseErrorKind>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ParseIntegerError {
        #[inline]
        fn eq(&self, __arg_0: &ParseIntegerError) -> bool {
            match *__arg_0 {
                ParseIntegerError { kind: ref __self_1_0 } => {
                    match *self {
                        ParseIntegerError { kind: ref __self_0_0 } => {
                            true && (*__self_0_0) == (*__self_1_0)
                        }
                    }
                }
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &ParseIntegerError) -> bool {
            match *__arg_0 {
                ParseIntegerError { kind: ref __self_1_0 } => {
                    match *self {
                        ParseIntegerError { kind: ref __self_0_0 } => {
                            false || (*__self_0_0) != (*__self_1_0)
                        }
                    }
                }
            }
        }
    }
    #[structural_match]
    enum ParseErrorKind {
        InvalidDigit,
        NoDigits,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ParseErrorKind {
        #[inline]
        fn clone(&self) -> ParseErrorKind {
            match (&*self,) {
                (&ParseErrorKind::InvalidDigit,) => ParseErrorKind::InvalidDigit,
                (&ParseErrorKind::NoDigits,) => ParseErrorKind::NoDigits,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ParseErrorKind {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&ParseErrorKind::InvalidDigit,) => {
                    let mut builder = __arg_0.debug_tuple("InvalidDigit");
                    builder.finish()
                }
                (&ParseErrorKind::NoDigits,) => {
                    let mut builder = __arg_0.debug_tuple("NoDigits");
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for ParseErrorKind {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ParseErrorKind {
        #[inline]
        fn eq(&self, __arg_0: &ParseErrorKind) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*__arg_0) } as
                                 isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*__arg_0) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    impl Error for ParseIntegerError {
        fn description(&self) -> &str {
            use self::ParseErrorKind::*;
            match self.kind {
                InvalidDigit => "invalid digit found in string",
                NoDigits => "string has no digits",
            }
        }
    }
    impl Display for ParseIntegerError {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            Debug::fmt(self, f)
        }
    }
    fn bitcount_to_u32(bits: gmp::bitcnt_t) -> Option<u32> {
        let max: gmp::bitcnt_t = !0;
        if bits == max {
            None
        } else if bits > (u32::MAX as gmp::bitcnt_t) {
            {
                ::rt::begin_panic("overflow", {
                    static _FILE_LINE: (&'static str, u32) = ("src/integer_old.rs", 1642u32);
                    &_FILE_LINE
                })
            }
        } else {
            Some(bits as u32)
        }
    }
}

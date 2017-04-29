// Forked from rugint

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
use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
               DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign,
               Sub, SubAssign};
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
/// use gmp_to_flint_adaptor_lib::integer_old::{IntegerOld, SubFromAssign};
/// let mut rhs = IntegerOld::from(10);
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
/// use gmp_to_flint_adaptor_lib::integer_old::{DivFromAssign, IntegerOld};
/// let lhs = IntegerOld::from(50);
/// let mut rhs = IntegerOld::from(5);
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
/// use gmp_to_flint_adaptor_lib::integer_old::{IntegerOld, RemFromAssign};
/// let lhs = IntegerOld::from(17);
/// let mut rhs = IntegerOld::from(2);
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
/// use gmp_to_flint_adaptor_lib::integer_old::IntegerOld;
///
/// let mut i = IntegerOld::from(1);
/// i = i << 1000;
/// // i is now 1000000... (1000 zeros)
/// assert!(i.significant_bits() == 1001);
/// assert!(i.find_one(0) == Some(1000));
/// i -= 1;
/// // i is now 111111... (1000 ones)
/// assert!(i.count_ones() == Some(1000));
///
/// let a = IntegerOld::from(0xf00d);
/// let all_ones_xor_a = IntegerOld::from(-1) ^ &a;
/// // a is unchanged as we borrowed it
/// let complement_a = !a;
/// // now a has been moved, so this would cause an error:
/// // assert!(a > 0);
/// assert!(all_ones_xor_a == complement_a);
/// assert!(complement_a == -0xf00e);
/// assert!(format!("{:x}", complement_a) == "-f00e");
/// ```
pub struct IntegerOld {
    inner: mpz_t,
}

impl Drop for IntegerOld {
    fn drop(&mut self) {
        unsafe {
            gmp::mpz_clear(&mut self.inner);
        }
    }
}

impl Default for IntegerOld {
    fn default() -> IntegerOld {
        IntegerOld::new()
    }
}

impl Clone for IntegerOld {
    fn clone(&self) -> IntegerOld {
        let mut ret = IntegerOld::new();
        ret.assign(self);
        ret
    }

    fn clone_from(&mut self, source: &IntegerOld) {
        self.assign(source);
    }
}

pub struct IntegerContent<'a> {
    x: &'a IntegerOld,
    i: u32,
    mask: u32,
    length: u32,
}

impl<'a> IntegerContent<'a> {
    pub fn new(x: &'a IntegerOld) -> IntegerContent {
        IntegerContent {
            x: x,
            i: 0,
            mask: 1,
            length: x.significant_bits(),
        }
    }
}

impl<'a> Iterator for IntegerContent<'a> {
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

impl IntegerOld {
    /// Constructs a new arbitrary-precision integer with value 0.
    pub fn new() -> IntegerOld {
        unsafe {
            let mut inner: mpz_t = mem::uninitialized();
            gmp::mpz_init(&mut inner);
            IntegerOld { inner: inner }
        }
    }

    /// Converts to a `u32` if the value fits.
    pub fn to_u32(&self) -> Option<u32> {
        if self.sign() != Ordering::Less && *self <= u32::MAX {
            Some(self.to_u32_wrapping())
        } else {
            None
        }
    }

    /// Converts to a `u32`, wrapping if the value is too large.
    pub fn to_u32_wrapping(&self) -> u32 {
        let u = unsafe { gmp::mpz_get_ui(&self.inner) as u32 };
        if self.sign() == Ordering::Less {
            u.wrapping_neg()
        } else {
            u
        }
    }

    /// Converts to an `i32` if the value fits.
    ///
    /// # Examples
    /// ```rust
    /// use gmp_to_flint_adaptor_lib::integer_old::IntegerOld;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(IntegerOld::from(1000000).to_i32(), Some(1000000));
    /// assert_eq!(IntegerOld::from(-1000000).to_i32(), Some(-1000000));
    /// assert_eq!(IntegerOld::from_str("1000000000000").unwrap().to_i32(), None);
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
    /// use gmp_to_flint_adaptor_lib::integer_old::IntegerOld;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(IntegerOld::from(1000000).to_i32_wrapping(), 1000000);
    /// assert_eq!(IntegerOld::from(-1000000).to_i32_wrapping(), -1000000);
    /// assert_eq!(IntegerOld::from_str("1000000000000").unwrap().to_i32_wrapping(), -727379968);
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
    pub fn div_rem(&mut self, divisor: &mut IntegerOld) {
        assert!(divisor.sign() != Ordering::Equal, "division by zero");
        unsafe {
            gmp::mpz_tdiv_qr(&mut self.inner,
                             &mut divisor.inner,
                             &self.inner,
                             &divisor.inner)
        };
    }

    /// Computes the absolute value of `self`.
    pub fn abs(&mut self) -> &mut IntegerOld {
        unsafe {
            gmp::mpz_abs(&mut self.inner, &self.inner);
        }
        self
    }

    /// Divides `self` by `other`. This is much faster than normal
    /// division, but produces correct results only when the division
    /// is exact.
    ///
    /// # Panics
    ///
    /// Panics if `other` is zero.
    pub fn div_exact(&mut self, other: &IntegerOld) -> &mut IntegerOld {
        assert!(other.sign() != Ordering::Equal, "division by zero");
        unsafe {
            gmp::mpz_divexact(&mut self.inner, &self.inner, &other.inner);
        }
        self
    }

    /// Returns `true` if `self` is divisible by `other`.
    pub fn is_divisible(&self, other: &IntegerOld) -> bool {
        unsafe { gmp::mpz_divisible_p(&self.inner, &other.inner) != 0 }
    }

    /// Returns `true` if `self` is congruent to `c` modulo `d`, that
    /// is, if there exists a `q` such that `self == c + q * d`.
    /// Unlike other division functions, `d` can be zero.
    pub fn is_congruent(&self, c: &IntegerOld, d: &IntegerOld) -> bool {
        unsafe { gmp::mpz_congruent_p(&self.inner, &c.inner, &d.inner) != 0 }
    }

    /// Computes the `n`th root of `self` and truncates the result.
    pub fn root(&mut self, n: u32) -> &mut IntegerOld {
        unsafe {
            gmp::mpz_root(&mut self.inner, &self.inner, n.into());
        }
        self
    }

    /// Computes the `n`th root of `self` and returns the truncated
    /// root and the remainder.  The remainder is `self` minus the
    /// truncated root raised to the power of `n`.
    /// The remainder is stored in `buf`.
    pub fn root_rem(&mut self, buf: &mut IntegerOld, n: u32) {
        unsafe {
            gmp::mpz_rootrem(&mut self.inner, &mut buf.inner, &self.inner, n.into());
        }
    }

    /// Computes the square root of `self` and truncates the result.
    pub fn sqrt(&mut self) -> &mut IntegerOld {
        unsafe {
            gmp::mpz_sqrt(&mut self.inner, &self.inner);
        }
        self
    }

    /// Computes the square root of `self` and returns the truncated
    /// root and the remainder.  The remainder is `self` minus the
    /// truncated root squared.
    /// The remainder is stored in `buf`.
    pub fn sqrt_rem(&mut self, buf: &mut IntegerOld) {
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
    pub fn gcd(&mut self, other: &IntegerOld) -> &mut IntegerOld {
        unsafe {
            gmp::mpz_gcd(&mut self.inner, &self.inner, &other.inner);
        }
        self
    }

    /// Finds the least common multiple. The result is always positive
    /// except when one or both inputs are zero.
    pub fn lcm(&mut self, other: &IntegerOld) -> &mut IntegerOld {
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
    pub fn invert(&mut self, m: &IntegerOld) -> Option<&mut IntegerOld> {
        assert!(m.sign() != Ordering::Equal, "division by zero");
        let exists = unsafe { gmp::mpz_invert(&mut self.inner, &self.inner, &m.inner) != 0 };
        if exists { Some(self) } else { None }
    }

    /// Computes the factorial of `n`.
    /// The value of `self` is ignored.
    pub fn set_factorial(&mut self, n: u32) -> &mut IntegerOld {
        unsafe {
            gmp::mpz_fac_ui(&mut self.inner, n.into());
        }
        self
    }

    /// Computes the double factorial of `n`.
    /// The value of `self` is ignored.
    pub fn set_factorial_2(&mut self, n: u32) -> &mut IntegerOld {
        unsafe {
            gmp::mpz_2fac_ui(&mut self.inner, n.into());
        }
        self
    }

    /// Computes the `m`-multi factorial of `n`.
    /// The value of `self` is ignored.
    pub fn set_factorial_m(&mut self, n: u32, m: u32) -> &mut IntegerOld {
        unsafe {
            gmp::mpz_mfac_uiui(&mut self.inner, n.into(), m.into());
        }
        self
    }

    /// Computes the primorial of `n`.
    /// The value of `self` is ignored.
    pub fn set_primorial(&mut self, n: u32) -> &mut IntegerOld {
        unsafe {
            gmp::mpz_primorial_ui(&mut self.inner, n.into());
        }
        self
    }

    /// Computes the binomial coefficient `self` over `k`.
    pub fn binomial(&mut self, k: u32) -> &mut IntegerOld {
        unsafe {
            gmp::mpz_bin_ui(&mut self.inner, &self.inner, k.into());
        }
        self
    }

    /// Computes the binomial coefficient `n` over `k`.
    /// The value of `self` is ignored.
    pub fn set_binomial(&mut self, n: u32, k: u32) -> &mut IntegerOld {
        unsafe {
            gmp::mpz_bin_uiui(&mut self.inner, n.into(), k.into());
        }
        self
    }

    /// Compares the absolute values of `self` and `other`.
    pub fn cmp_abs(&self, other: &IntegerOld) -> Ordering {
        unsafe { gmp::mpz_cmpabs(&self.inner, &other.inner).cmp(&0) }
    }

    /// Returns the same result as self.cmp(0), but is faster.
    pub fn sign(&self) -> Ordering {
        unsafe { gmp::mpz_sgn(&self.inner).cmp(&0) }
    }

    /// Returns the number of bits required to represent the absolute
    /// value of `self`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gmp_to_flint_adaptor_lib::integer_old::IntegerOld;
    ///
    /// assert!(IntegerOld::from(0).significant_bits() == 0);
    /// assert!(IntegerOld::from(1).significant_bits() == 1);
    /// assert!(IntegerOld::from(-1).significant_bits() == 1);
    /// assert!(IntegerOld::from(4).significant_bits() == 3);
    /// assert!(IntegerOld::from(-4).significant_bits() == 3);
    /// assert!(IntegerOld::from(7).significant_bits() == 3);
    /// assert!(IntegerOld::from(-7).significant_bits() == 3);
    /// ```
    pub fn significant_bits(&self) -> u32 {
        let bits = unsafe { gmp::mpz_sizeinbase(&self.inner, 2) };
        if bits > u32::MAX as usize {
            panic!("overflow");
        }
        // sizeinbase returns 1 if number is 0
        if bits == 1 && *self == 0 {
            0
        } else {
            bits as u32
        }
    }

    /// Returns the number of ones in `self` if the value >= 0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gmp_to_flint_adaptor_lib::integer_old::IntegerOld;
    /// assert!(IntegerOld::from(0).count_ones() == Some(0));
    /// assert!(IntegerOld::from(15).count_ones() == Some(4));
    /// assert!(IntegerOld::from(-1).count_ones() == None);
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
    /// use gmp_to_flint_adaptor_lib::integer_old::IntegerOld;
    /// let i = IntegerOld::from(-1);
    /// assert!(IntegerOld::from(0).ham_dist(&i) == None);
    /// assert!(IntegerOld::from(-1).ham_dist(&i) == Some(0));
    /// assert!(IntegerOld::from(-13).ham_dist(&i) == Some(2));
    /// ```
    pub fn ham_dist(&self, other: &IntegerOld) -> Option<u32> {
        bitcount_to_u32(unsafe { gmp::mpz_hamdist(&self.inner, &other.inner) })
    }

    /// Returns the location of the first zero, starting at `start`.
    /// If the bit at location `start` is zero, returns `start`.
    ///
    /// ```rust
    /// use gmp_to_flint_adaptor_lib::integer_old::IntegerOld;
    /// assert!(IntegerOld::from(-2).find_zero(0) == Some(0));
    /// assert!(IntegerOld::from(-2).find_zero(1) == None);
    /// assert!(IntegerOld::from(15).find_zero(0) == Some(4));
    /// assert!(IntegerOld::from(15).find_zero(20) == Some(20));
    pub fn find_zero(&self, start: u32) -> Option<u32> {
        bitcount_to_u32(unsafe { gmp::mpz_scan0(&self.inner, start.into()) })
    }

    /// Returns the location of the first one, starting at `start`.
    /// If the bit at location `start` is one, returns `start`.
    ///
    /// ```rust
    /// use gmp_to_flint_adaptor_lib::integer_old::IntegerOld;
    /// assert!(IntegerOld::from(1).find_one(0) == Some(0));
    /// assert!(IntegerOld::from(1).find_one(1) == None);
    /// assert!(IntegerOld::from(-16).find_one(0) == Some(4));
    /// assert!(IntegerOld::from(-16).find_one(20) == Some(20));
    pub fn find_one(&self, start: u32) -> Option<u32> {
        bitcount_to_u32(unsafe { gmp::mpz_scan1(&self.inner, start.into()) })
    }

    /// Sets the bit at location `index` to 1 if `val` is `true` or 0
    /// if `val` is `false`.
    pub fn set_bit(&mut self, index: u32, val: bool) -> &mut IntegerOld {
        unsafe {
            if val {
                gmp::mpz_setbit(&mut self.inner, index.into());
            } else {
                gmp::mpz_clrbit(&mut self.inner, index.into());
            }
        }
        self
    }

    /// Returns `true` if the bit at location `index` is 1 or `false`
    /// if the bit is 0.
    pub fn get_bit(&self, index: u32) -> bool {
        unsafe { gmp::mpz_tstbit(&self.inner, index.into()) != 0 }
    }

    /// Toggles the bit at location `index`.
    pub fn invert_bit(&mut self, index: u32) -> &mut IntegerOld {
        unsafe {
            gmp::mpz_combit(&mut self.inner, index.into());
        }
        self
    }

    pub fn to_u32s<'a>(&'a self) -> IntegerContent<'a> {
        IntegerContent::new(self)
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
        // Avoid conditions and overflow, equivalent to:
        // let total_limbs = whole_limbs + if extra_bits == 0 { 0 } else { 1 };
        let total_limbs = whole_limbs + ((extra_bits + limb_bits - 1) / limb_bits) as usize;
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
                limbs_used = i as c_int + 1;
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
    /// use gmp_to_flint_adaptor_lib::integer_old::IntegerOld;
    ///
    /// fn main() {
    ///     let mut rng = rand::thread_rng();
    ///     let mut i = IntegerOld::new();
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
        // Avoid conditions and overflow, equivalent to:
        // let total_limbs = whole_limbs + if extra_bits == 0 { 0 } else { 1 };
        let total_limbs = whole_limbs + ((extra_bits + limb_bits - 1) / limb_bits) as usize;
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
            limbs_used = i as c_int + 1;
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

    pub fn assign_random_bits_unsigned_variable<R: Rng>(&mut self, max_bits: u32, rng: &mut R) {
        let bits = Range::new(0, max_bits + 1).ind_sample(rng);
        self.assign_random_bits_unsigned(bits, rng);
    }

    pub fn assign_random_bits_variable<R: Rng>(&mut self, max_bits: u32, rng: &mut R) {
        let bits = Range::new(0, max_bits + 1).ind_sample(rng);
        self.assign_random_bits(bits, rng);
    }

    pub fn assign_random_bits_nonzero_variable<R: Rng>(&mut self, max_bits: u32, rng: &mut R) {
        if max_bits == 0 {
            panic!("max_bits must be positive");
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
    /// use gmp_to_flint_adaptor_lib::integer_old::IntegerOld;
    ///
    /// fn main() {
    ///     let mut rng = rand::thread_rng();
    ///     let bound = IntegerOld::from(15);
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
    pub fn random_below<R: Rng>(&mut self, rng: &mut R) -> &mut IntegerOld {
        assert!(self.sign() == Ordering::Greater, "cannot be below zero");
        let bits = self.significant_bits();
        let limb_bits = gmp::LIMB_BITS as u32;
        let whole_limbs = (bits / limb_bits) as usize;
        let extra_bits = bits % limb_bits;
        // Avoid conditions and overflow, equivalent to:
        // let total_limbs = whole_limbs + if extra_bits == 0 { 0 } else { 1 };
        let total_limbs = whole_limbs + ((extra_bits + limb_bits - 1) / limb_bits) as usize;
        let limbs = unsafe { slice::from_raw_parts_mut(self.inner.d, total_limbs) };
        // if the random number is >= bound, restart
        'restart: loop {
            let mut limbs_used: c_int = 0;
            let mut still_equal = true;
            'next_limb: for i in (0..total_limbs).rev() {
                let mut val: gmp::limb_t = rng.gen();
                if i == whole_limbs {
                    val &= ((1 as gmp::limb_t) << extra_bits) - 1;
                }
                if limbs_used == 0 && val != 0 {
                    limbs_used = i as c_int + 1;
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
    /// use gmp_to_flint_adaptor_lib::integer_old::{Assign, IntegerOld};
    /// let mut i = IntegerOld::new();
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
    pub fn from_str_radix(src: &str, radix: i32) -> Result<IntegerOld, ParseIntegerError> {
        let mut i = IntegerOld::new();
        i.assign_str_radix(src, radix)?;
        Ok(i)
    }

    /// Parses an `Integer` from a string in decimal.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gmp_to_flint_adaptor_lib::integer_old::IntegerOld;
    /// let mut i = IntegerOld::new();
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
    /// use gmp_to_flint_adaptor_lib::integer_old::IntegerOld;
    /// let mut i = IntegerOld::new();
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
        assert!(err == 0);
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

    assert!(radix >= 2 && radix <= 36, "radix out of range");
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
            '0'...'9' => c as i32 - '0' as i32,
            'a'...'z' => c as i32 - 'a' as i32 + 10,
            'A'...'Z' => c as i32 - 'A' as i32 + 10,
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

impl FromStr for IntegerOld {
    type Err = ParseIntegerError;

    /// Parses an `Integer`.
    ///
    /// See the [corresponding assignment](#method.assign_str).
    fn from_str(src: &str) -> Result<IntegerOld, ParseIntegerError> {
        let mut i = IntegerOld::new();
        i.assign_str(src)?;
        Ok(i)
    }
}

macro_rules! from_borrow {
    { $d: expr, $t:ty } => {
        impl<'a> From<&'a $t> for IntegerOld {
            /// Constructs an `Integer` from
            #[doc=$d]
            fn from(t: &$t) -> IntegerOld {
                let mut ret = IntegerOld::new();
                ret.assign(t);
                ret
            }
        }
    };
}

macro_rules! from {
    { $d: expr, $t:ty } => {
        impl From<$t> for IntegerOld {
            /// Constructs an `Integer` from
            #[doc=$d]
            fn from(t: $t) -> IntegerOld {
                let mut ret = IntegerOld::new();
                ret.assign(t);
                ret
            }
        }
    };
}

from_borrow! { "another `Integer`.", IntegerOld }
from! { "a `u32`.", u32 }
from! { "an `i32`.", i32 }

impl<'a> Assign<&'a IntegerOld> for IntegerOld {
    /// Assigns from another `Integer`.
    fn assign(&mut self, other: &'a IntegerOld) {
        unsafe {
            gmp::mpz_set(&mut self.inner, &other.inner);
        }
    }
}

impl<'a> Assign<IntegerOld> for IntegerOld {
    /// Assigns from another `Integer`.
    fn assign(&mut self, other: IntegerOld) {
        self.assign(&other);
    }
}

impl Assign<u32> for IntegerOld {
    /// Assigns from a `u32`.
    fn assign(&mut self, val: u32) {
        unsafe { gmp::mpz_set_ui(&mut self.inner, val.into()) }
    }
}

impl Assign<i32> for IntegerOld {
    /// Assigns from an `i32`.
    fn assign(&mut self, val: i32) {
        unsafe { gmp::mpz_set_si(&mut self.inner, val.into()) }
    }
}

impl Assign<f64> for IntegerOld {
    /// Assigns from an `f64`, rounding towards zero.
    fn assign(&mut self, val: f64) {
        unsafe {
            gmp::mpz_set_d(&mut self.inner, val);
        }
    }
}

impl Assign<f32> for IntegerOld {
    /// Assigns from an `f32`, rounding towards zero.
    fn assign(&mut self, val: f32) {
        self.assign(val as f64);
    }
}

macro_rules! arith_integer {
    {
        $imp:ident $method:ident,
        $imp_assign:ident $method_assign:ident,
        $func:path
    } => {
        impl<'a> $imp<&'a IntegerOld> for IntegerOld {
            type Output = IntegerOld;
            fn $method(mut self, op: &'a IntegerOld) -> IntegerOld {
                $imp_assign::<&'a IntegerOld>::$method_assign(&mut self, op);
                self
            }
        }

        impl $imp<IntegerOld> for IntegerOld {
            type Output = IntegerOld;
            fn $method(self, op: IntegerOld) -> IntegerOld {
                self.$method(&op)
            }
        }

        impl<'a> $imp_assign<&'a IntegerOld> for IntegerOld {
            fn $method_assign(&mut self, op: &'a IntegerOld) {
                unsafe {
                    $func(&mut self.inner, &self.inner, &op.inner);
                }
            }
        }

        impl $imp_assign<IntegerOld> for IntegerOld {
            fn $method_assign(&mut self, op: IntegerOld) {
                self.$method_assign(&op);
            }
        }
    };
}

macro_rules! arith_noncommut_integer {
    {
        $imp:ident $method:ident,
        $imp_assign:ident $method_assign:ident,
        $imp_from_assign:ident $method_from_assign:ident,
        $func:path
    } => {
        arith_integer! { $imp $method, $imp_assign $method_assign, $func }

        impl<'a> $imp_from_assign<&'a IntegerOld> for IntegerOld {
            fn $method_from_assign(&mut self, lhs: &'a IntegerOld) {
                unsafe {
                    $func(&mut self.inner, &lhs.inner, &self.inner);
                }
            }
        }

        impl $imp_from_assign<IntegerOld> for IntegerOld {
            fn $method_from_assign(&mut self, lhs: IntegerOld) {
                self.$method_from_assign(&lhs);
            }
        }

    };
}

arith_integer! { Add add, AddAssign add_assign, gmp::mpz_add }
arith_noncommut_integer! { Sub sub, SubAssign sub_assign,
                           SubFromAssign sub_from_assign, gmp::mpz_sub }
arith_integer! { Mul mul, MulAssign mul_assign, gmp::mpz_mul }
arith_noncommut_integer! { Div div, DivAssign div_assign,
                           DivFromAssign div_from_assign, mpz_tdiv_q }
arith_noncommut_integer! { Rem rem, RemAssign rem_assign,
                           RemFromAssign rem_from_assign, mpz_tdiv_r }
arith_integer! { BitAnd bitand, BitAndAssign bitand_assign, gmp::mpz_and }
arith_integer! { BitOr bitor, BitOrAssign bitor_assign, gmp::mpz_ior }
arith_integer! { BitXor bitxor, BitXorAssign bitxor_assign, gmp::mpz_xor }

impl Neg for IntegerOld {
    type Output = IntegerOld;
    fn neg(mut self) -> IntegerOld {
        self.neg_assign();
        self
    }
}

impl NegAssign for IntegerOld {
    fn neg_assign(&mut self) {
        unsafe {
            gmp::mpz_neg(&mut self.inner, &self.inner);
        }
    }
}

impl Not for IntegerOld {
    type Output = IntegerOld;
    fn not(mut self) -> IntegerOld {
        self.not_assign();
        self
    }
}

impl NotAssign for IntegerOld {
    fn not_assign(&mut self) {
        unsafe {
            gmp::mpz_com(&mut self.inner, &self.inner);
        }
    }
}

unsafe fn mpz_tdiv_q(q: *mut mpz_t, n: *const mpz_t, d: *const mpz_t) {
    assert!(gmp::mpz_sgn(d) != 0, "division by zero");
    gmp::mpz_tdiv_q(q, n, d);
}

unsafe fn mpz_tdiv_r(q: *mut mpz_t, n: *const mpz_t, d: *const mpz_t) {
    assert!(gmp::mpz_sgn(d) != 0, "division by zero");
    gmp::mpz_tdiv_r(q, n, d);
}

macro_rules! arith_prim_for_integer {
    ($imp:ident $method:ident,
     $imp_assign:ident $method_assign:ident,
     $t:ty,
     $func:path) => {
        impl $imp<$t> for IntegerOld {
            type Output = IntegerOld;
            fn $method(mut self, op: $t) -> IntegerOld {
                self.$method_assign(op);
                self
            }
        }

        impl $imp_assign<$t> for IntegerOld {
            fn $method_assign(&mut self, op: $t) {
                unsafe {
                    $func(&mut self.inner, &self.inner, op.into());
                }
            }
        }
    };
}

macro_rules! arith_prim_non_commut {
    ($imp:ident $method:ident,
     $imp_assign:ident $method_assign:ident,
     $imp_from_assign:ident $method_from_assign:ident,
     $t:ty,
     $func:path,
     $func_from:path) => {
        arith_prim_for_integer! {
            $imp $method,
            $imp_assign $method_assign,
            $t,
            $func
        }

        impl $imp<IntegerOld> for $t {
            type Output = IntegerOld;
            fn $method(self, mut op: IntegerOld) -> IntegerOld {
                op.$method_from_assign(self);
                op
            }
        }

        impl<'a> $imp<&'a IntegerOld> for $t {
            type Output = IntegerOld;
            fn $method(self, op: &'a IntegerOld) -> IntegerOld {
                self.$method(op.clone())
            }
        }

        impl $imp_from_assign<$t> for IntegerOld {
            fn $method_from_assign(&mut self, lhs: $t) {
                unsafe {
                    $func_from(&mut self.inner, lhs.into(), &self.inner);
                }
            }
        }
    };
}

macro_rules! arith_prim_commut {
    ($imp:ident $method:ident,
     $imp_assign:ident $method_assign:ident,
     $t:ty,
     $func:path) => {
        arith_prim_for_integer! {
            $imp $method,
            $imp_assign $method_assign,
            $t,
            $func
        }

        impl $imp<IntegerOld> for $t {
            type Output = IntegerOld;
            fn $method(self, op: IntegerOld) -> IntegerOld {
                op.$method(self)
            }
        }

        impl<'a> $imp<&'a IntegerOld> for $t {
            type Output = IntegerOld;
            fn $method(self, op: &'a IntegerOld) -> IntegerOld {
                self.$method(op.clone())
            }
        }
    };
}

arith_prim_commut! { Add add, AddAssign add_assign, u32, gmp::mpz_add_ui }
arith_prim_non_commut! { Sub sub, SubAssign sub_assign,
                         SubFromAssign sub_from_assign,
                         u32, gmp::mpz_sub_ui, gmp::mpz_ui_sub }
arith_prim_commut! { Mul mul, MulAssign mul_assign, u32, gmp::mpz_mul_ui }
arith_prim_non_commut! { Div div, DivAssign div_assign,
                         DivFromAssign div_from_assign,
                         u32, mpz_tdiv_q_ui, mpz_ui_tdiv_q }
arith_prim_non_commut! { Rem rem, RemAssign rem_assign,
                         RemFromAssign rem_from_assign,
                         u32, mpz_tdiv_r_ui, mpz_ui_tdiv_r }
arith_prim_for_integer! { Shl shl, ShlAssign shl_assign, u32,
                          gmp::mpz_mul_2exp }
arith_prim_for_integer! { Shr shr, ShrAssign shr_assign, u32,
                          gmp::mpz_fdiv_q_2exp }
arith_prim_for_integer! { Pow pow, PowAssign pow_assign, u32,
                          gmp::mpz_pow_ui }
arith_prim_commut! { BitAnd bitand, BitAndAssign bitand_assign, u32, bitand_ui }
arith_prim_commut! { BitOr bitor, BitOrAssign bitor_assign, u32, bitor_ui }
arith_prim_commut! { BitXor bitxor, BitXorAssign bitxor_assign, u32, bitxor_ui }

arith_prim_commut! { Add add, AddAssign add_assign, i32, mpz_add_si }
arith_prim_non_commut! { Sub sub, SubAssign sub_assign,
                         SubFromAssign sub_from_assign,
                         i32, mpz_sub_si, mpz_si_sub }
arith_prim_commut! { Mul mul, MulAssign mul_assign, i32, gmp::mpz_mul_si }
arith_prim_non_commut! { Div div, DivAssign div_assign,
                         DivFromAssign div_from_assign,
                         i32, mpz_tdiv_q_si, mpz_si_tdiv_q }
arith_prim_non_commut! { Rem rem, RemAssign rem_assign,
                         RemFromAssign rem_from_assign,
                         i32, mpz_tdiv_r_si, mpz_si_tdiv_r }
arith_prim_for_integer! { Shl shl, ShlAssign shl_assign, i32,
                          mpz_lshift_si }
arith_prim_for_integer! { Shr shr, ShrAssign shr_assign, i32,
                          mpz_rshift_si }

unsafe fn mpz_tdiv_q_ui(q: *mut mpz_t, n: *const mpz_t, d: c_ulong) {
    assert!(d != 0, "division by zero");
    gmp::mpz_tdiv_q_ui(q, n, d);
}

unsafe fn mpz_ui_tdiv_q(q: *mut mpz_t, n: c_ulong, d: *const mpz_t) {
    let sgn_d = gmp::mpz_sgn(d);
    assert!(sgn_d != 0, "division by zero");
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
    assert!(d != 0, "division by zero");
    gmp::mpz_tdiv_r_ui(q, n, d);
}

unsafe fn mpz_ui_tdiv_r(q: *mut mpz_t, n: c_ulong, d: *const mpz_t) {
    assert!(gmp::mpz_sgn(d) != 0, "division by zero");
    if gmp::mpz_cmpabs_ui(d, n) > 0 {
        gmp::mpz_set_ui(q, n);
    } else {
        let ui = n % gmp::mpz_get_ui(d);
        gmp::mpz_set_ui(q, ui);
    }
}

unsafe fn bitand_ui(rop: *mut mpz_t, op1: *const mpz_t, op2: c_ulong) {
    assert!(mem::size_of::<c_long>() <= mem::size_of::<gmp::limb_t>());
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
    assert!(mem::size_of::<c_long>() <= mem::size_of::<gmp::limb_t>());
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
    assert!(mem::size_of::<c_long>() <= mem::size_of::<gmp::limb_t>());
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
        // is this the best we can do?
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

impl Eq for IntegerOld {}

impl Ord for IntegerOld {
    fn cmp(&self, other: &IntegerOld) -> Ordering {
        let ord = unsafe { gmp::mpz_cmp(&self.inner, &other.inner) };
        ord.cmp(&0)
    }
}

impl PartialEq for IntegerOld {
    fn eq(&self, other: &IntegerOld) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for IntegerOld {
    fn partial_cmp(&self, other: &IntegerOld) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<f64> for IntegerOld {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        if other.is_nan() {
            None
        } else {
            let ord = unsafe { gmp::mpz_cmp_d(&self.inner, *other) };
            Some(ord.cmp(&0))
        }
    }
}

impl PartialEq<f64> for IntegerOld {
    fn eq(&self, other: &f64) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd<IntegerOld> for f64 {
    fn partial_cmp(&self, other: &IntegerOld) -> Option<Ordering> {
        match other.partial_cmp(self) {
            None => None,
            Some(x) => Some(x.reverse()),
        }
    }
}

impl PartialEq<IntegerOld> for f64 {
    fn eq(&self, other: &IntegerOld) -> bool {
        other.eq(self)
    }
}

impl PartialOrd<f32> for IntegerOld {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        let o = *other as f64;
        self.partial_cmp(&o)
    }
}

impl PartialEq<f32> for IntegerOld {
    fn eq(&self, other: &f32) -> bool {
        let o = *other as f64;
        self.eq(&o)
    }
}

impl PartialEq<IntegerOld> for f32 {
    fn eq(&self, other: &IntegerOld) -> bool {
        other.eq(self)
    }
}

impl PartialOrd<IntegerOld> for f32 {
    fn partial_cmp(&self, other: &IntegerOld) -> Option<Ordering> {
        match other.partial_cmp(self) {
            None => None,
            Some(x) => Some(x.reverse()),
        }
    }
}

impl Hash for IntegerOld {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sign().hash(state);
        for i in self.to_u32s() {
            i.hash(state);
        }
    }
}

macro_rules! cmp_int {
    { $t:ty, $func:path } => {
        impl PartialOrd<$t> for IntegerOld {
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                let ord = unsafe { $func(&self.inner, (*other).into()) };
                Some(ord.cmp(&0))
            }
        }

        impl PartialEq<$t> for IntegerOld {
            fn eq(&self, other: &$t) -> bool {
                self.partial_cmp(other) == Some(Ordering::Equal)
            }
        }

        impl PartialOrd<IntegerOld> for $t {
            fn partial_cmp(&self, other: &IntegerOld) -> Option<Ordering> {
                match other.partial_cmp(self) {
                    Some(x) => Some(x.reverse()),
                    None => None,
                }
            }
        }

        impl PartialEq<IntegerOld> for $t {
            fn eq(&self, other: &IntegerOld) -> bool {
                other.eq(self)
            }
        }
    };
}

cmp_int! { u32, gmp::mpz_cmp_ui }
cmp_int! { i32, gmp::mpz_cmp_si }

fn make_string(i: &IntegerOld, radix: i32, to_upper: bool) -> String {
    assert!(radix >= 2 && radix <= 36, "radix out of range");
    let i_size = unsafe { gmp::mpz_sizeinbase(&i.inner, radix) };
    // size + 2 for '-' and nul
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

fn fmt_radix(i: &IntegerOld,
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

impl Display for IntegerOld {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_radix(self, f, 10, false, "")
    }
}

impl Debug for IntegerOld {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_radix(self, f, 10, false, "")
    }
}

impl Binary for IntegerOld {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_radix(self, f, 2, false, "0b")
    }
}

impl Octal for IntegerOld {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_radix(self, f, 8, false, "0o")
    }
}

impl LowerHex for IntegerOld {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_radix(self, f, 16, false, "0x")
    }
}

impl UpperHex for IntegerOld {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_radix(self, f, 16, true, "0x")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// An error which can be returned when parsing an `Integer`.
pub struct ParseIntegerError {
    kind: ParseErrorKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ParseErrorKind {
    InvalidDigit,
    NoDigits,
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
    } else if bits > u32::MAX as gmp::bitcnt_t {
        panic!("overflow")
    } else {
        Some(bits as u32)
    }
}

#[cfg(test)]
mod tests {
    use integer_old::*;
    use std::{f32, f64, i32, u32};
    use std::cmp::Ordering;
    use std::mem;

    #[test]
    fn check_arith_u_s() {
        let large = [(1, 100), (-11, 200), (33, 150)];
        let u = [0, 1, 100, u32::MAX];
        let s = [i32::MIN, -100, -1, 0, 1, 100, i32::MAX];
        for &op in &u {
            let iop = IntegerOld::from(op);
            let against = (large.iter().map(|&(n, s)| IntegerOld::from(n) << s))
                .chain(s.iter().map(|&x| IntegerOld::from(x)))
                .chain(u.iter().map(|&x| IntegerOld::from(x)));
            for b in against {
                assert!(b.clone() + op == b.clone() + &iop);
                assert!(b.clone() - op == b.clone() - &iop);
                assert!(b.clone() * op == b.clone() * &iop);
                if op != 0 {
                    assert!(b.clone() / op == b.clone() / &iop);
                    assert!(b.clone() % op == b.clone() % &iop);
                }
                assert!(b.clone() & op == b.clone() & &iop);
                assert!(b.clone() | op == b.clone() | &iop);
                assert!(b.clone() ^ op == b.clone() ^ &iop);
                assert!(op + &b == iop.clone() + &b);
                assert!(op - &b == iop.clone() - &b);
                assert!(op * &b == iop.clone() * &b);
                if b.sign() != Ordering::Equal {
                    assert!(op / &b == iop.clone() / &b);
                    assert!(op % &b == iop.clone() % &b);
                }
                assert!(op & &b == iop.clone() & &b);
                assert!(op | &b == iop.clone() | &b);
                assert!(op ^ &b == iop.clone() ^ &b);
            }
        }
        for &op in &s {
            let iop = IntegerOld::from(op);
            let against = (large.iter().map(|&(n, s)| IntegerOld::from(n) << s))
                .chain(s.iter().map(|&x| IntegerOld::from(x)))
                .chain(u.iter().map(|&x| IntegerOld::from(x)));
            for b in against {
                assert!(b.clone() + op == b.clone() + &iop);
                assert!(b.clone() - op == b.clone() - &iop);
                assert!(b.clone() * op == b.clone() * &iop);
                if op != 0 {
                    assert!(b.clone() / op == b.clone() / &iop);
                    assert!(b.clone() % op == b.clone() % &iop);
                }
                assert!(op + &b == iop.clone() + &b);
                assert!(op - &b == iop.clone() - &b);
                assert!(op * &b == iop.clone() * &b);
                if b.sign() != Ordering::Equal {
                    assert!(op / &b == iop.clone() / &b);
                    assert!(op % &b == iop.clone() % &b);
                }
            }
        }
    }

    #[test]
    fn check_shift_u_s() {
        let pos: IntegerOld = IntegerOld::from(11) << 100;
        let neg: IntegerOld = IntegerOld::from(-33) << 50;
        assert!(pos.clone() << 10 == pos.clone() >> -10);
        assert!(pos.clone() << 10 == IntegerOld::from(11) << 110);
        assert!(pos.clone() << -100 == pos.clone() >> 100);
        assert!(pos.clone() << -100 == 11);
        assert!(neg.clone() << 10 == neg.clone() >> -10);
        assert!(neg.clone() << 10 == IntegerOld::from(-33) << 60);
        assert!(neg.clone() << -100 == neg.clone() >> 100);
        assert!(neg.clone() << -100 == -1);
    }

    #[test]
    fn check_int_conversions() {
        let mut i = IntegerOld::from(-1);
        assert!(i.to_u32_wrapping() == u32::MAX);
        assert!(i.to_i32_wrapping() == -1);
        i.assign(0xff000000u32);
        i <<= 4;
        assert!(i.to_u32_wrapping() == 0xf0000000u32);
        assert!(i.to_i32_wrapping() == 0xf0000000u32 as i32);
        i = i.clone() << 32 | i;
        assert!(i.to_u32_wrapping() == 0xf0000000u32);
        assert!(i.to_i32_wrapping() == 0xf0000000u32 as i32);
        i.neg_assign();
        assert!(i.to_u32_wrapping() == 0x10000000u32);
        assert!(i.to_i32_wrapping() == 0x10000000i32);
    }

    #[test]
    fn check_option_conversion() {
        let mut i = IntegerOld::new();
        assert!(i.to_u32() == Some(0));
        assert!(i.to_i32() == Some(0));
        i -= 1;
        assert!(i.to_u32() == None);
        assert!(i.to_i32() == Some(-1));
        i.assign(i32::MIN);
        assert!(i.to_u32() == None);
        assert!(i.to_i32() == Some(i32::MIN));
        i -= 1;
        assert!(i.to_u32() == None);
        assert!(i.to_i32() == None);
        i.assign(i32::MAX);
        assert!(i.to_u32() == Some(i32::MAX as u32));
        assert!(i.to_i32() == Some(i32::MAX));
        i += 1;
        assert!(i.to_u32() == Some(i32::MAX as u32 + 1));
        assert!(i.to_i32() == None);
        i.assign(u32::MAX);
        assert!(i.to_u32() == Some(u32::MAX));
        assert!(i.to_i32() == None);
        i += 1;
        assert!(i.to_u32() == None);
        assert!(i.to_i32() == None);
    }

    #[test]
    fn check_float_conversions() {
        let mut i = IntegerOld::from(0);
        assert!(i.to_f32() == 0.0);
        assert!(i.to_f64() == 0.0);
        i.assign(0xff);
        assert!(i.to_f32() == 255.0);
        assert!(i.to_f64() == 255.0);
        i <<= 80;
        assert!(i.to_f32() == 255.0 * 2f32.powi(80));
        assert!(i.to_f64() == 255.0 * 2f64.powi(80));
        i = i.clone() << 30 | i;
        assert!(i.to_f32() == 255.0 * 2f32.powi(110));
        assert!(i.to_f64() == 255.0 * (2f64.powi(80) + 2f64.powi(110)));
        i <<= 100;
        assert!(i.to_f32() == f32::INFINITY);
        assert!(i.to_f64() == 255.0 * (2f64.powi(180) + 2f64.powi(210)));
        i <<= 1000;
        assert!(i.to_f32() == f32::INFINITY);
        assert!(i.to_f64() == f64::INFINITY);
    }

    #[test]
    fn check_from_str() {
        let mut i: IntegerOld = "+134".parse().unwrap();
        assert!(i == 134);
        i.assign_str_radix("-ffFFffffFfFfffffffffffffffffffff", 16).unwrap();
        assert!(i.significant_bits() == 128);
        i -= 1;
        assert!(i.significant_bits() == 129);

        let bad_strings = [(" 1", None),
                           ("+-3", None),
                           ("-+3", None),
                           ("++3", None),
                           ("--3", None),
                           ("0+3", None),
                           ("0 ", None),
                           ("", None),
                           ("80", Some(8)),
                           ("0xf", Some(16)),
                           ("9", Some(9))];
        for &(s, radix) in bad_strings.into_iter() {
            assert!(IntegerOld::valid_str_radix(s, radix.unwrap_or(10)).is_err());
        }
        let good_strings = [("0", 10, 0),
                            ("+0", 16, 0),
                            ("-0", 2, 0),
                            ("99", 10, 99),
                            ("+Cc", 16, 0xcc),
                            ("-77", 8, -0o77)];
        for &(s, radix, i) in good_strings.into_iter() {
            assert!(IntegerOld::from_str_radix(s, radix).unwrap() == i);
        }
    }

    #[test]
    fn check_formatting() {
        let i = IntegerOld::from((-11));
        assert!(format!("{}", i) == "-11");
        assert!(format!("{:?}", i) == "-11");
        assert!(format!("{:b}", i) == "-1011");
        assert!(format!("{:#b}", i) == "-0b1011");
        assert!(format!("{:o}", i) == "-13");
        assert!(format!("{:#o}", i) == "-0o13");
        assert!(format!("{:x}", i) == "-b");
        assert!(format!("{:X}", i) == "-B");
        assert!(format!("{:8x}", i) == "      -b");
        assert!(format!("{:08X}", i) == "-000000B");
        assert!(format!("{:#08x}", i) == "-0x0000b");
        assert!(format!("{:#8X}", i) == "    -0xB");
    }

    #[test]
    fn check_no_nails() {
        // we assume no nail bits when we use limbs
        assert!(gmp::NAIL_BITS == 0);
        assert!(gmp::NUMB_BITS == gmp::LIMB_BITS);
        assert!(gmp::NUMB_BITS as usize == 8 * mem::size_of::<gmp::limb_t>());
    }
}

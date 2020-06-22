use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwo, ModPowerOfTwoAssign, ModPowerOfTwoMul, ModPowerOfTwoMulAssign, ShrRound,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;

use natural::arithmetic::mod_power_of_two::limbs_vec_mod_power_of_two_in_place;
use natural::arithmetic::mul::limbs_mul;
use natural::arithmetic::mul::mul_low::limbs_mul_low_same_length;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::{DoubleLimb, Limb};

/// Interpreting two `Vec<Limb>`s as the limbs (in ascending order) of two `Natural`s, returns a
/// `Vec` of the limbs of the product of the `Natural`s mod 2<sup>`pow`</sup>. Assumes the inputs
/// are already reduced mod 2<sup>`pow`</sup>. The input `Vec`s may be mutated. Neither input may be
/// empty or have trailing zeros.
///
/// Time: worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: worst case O(n * log(n))
///
/// where n = `pow`
///
/// # Panics
/// Panics if either input is empty. May panic if either input has trailing zeros.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two_mul::limbs_mod_power_of_two_mul;
///
/// assert_eq!(
///     limbs_mod_power_of_two_mul(&mut vec![100, 101, 102], &mut vec![102, 101, 100], 90),
///     &[10_200, 20_402, 30_605]
/// );
/// assert_eq!(
///     limbs_mod_power_of_two_mul(&mut vec![100, 101, 102], &mut vec![102, 101, 100], 140),
///     &[10_200, 20_402, 30_605, 20_402, 2_008]
/// );
/// assert_eq!(
///     limbs_mod_power_of_two_mul(&mut vec![100, 101, 102], &mut vec![102, 101, 100], 1_000),
///     &[10_200, 20_402, 30_605, 20_402, 10_200, 0]
/// );
/// ```
pub fn limbs_mod_power_of_two_mul(xs: &mut Vec<Limb>, ys: &mut Vec<Limb>, pow: u64) -> Vec<Limb> {
    let xs_len = xs.len();
    assert_ne!(xs_len, 0);
    let ys_len = ys.len();
    assert_ne!(ys_len, 0);
    let max_len = usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling));
    if max_len > xs_len + ys_len + 1 {
        return limbs_mul(xs, ys);
    }
    // Should really be max_len / sqrt(2); 0.75 * max_len is close enough
    let limit = max_len.checked_mul(3).unwrap() >> 2;
    let mut product_limbs = if xs_len >= limit && ys_len >= limit {
        if xs_len != max_len {
            xs.resize(max_len, 0);
        }
        if ys_len != max_len {
            ys.resize(max_len, 0);
        }
        let mut product_limbs = vec![0; max_len];
        limbs_mul_low_same_length(&mut product_limbs, xs, ys);
        product_limbs
    } else {
        limbs_mul(xs, ys)
    };
    limbs_vec_mod_power_of_two_in_place(&mut product_limbs, pow);
    product_limbs
}

/// Interpreting a slice of `Limb` and a `Vec<Limb>`s as the limbs (in ascending order) of two
/// `Natural`s, returns a `Vec` of the limbs of the product of the `Natural`s mod 2<sup>`pow`</sup>.
/// Assumes the inputs are already reduced mod 2<sup>`pow`</sup>. The input `Vec` may be mutated.
/// Neither input may be empty or have trailing zeros.
///
/// Time: worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: worst case O(n * log(n))
///
/// where n = `pow`
///
/// # Panics
/// Panics if either input is empty. May panic if either input has trailing zeros.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two_mul::limbs_mod_power_of_two_mul_val_ref;
///
/// assert_eq!(
///     limbs_mod_power_of_two_mul_val_ref(&mut vec![100, 101, 102], &[102, 101, 100], 90),
///     &[10_200, 20_402, 30_605]
/// );
/// assert_eq!(
///     limbs_mod_power_of_two_mul_val_ref(&mut vec![100, 101, 102], &[102, 101, 100], 140),
///     &[10_200, 20_402, 30_605, 20_402, 2_008]
/// );
/// assert_eq!(
///     limbs_mod_power_of_two_mul_val_ref(&mut vec![100, 101, 102], &[102, 101, 100], 1_000),
///     &[10_200, 20_402, 30_605, 20_402, 10_200, 0]
/// );
/// ```
pub fn limbs_mod_power_of_two_mul_val_ref(xs: &mut Vec<Limb>, ys: &[Limb], pow: u64) -> Vec<Limb> {
    let xs_len = xs.len();
    assert_ne!(xs_len, 0);
    let ys_len = ys.len();
    assert_ne!(ys_len, 0);
    let max_len = usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling));
    if max_len > xs_len + ys_len + 1 {
        return limbs_mul(xs, ys);
    }
    // Should really be max_len / sqrt(2); 0.75 * max_len is close enough
    let limit = max_len.checked_mul(3).unwrap() >> 2;
    let mut product_limbs = if xs_len >= limit && ys_len >= limit {
        if xs_len != max_len {
            xs.resize(max_len, 0);
        }
        let mut ys_adjusted_vec;
        let ys_adjusted = if ys_len == max_len {
            ys
        } else {
            ys_adjusted_vec = vec![0; max_len];
            ys_adjusted_vec[..ys_len].copy_from_slice(ys);
            &ys_adjusted_vec
        };
        let mut product_limbs = vec![0; max_len];
        limbs_mul_low_same_length(&mut product_limbs, xs, ys_adjusted);
        product_limbs
    } else {
        limbs_mul(xs, ys)
    };
    limbs_vec_mod_power_of_two_in_place(&mut product_limbs, pow);
    product_limbs
}

/// Interpreting two slices of `Limb` as the limbs (in ascending order) of two `Natural`s, returns a
/// `Vec` of the limbs of the product of the `Natural`s mod 2<sup>`pow`</sup>. Assumes the inputs
/// are already reduced mod 2<sup>`pow`</sup>. Neither input may be empty or have trailing zeros.
///
/// Time: worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: worst case O(n * log(n))
///
/// where n = `pow`
///
/// # Panics
/// Panics if either input is empty. May panic if either input has trailing zeros.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two_mul::limbs_mod_power_of_two_mul_ref_ref;
///
/// assert_eq!(
///     limbs_mod_power_of_two_mul_ref_ref(&[100, 101, 102], &[102, 101, 100], 90),
///     &[10_200, 20_402, 30_605]
/// );
/// assert_eq!(
///     limbs_mod_power_of_two_mul_ref_ref(&[100, 101, 102], &[102, 101, 100], 140),
///     &[10_200, 20_402, 30_605, 20_402, 2_008]
/// );
/// assert_eq!(
///     limbs_mod_power_of_two_mul_ref_ref(&[100, 101, 102], &[102, 101, 100], 1_000),
///     &[10_200, 20_402, 30_605, 20_402, 10_200, 0]
/// );
/// ```
pub fn limbs_mod_power_of_two_mul_ref_ref(xs: &[Limb], ys: &[Limb], pow: u64) -> Vec<Limb> {
    let xs_len = xs.len();
    assert_ne!(xs_len, 0);
    let ys_len = ys.len();
    assert_ne!(ys_len, 0);
    let max_len = usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling));
    if max_len > xs_len + ys_len + 1 {
        return limbs_mul(xs, ys);
    }
    // Should really be max_len / sqrt(2); 0.75 * max_len is close enough
    let limit = max_len.checked_mul(3).unwrap() >> 2;
    let mut product_limbs = if xs_len >= limit && ys_len >= limit {
        let mut xs_adjusted_vec;
        let mut ys_adjusted_vec;
        let xs_adjusted = if xs_len == max_len {
            xs
        } else {
            xs_adjusted_vec = vec![0; max_len];
            xs_adjusted_vec[..xs_len].copy_from_slice(xs);
            &xs_adjusted_vec
        };
        let ys_adjusted = if ys_len == max_len {
            ys
        } else {
            ys_adjusted_vec = vec![0; max_len];
            ys_adjusted_vec[..ys_len].copy_from_slice(ys);
            &ys_adjusted_vec
        };
        let mut product_limbs = vec![0; max_len];
        limbs_mul_low_same_length(&mut product_limbs, xs_adjusted, ys_adjusted);
        product_limbs
    } else {
        limbs_mul(xs, ys)
    };
    limbs_vec_mod_power_of_two_in_place(&mut product_limbs, pow);
    product_limbs
}

impl Natural {
    fn mod_power_of_two_mul_limb_ref(&self, y: Limb, pow: u64) -> Natural {
        match (&*self, y, pow) {
            (_, 0, _) | (&natural_zero!(), _, _) => Natural::ZERO,
            (_, 1, _) => self.clone(),
            (&natural_one!(), _, _) => Natural(Small(y)),
            (&Natural(Small(small)), other, pow) if pow <= Limb::WIDTH => {
                Natural(Small(small.mod_power_of_two_mul(other, pow)))
            }
            (&Natural(Small(small)), other, pow) => Natural::from(
                (DoubleLimb::from(small) * DoubleLimb::from(other)).mod_power_of_two(pow),
            ),
            (x, other, pow) => (x * Natural::from(other)).mod_power_of_two(pow),
        }
    }

    fn mod_power_of_two_mul_limb_assign(&mut self, y: Limb, pow: u64) {
        match (&mut *self, y, pow) {
            (_, 1, _) | (&mut natural_zero!(), _, _) => {}
            (_, 0, _) => *self = Natural::ZERO,
            (&mut natural_one!(), _, _) => *self = Natural(Small(y)),
            (&mut Natural(Small(ref mut small)), other, pow) if pow <= Limb::WIDTH => {
                small.mod_power_of_two_mul_assign(other, pow);
            }
            (&mut Natural(Small(small)), other, pow) => {
                *self = Natural::from(
                    (DoubleLimb::from(small) * DoubleLimb::from(other)).mod_power_of_two(pow),
                )
            }
            (x, other, pow) => {
                *x *= Natural::from(other);
                x.mod_power_of_two_assign(pow);
            }
        }
    }
}

impl ModPowerOfTwoMul<Natural> for Natural {
    type Output = Natural;

    /// Multiplies a `Natural` by a `Natural` mod 2<sup>`pow`</sup>, taking both `Natural`s by
    /// value. Assumes the inputs are already reduced mod 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: worst case O(n * log(n))
    ///
    /// where n = `pow`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_power_of_two_mul(Natural::from(2u32), 5).to_string(),
    ///     "6"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).mod_power_of_two_mul(Natural::from(14u32), 4).to_string(),
    ///     "12"
    /// );
    /// ```
    fn mod_power_of_two_mul(mut self, other: Natural, pow: u64) -> Natural {
        self.mod_power_of_two_mul_assign(other, pow);
        self
    }
}

impl<'a> ModPowerOfTwoMul<&'a Natural> for Natural {
    type Output = Natural;

    /// Multiplies a `Natural` by a `Natural` mod 2<sup>`pow`</sup>, taking the first `Natural` by
    /// value and the second by reference. Assumes the inputs are already reduced mod
    /// 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: worst case O(n * log(n))
    ///
    /// where n = `pow`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_power_of_two_mul(&Natural::from(2u32), 5).to_string(),
    ///     "6"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).mod_power_of_two_mul(&Natural::from(14u32), 4).to_string(),
    ///     "12"
    /// );
    /// ```
    fn mod_power_of_two_mul(mut self, other: &'a Natural, pow: u64) -> Natural {
        self.mod_power_of_two_mul_assign(other, pow);
        self
    }
}

impl<'a> ModPowerOfTwoMul<Natural> for &'a Natural {
    type Output = Natural;

    /// Multiplies a `Natural` by a `Natural` mod 2<sup>`pow`</sup>, taking the first `Natural` by
    /// reference and the second by value. Assumes the inputs are already reduced mod
    /// 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: worst case O(n * log(n))
    ///
    /// where n = `pow`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).mod_power_of_two_mul(Natural::from(2u32), 5).to_string(),
    ///     "6"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_power_of_two_mul(Natural::from(14u32), 4).to_string(),
    ///     "12"
    /// );
    /// ```
    fn mod_power_of_two_mul(self, mut other: Natural, pow: u64) -> Natural {
        other.mod_power_of_two_mul_assign(self, pow);
        other
    }
}

impl<'a, 'b> ModPowerOfTwoMul<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Multiplies a `Natural` by a `Natural` mod 2<sup>`pow`</sup>, taking both `Natural`s by
    /// reference. Assumes the inputs are already reduced mod 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: worst case O(n * log(n))
    ///
    /// where n = `pow`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).mod_power_of_two_mul(&Natural::from(2u32), 5).to_string(),
    ///     "6"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_power_of_two_mul(&Natural::from(14u32), 4).to_string(),
    ///     "12"
    /// );
    /// ```
    fn mod_power_of_two_mul(self, other: &'b Natural, pow: u64) -> Natural {
        match (self, other) {
            //TODO use mod_square
            (x, &Natural(Small(y))) => x.mod_power_of_two_mul_limb_ref(y, pow),
            (&Natural(Small(x)), y) => y.mod_power_of_two_mul_limb_ref(x, pow),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_mod_power_of_two_mul_ref_ref(xs, ys, pow))
            }
        }
    }
}

impl ModPowerOfTwoMulAssign<Natural> for Natural {
    /// Multiplies a `Natural` by a `Natural` mod 2<sup>`pow`</sup> in place, taking the `Natural`
    /// on the RHS by value. Assumes the inputs are already reduced mod 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `pow`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_power_of_two_mul_assign(Natural::from(2u32), 5);
    /// assert_eq!(x.to_string(), "6");
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_power_of_two_mul_assign(Natural::from(14u32), 4);
    /// assert_eq!(x.to_string(), "12");
    /// ```
    fn mod_power_of_two_mul_assign(&mut self, mut other: Natural, pow: u64) {
        match (&mut *self, &mut other) {
            (x, &mut Natural(Small(y))) => x.mod_power_of_two_mul_limb_assign(y, pow),
            (&mut Natural(Small(x)), y) => {
                y.mod_power_of_two_mul_limb_assign(x, pow);
                *self = other;
            }
            (&mut Natural(Large(ref mut xs)), &mut Natural(Large(ref mut ys))) => {
                *xs = limbs_mod_power_of_two_mul(xs, ys, pow);
                self.trim();
            }
        }
    }
}

impl<'a> ModPowerOfTwoMulAssign<&'a Natural> for Natural {
    /// Multiplies a `Natural` by a `Natural` mod 2<sup>`pow`</sup> in place, taking the `Natural`
    /// on the RHS by reference. Assumes the inputs are already reduced mod 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `pow`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_power_of_two_mul_assign(&Natural::from(2u32), 5);
    /// assert_eq!(x.to_string(), "6");
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_power_of_two_mul_assign(&Natural::from(14u32), 4);
    /// assert_eq!(x.to_string(), "12");
    /// ```
    fn mod_power_of_two_mul_assign(&mut self, other: &'a Natural, pow: u64) {
        match (&mut *self, other) {
            (x, &Natural(Small(y))) => x.mod_power_of_two_mul_limb_assign(y, pow),
            (&mut Natural(Small(x)), y) => {
                *self = y.mod_power_of_two_mul_limb_ref(x, pow);
            }
            (&mut Natural(Large(ref mut xs)), &Natural(Large(ref ys))) => {
                *xs = limbs_mod_power_of_two_mul_val_ref(xs, ys, pow);
                self.trim();
            }
        }
    }
}

use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwoAdd, ModPowerOfTwoAddAssign, ModPowerOfTwoShl, ModPowerOfTwoShlAssign, ShrRound,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use natural::arithmetic::add::{
    limbs_add_limb, limbs_slice_add_greater_in_place_left, limbs_slice_add_limb_in_place,
    limbs_slice_add_same_length_in_place_left, limbs_vec_add_in_place_left,
};
use natural::logic::bit_access::limbs_clear_bit;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the sum of the `Natural` and a `Limb`, mod 2<sup>`pow`</sup>. Assumes the input is
/// already reduced mod 2<sup>`pow`</sup>.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `xs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two_add::limbs_mod_power_of_two_add_limb;
///
/// assert_eq!(limbs_mod_power_of_two_add_limb(&[123, 456], 789, 41), &[912, 456]);
/// assert_eq!(limbs_mod_power_of_two_add_limb(&[u32::MAX, 3], 2, 34), &[1, 0]);
/// assert_eq!(limbs_mod_power_of_two_add_limb(&[u32::MAX, 3], 2, 35), &[1, 4]);
/// ```
pub fn limbs_mod_power_of_two_add_limb(xs: &[Limb], y: Limb, pow: u64) -> Vec<Limb> {
    if xs.len() < usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling)) {
        limbs_add_limb(xs, y)
    } else {
        let mut out = xs.to_vec();
        if !limbs_slice_add_limb_in_place(&mut out, y) {
            limbs_clear_bit(&mut out, pow);
        }
        out
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the sum of the `Natural` and a `Limb`, mod 2<sup>`pow`</sup>, to the input slice.
/// Returns whether there is a carry. Assumes the input is already reduced mod 2<sup>`pow`</sup>.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two_add::*;
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_slice_mod_power_of_two_add_limb_in_place(&mut xs, 789, 41), false);
/// assert_eq!(xs, &[912, 456]);
///
/// let mut xs = vec![u32::MAX];
/// assert_eq!(limbs_slice_mod_power_of_two_add_limb_in_place(&mut xs, 2, 33), true);
/// assert_eq!(xs, &[1]);
///
/// let mut xs = vec![u32::MAX];
/// assert_eq!(limbs_slice_mod_power_of_two_add_limb_in_place(&mut xs, 2, 32), false);
/// assert_eq!(xs, &[1]);
/// ```
pub fn limbs_slice_mod_power_of_two_add_limb_in_place(xs: &mut [Limb], y: Limb, pow: u64) -> bool {
    if xs.len() < usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling)) {
        limbs_slice_add_limb_in_place(xs, y)
    } else {
        if !limbs_slice_add_limb_in_place(xs, y) {
            limbs_clear_bit(xs, pow);
        }
        false
    }
}

/// Interpreting a nonempty `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`,
/// writes the limbs of the sum of the `Natural` and a `Limb`, mod 2<sup>`pow`</sup>, to the input
/// `Vec`. Assumes the input is already reduced mod 2<sup>`pow`</sup>.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two_add::*;
///
/// let mut xs = vec![123, 456];
/// limbs_vec_mod_power_of_two_add_limb_in_place(&mut xs, 789, 41);
/// assert_eq!(xs, &[912, 456]);
///
/// let mut xs = vec![u32::MAX];
/// limbs_vec_mod_power_of_two_add_limb_in_place(&mut xs, 2, 33);
/// assert_eq!(xs, &[1, 1]);
///
/// let mut xs = vec![u32::MAX];
/// limbs_vec_mod_power_of_two_add_limb_in_place(&mut xs, 2, 32);
/// assert_eq!(xs, &[1]);
/// ```
pub fn limbs_vec_mod_power_of_two_add_limb_in_place(xs: &mut Vec<Limb>, y: Limb, pow: u64) {
    assert!(!xs.is_empty());
    if limbs_slice_mod_power_of_two_add_limb_in_place(xs, y, pow) {
        xs.push(1);
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, where
/// the first slice is at least as long as the second, returns a `Vec` of the limbs of the sum of
/// the `Natural`s mod 2<sup>`pow`</sup>. Assumes the inputs are already reduced mod
/// 2<sup>`pow`</sup>.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is shorter than `ys`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two_add::limbs_mod_power_of_two_add_greater;
///
/// assert_eq!(limbs_mod_power_of_two_add_greater(&[1, 2, 3], &[6, 7], 100), &[7, 9, 3]);
/// assert_eq!(
///     limbs_mod_power_of_two_add_greater(&[100, 101, u32::MAX], &[102, 101, 2], 97),
///     &[202, 202, 1, 1]
/// );
/// assert_eq!(
///     limbs_mod_power_of_two_add_greater(&[100, 101, u32::MAX], &[102, 101, 2], 96),
///     &[202, 202, 1]
/// );
/// ```
pub fn limbs_mod_power_of_two_add_greater(xs: &[Limb], ys: &[Limb], pow: u64) -> Vec<Limb> {
    let mut out = xs.to_vec();
    if limbs_slice_mod_power_of_two_add_greater_in_place_left(&mut out, ys, pow) {
        out.push(1);
    }
    out
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns
/// a `Vec` of the limbs of the sum of the `Natural`s mod 2<sup>`pow`</sup>. Assumes the inputs are
/// already reduced mod 2<sup>`pow`</sup>.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two_add::limbs_mod_power_of_two_add;
///
/// assert_eq!(limbs_mod_power_of_two_add(&[6, 7], &[1, 2, 3], 100), &[7, 9, 3]);
/// assert_eq!(
///     limbs_mod_power_of_two_add(&[100, 101, u32::MAX], &[102, 101, 2], 97),
///     &[202, 202, 1, 1]
/// );
/// assert_eq!(
///     limbs_mod_power_of_two_add(&[100, 101, u32::MAX], &[102, 101, 2], 96),
///     &[202, 202, 1]
/// );
/// ```
pub fn limbs_mod_power_of_two_add(xs: &[Limb], ys: &[Limb], pow: u64) -> Vec<Limb> {
    if xs.len() >= ys.len() {
        limbs_mod_power_of_two_add_greater(xs, ys, pow)
    } else {
        limbs_mod_power_of_two_add_greater(ys, xs, pow)
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, where
/// the length of the first slice is greater than or equal to the length of the second, writes the
/// `xs.len()` least-significant limbs of the sum of the `Natural`s, mod 2<sup>`pow`</sup>, to the
/// first (left) slice. Returns whether there is a carry. Assumes the inputs are already reduced mod
/// 2<sup>`pow`</sup>.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is shorter than `ys`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two_add::*;
///
/// let xs = &mut [6, 7, 8];
/// assert_eq!(limbs_slice_mod_power_of_two_add_greater_in_place_left(xs, &[1, 2], 68), false);
/// assert_eq!(xs, &[7, 9, 8]);
///
/// let xs = &mut [100, 101, u32::MAX];
/// assert_eq!(
///     limbs_slice_mod_power_of_two_add_greater_in_place_left(xs, &[102, 101, 2], 97),
///     true
/// );
/// assert_eq!(xs, &[202, 202, 1]);
///
/// let xs = &mut [100, 101, u32::MAX];
/// assert_eq!(
///     limbs_slice_mod_power_of_two_add_greater_in_place_left(xs, &[102, 101, 2], 96),
///     false
/// );
/// assert_eq!(xs, &[202, 202, 1]);
/// ```
pub fn limbs_slice_mod_power_of_two_add_greater_in_place_left(
    xs: &mut [Limb],
    ys: &[Limb],
    pow: u64,
) -> bool {
    if xs.len() < usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling)) {
        limbs_slice_add_greater_in_place_left(xs, ys)
    } else {
        if !limbs_slice_add_greater_in_place_left(xs, ys) {
            limbs_clear_bit(xs, pow);
        }
        false
    }
}

/// Interpreting a `Vec` of `Limb`s and a slice of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, writes the limbs of the sum of the `Natural`s, mod 2<sup>`pow`</sup>, to the first
/// (left) slice. Assumes the inputs are already reduced mod 2<sup>`pow`</sup>.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = max(`xs.len()`, `ys.len()`), m = max(1, ys.len() - xs.len())
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two_add::*;
///
/// let mut xs = vec![6, 7, 8];
/// limbs_vec_mod_power_of_two_add_in_place_left(&mut xs, &[1, 2], 68);
/// assert_eq!(xs, &[7, 9, 8]);
///
/// let mut xs = vec![100, 101, u32::MAX];
/// limbs_vec_mod_power_of_two_add_in_place_left(&mut xs, &[102, 101, 2], 97);
/// assert_eq!(xs, &[202, 202, 1, 1]);
///
/// let mut xs = vec![100, 101, u32::MAX];
/// limbs_vec_mod_power_of_two_add_in_place_left(&mut xs, &[102, 101, 2], 96);
/// assert_eq!(xs, &[202, 202, 1]);
/// ```
pub fn limbs_vec_mod_power_of_two_add_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb], pow: u64) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let max_len = usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling));
    if xs_len < max_len && ys_len < max_len {
        limbs_vec_add_in_place_left(xs, ys)
    } else {
        let carry = if xs_len >= ys_len {
            limbs_slice_mod_power_of_two_add_greater_in_place_left(xs, ys, pow)
        } else {
            let (ys_lo, ys_hi) = ys.split_at(xs_len);
            let mut carry = limbs_slice_add_same_length_in_place_left(xs, ys_lo);
            xs.extend_from_slice(ys_hi);
            if carry {
                carry = limbs_slice_add_limb_in_place(&mut xs[xs_len..], 1);
            }
            carry
        };
        if !carry {
            limbs_clear_bit(xs, pow);
        }
    }
}

/// Interpreting two `Vec`s of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the limbs of the sum of the `Natural`s, mod 2<sup>`pow`</sup>, to the longer slice (or the first
/// one, if they are equally long). Returns a `bool` which is `false` when the output is to the
/// first `Vec` and `true` when it's to the second `Vec`. Assumes the inputs are already reduced mod
/// 2<sup>`pow`</sup>.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len`, `ys.len()`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two_add::*;
///
/// let mut xs = vec![6, 7];
/// let mut ys = vec![1, 2, 3];
/// assert_eq!(limbs_mod_power_of_two_add_in_place_either(&mut xs, &mut ys, 67), true);
/// assert_eq!(xs, &[6, 7]);
/// assert_eq!(ys, &[7, 9, 3]);
///
/// let mut xs = vec![1, 2, 3];
/// let mut ys = vec![6, 7];
/// assert_eq!(limbs_mod_power_of_two_add_in_place_either(&mut xs, &mut ys, 67), false);
/// assert_eq!(xs, &[7, 9, 3]);
/// assert_eq!(ys, &[6, 7]);
///
/// let mut xs = vec![100, 101, u32::MAX];
/// let mut ys = vec![102, 101, 2];
/// assert_eq!(limbs_mod_power_of_two_add_in_place_either(&mut xs, &mut ys, 97), false);
/// assert_eq!(xs, &[202, 202, 1, 1]);
/// assert_eq!(ys, &[102, 101, 2]);
///
/// let mut xs = vec![100, 101, u32::MAX];
/// let mut ys = vec![102, 101, 2];
/// assert_eq!(limbs_mod_power_of_two_add_in_place_either(&mut xs, &mut ys, 96), false);
/// assert_eq!(xs, &[202, 202, 1]);
/// assert_eq!(ys, &[102, 101, 2]);
/// ```
pub fn limbs_mod_power_of_two_add_in_place_either(
    xs: &mut Vec<Limb>,
    ys: &mut Vec<Limb>,
    pow: u64,
) -> bool {
    if xs.len() >= ys.len() {
        if limbs_slice_mod_power_of_two_add_greater_in_place_left(xs, ys, pow) {
            xs.push(1);
        }
        false
    } else {
        if limbs_slice_mod_power_of_two_add_greater_in_place_left(ys, xs, pow) {
            ys.push(1);
        }
        true
    }
}

impl Natural {
    fn mod_power_of_two_add_limb_ref(&self, y: Limb, pow: u64) -> Natural {
        match (&*self, y, pow) {
            (_, 0, _) => self.clone(),
            (&natural_zero!(), _, _) => Natural(Small(y)),
            (&Natural(Small(small)), other, pow) if pow <= Limb::WIDTH => {
                Natural(Small(small.mod_power_of_two_add(other, pow)))
            }
            (&Natural(Small(small)), other, _) => {
                let (sum, overflow) = small.overflowing_add(other);
                if overflow {
                    Natural(Large(vec![sum, 1]))
                } else {
                    Natural(Small(sum))
                }
            }
            (&Natural(Large(ref limbs)), other, pow) => {
                Natural::from_owned_limbs_asc(limbs_mod_power_of_two_add_limb(limbs, other, pow))
            }
        }
    }

    fn mod_power_of_two_add_assign_limb(&mut self, y: Limb, pow: u64) {
        match (&mut *self, y, pow) {
            (_, 0, _) => {}
            (&mut natural_zero!(), _, _) => *self = Natural(Small(y)),
            (&mut Natural(Small(ref mut small)), other, pow) if pow <= Limb::WIDTH => {
                small.mod_power_of_two_add_assign(other, pow)
            }
            (&mut Natural(Small(ref mut small)), other, _) => {
                let (sum, overflow) = small.overflowing_add(other);
                if overflow {
                    *self = Natural(Large(vec![sum, 1]));
                } else {
                    *small = sum;
                }
            }
            (&mut Natural(Large(ref mut limbs)), y, pow) => {
                limbs_vec_mod_power_of_two_add_limb_in_place(limbs, y, pow);
                self.trim();
            }
        }
    }
}

impl ModPowerOfTwoAdd<Natural> for Natural {
    type Output = Natural;

    /// Adds a `Natural` to a `Natural` mod 2<sup>`pow`</sup>, taking both `Natural`s by value.
    /// Assumes the inputs are already reduced mod 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `min(self.significant_bits(), other.significant_bits)`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.mod_power_of_two_add(Natural::from(2u32), 5).to_string(), "2");
    /// assert_eq!(
    ///     Natural::from(10u32).mod_power_of_two_add(Natural::from(14u32), 4).to_string(),
    ///     "8"
    /// );
    /// ```
    fn mod_power_of_two_add(mut self, other: Natural, pow: u64) -> Natural {
        self.mod_power_of_two_add_assign(other, pow);
        self
    }
}

impl<'a> ModPowerOfTwoAdd<&'a Natural> for Natural {
    type Output = Natural;

    /// Adds a `Natural` to a `Natural` mod 2<sup>`pow`</sup>, taking the left `Natural` by value
    /// and the right `Natural` by reference. Assumes the inputs are already reduced mod
    /// 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `other.significant_bits`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.mod_power_of_two_add(&Natural::from(2u32), 5).to_string(), "2");
    /// assert_eq!(
    ///     Natural::from(10u32).mod_power_of_two_add(&Natural::from(14u32), 4).to_string(),
    ///     "8"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_two_add(mut self, other: &'a Natural, pow: u64) -> Natural {
        self.mod_power_of_two_add_assign(other, pow);
        self
    }
}

impl<'a> ModPowerOfTwoAdd<Natural> for &'a Natural {
    type Output = Natural;

    /// Adds a `Natural` to a `Natural` mod 2<sup>`pow`</sup>, taking the left `Natural` by
    /// reference and the right `Natural` by value. Assumes the inputs are already reduced mod
    /// 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).mod_power_of_two_add(Natural::from(2u32), 5).to_string(), "2");
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_power_of_two_add(Natural::from(14u32), 4).to_string(),
    ///     "8"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_two_add(self, mut other: Natural, pow: u64) -> Natural {
        other.mod_power_of_two_add_assign(self, pow);
        other
    }
}

impl<'a, 'b> ModPowerOfTwoAdd<&'a Natural> for &'b Natural {
    type Output = Natural;

    /// Adds a `Natural` to a `Natural` mod 2<sup>`pow`</sup>, taking both `Natural`s by reference.
    /// Assumes the inputs are already reduced mod 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `max(self.significant_bits(), other.significant_bits)`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).mod_power_of_two_add(&Natural::from(2u32), 5).to_string(), "2");
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_power_of_two_add(&Natural::from(14u32), 4).to_string(),
    ///     "8"
    /// );
    /// ```
    fn mod_power_of_two_add(self, other: &'a Natural, pow: u64) -> Natural {
        match (self, other) {
            (x, y) if x as *const Natural == y as *const Natural => {
                self.mod_power_of_two_shl(1, pow)
            }
            (x, &Natural(Small(y))) => x.mod_power_of_two_add_limb_ref(y, pow),
            (&Natural(Small(x)), y) => y.mod_power_of_two_add_limb_ref(x, pow),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_mod_power_of_two_add(xs, ys, pow))
            }
        }
    }
}

impl ModPowerOfTwoAddAssign<Natural> for Natural {
    /// Adds a `Natural` to a `Natural` mod 2<sup>`pow`</sup> in place, taking the `Natural` on the
    /// RHS by value. Assumes the inputs are already reduced mod 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `min(self.significant_bits(), other.significant_bits)`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoAddAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x.mod_power_of_two_add_assign(Natural::from(2u32), 5);
    /// assert_eq!(x.to_string(), "2");
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_power_of_two_add_assign(Natural::from(14u32), 4);
    /// assert_eq!(x.to_string(), "8");
    /// ```
    fn mod_power_of_two_add_assign(&mut self, mut other: Natural, pow: u64) {
        match (&mut *self, &mut other) {
            (x, &mut Natural(Small(y))) => x.mod_power_of_two_add_assign_limb(y, pow),
            (&mut Natural(Small(x)), y) => *self = y.mod_power_of_two_add_limb_ref(x, pow),
            (&mut Natural(Large(ref mut xs)), _) => {
                if let Natural(Large(mut ys)) = other {
                    if limbs_mod_power_of_two_add_in_place_either(xs, &mut ys, pow) {
                        *xs = ys;
                    }
                    self.trim();
                }
            }
        }
    }
}

impl<'a> ModPowerOfTwoAddAssign<&'a Natural> for Natural {
    /// Adds a `Natural` to a `Natural` mod 2<sup>`pow`</sup> in place, taking the `Natural` on the
    /// RHS by reference. Assumes the inputs are already reduced mod 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `other.significant_bits`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoAddAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x.mod_power_of_two_add_assign(&Natural::from(2u32), 5);
    /// assert_eq!(x.to_string(), "2");
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_power_of_two_add_assign(&Natural::from(14u32), 4);
    /// assert_eq!(x.to_string(), "8");
    /// ```
    fn mod_power_of_two_add_assign(&mut self, other: &'a Natural, pow: u64) {
        match (&mut *self, other) {
            (x, y) if x as *const Natural == y as *const Natural => {
                self.mod_power_of_two_shl_assign(pow, 1);
            }
            (x, &Natural(Small(y))) => x.mod_power_of_two_add_assign_limb(y, pow),
            (&mut Natural(Small(x)), y) => *self = y.mod_power_of_two_add_limb_ref(x, pow),
            (&mut Natural(Large(ref mut xs)), &Natural(Large(ref ys))) => {
                limbs_vec_mod_power_of_two_add_in_place_left(xs, ys, pow);
                self.trim();
            }
        }
    }
}

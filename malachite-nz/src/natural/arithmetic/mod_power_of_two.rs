use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwo, ModPowerOfTwoAssign, NegModPowerOfTwo, NegModPowerOfTwoAssign, RemPowerOfTwo,
    RemPowerOfTwoAssign,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::slices::slice_set_zero;

use integer::conversion::to_twos_complement_limbs::limbs_twos_complement_in_place;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the `Natural` mod two raised to `pow`. Equivalently, retains only the least-significant
/// `pow` bits.
///
/// Time: worst case O(`pow`)
///
/// Additional memory: worst case O(`pow`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two::limbs_mod_power_of_two;
///
/// assert_eq!(limbs_mod_power_of_two(&[123, 456], 10), &[123]);
/// assert_eq!(limbs_mod_power_of_two(&[123, 456], 33), &[123, 0]);
/// assert_eq!(limbs_mod_power_of_two(&[123, 456], 40), &[123, 200]);
/// ```
///
/// This is mpz_tdiv_r_2exp from mpz/tdiv_r_2exp.c, GMP 6.1.2, where in is non-negative and the
/// result is returned.
pub fn limbs_mod_power_of_two(xs: &[Limb], pow: u64) -> Vec<Limb> {
    if pow == 0 {
        return Vec::new();
    }
    let result_limb_count = pow >> Limb::LOG_WIDTH;
    let leftover_bits = pow & Limb::WIDTH_MASK;
    let result_limb_count = usize::exact_from(result_limb_count);
    if result_limb_count >= xs.len() {
        return xs.to_vec();
    }
    let mut result = xs[..result_limb_count].to_vec();
    if leftover_bits != 0 {
        result.push(xs[result_limb_count].mod_power_of_two(leftover_bits));
    }
    result
}

//TODO test
pub fn limbs_slice_mod_power_of_two_in_place(xs: &mut [Limb], pow: u64) {
    if pow == 0 {
        slice_set_zero(xs);
        return;
    }
    let mut new_limb_count = pow >> Limb::LOG_WIDTH;
    let leftover_bits = pow & Limb::WIDTH_MASK;
    if leftover_bits != 0 {
        new_limb_count += 1;
    }
    let new_limb_count = usize::exact_from(new_limb_count);
    if new_limb_count > xs.len() {
        return;
    }
    slice_set_zero(&mut xs[new_limb_count..]);
    if leftover_bits != 0 {
        xs[new_limb_count - 1].mod_power_of_two_assign(leftover_bits);
    }
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` mod two raised to `pow` to the input `Vec`. Equivalently, retains only
/// the least-significant `pow` bits.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two::limbs_vec_mod_power_of_two_in_place;
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_mod_power_of_two_in_place(&mut limbs, 10);
/// assert_eq!(limbs, &[123]);
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_mod_power_of_two_in_place(&mut limbs, 33);
/// assert_eq!(limbs, &[123, 0]);
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_mod_power_of_two_in_place(&mut limbs, 40);
/// assert_eq!(limbs, &[123, 200]);
/// ```
///
/// This is mpz_tdiv_r_2exp from mpz/tdiv_r_2exp.c, GMP 6.1.2, where in is non-negative and
/// res == in.
pub fn limbs_vec_mod_power_of_two_in_place(xs: &mut Vec<Limb>, pow: u64) {
    if pow == 0 {
        xs.clear();
        return;
    }
    let mut new_limb_count = pow >> Limb::LOG_WIDTH;
    let leftover_bits = pow & Limb::WIDTH_MASK;
    if leftover_bits != 0 {
        new_limb_count += 1;
    }
    let new_limb_count = usize::exact_from(new_limb_count);
    if new_limb_count > xs.len() {
        return;
    }
    xs.truncate(new_limb_count);
    if leftover_bits != 0 {
        xs[new_limb_count - 1].mod_power_of_two_assign(leftover_bits);
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the negative of the `Natural` mod two raised to `pow`. Equivalently, takes the two's
/// complement and retains only the least-significant `pow` bits.
///
/// Time: worst case O(`pow`)
///
/// Additional memory: worst case O(`pow`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two::limbs_neg_mod_power_of_two;
///
/// assert_eq!(limbs_neg_mod_power_of_two(&[123, 456], 10), &[901]);
/// assert_eq!(limbs_neg_mod_power_of_two(&[123, 456], 33), &[4_294_967_173, 1]);
/// assert_eq!(limbs_neg_mod_power_of_two(&[123, 456], 40), &[4_294_967_173, 55]);
/// ```
///
/// This is mpz_tdiv_r_2exp from mpz/tdiv_r_2exp.c, GMP 6.1.2, where in is negative and the result
/// is returned. `limbs` are the limbs of -in.
pub fn limbs_neg_mod_power_of_two(xs: &[Limb], pow: u64) -> Vec<Limb> {
    let mut result_limbs = xs.to_vec();
    limbs_neg_mod_power_of_two_in_place(&mut result_limbs, pow);
    result_limbs
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the negative of the `Natural` mod two raised to `pow` to the input `Vec`. Equivalently,
/// takes the two's complement and retains only the least-significant `pow` bits.
///
/// Time: worst case O(`pow`)
///
/// Additional memory: worst case O(`pow`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two::limbs_neg_mod_power_of_two_in_place;
///
/// let mut limbs = vec![123, 456];
/// limbs_neg_mod_power_of_two_in_place(&mut limbs, 10);
/// assert_eq!(limbs, &[901]);
///
/// let mut limbs = vec![123, 456];
/// limbs_neg_mod_power_of_two_in_place(&mut limbs, 33);
/// assert_eq!(limbs, &[4_294_967_173, 1]);
///
/// let mut limbs = vec![123, 456];
/// limbs_neg_mod_power_of_two_in_place(&mut limbs, 40);
/// assert_eq!(limbs, &[4_294_967_173, 55]);
/// ```
///
/// This is mpz_tdiv_r_2exp from mpz/tdiv_r_2exp.c, GMP 6.1.2, where in is negative and res == in.
/// `limbs` are the limbs of -in.
pub fn limbs_neg_mod_power_of_two_in_place(xs: &mut Vec<Limb>, pow: u64) {
    let mut new_limb_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let leftover_bits = pow & Limb::WIDTH_MASK;
    if leftover_bits != 0 {
        new_limb_count += 1;
    }
    xs.resize(new_limb_count, 0);
    limbs_twos_complement_in_place(xs);
    if leftover_bits != 0 {
        xs[new_limb_count - 1].mod_power_of_two_assign(leftover_bits);
    }
}

impl ModPowerOfTwo for Natural {
    type Output = Natural;

    /// Takes a `Natural` mod a power of 2, taking the `Natural` by value. In other words, returns
    /// r, where `self` = q * 2<sup>`other`</sup> + r and 0 <= r < 2<sup>`other`</sup>.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwo;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Natural::from(260u32).mod_power_of_two(8).to_string(), "4");
    ///
    /// // 100 * 2^4 + 11 = 1611
    /// assert_eq!(Natural::from(1611u32).mod_power_of_two(4).to_string(), "11");
    /// ```
    #[inline]
    fn mod_power_of_two(mut self, pow: u64) -> Natural {
        self.mod_power_of_two_assign(pow);
        self
    }
}

impl<'a> ModPowerOfTwo for &'a Natural {
    type Output = Natural;

    /// Takes a `Natural` mod a power of 2, taking the `Natural` by reference. In other words,
    /// returns r, where `self` = q * 2<sup>`other`</sup> + r and 0 <= r < 2<sup>`other`</sup>.
    ///
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwo;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!((&Natural::from(260u32)).mod_power_of_two(8).to_string(), "4");
    /// // 100 * 2^4 + 11 = 1611
    /// assert_eq!((&Natural::from(1611u32)).mod_power_of_two(4).to_string(), "11");
    /// ```
    fn mod_power_of_two(self, pow: u64) -> Natural {
        match *self {
            Natural(Small(ref small)) => Natural(Small(small.mod_power_of_two(pow))),
            Natural(Large(ref limbs)) => {
                let mut result = Natural(Large(limbs_mod_power_of_two(limbs, pow)));
                result.trim();
                result
            }
        }
    }
}

impl ModPowerOfTwoAssign for Natural {
    /// Takes a `Natural` mod a power of 2 in place. In other words, replaces `self` with r, where
    /// `self` = q * 2<sup>`other`</sup> + r and 0 <= r < 2<sup>`other`</sup>.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// let mut x = Natural::from(260u32);
    /// x.mod_power_of_two_assign(8);
    /// assert_eq!(x.to_string(), "4");
    ///
    /// // 100 * 2^4 + 11 = 1611
    /// let mut x = Natural::from(1611u32);
    /// x.mod_power_of_two_assign(4);
    /// assert_eq!(x.to_string(), "11");
    /// ```
    fn mod_power_of_two_assign(&mut self, pow: u64) {
        match *self {
            Natural(Small(ref mut small)) => {
                small.mod_power_of_two_assign(pow);
                return;
            }
            Natural(Large(ref mut limbs)) => limbs_vec_mod_power_of_two_in_place(limbs, pow),
        }
        self.trim();
    }
}

impl RemPowerOfTwo for Natural {
    type Output = Natural;

    /// Takes a `Natural` rem a power of 2, taking the `Natural` by value. For `Natural`s, rem is
    /// equivalent to mod.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::RemPowerOfTwo;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Natural::from(260u32).rem_power_of_two(8).to_string(), "4");
    ///
    /// // 100 * 2^4 + 11 = 1611
    /// assert_eq!(Natural::from(1611u32).rem_power_of_two(4).to_string(), "11");
    /// ```
    #[inline]
    fn rem_power_of_two(self, pow: u64) -> Natural {
        self.mod_power_of_two(pow)
    }
}

impl<'a> RemPowerOfTwo for &'a Natural {
    type Output = Natural;

    /// Takes a `Natural` rem a power of 2, taking the `Natural` by reference. For `Natural`s, rem
    /// is equivalent to mod.
    ///
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::RemPowerOfTwo;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!((&Natural::from(260u32)).rem_power_of_two(8).to_string(), "4");
    /// // 100 * 2^4 + 11 = 1611
    /// assert_eq!((&Natural::from(1611u32)).rem_power_of_two(4).to_string(), "11");
    /// ```
    #[inline]
    fn rem_power_of_two(self, pow: u64) -> Natural {
        self.mod_power_of_two(pow)
    }
}

impl RemPowerOfTwoAssign for Natural {
    /// Takes a `Natural` rem a power of 2 in place. For `Natural`s, rem is equivalent to mod.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::RemPowerOfTwoAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// let mut x = Natural::from(260u32);
    /// x.rem_power_of_two_assign(8);
    /// assert_eq!(x.to_string(), "4");
    ///
    /// // 100 * 2^4 + 11 = 1611
    /// let mut x = Natural::from(1611u32);
    /// x.rem_power_of_two_assign(4);
    /// assert_eq!(x.to_string(), "11");
    /// ```
    #[inline]
    fn rem_power_of_two_assign(&mut self, pow: u64) {
        self.mod_power_of_two_assign(pow);
    }
}

impl NegModPowerOfTwo for Natural {
    type Output = Natural;

    /// Takes the negative of a `Natural` mod a power of 2, taking the `Natural` by value. In other
    /// words, returns r, where `self` = q * 2<sup>`other`</sup> - r and
    /// 0 <= r < 2<sup>`other`</sup>.
    ///
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::NegModPowerOfTwo;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 2^8 - 252 = 260
    /// assert_eq!(Natural::from(260u32).neg_mod_power_of_two(8).to_string(), "252");
    ///
    /// // 101 * 2^4 - 5 = 1611
    /// assert_eq!(Natural::from(1611u32).neg_mod_power_of_two(4).to_string(), "5");
    /// ```
    #[inline]
    fn neg_mod_power_of_two(mut self, other: u64) -> Natural {
        self.neg_mod_power_of_two_assign(other);
        self
    }
}

impl<'a> NegModPowerOfTwo for &'a Natural {
    type Output = Natural;

    /// Takes the negative of a `Natural` mod a power of 2, taking the `Natural` by reference. In
    /// other words, returns r, where `self` = q * 2<sup>`other`</sup> - r and
    /// 0 <= r < 2<sup>`other`</sup>.
    ///
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::NegModPowerOfTwo;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 2^8 - 252 = 260
    /// assert_eq!((&Natural::from(260u32)).neg_mod_power_of_two(8).to_string(), "252");
    /// // 101 * 2^4 - 5 = 1611
    /// assert_eq!((&Natural::from(1611u32)).neg_mod_power_of_two(4).to_string(), "5");
    /// ```
    fn neg_mod_power_of_two(self, pow: u64) -> Natural {
        if *self == 0 {
            Natural::ZERO
        } else if pow <= Limb::WIDTH {
            Natural::from(Limb::wrapping_from(self).neg_mod_power_of_two(pow))
        } else {
            let mut result = Natural(Large(match *self {
                Natural(Small(small)) => limbs_neg_mod_power_of_two(&[small], pow),
                Natural(Large(ref limbs)) => limbs_neg_mod_power_of_two(limbs, pow),
            }));
            result.trim();
            result
        }
    }
}

impl NegModPowerOfTwoAssign for Natural {
    /// Takes the negative of a `Natural` mod a power of 2 in place. In other words, replaces `self`
    /// with r, where `self` = q * 2<sup>`other`</sup> - r and 0 <= r < 2<sup>`other`</sup>.
    ///
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::NegModPowerOfTwoAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 2^8 - 252 = 260
    /// let mut x = Natural::from(260u32);
    /// x.neg_mod_power_of_two_assign(8);
    /// assert_eq!(x.to_string(), "252");
    ///
    /// // 101 * 2^4 - 5 = 1611
    /// let mut x = Natural::from(1611u32);
    /// x.neg_mod_power_of_two_assign(4);
    /// assert_eq!(x.to_string(), "5");
    /// ```
    fn neg_mod_power_of_two_assign(&mut self, pow: u64) {
        if *self == 0 {
        } else if pow <= Limb::WIDTH {
            *self = Natural::from(Limb::wrapping_from(&*self).neg_mod_power_of_two(pow));
        } else {
            let limbs = self.promote_in_place();
            limbs_neg_mod_power_of_two_in_place(limbs, pow);
            self.trim();
        }
    }
}

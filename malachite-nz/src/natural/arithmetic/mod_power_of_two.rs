use integer::conversion::to_twos_complement_limbs::limbs_twos_complement_in_place;
use malachite_base::conversion::CheckedFrom;
use malachite_base::num::integers::PrimitiveInteger;
use malachite_base::num::traits::{
    Assign, ModPowerOfTwo, ModPowerOfTwoAssign, NegModPowerOfTwo, NegModPowerOfTwoAssign,
    RemPowerOfTwo, RemPowerOfTwoAssign, Zero,
};
use natural::Natural::{self, Large, Small};
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
/// This is mpz_tdiv_r_2exp from mpz/tdiv_r_2exp.c, where in is non-negative and the result is
/// returned.
pub fn limbs_mod_power_of_two(limbs: &[Limb], pow: u64) -> Vec<Limb> {
    if pow == 0 {
        return Vec::new();
    }
    let result_limb_count = pow >> Limb::LOG_WIDTH;
    let leftover_bits = pow & u64::from(Limb::WIDTH_MASK);
    let result_limb_count = usize::checked_from(result_limb_count).unwrap();
    if result_limb_count >= limbs.len() {
        return limbs.to_vec();
    }
    let mut result = limbs[..result_limb_count].to_vec();
    if leftover_bits != 0 {
        result.push(limbs[result_limb_count].mod_power_of_two(leftover_bits));
    }
    result
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
/// use malachite_nz::natural::arithmetic::mod_power_of_two::limbs_mod_power_of_two_in_place;
///
/// let mut limbs = vec![123, 456];
/// limbs_mod_power_of_two_in_place(&mut limbs, 10);
/// assert_eq!(limbs, &[123]);
///
/// let mut limbs = vec![123, 456];
/// limbs_mod_power_of_two_in_place(&mut limbs, 33);
/// assert_eq!(limbs, &[123, 0]);
///
/// let mut limbs = vec![123, 456];
/// limbs_mod_power_of_two_in_place(&mut limbs, 40);
/// assert_eq!(limbs, &[123, 200]);
/// ```
///
/// This is mpz_tdiv_r_2exp from mpz/tdiv_r_2exp.c, where in is non-negative and res == in.
pub fn limbs_mod_power_of_two_in_place(limbs: &mut Vec<Limb>, pow: u64) {
    if pow == 0 {
        limbs.clear();
        return;
    }
    let mut new_limb_count = pow >> Limb::LOG_WIDTH;
    let leftover_bits = pow & u64::from(Limb::WIDTH_MASK);
    if leftover_bits != 0 {
        new_limb_count += 1;
    }
    let new_limb_count = usize::checked_from(new_limb_count).unwrap();
    if new_limb_count > limbs.len() {
        return;
    }
    limbs.truncate(new_limb_count);
    if leftover_bits != 0 {
        limbs[new_limb_count - 1].mod_power_of_two_assign(leftover_bits);
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
/// This is mpz_tdiv_r_2exp from mpz/tdiv_r_2exp.c, where in is negative and the result is returned.
/// `limbs` are the limbs of -in.
pub fn limbs_neg_mod_power_of_two(limbs: &[Limb], pow: u64) -> Vec<Limb> {
    let mut result_limbs = limbs.to_vec();
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
/// This is mpz_tdiv_r_2exp from mpz/tdiv_r_2exp.c, where in is negative and res == in. `limbs` are
/// the limbs of -in.
pub fn limbs_neg_mod_power_of_two_in_place(limbs: &mut Vec<Limb>, pow: u64) {
    let mut new_limb_count = usize::checked_from(pow >> Limb::LOG_WIDTH).unwrap();
    let leftover_bits = pow & u64::from(Limb::WIDTH_MASK);
    if leftover_bits != 0 {
        new_limb_count += 1;
    }
    limbs.resize(new_limb_count, 0);
    limbs_twos_complement_in_place(limbs);
    if leftover_bits != 0 {
        limbs[new_limb_count - 1].mod_power_of_two_assign(leftover_bits);
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
    /// use malachite_base::num::traits::ModPowerOfTwo;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 1 * 2^8 + 4 = 260
    ///     assert_eq!(Natural::from(260u32).mod_power_of_two(8).to_string(), "4");
    ///
    ///     // 100 * 2^4 + 11 = 1611
    ///     assert_eq!(Natural::from(1611u32).mod_power_of_two(4).to_string(), "11");
    /// }
    /// ```
    #[inline]
    fn mod_power_of_two(mut self, other: u64) -> Natural {
        self.mod_power_of_two_assign(other);
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
    /// use malachite_base::num::traits::ModPowerOfTwo;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 1 * 2^8 + 4 = 260
    ///     assert_eq!((&Natural::from(260u32)).mod_power_of_two(8).to_string(), "4");
    ///     // 100 * 2^4 + 11 = 1611
    ///     assert_eq!((&Natural::from(1611u32)).mod_power_of_two(4).to_string(), "11");
    /// }
    /// ```
    fn mod_power_of_two(self, other: u64) -> Natural {
        match *self {
            Small(ref small) => Small(small.mod_power_of_two(other)),
            Large(ref limbs) => {
                let mut result = Large(limbs_mod_power_of_two(limbs, other));
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
    /// use malachite_base::num::traits::ModPowerOfTwoAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 1 * 2^8 + 4 = 260
    ///     let mut x = Natural::from(260u32);
    ///     x.mod_power_of_two_assign(8);
    ///     assert_eq!(x.to_string(), "4");
    ///
    ///     // 100 * 2^4 + 11 = 1611
    ///     let mut x = Natural::from(1611u32);
    ///     x.mod_power_of_two_assign(4);
    ///     assert_eq!(x.to_string(), "11");
    /// }
    /// ```
    fn mod_power_of_two_assign(&mut self, other: u64) {
        match *self {
            Small(ref mut small) => {
                small.mod_power_of_two_assign(other);
                return;
            }
            Large(ref mut limbs) => limbs_mod_power_of_two_in_place(limbs, other),
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
    /// use malachite_base::num::traits::RemPowerOfTwo;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 1 * 2^8 + 4 = 260
    ///     assert_eq!(Natural::from(260u32).rem_power_of_two(8).to_string(), "4");
    ///
    ///     // 100 * 2^4 + 11 = 1611
    ///     assert_eq!(Natural::from(1611u32).rem_power_of_two(4).to_string(), "11");
    /// }
    /// ```
    #[inline]
    fn rem_power_of_two(self, other: u64) -> Natural {
        self.mod_power_of_two(other)
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
    /// use malachite_base::num::traits::RemPowerOfTwo;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 1 * 2^8 + 4 = 260
    ///     assert_eq!((&Natural::from(260u32)).rem_power_of_two(8).to_string(), "4");
    ///     // 100 * 2^4 + 11 = 1611
    ///     assert_eq!((&Natural::from(1611u32)).rem_power_of_two(4).to_string(), "11");
    /// }
    /// ```
    #[inline]
    fn rem_power_of_two(self, other: u64) -> Natural {
        self.mod_power_of_two(other)
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
    /// use malachite_base::num::traits::RemPowerOfTwoAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 1 * 2^8 + 4 = 260
    ///     let mut x = Natural::from(260u32);
    ///     x.rem_power_of_two_assign(8);
    ///     assert_eq!(x.to_string(), "4");
    ///
    ///     // 100 * 2^4 + 11 = 1611
    ///     let mut x = Natural::from(1611u32);
    ///     x.rem_power_of_two_assign(4);
    ///     assert_eq!(x.to_string(), "11");
    /// }
    /// ```
    #[inline]
    fn rem_power_of_two_assign(&mut self, other: u64) {
        self.mod_power_of_two_assign(other);
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
    /// use malachite_base::num::traits::NegModPowerOfTwo;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 2^8 - 252 = 260
    ///     assert_eq!(Natural::from(260u32).neg_mod_power_of_two(8).to_string(), "252");
    ///
    ///     // 101 * 2^4 - 5 = 1611
    ///     assert_eq!(Natural::from(1611u32).neg_mod_power_of_two(4).to_string(), "5");
    /// }
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
    /// use malachite_base::num::traits::NegModPowerOfTwo;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 2^8 - 252 = 260
    ///     assert_eq!((&Natural::from(260u32)).neg_mod_power_of_two(8).to_string(), "252");
    ///     // 101 * 2^4 - 5 = 1611
    ///     assert_eq!((&Natural::from(1611u32)).neg_mod_power_of_two(4).to_string(), "5");
    /// }
    /// ```
    fn neg_mod_power_of_two(self, other: u64) -> Natural {
        if other == 0 {
            Natural::ZERO
        } else {
            match *self {
                Small(small) => {
                    if small == 0 {
                        Natural::ZERO
                    } else if other < u64::from(Limb::WIDTH) {
                        Small(small.wrapping_neg().mod_power_of_two(other))
                    } else {
                        let mut result = Large(limbs_neg_mod_power_of_two(&[small], other));
                        result.trim();
                        result
                    }
                }
                Large(ref limbs) => {
                    let mut result = Large(limbs_neg_mod_power_of_two(limbs, other));
                    result.trim();
                    result
                }
            }
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
    /// use malachite_base::num::traits::NegModPowerOfTwoAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 2^8 - 252 = 260
    ///     let mut x = Natural::from(260u32);
    ///     x.neg_mod_power_of_two_assign(8);
    ///     assert_eq!(x.to_string(), "252");
    ///
    ///     // 101 * 2^4 - 5 = 1611
    ///     let mut x = Natural::from(1611u32);
    ///     x.neg_mod_power_of_two_assign(4);
    ///     assert_eq!(x.to_string(), "5");
    /// }
    /// ```
    fn neg_mod_power_of_two_assign(&mut self, other: u64) {
        if other == 0 {
            self.assign(Limb::ZERO);
            return;
        }
        mutate_with_possible_promotion!(
            self,
            small,
            limbs,
            {
                if *small == 0 {
                    Some(0)
                } else if other < u64::from(Limb::WIDTH) {
                    Some(small.wrapping_neg().mod_power_of_two(other))
                } else {
                    None
                }
            },
            {
                limbs_neg_mod_power_of_two_in_place(limbs, other);
            }
        );
        self.trim();
    }
}

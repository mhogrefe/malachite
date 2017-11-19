use gmp_mpfr_sys::gmp;
use malachite_base::num::{get_lower, get_upper};
use natural::Natural::{self, Large, Small};
use std::slice::from_raw_parts;

impl Natural {
    /// Returns the limbs, or base-2^(32) digits, of a `Natural`, in little-endian order, so that
    /// less significant limbs have lower indices in the output vector. There are no trailing zero
    /// limbs. Although GMP may use 32- or 64-bit limbs internally, this method always returns
    /// 32-bit limbs.
    ///
    /// This method is more efficient than `Natural::limbs_be`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_gmp;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_gmp::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert!(Natural::zero().to_limbs_le().is_empty());
    ///     assert_eq!(Natural::from(123u32).to_limbs_le(), vec![123]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Natural::from_str("1000000000000").unwrap().to_limbs_le(),
    ///             vec![3567587328, 232]);
    /// }
    /// ```
    pub fn to_limbs_le(&self) -> Vec<u32> {
        match *self {
            Small(0) => Vec::new(),
            Small(small) => vec![small],
            Large(ref large) => {
                let raw_limbs =
                    unsafe { from_raw_parts(gmp::mpz_limbs_read(large), gmp::mpz_size(large)) };
                let mut out_limbs: Vec<u32> = Vec::with_capacity(raw_limbs.len() << 1);
                for &limb in raw_limbs {
                    let limb = limb as u64;
                    out_limbs.push(get_lower(limb));
                    out_limbs.push(get_upper(limb));
                }
                if out_limbs.last().unwrap() == &0 {
                    out_limbs.pop();
                }
                out_limbs
            }
        }
    }

    /// Returns the limbs, or base-2^(32) digits, of a `Natural`, in big-endian order, so that less
    /// significant limbs have higher indices in the output vector. There are no leading zero limbs.
    /// Although GMP may use 32- or 64-bit limbs internally, this method always returns 32-bit
    /// limbs.
    ///
    /// This method is less efficient than `Natural::limbs_le`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_gmp;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_gmp::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert!(Natural::zero().to_limbs_be().is_empty());
    ///     assert_eq!(Natural::from(123u32).to_limbs_be(), vec![123]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Natural::from_str("1000000000000").unwrap().to_limbs_be(),
    ///             vec![232, 3567587328]);
    /// }
    /// ```
    pub fn to_limbs_be(&self) -> Vec<u32> {
        self.to_limbs_le().into_iter().rev().collect()
    }
}

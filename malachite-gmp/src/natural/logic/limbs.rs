use gmp_mpfr_sys::gmp;
use natural::{get_lower, get_limb_size, get_upper, LimbSize};
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
    /// use malachite_gmp::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert!(Natural::from(0u32).limbs_le().is_empty());
    /// assert_eq!(Natural::from(123u32).limbs_le(), vec![123]);
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().limbs_le(), vec![3567587328, 232]);
    /// ```
    pub fn limbs_le(&self) -> Vec<u32> {
        match *self {
            Small(0) => Vec::new(),
            Small(small) => vec![small],
            Large(ref large) => {
                let raw_limbs =
                    unsafe { from_raw_parts(gmp::mpz_limbs_read(large), gmp::mpz_size(large)) };
                match get_limb_size() {
                    LimbSize::U32 => raw_limbs.iter().map(|&i| i as u32).collect(),
                    LimbSize::U64 => {
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
    /// use malachite_gmp::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert!(Natural::from(0u32).limbs_be().is_empty());
    /// assert_eq!(Natural::from(123u32).limbs_be(), vec![123]);
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().limbs_be(), vec![232, 3567587328]);
    /// ```
    pub fn limbs_be(&self) -> Vec<u32> {
        self.limbs_le()
            .into_iter()
            .rev()
            .collect()
    }
}

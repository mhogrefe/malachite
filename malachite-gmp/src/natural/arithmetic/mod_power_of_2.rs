use gmp_mpfr_sys::gmp;
use malachite_base::traits::{Assign, Zero};
use natural::Natural::{self, Large, Small};
use std::mem;

impl Natural {
    /// Takes a `Natural` mod a power of 2, taking the `Natural` by value. In other words, returns
    /// r, where `self` = q * 2^(`other`) + r and 0 <= r < 2^(`other`).
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Natural::from(260u32).mod_power_of_2(8).to_string(), "4");
    ///
    /// // 100 * 2^4 + 11 = 1611
    /// assert_eq!(Natural::from(1611u32).mod_power_of_2(4).to_string(), "11");
    /// ```
    pub fn mod_power_of_2(mut self, other: u32) -> Natural {
        self.mod_power_of_2_assign(other);
        self
    }

    /// Takes a `Natural` mod a power of 2, taking the `Natural` by reference. In other words,
    /// returns r, where `self` = q * 2^(`other`) + r and 0 <= r < 2^(`other`).
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Natural::from(260u32).mod_power_of_2_ref(8).to_string(), "4");
    /// // 100 * 2^4 + 11 = 1611
    /// assert_eq!(Natural::from(1611u32).mod_power_of_2_ref(4).to_string(), "11");
    /// ```
    pub fn mod_power_of_2_ref(&self, other: u32) -> Natural {
        if other == 0 {
            Natural::ZERO
        } else if *self == 0 {
            self.clone()
        } else {
            match self {
                &Small(_) if other >= 32 => self.clone(),
                &Small(ref small) => Small(small & ((1 << other) - 1)),
                &Large(ref large) => unsafe {
                    let mut result = mem::uninitialized();
                    gmp::mpz_init(&mut result);
                    gmp::mpz_tdiv_r_2exp(&mut result, large, other.into());
                    let mut result = Large(result);
                    result.demote_if_small();
                    result
                },
            }
        }
    }

    /// Takes a `Natural` mod a power of 2 in place. In other words, replaces `self` with r, where
    /// `self` = q * 2^(`other`) + r and 0 <= r < 2^(`other`).
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// let mut x = Natural::from(260u32);
    /// x.mod_power_of_2_assign(8);
    /// assert_eq!(x.to_string(), "4");
    ///
    /// // 100 * 2^4 + 11 = 1611
    /// let mut x = Natural::from(1611u32);
    /// x.mod_power_of_2_assign(4);
    /// assert_eq!(x.to_string(), "11");
    /// ```
    pub fn mod_power_of_2_assign(&mut self, other: u32) {
        if other == 0 {
            self.assign(0u32);
        } else if *self == 0 {
            return;
        } else {
            match self {
                &mut Small(_) if other >= 32 => return,
                &mut Small(ref mut small) => {
                    *small &= (1 << other) - 1;
                    return;
                }
                &mut Large(ref mut large) => unsafe {
                    gmp::mpz_tdiv_r_2exp(large, large, other.into())
                },
            }
            self.demote_if_small();
        }
    }

    /// Takes a `Natural` complement-mod a power of 2, taking the `Natural` by value. In other
    /// words, returns r, where `self` = q * 2^(`other`) - r and 0 <= r < 2^(`other`).
    ///
    /// I call this the complement-mod because if b does not divide a,
    /// a.mod(b) + a.complement_mod(b) = b. (If b|a then both are 0).
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    ///
    /// // 2 * 2^8 - 252 = 260
    /// assert_eq!(Natural::from(260u32).complement_mod_power_of_2(8).to_string(), "252");
    ///
    /// // 101 * 2^4 - 5 = 1611
    /// assert_eq!(Natural::from(1611u32).complement_mod_power_of_2(4).to_string(), "5");
    /// ```
    pub fn complement_mod_power_of_2(mut self, other: u32) -> Natural {
        self.complement_mod_power_of_2_assign(other);
        self
    }

    /// Takes a `Natural` complement-mod a power of 2, taking the `Natural` by reference. In other
    /// words, returns r, where `self` = q * 2^(`other`) - r and 0 <= r < 2^(`other`).
    ///
    /// I call this the complement-mod because if b does not divide a,
    /// a.mod(b) + a.complement_mod(b) = b. (If b|a then both are 0).
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    ///
    /// // 2 * 2^8 - 252 = 260
    /// assert_eq!(Natural::from(260u32).complement_mod_power_of_2_ref(8).to_string(), "252");
    /// // 101 * 2^4 - 5 = 1611
    /// assert_eq!(Natural::from(1611u32).complement_mod_power_of_2_ref(4).to_string(), "5");
    /// ```
    pub fn complement_mod_power_of_2_ref(&self, other: u32) -> Natural {
        if other == 0 {
            Natural::ZERO
        } else if *self == 0 {
            self.clone()
        } else {
            match self {
                &Small(small) if other >= 32 => unsafe {
                    let mut result = mem::uninitialized();
                    gmp::mpz_init_set_ui(&mut result, small.into());
                    gmp::mpz_cdiv_r_2exp(&mut result, &result, other.into());
                    gmp::mpz_neg(&mut result, &result);
                    let mut result = Large(result);
                    result.demote_if_small();
                    result
                },
                &Small(ref small) => Small(small.wrapping_neg() & ((1 << other) - 1)),
                &Large(ref large) => unsafe {
                    let mut result = mem::uninitialized();
                    gmp::mpz_init(&mut result);
                    gmp::mpz_cdiv_r_2exp(&mut result, large, other.into());
                    gmp::mpz_neg(&mut result, &result);
                    let mut result = Large(result);
                    result.demote_if_small();
                    result
                },
            }
        }
    }

    /// Takes a `Natural` complement-mod a power of 2 in place. In other words, replaces `self` with
    /// r, where `self` = q * 2^(`other`) - r and 0 <= r < 2^(`other`).
    ///
    /// I call this the complement-mod because if b does not divide a,
    /// a.mod(b) + a.complement_mod(b) = b. (If b|a then both are 0).
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    ///
    /// // 2 * 2^8 - 252 = 260
    /// let mut x = Natural::from(260u32);
    /// x.complement_mod_power_of_2_assign(8);
    /// assert_eq!(x.to_string(), "252");
    ///
    /// // 101 * 2^4 - 5 = 1611
    /// let mut x = Natural::from(1611u32);
    /// x.complement_mod_power_of_2_assign(4);
    /// assert_eq!(x.to_string(), "5");
    /// ```
    pub fn complement_mod_power_of_2_assign(&mut self, other: u32) {
        if other == 0 {
            self.assign(0u32);
        } else if *self == 0 {
            return;
        } else {
            match self {
                &mut Small(small) if other >= 32 => unsafe {
                    let mut result = mem::uninitialized();
                    gmp::mpz_init_set_ui(&mut result, small.into());
                    gmp::mpz_cdiv_r_2exp(&mut result, &result, other.into());
                    gmp::mpz_neg(&mut result, &result);
                    *self = Large(result);
                },
                &mut Small(ref mut small) => {
                    *small = small.wrapping_neg() & ((1 << other) - 1);
                    return;
                }
                &mut Large(ref mut large) => unsafe {
                    gmp::mpz_cdiv_r_2exp(large, large, other.into());
                    gmp::mpz_neg(large, large);
                },
            }
            self.demote_if_small();
        }
    }
}

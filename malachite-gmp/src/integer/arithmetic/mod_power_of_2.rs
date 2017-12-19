use gmp_mpfr_sys::gmp;
use malachite_base::traits::{Assign, Zero};
use integer::Integer;
use natural::Natural;
use std::{i32, mem};

impl Integer {
    /// Takes a `Integer` mod a power of 2, taking the `Integer` by value. In other words, returns
    /// r, where `self` = q * 2^(`other`) + r and 0 <= r < 2^(`other`).
    ///
    /// Unlike rem_power_of_2, this function always returns a non-negative number.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Integer::from(260).mod_power_of_2(8).to_string(), "4");
    ///
    /// // -101 * 2^4 + 5 = -1611
    /// assert_eq!(Integer::from(-1611).mod_power_of_2(4).to_string(), "5");
    /// ```
    pub fn mod_power_of_2(mut self, other: u32) -> Natural {
        self.mod_power_of_2_assign(other);
        self.into_natural().unwrap()
    }

    /// Takes a `Integer` mod a power of 2, taking the `Integer` by reference. In other words,
    /// returns r, where `self` = q * 2^(`other`) + r and 0 <= r < 2^(`other`).
    ///
    /// Unlike rem_power_of_2_ref, this function always returns a non-negative number.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Integer::from(260).mod_power_of_2_ref(8).to_string(), "4");
    /// // -101 * 2^4 + 5 = -1611
    /// assert_eq!(Integer::from(-1611).mod_power_of_2_ref(4).to_string(), "5");
    /// ```
    pub fn mod_power_of_2_ref(&self, other: u32) -> Natural {
        if other == 0 || *self == 0 {
            Natural::ZERO
        } else {
            match self {
                &Integer::Small(small) if small >= 0 && other >= 32 => Natural::Small(small as u32),
                &Integer::Small(small) if other >= 32 => unsafe {
                    let mut result = mem::uninitialized();
                    gmp::mpz_init_set_si(&mut result, small.into());
                    gmp::mpz_fdiv_r_2exp(&mut result, &result, other.into());
                    let mut result = Natural::Large(result);
                    result.demote_if_small();
                    result
                },
                &Integer::Small(small) => Natural::Small((small & ((1 << other) - 1)) as u32),
                &Integer::Large(ref large) => unsafe {
                    let mut result = mem::uninitialized();
                    gmp::mpz_init(&mut result);
                    gmp::mpz_fdiv_r_2exp(&mut result, large, other.into());
                    let mut result = Natural::Large(result);
                    result.demote_if_small();
                    result
                },
            }
        }
    }

    /// Takes a `Integer` mod a power of 2 in place. In other words, replaces `self` with r, where
    /// `self` = q * 2^(`other`) + r and 0 <= r < 2^(`other`).
    ///
    /// Unlike rem_power_of_2_assign, this function always assigns a non-negative number.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// let mut x = Integer::from(260);
    /// x.mod_power_of_2_assign(8);
    /// assert_eq!(x.to_string(), "4");
    ///
    /// // -101 * 2^4 + 5 = -1611
    /// let mut x = Integer::from(-1611);
    /// x.mod_power_of_2_assign(4);
    /// assert_eq!(x.to_string(), "5");
    /// ```
    pub fn mod_power_of_2_assign(&mut self, other: u32) {
        if other == 0 {
            self.assign(0u32);
        } else if *self == 0 {
            return;
        } else {
            match self {
                &mut Integer::Small(small) if small >= 0 && other >= 32 => return,
                &mut Integer::Small(small) if other >= 32 => unsafe {
                    let mut large = mem::uninitialized();
                    gmp::mpz_init_set_si(&mut large, small.into());
                    gmp::mpz_fdiv_r_2exp(&mut large, &large, other.into());
                    *self = Integer::Large(large);
                },
                &mut Integer::Small(ref mut small) => {
                    *small &= (1 << other) - 1;
                    return;
                }
                &mut Integer::Large(ref mut large) => unsafe {
                    gmp::mpz_fdiv_r_2exp(large, large, other.into())
                },
            }
            self.demote_if_small();
        }
    }

    /// Takes a `Integer` rem a power of 2, taking the `Integer` by value. In other words, returns
    /// r, where `self` = q * 2^(`other`) + r, r == 0 or (sgn(r) == sgn(`self`)), and
    /// 0 <= |r| < 2^(`other`).
    ///
    /// Unlike `mod_power_of_2`, this function always returns zero or a number with the same sign as
    /// `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Integer::from(260).rem_power_of_2(8).to_string(), "4");
    ///
    /// // -100 * 2^4 + -11 = -1611
    /// assert_eq!(Integer::from(-1611).rem_power_of_2(4).to_string(), "-11");
    /// ```
    pub fn rem_power_of_2(mut self, other: u32) -> Integer {
        self.rem_power_of_2_assign(other);
        self
    }

    /// Takes a `Integer` rem a power of 2, taking the `Integer` by reference. In other words,
    /// returns r, where `self` = q * 2^(`other`) + r, (r == 0 or sgn(r) == sgn(`self`)), and
    /// 0 <= |r| < 2^(`other`).
    ///
    /// Unlike `mod_power_of_2_ref  , this function always returns zero or a number with the same
    /// sign as `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Integer::from(260).rem_power_of_2_ref(8).to_string(), "4");
    /// // -100 * 2^4 + -11 = -1611
    /// assert_eq!(Integer::from(-1611).rem_power_of_2_ref(4).to_string(), "-11");
    /// ```
    pub fn rem_power_of_2_ref(&self, other: u32) -> Integer {
        if other == 0 || *self == 0 {
            Integer::ZERO
        } else {
            match self {
                &Integer::Small(i32::MIN) if other < 32 => Integer::ZERO,
                &Integer::Small(_) if other >= 31 => self.clone(),
                &Integer::Small(small) if small >= 0 => {
                    Integer::Small((small & ((1 << other) - 1)))
                }
                &Integer::Small(small) => {
                    let mask = (1 << other) - 1;
                    if (small & mask) == 0 {
                        Integer::ZERO
                    } else {
                        Integer::Small(small | !mask)
                    }
                }
                &Integer::Large(ref large) => unsafe {
                    let mut result = mem::uninitialized();
                    gmp::mpz_init(&mut result);
                    gmp::mpz_tdiv_r_2exp(&mut result, large, other.into());
                    let mut result = Integer::Large(result);
                    result.demote_if_small();
                    result
                },
            }
        }
    }

    /// Takes a `Integer` rem a power of 2 in place. In other words, replaces `self` with r, where
    /// `self` = q * 2^(`other`) + r, (r == 0 or sgn(r) == sgn(`self`)), and 0 <= |r| < 2^(`other`).
    ///
    /// Unlike `mod_power_of_2_assign, this function does never changes the sign of `self`, except
    /// possibly to set `self` to 0.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// let mut x = Integer::from(260);
    /// x.rem_power_of_2_assign(8);
    /// assert_eq!(x.to_string(), "4");
    ///
    /// // -100 * 2^4 + -11 = -1611
    /// let mut x = Integer::from(-1611);
    /// x.rem_power_of_2_assign(4);
    /// assert_eq!(x.to_string(), "-11");
    /// ```
    pub fn rem_power_of_2_assign(&mut self, other: u32) {
        if other == 0 {
            self.assign(0u32);
        } else if *self == 0 {
            return;
        } else {
            match self {
                &mut Integer::Small(i32::MIN) if other < 32 => {
                    *self = Integer::ZERO;
                    return;
                }
                &mut Integer::Small(_) if other >= 31 => return,
                &mut Integer::Small(ref mut small) if *small >= 0 => {
                    *small &= (1 << other) - 1;
                    return;
                }
                &mut Integer::Small(ref mut small) => {
                    let mask = (1 << other) - 1;
                    if (*small & mask) == 0 {
                        *small = 0;
                    } else {
                        *small |= !mask;
                    }
                    return;
                }
                &mut Integer::Large(ref mut large) => unsafe {
                    gmp::mpz_tdiv_r_2exp(large, large, other.into())
                },
            }
            self.demote_if_small();
        }
    }

    /// Takes a `Integer` ceiling-mod a power of 2, taking the `Integer` by value. In other words,
    /// returns r, where `self` = q * 2^(`other`) + r and 0 <= -r < 2^(`other`).
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    ///
    /// // 2 * 2^8 + -252 = 260
    /// assert_eq!(Integer::from(260).ceiling_mod_power_of_2(8).to_string(), "-252");
    ///
    /// // -100 * 2^4 + -11 = -1611
    /// assert_eq!(Integer::from(-1611).ceiling_mod_power_of_2(4).to_string(), "-11");
    /// ```
    pub fn ceiling_mod_power_of_2(mut self, other: u32) -> Integer {
        self.ceiling_mod_power_of_2_assign(other);
        self
    }

    /// Takes a `Integer` ceiling-mod a power of 2, taking the `Integer` by reference. In other
    /// words, returns r, where `self` = q * 2^(`other`) + r and 0 <= -r < 2^(`other`).
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    ///
    /// // 2 * 2^8 + -252 = 260
    /// assert_eq!(Integer::from(260).ceiling_mod_power_of_2_ref(8).to_string(), "-252");
    /// // -100 * 2^4 + -11 = -1611
    /// assert_eq!(Integer::from(-1611).ceiling_mod_power_of_2_ref(4).to_string(), "-11");
    /// ```
    pub fn ceiling_mod_power_of_2_ref(&self, other: u32) -> Integer {
        if other == 0 || *self == 0 {
            Integer::ZERO
        } else {
            match self {
                &Integer::Small(i32::MIN) if other < 32 => Integer::ZERO,
                &Integer::Small(small) if small < 0 && other >= 31 => self.clone(),
                &Integer::Small(small) if small >= 0 && other < 31 || small < 0 => {
                    let mask = (1 << other) - 1;
                    if (small & mask) == 0 {
                        Integer::ZERO
                    } else {
                        Integer::Small(small | !mask)
                    }
                }
                &Integer::Small(small) => unsafe {
                    let mut result = mem::uninitialized();
                    gmp::mpz_init_set_si(&mut result, small.into());
                    gmp::mpz_cdiv_r_2exp(&mut result, &result, other.into());
                    let mut result = Integer::Large(result);
                    result.demote_if_small();
                    result
                },
                &Integer::Large(ref large) => unsafe {
                    let mut result = mem::uninitialized();
                    gmp::mpz_init(&mut result);
                    gmp::mpz_cdiv_r_2exp(&mut result, large, other.into());
                    let mut result = Integer::Large(result);
                    result.demote_if_small();
                    result
                },
            }
        }
    }

    /// Takes a `Integer` ceiling-mod a power of 2 in place. In other words, replaces `self` with r,
    /// where `self` = q * 2^(`other`) + r and 0 <= -r < 2^(`other`).
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    ///
    /// // 2 * 2^8 + -252 = 260
    /// let mut x = Integer::from(260);
    /// x.ceiling_mod_power_of_2_assign(8);
    /// assert_eq!(x.to_string(), "-252");
    ///
    /// // -100 * 2^4 + -11 = -1611
    /// let mut x = Integer::from(-1611);
    /// x.ceiling_mod_power_of_2_assign(4);
    /// assert_eq!(x.to_string(), "-11");
    /// ```
    pub fn ceiling_mod_power_of_2_assign(&mut self, other: u32) {
        if other == 0 {
            self.assign(0u32);
        } else if *self == 0 {
            return;
        } else {
            match self {
                &mut Integer::Small(i32::MIN) if other < 32 => {
                    *self = Integer::ZERO;
                }
                &mut Integer::Small(small) if small < 0 && other >= 31 => return,
                &mut Integer::Small(ref mut small) if *small >= 0 && other < 31 || *small < 0 => {
                    let mask = (1 << other) - 1;
                    if (*small & mask) == 0 {
                        *small = 0;
                    } else {
                        *small |= !mask;
                    }
                }
                &mut Integer::Small(small) => unsafe {
                    let mut result = mem::uninitialized();
                    gmp::mpz_init_set_si(&mut result, small.into());
                    gmp::mpz_cdiv_r_2exp(&mut result, &result, other.into());
                    *self = Integer::Large(result);
                },
                &mut Integer::Large(ref mut large) => unsafe {
                    gmp::mpz_cdiv_r_2exp(large, large, other.into())
                },
            }
            self.demote_if_small();
        }
    }
}

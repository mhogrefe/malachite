use malachite_base::traits::{Assign, Zero};
use natural::arithmetic::neg::mpn_neg_in_place;
use natural::{LIMB_BITS_MASK, LOG_LIMB_BITS};
use natural::Natural::{self, Large, Small};

impl Natural {
    /// Takes a `Natural` mod a power of 2, taking the `Natural` by value. In other words, returns
    /// r, where `self` = q * 2^(`other`) + r and 0 <= r < 2^(`other`).
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_native::natural::Natural;
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
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// use malachite_native::natural::Natural;
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
                &Large(ref limbs) => {
                    let result_limb_count = other >> LOG_LIMB_BITS;
                    let leftover_bits = other & LIMB_BITS_MASK;
                    let result_limb_count = result_limb_count as usize;
                    if result_limb_count >= limbs.len() {
                        return self.clone();
                    }
                    let mut result = limbs[0..result_limb_count].to_vec();
                    if leftover_bits != 0 {
                        result.push(limbs[result_limb_count] & ((1 << leftover_bits) - 1));
                    }
                    let mut result = Large(result);
                    result.trim();
                    result
                }
            }
        }
    }

    /// Takes a `Natural` mod a power of 2 in place. In other words, replaces `self` with r, where
    /// `self` = q * 2^(`other`) + r and 0 <= r < 2^(`other`).
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_native::natural::Natural;
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
                &mut Large(ref mut limbs) => {
                    let mut new_limb_count = other >> LOG_LIMB_BITS;
                    let leftover_bits = other & LIMB_BITS_MASK;
                    if leftover_bits != 0 {
                        new_limb_count += 1;
                    }
                    let new_limb_count = new_limb_count as usize;
                    if new_limb_count > limbs.len() {
                        return;
                    }
                    limbs.truncate(new_limb_count);
                    if leftover_bits != 0 {
                        limbs[new_limb_count - 1] &= (1 << leftover_bits) - 1;
                    }
                }
            }
            self.trim();
        }
    }

    /// Takes a `Natural` complement-mod a power of 2, taking the `Natural` by value. In other
    /// words, returns r, where `self` = q * 2^(`other`) - r and 0 <= r < 2^(`other`).
    ///
    /// I call this the complement-mod because if b does not divide a,
    /// a.mod(b) + a.complement_mod(b) = b. (If b|a then both are 0).
    ///
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// use malachite_native::natural::Natural;
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
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// use malachite_native::natural::Natural;
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
            if let &Small(ref small) = self {
                if other < 32 {
                    return Small(small.wrapping_neg() & ((1 << other) - 1));
                }
            }
            let mut limbs = self.to_limbs_le();
            let mut new_limb_count = other >> LOG_LIMB_BITS;
            let leftover_bits = other & LIMB_BITS_MASK;
            if leftover_bits != 0 {
                new_limb_count += 1;
            }
            let new_limb_count = new_limb_count as usize;
            limbs.resize(new_limb_count, 0);
            mpn_neg_in_place(&mut limbs);
            if leftover_bits != 0 {
                limbs[new_limb_count - 1] &= (1 << leftover_bits) - 1;
            }
            let mut result = Large(limbs);
            result.trim();
            result
        }
    }

    /// Takes a `Natural` complement-mod a power of 2 in place. In other words, replaces `self` with
    /// r, where `self` = q * 2^(`other`) - r and 0 <= r < 2^(`other`).
    ///
    /// I call this the complement-mod because if b does not divide a,
    /// a.mod(b) + a.complement_mod(b) = b. (If b|a then both are 0).
    ///
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// use malachite_native::natural::Natural;
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
            if let &mut Small(ref mut small) = self {
                if other < 32 {
                    *small = small.wrapping_neg() & ((1 << other) - 1);
                    return;
                }
            }
            {
                let limbs = self.promote_in_place();
                let mut new_limb_count = other >> LOG_LIMB_BITS;
                let leftover_bits = other & LIMB_BITS_MASK;
                if leftover_bits != 0 {
                    new_limb_count += 1;
                }
                let new_limb_count = new_limb_count as usize;
                limbs.resize(new_limb_count, 0);
                mpn_neg_in_place(limbs);
                if leftover_bits != 0 {
                    limbs[new_limb_count - 1] &= (1 << leftover_bits) - 1;
                }
            }
            self.trim();
        }
    }
}

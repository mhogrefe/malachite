use integer::conversion::to_twos_complement_limbs::limbs_slice_to_twos_complement_limbs_negative;
use malachite_base::num::{Assign, PrimitiveInteger, Zero};
use natural::Natural::{self, Large, Small};

impl Natural {
    /// Takes a `Natural` mod a power of 2, taking the `Natural` by value. In other words, returns
    /// r, where `self` = q * 2<sup>`other`</sup> + r and 0 <= r < 2<sup>`other`</sup>.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Natural::from(260u32).mod_power_of_two(8).to_string(), "4");
    ///
    /// // 100 * 2^4 + 11 = 1611
    /// assert_eq!(Natural::from(1611u32).mod_power_of_two(4).to_string(), "11");
    /// ```
    pub fn mod_power_of_two(mut self, other: u32) -> Natural {
        self.mod_power_of_two_assign(other);
        self
    }

    /// Takes a `Natural` mod a power of 2, taking the `Natural` by reference. In other words,
    /// returns r, where `self` = q * 2<sup>`other`</sup> + r and 0 <= r < 2<sup>`other`</sup>.
    ///
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Natural::from(260u32).mod_power_of_two_ref(8).to_string(), "4");
    /// // 100 * 2^4 + 11 = 1611
    /// assert_eq!(Natural::from(1611u32).mod_power_of_two_ref(4).to_string(), "11");
    /// ```
    pub fn mod_power_of_two_ref(&self, other: u32) -> Natural {
        if other == 0 {
            Natural::ZERO
        } else if *self == 0 {
            self.clone()
        } else {
            match *self {
                Small(_) if other >= u32::WIDTH => self.clone(),
                Small(ref small) => Small(small & ((1 << other) - 1)),
                Large(ref limbs) => {
                    let result_limb_count = other >> u32::LOG_WIDTH;
                    let leftover_bits = other & u32::WIDTH_MASK;
                    let result_limb_count = result_limb_count as usize;
                    if result_limb_count >= limbs.len() {
                        return self.clone();
                    }
                    let mut result = limbs[..result_limb_count].to_vec();
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
    /// `self` = q * 2<sup>`other`</sup> + r and 0 <= r < 2<sup>`other`</sup>.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
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
    pub fn mod_power_of_two_assign(&mut self, other: u32) {
        if other == 0 {
            self.assign(0u32);
        } else if *self == 0 {
            return;
        } else {
            match *self {
                Small(_) if other >= u32::WIDTH => return,
                Small(ref mut small) => {
                    *small &= (1 << other) - 1;
                    return;
                }
                Large(ref mut limbs) => {
                    let mut new_limb_count = other >> u32::LOG_WIDTH;
                    let leftover_bits = other & u32::WIDTH_MASK;
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
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 2^8 - 252 = 260
    /// assert_eq!(Natural::from(260u32).neg_mod_power_of_two(8).to_string(), "252");
    ///
    /// // 101 * 2^4 - 5 = 1611
    /// assert_eq!(Natural::from(1611u32).neg_mod_power_of_two(4).to_string(), "5");
    /// ```
    pub fn neg_mod_power_of_two(mut self, other: u32) -> Natural {
        self.neg_mod_power_of_two_assign(other);
        self
    }

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
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 2^8 - 252 = 260
    /// assert_eq!(Natural::from(260u32).neg_mod_power_of_two_ref(8).to_string(), "252");
    /// // 101 * 2^4 - 5 = 1611
    /// assert_eq!(Natural::from(1611u32).neg_mod_power_of_two_ref(4).to_string(), "5");
    /// ```
    pub fn neg_mod_power_of_two_ref(&self, other: u32) -> Natural {
        if other == 0 {
            Natural::ZERO
        } else if *self == 0 {
            self.clone()
        } else {
            if let Small(ref small) = *self {
                if other < u32::WIDTH {
                    return Small(small.wrapping_neg() & ((1 << other) - 1));
                }
            }
            let mut limbs = self.to_limbs_asc();
            let mut new_limb_count = other >> u32::LOG_WIDTH;
            let leftover_bits = other & u32::WIDTH_MASK;
            if leftover_bits != 0 {
                new_limb_count += 1;
            }
            let new_limb_count = new_limb_count as usize;
            limbs.resize(new_limb_count, 0);
            limbs_slice_to_twos_complement_limbs_negative(&mut limbs);
            if leftover_bits != 0 {
                limbs[new_limb_count - 1] &= (1 << leftover_bits) - 1;
            }
            let mut result = Large(limbs);
            result.trim();
            result
        }
    }

    /// Takes the negative of a `Natural` mod a power of 2 in place. In other words, replaces `self`
    /// with r, where `self` = q * 2<sup>`other`</sup> - r and 0 <= r < 2<sup>`other`</sup>.
    ///
    /// Time: worst case O(`other`)
    ///
    /// Additional memory: worst case O(`other`)
    ///
    /// # Examples
    /// ```
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
    pub fn neg_mod_power_of_two_assign(&mut self, other: u32) {
        if other == 0 {
            self.assign(0u32);
        } else if *self == 0 {
            return;
        } else {
            if let Small(ref mut small) = *self {
                if other < u32::WIDTH {
                    *small = small.wrapping_neg() & ((1 << other) - 1);
                    return;
                }
            }
            {
                let limbs = self.promote_in_place();
                let mut new_limb_count = other >> u32::LOG_WIDTH;
                let leftover_bits = other & u32::WIDTH_MASK;
                if leftover_bits != 0 {
                    new_limb_count += 1;
                }
                let new_limb_count = new_limb_count as usize;
                limbs.resize(new_limb_count, 0);
                limbs_slice_to_twos_complement_limbs_negative(limbs);
                if leftover_bits != 0 {
                    limbs[new_limb_count - 1] &= (1 << leftover_bits) - 1;
                }
            }
            self.trim();
        }
    }
}

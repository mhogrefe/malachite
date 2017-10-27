use gmp_mpfr_sys::gmp;
use integer::Integer;
use rand::Rng;
use std::cmp::Ordering;
use std::os::raw::c_int;
use std::slice;

impl Integer {
    //TODO test
    pub fn random_below<R: Rng>(&mut self, rng: &mut R) {
        self.random_below_raw(rng);
        self.demote_if_small();
    }

    pub fn random_below_raw<R: Rng>(&mut self, rng: &mut R) {
        assert_eq!(self.sign(), Ordering::Greater, "cannot be below zero");
        let bits = self.significant_bits();
        let limb_bits = gmp::LIMB_BITS as u64;
        let whole_limbs = (bits / limb_bits) as usize;
        let extra_bits = bits % limb_bits;
        // Avoid conditions and overflow, equivalent to:
        // let total_limbs = whole_limbs + if extra_bits == 0 { 0 } else { 1 };
        let total_limbs = whole_limbs + ((extra_bits + limb_bits - 1) / limb_bits) as usize;
        let s = self.promote_in_place();
        let limbs = unsafe { slice::from_raw_parts_mut(s.d, total_limbs) };
        // if the random number is >= bound, restart
        'restart: loop {
            let mut limbs_used: c_int = 0;
            let mut still_equal = true;
            'next_limb: for i in (0..total_limbs).rev() {
                let mut val: gmp::limb_t = rng.gen();
                if i == whole_limbs {
                    val &= ((1 as gmp::limb_t) << extra_bits) - 1;
                }
                if limbs_used == 0 && val != 0 {
                    limbs_used = i as c_int + 1;
                }
                if still_equal {
                    if val > limbs[i] {
                        continue 'restart;
                    }
                    if val == limbs[i] {
                        continue 'next_limb;
                    }
                    still_equal = false;
                }
                limbs[i] = val;
            }
            if !still_equal {
                s.size = limbs_used;
                return;
            }
        }
    }
}

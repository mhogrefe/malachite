use gmp_mpfr_sys::gmp::{self, mpz_t};
use natural::{get_limb_size, Large, LimbSize, make_u64, Natural};
use std::mem;
use std::slice::from_raw_parts_mut;
use traits::Assign;

impl Natural {
    /// Assigns a slice of limbs, or base-2^(32) digits, to `self`, in little-endian order, so that
    /// less significant limbs have lower indices in the input slice. Although GMP may use 32- or
    /// 64-bit limbs internally, this method always takes 32-bit limbs.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    ///
    /// let mut x = Natural::new();
    /// x.assign_limbs_le(&[]);
    /// assert_eq!(x.to_string(), "0");
    /// x.assign_limbs_le(&[123]);
    /// assert_eq!(x.to_string(), "123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// x.assign_limbs_le(&[3567587328, 232]);
    /// assert_eq!(x.to_string(), "1000000000000");
    /// ```
    pub fn assign_limbs_le(&mut self, limbs: &[u32]) {
        let mut sig_size = 0;
        for (i, limb) in limbs.iter().enumerate().rev() {
            if *limb != 0 {
                sig_size = i + 1;
                break;
            }
        }
        let limbs = &limbs[0..sig_size];
        match sig_size {
            0 => self.assign(0),
            1 => self.assign(limbs[0]),
            _ => unsafe {
                let mut x: mpz_t = mem::uninitialized();
                gmp::mpz_init(&mut x);
                match get_limb_size() {
                    LimbSize::U32 => {
                        let mut raw_limbs =
                            from_raw_parts_mut(gmp::mpz_limbs_write(&mut x, sig_size as i64),
                                               sig_size);
                        for (i, limb) in limbs.iter().enumerate() {
                            raw_limbs[i] = (*limb).into();
                        }
                        gmp::mpz_limbs_finish(&mut x, sig_size as i64);
                    }
                    LimbSize::U64 => {
                        let raw_sig_size = if sig_size & 1 == 0 {
                            sig_size >> 1
                        } else {
                            (sig_size >> 1) + 1
                        };
                        let mut raw_limbs =
                            from_raw_parts_mut(gmp::mpz_limbs_write(&mut x, raw_sig_size as i64),
                                               raw_sig_size);
                        for (i, chunk) in limbs.chunks(2).enumerate() {
                            if chunk.len() == 2 {
                                raw_limbs[i] = make_u64(chunk[1], chunk[0]);
                            } else {
                                raw_limbs[i] = chunk[0] as u64;
                            }
                        }
                        gmp::mpz_limbs_finish(&mut x, raw_sig_size as i64);
                    }
                }
                *self = Large(x);
            },
        }
    }
}

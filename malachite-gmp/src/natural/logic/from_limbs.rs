use gmp_mpfr_sys::gmp::{self, mpz_t};
use natural::{get_limb_size, Large, LimbSize, make_u64, Natural, Small};
use std::mem;
use std::slice::from_raw_parts_mut;

impl Natural {
    /// Converts a slice of limbs, or base-2^(32) digits, to a `Natural`, in little-endian order, so
    /// that less significant limbs have lower indices in the input slice. Although GMP may use 32-
    /// or 64-bit limbs internally, this method always takes 32-bit limbs.
    ///
    /// This method is more efficient than `from_limbs_be`.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    ///
    /// assert_eq!(Natural::from_limbs_le(&[]).to_string(), "0");
    /// assert_eq!(Natural::from_limbs_le(&[123]).to_string(), "123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::from_limbs_le(&[3567587328, 232]).to_string(), "1000000000000");
    /// ```
    pub fn from_limbs_le(limbs: &[u32]) -> Natural {
        let mut sig_size = 0;
        for (i, limb) in limbs.iter().enumerate().rev() {
            if *limb != 0 {
                sig_size = i + 1;
                break;
            }
        }
        let limbs = &limbs[0..sig_size];
        match sig_size {
            0 => Small(0),
            1 => Small(limbs[0]),
            _ => unsafe {
                let mut large: mpz_t = mem::uninitialized();
                gmp::mpz_init(&mut large);
                match get_limb_size() {
                    LimbSize::U32 => {
                        let raw_limbs = from_raw_parts_mut(
                            gmp::mpz_limbs_write(&mut large, sig_size as i64),
                            sig_size,
                        );
                        for (i, limb) in limbs.iter().enumerate() {
                            raw_limbs[i] = (*limb).into();
                        }
                        gmp::mpz_limbs_finish(&mut large, sig_size as i64);
                    }
                    LimbSize::U64 => {
                        let raw_sig_size = if sig_size & 1 == 0 {
                            sig_size >> 1
                        } else {
                            (sig_size >> 1) + 1
                        };
                        let raw_limbs = from_raw_parts_mut(
                            gmp::mpz_limbs_write(&mut large, raw_sig_size as i64),
                            raw_sig_size,
                        );
                        for (i, chunk) in limbs.chunks(2).enumerate() {
                            if chunk.len() == 2 {
                                raw_limbs[i] = make_u64(chunk[1], chunk[0]);
                            } else {
                                raw_limbs[i] = chunk[0] as u64;
                            }
                        }
                        gmp::mpz_limbs_finish(&mut large, raw_sig_size as i64);
                    }
                }
                Large(large)
            },
        }
    }

    /// Converts a slice of limbs, or base-2^(32) digits, to a `Natural`, in big-endian order, so
    /// that less significant limbs have higher indices in the input slice. Although GMP may use 32-
    /// or 64-bit limbs internally, this method always takes 32-bit limbs.
    ///
    /// This method is less efficient than `from_limbs_le`.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    ///
    /// assert_eq!(Natural::from_limbs_be(&[]).to_string(), "0");
    /// assert_eq!(Natural::from_limbs_be(&[123]).to_string(), "123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::from_limbs_be(&[232, 3567587328]).to_string(), "1000000000000");
    /// ```
    pub fn from_limbs_be(limbs: &[u32]) -> Natural {
        Natural::from_limbs_le(&limbs.iter().cloned().rev().collect::<Vec<u32>>())
    }
}

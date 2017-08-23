use gmp_mpfr_sys::gmp::{self, mpz_t};
use natural::Natural::{self, Large, Small};
use std::mem;
use traits::{AddMul, AddMulAssign};

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), taking `self` and b
/// by value.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!(Natural::from(10u32).add_mul(Natural::from(3u32), 4), 22);
/// assert_eq!(Natural::from_str("1000000000000").unwrap()
///                     .add_mul(Natural::from(65536u32), 65536).to_string(),
///            "1004294967296");
/// ```
impl AddMul<Natural, u32> for Natural {
    type Output = Natural;

    fn add_mul(mut self, b: Natural, c: u32) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), taking `self` by
/// value and b by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!(Natural::from(10u32).add_mul(&Natural::from(3u32), 4), 22);
/// assert_eq!(Natural::from_str("1000000000000").unwrap()
///                     .add_mul(&Natural::from(65536u32), 65536).to_string(),
///            "1004294967296");
/// ```
impl<'a> AddMul<&'a Natural, u32> for Natural {
    type Output = Natural;

    fn add_mul(mut self, b: &'a Natural, c: u32) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), taking `self` by
/// reference and b by value.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!((&Natural::from(10u32)).add_mul(Natural::from(3u32), 4), 22);
/// assert_eq!((&Natural::from_str("1000000000000").unwrap())
///                     .add_mul(Natural::from(65536u32), 65536).to_string(),
///            "1004294967296");
/// ```
impl<'a> AddMul<Natural, u32> for &'a Natural {
    type Output = Natural;

    fn add_mul(self, b: Natural, c: u32) -> Natural {
        self.add_mul(&b, c)
    }
}

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), taking `self` and b
/// by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!((&Natural::from(10u32)).add_mul(&Natural::from(3u32), 4), 22);
/// assert_eq!((&Natural::from_str("1000000000000").unwrap())
///                     .add_mul(&Natural::from(65536u32), 65536).to_string(),
///             "1004294967296");
/// ```
impl<'a, 'b> AddMul<&'a Natural, u32> for &'b Natural {
    type Output = Natural;

    fn add_mul(self, b: &'a Natural, c: u32) -> Natural {
        if c == 0 || *b == 0 {
            return self.clone();
        }
        if let Small(small_b) = *b {
            if let Some(product) = small_b.checked_mul(c) {
                return self + product;
            }
        }
        unsafe {
            let mut result: mpz_t = mem::uninitialized();
            match *self {
                Small(small) => gmp::mpz_init_set_ui(&mut result, small.into()),
                Large(ref large) => gmp::mpz_init_set(&mut result, large),
            }
            match b {
                &Small(small) => {
                    let mut large_b: mpz_t = mem::uninitialized();
                    gmp::mpz_init_set_ui(&mut large_b, small.into());
                    gmp::mpz_addmul_ui(&mut result, &large_b, c.into());
                }
                &Large(ref large_b) => gmp::mpz_addmul_ui(&mut result, large_b, c.into()),
            }
            Large(result)
        }
    }
}

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), in place, taking b by
/// value.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::AddMulAssign;
/// use std::str::FromStr;
///
/// let mut x = Natural::from(10u32);
/// x.add_mul_assign(Natural::from(3u32), 4);
/// assert_eq!(x, 22);
///
/// let mut x = Natural::from_str("1000000000000").unwrap();
/// x.add_mul_assign(Natural::from(65536u32), 65536);
/// assert_eq!(x.to_string(), "1004294967296");
/// ```
impl AddMulAssign<Natural, u32> for Natural {
    fn add_mul_assign(&mut self, b: Natural, c: u32) {
        self.add_mul_assign(&b, c);
    }
}

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), in place, taking b by
/// reference.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::AddMulAssign;
/// use std::str::FromStr;
///
/// let mut x = Natural::from(10u32);
/// x.add_mul_assign(&Natural::from(3u32), 4);
/// assert_eq!(x, 22);
///
/// let mut x = Natural::from_str("1000000000000").unwrap();
/// x.add_mul_assign(&Natural::from(65536u32), 65536);
/// assert_eq!(x.to_string(), "1004294967296");
/// ```
impl<'a> AddMulAssign<&'a Natural, u32> for Natural {
    fn add_mul_assign(&mut self, b: &'a Natural, c: u32) {
        if c == 0 || *b == 0 {
            return;
        }
        if let Small(small_b) = *b {
            if let Some(product) = small_b.checked_mul(c) {
                *self += product;
                return;
            }
        }
        let large_self = self.promote_in_place();
        unsafe {
            match b {
                &Small(small) => {
                    let mut large_b: mpz_t = mem::uninitialized();
                    gmp::mpz_init_set_ui(&mut large_b, small.into());
                    gmp::mpz_addmul_ui(large_self, &large_b, c.into());
                }
                &Large(ref large_b) => gmp::mpz_addmul_ui(large_self, large_b, c.into()),
            }
        }
    }
}

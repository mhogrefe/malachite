use malachite_base::num::arithmetic::traits::{Parity, Pow, PowAssign};

use integer::Integer;

impl Pow<u64> for Integer {
    type Output = Integer;

    /// TODO doc
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Integer::from(-3).pow(100).to_string(),
    ///     "515377520732011331036461129765621272702107522001"
    /// );
    /// assert_eq!(
    ///     Integer::from_str("-12345678987654321").unwrap().pow(3).to_string(),
    ///     "-1881676411868862234942354805142998028003108518161"
    /// );
    /// ```
    #[inline]
    fn pow(mut self, exp: u64) -> Integer {
        self.pow_assign(exp);
        self
    }
}

impl<'a> Pow<u64> for &'a Integer {
    type Output = Integer;

    /// TODO doc
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     (&Integer::from(-3)).pow(100).to_string(),
    ///     "515377520732011331036461129765621272702107522001"
    /// );
    /// assert_eq!(
    ///     (&Integer::from_str("-12345678987654321").unwrap()).pow(3).to_string(),
    ///     "-1881676411868862234942354805142998028003108518161"
    /// );
    /// ```
    #[inline]
    fn pow(self, exp: u64) -> Integer {
        Integer {
            sign: exp.even() || self.sign,
            abs: (&self.abs).pow(exp),
        }
    }
}

impl PowAssign<u64> for Integer {
    /// TODO doc
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// let mut x = Integer::from(-3);
    /// x.pow_assign(100);
    /// assert_eq!(x.to_string(), "515377520732011331036461129765621272702107522001");
    ///
    /// let mut x = Integer::from_str("-12345678987654321").unwrap();
    /// x.pow_assign(3);
    /// assert_eq!(x.to_string(), "-1881676411868862234942354805142998028003108518161");
    /// ```
    fn pow_assign(&mut self, exp: u64) {
        self.sign = self.sign || exp.even();
        self.abs.pow_assign(exp);
    }
}

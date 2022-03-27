use malachite_base::num::arithmetic::traits::{
    RoundToMultipleOfPowerOf2, RoundToMultipleOfPowerOf2Assign,
};
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use Rational;

impl RoundToMultipleOfPowerOf2<i64> for Rational {
    type Output = Rational;

    /// Rounds `self` to a multiple of a power of 2, according to a specified rounding mode, taking
    /// `self` by value.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of 2.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_q::Rational;
    ///
    /// let q = Rational::from(std::f64::consts::PI);
    /// assert_eq!(
    ///     q.clone().round_to_multiple_of_power_of_2(-3, RoundingMode::Floor).to_string(),
    ///     "25/8"
    /// );
    /// assert_eq!(
    ///     q.clone().round_to_multiple_of_power_of_2(-3, RoundingMode::Down).to_string(),
    ///     "25/8"
    /// );
    /// assert_eq!(
    ///     q.clone().round_to_multiple_of_power_of_2(-3, RoundingMode::Ceiling).to_string(),
    ///     "13/4"
    /// );
    /// assert_eq!(
    ///     q.clone().round_to_multiple_of_power_of_2(-3, RoundingMode::Up).to_string(),
    ///     "13/4"
    /// );
    /// assert_eq!(
    ///     q.clone().round_to_multiple_of_power_of_2(-3, RoundingMode::Nearest).to_string(),
    ///     "25/8"
    /// );
    /// ```
    #[inline]
    fn round_to_multiple_of_power_of_2(mut self, pow: i64, rm: RoundingMode) -> Rational {
        self.round_to_multiple_of_power_of_2_assign(pow, rm);
        self
    }
}

impl<'a> RoundToMultipleOfPowerOf2<i64> for &'a Rational {
    type Output = Rational;

    /// Rounds `self` to a multiple of a power of 2, according to a specified rounding mode, taking
    /// `self` by reference.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of 2.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_q::Rational;
    ///
    /// let q = Rational::from(std::f64::consts::PI);
    /// assert_eq!(
    ///     (&q).round_to_multiple_of_power_of_2(-3, RoundingMode::Floor).to_string(),
    ///     "25/8"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple_of_power_of_2(-3, RoundingMode::Down).to_string(),
    ///     "25/8"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple_of_power_of_2(-3, RoundingMode::Ceiling).to_string(),
    ///     "13/4"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple_of_power_of_2(-3, RoundingMode::Up).to_string(),
    ///     "13/4"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple_of_power_of_2(-3, RoundingMode::Nearest).to_string(),
    ///     "25/8"
    /// );
    /// ```
    fn round_to_multiple_of_power_of_2(self, pow: i64, rm: RoundingMode) -> Rational {
        Rational::from(Integer::rounding_from(self >> pow, rm)) << pow
    }
}

impl RoundToMultipleOfPowerOf2Assign<i64> for Rational {
    /// Rounds `self` to a multiple of a power of 2, according to a specified rounding mode, in
    /// place.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of 2.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2Assign;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_q::Rational;
    ///
    /// let q = Rational::from(std::f64::consts::PI);
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_of_power_of_2_assign(-3, RoundingMode::Floor);
    /// assert_eq!(x.to_string(), "25/8");
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_of_power_of_2_assign(-3, RoundingMode::Down);
    /// assert_eq!(x.to_string(), "25/8");
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_of_power_of_2_assign(-3, RoundingMode::Ceiling);
    /// assert_eq!(x.to_string(), "13/4");
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_of_power_of_2_assign(-3, RoundingMode::Up);
    /// assert_eq!(x.to_string(), "13/4");
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_of_power_of_2_assign(-3, RoundingMode::Nearest);
    /// assert_eq!(x.to_string(), "25/8");
    /// ```
    fn round_to_multiple_of_power_of_2_assign(&mut self, pow: i64, rm: RoundingMode) {
        *self >>= pow;
        *self = Rational::from(Integer::rounding_from(&*self, rm)) << pow
    }
}

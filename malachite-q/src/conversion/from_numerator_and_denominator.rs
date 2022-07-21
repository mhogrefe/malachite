use malachite_base::num::arithmetic::traits::{DivExact, Gcd, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use crate::Rational;

impl Rational {
    /// Converts two [`Natural`](malachite_nz::natural::Natural)s to a [`Rational`], taking the
    /// [`Natural`](malachite_nz::natural::Natural)s by value.
    ///
    /// The [`Natural`]s become the [`Rational`]'s numerator and denominator. Only non-negative
    /// [`Rational`]s can be produced with this function.
    ///
    /// The denominator may not be zero.
    ///
    /// The input [`Natural`](malachite_nz::natural::Natural)s may have common factors; this
    /// function reduces them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(numerator.significant_bits(), denominator.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `denominator` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from_naturals(Natural::from(4u32), Natural::from(6u32)).to_string(),
    ///     "2/3"
    /// );
    /// assert_eq!(Rational::from_naturals(Natural::ZERO, Natural::from(6u32)), 0);
    /// ```
    pub fn from_naturals(numerator: Natural, denominator: Natural) -> Rational {
        assert_ne!(denominator, 0);
        let gcd = (&numerator).gcd(&denominator);
        Rational {
            sign: true,
            numerator: numerator.div_exact(&gcd),
            denominator: denominator.div_exact(gcd),
        }
    }

    /// Converts two [`Natural`](malachite_nz::natural::Natural)s to a [`Rational`], taking the
    /// [`Natural`](malachite_nz::natural::Natural)s by reference.
    ///
    /// The [`Natural`](malachite_nz::natural::Natural)s become the [`Rational`]'s numerator and
    /// denominator. Only non-negative [`Rational`]s can be produced with this function.
    ///
    /// The denominator may not be zero.
    ///
    /// The input [`Natural`](malachite_nz::natural::Natural)s may have common factors; this
    /// function reduces them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(numerator.significant_bits(), denominator.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `denominator` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from_naturals_ref(&Natural::from(4u32), &Natural::from(6u32)).to_string(),
    ///     "2/3"
    /// );
    /// assert_eq!(Rational::from_naturals_ref(&Natural::ZERO, &Natural::from(6u32)), 0);
    /// ```
    pub fn from_naturals_ref(numerator: &Natural, denominator: &Natural) -> Rational {
        assert_ne!(*denominator, 0);
        let gcd = numerator.gcd(denominator);
        Rational {
            sign: true,
            numerator: numerator.div_exact(&gcd),
            denominator: denominator.div_exact(gcd),
        }
    }

    /// Converts two unsigned primitive integers to a [`Rational`].
    ///
    /// The integers become the [`Rational`]'s numerator and denominator. Only non-negative
    /// [`Rational`]s can be produced with this function.
    ///
    /// The denominator may not be zero.
    ///
    /// The input integers may have common factors; this function reduces them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `denominator` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from_unsigneds(4u32, 6).to_string(), "2/3");
    /// assert_eq!(Rational::from_unsigneds(0u32, 6), 0);
    /// ```
    #[inline]
    pub fn from_unsigneds<T: PrimitiveUnsigned>(numerator: T, denominator: T) -> Rational
    where
        Natural: From<T>,
    {
        Rational::from_naturals(Natural::from(numerator), Natural::from(denominator))
    }

    /// Converts two [`Integer`](malachite_nz::integer::Integer)s to a [`Rational`], taking the
    /// [`Integer`](malachite_nz::integer::Integer)s by value.
    ///
    /// The absolute values of the [`Integer`](malachite_nz::integer::Integer)s become the
    /// [`Rational`]'s numerator and denominator. The sign of the [`Rational`] is the sign of the
    /// [`Integer`](malachite_nz::integer::Integer)s' quotient.
    ///
    /// The denominator may not be zero.
    ///
    /// The input [`Integer`](malachite_nz::integer::Integer)s may have common factors; this
    /// function reduces them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(numerator.significant_bits(), denominator.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `denominator` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from_integers(Integer::from(4), Integer::from(6)).to_string(),
    ///     "2/3"
    /// );
    /// assert_eq!(
    ///     Rational::from_integers(Integer::from(4), Integer::from(-6)).to_string(),
    ///     "-2/3"
    /// );
    /// assert_eq!(Rational::from_integers(Integer::ZERO, Integer::from(6)), 0);
    /// assert_eq!(Rational::from_integers(Integer::ZERO, Integer::from(-6)), 0);
    /// ```
    pub fn from_integers(numerator: Integer, denominator: Integer) -> Rational {
        assert_ne!(denominator, 0);
        let sign = numerator == 0 || ((numerator > 0) == (denominator > 0));
        let mut q = Rational::from_naturals(numerator.unsigned_abs(), denominator.unsigned_abs());
        q.sign = sign;
        q
    }

    /// Converts two [`Integer`](malachite_nz::integer::Integer)s to a [`Rational`], taking the
    /// [`Integer`](malachite_nz::integer::Integer)s by reference.
    ///
    /// The absolute values of the [`Integer`](malachite_nz::integer::Integer)s become the
    /// [`Rational`]'s numerator and denominator. The sign of the [`Rational`] is the sign of the
    /// [`Integer`](malachite_nz::integer::Integer)s' quotient.
    ///
    /// The denominator may not be zero.
    ///
    /// The input [`Integer`](malachite_nz::integer::Integer)s may have common factors; this
    /// function reduces them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(numerator.significant_bits(), denominator.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `denominator` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from_integers_ref(&Integer::from(4), &Integer::from(6)).to_string(),
    ///     "2/3"
    /// );
    /// assert_eq!(
    ///     Rational::from_integers_ref(&Integer::from(4), &Integer::from(-6)).to_string(),
    ///     "-2/3"
    /// );
    /// assert_eq!(Rational::from_integers_ref(&Integer::ZERO, &Integer::from(6)), 0);
    /// assert_eq!(Rational::from_integers_ref(&Integer::ZERO, &Integer::from(-6)), 0);
    /// ```
    pub fn from_integers_ref(numerator: &Integer, denominator: &Integer) -> Rational {
        assert_ne!(*denominator, 0);
        let mut q = Rational::from_naturals_ref(
            numerator.unsigned_abs_ref(),
            denominator.unsigned_abs_ref(),
        );
        q.sign = *numerator == 0 || ((*numerator > 0) == (*denominator > 0));
        q
    }

    /// Converts two signed primitive integers to a [`Rational]`.
    ///
    /// The absolute values of the integers become the [`Rational`]'s numerator and denominator.
    /// The sign of the [`Rational`] is the sign of the integers' quotient.
    ///
    /// The denominator may not be zero.
    ///
    /// The input integers may have common factors; this function reduces them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `denominator` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from_signeds(4i8, 6).to_string(), "2/3");
    /// assert_eq!(Rational::from_signeds(4i8, -6).to_string(), "-2/3");
    /// assert_eq!(Rational::from_signeds(0i8, 6), 0);
    /// assert_eq!(Rational::from_signeds(0i8, -6), 0);
    /// ```
    #[inline]
    pub fn from_signeds<T: PrimitiveSigned>(numerator: T, denominator: T) -> Rational
    where
        Integer: From<T>,
    {
        Rational::from_integers(Integer::from(numerator), Integer::from(denominator))
    }

    /// Converts a sign and two [`Natural`](malachite_nz::natural::Natural)s to a [`Rational`],
    /// taking the [`Natural`](malachite_nz::natural::Natural)s by value.
    ///
    /// The [`Natural`](malachite_nz::natural::Natural)s become the [`Rational`]'s numerator and
    /// denominator, and the sign indicates whether the [`Rational`] should be non-negative. If the
    /// numerator is zero, then the [`Rational`] will be non-negative regardless of the sign.
    ///
    /// The denominator may not be zero.
    ///
    /// The input [`Natural`](malachite_nz::natural::Natural)s may have common factors; this
    /// function reduces them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(numerator.significant_bits(), denominator.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `denominator` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from_sign_and_naturals(
    ///         true,
    ///         Natural::from(4u32),
    ///         Natural::from(6u32)
    ///     ).to_string(),
    ///     "2/3"
    /// );
    /// assert_eq!(
    ///     Rational::from_sign_and_naturals(
    ///         false,
    ///         Natural::from(4u32),
    ///         Natural::from(6u32)
    ///     ).to_string(),
    ///     "-2/3"
    /// );
    /// ```
    pub fn from_sign_and_naturals(
        sign: bool,
        numerator: Natural,
        denominator: Natural,
    ) -> Rational {
        assert_ne!(denominator, 0);
        let gcd = (&numerator).gcd(&denominator);
        Rational {
            sign: sign || numerator == 0,
            numerator: numerator.div_exact(&gcd),
            denominator: denominator.div_exact(gcd),
        }
    }

    /// Converts a sign and two [`Natural`](malachite_nz::natural::Natural)s to a [`Rational`],
    /// taking the [`Natural`](malachite_nz::natural::Natural)s by reference.
    ///
    /// The [`Natural`](malachite_nz::natural::Natural)s become the [`Rational`]'s numerator and
    /// denominator, and the sign indicates whether the [`Rational`] should be non-negative. If the
    /// numerator is zero, then the [`Rational`] will be non-negative regardless of the sign.
    ///
    /// The denominator may not be zero.
    ///
    /// The input [`Natural`](malachite_nz::natural::Natural)s may have common factors; this
    /// function reduces them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(numerator.significant_bits(), denominator.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `denominator` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from_sign_and_naturals_ref(
    ///         true,
    ///         &Natural::from(4u32),
    ///         &Natural::from(6u32)
    ///     ).to_string(),
    ///     "2/3"
    /// );
    /// assert_eq!(
    ///     Rational::from_sign_and_naturals_ref(
    ///         false,
    ///         &Natural::from(4u32),
    ///         &Natural::from(6u32)
    ///     ).to_string(),
    ///     "-2/3"
    /// );
    /// ```
    pub fn from_sign_and_naturals_ref(
        sign: bool,
        numerator: &Natural,
        denominator: &Natural,
    ) -> Rational {
        assert_ne!(*denominator, 0);
        let gcd = numerator.gcd(denominator);
        Rational {
            sign: sign || *numerator == 0,
            numerator: numerator.div_exact(&gcd),
            denominator: denominator.div_exact(gcd),
        }
    }

    /// Converts a sign and two primitive unsigned integers to a [`Rational`].
    ///
    /// The integers become the [`Rational`]'s numerator and denominator, and the sign indicates
    /// whether the [`Rational`] should be non-negative. If the numerator is zero, then the
    /// [`Rational`] will be non-negative regardless of the sign.
    ///
    /// The denominator may not be zero.
    ///
    /// The input integers may have common factors; this function reduces them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `denominator` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from_sign_and_unsigneds(true, 4u32, 6).to_string(), "2/3");
    /// assert_eq!(Rational::from_sign_and_unsigneds(false, 4u32, 6).to_string(), "-2/3");
    /// ```
    pub fn from_sign_and_unsigneds<T: PrimitiveUnsigned>(
        sign: bool,
        numerator: T,
        denominator: T,
    ) -> Rational
    where
        Natural: From<T>,
    {
        Rational::from_sign_and_naturals(sign, Natural::from(numerator), Natural::from(denominator))
    }
}

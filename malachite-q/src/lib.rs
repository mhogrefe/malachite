#[macro_use]
extern crate malachite_base;
extern crate malachite_nz;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[cfg(feature = "test_build")]
extern crate itertools;
#[cfg(feature = "test_build")]
extern crate num;
#[cfg(feature = "test_build")]
extern crate rug;

use malachite_base::named::Named;
#[cfg(feature = "test_build")]
use malachite_base::num::arithmetic::traits::CoprimeWith;
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign, Gcd, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{NegativeOne, One, OneHalf, Two, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

/// A rational number.
///
/// On a 64-bit system, a `Rational` takes up 72 bytes of space on the stack.
#[derive(Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Rational {
    // whether the `Rational` is non-negative
    #[cfg_attr(feature = "serde", serde(rename = "s"))]
    pub(crate) sign: bool,
    #[cfg_attr(feature = "serde", serde(rename = "n"))]
    pub(crate) numerator: Natural,
    #[cfg_attr(feature = "serde", serde(rename = "d"))]
    pub(crate) denominator: Natural,
}

impl Rational {
    // Returns true iff `self` is valid.
    //
    // To be valid, its denominator must be nonzero, its numerator and denominator must be
    // relatively prime, and if its numerator is zero, then `sign` must be `true`. All `Rational`s
    // must be valid.
    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        self.denominator != 0
            && (self.sign || self.numerator != 0)
            && (&self.numerator).coprime_with(&self.denominator)
    }

    /// Converts two `Natural`s to a `Rational`, taking the `Natural`s by value. The `Natural`s
    /// become the `Rational`'s numerator and denominator. Only non-negative `Rational`s can be
    /// produced with this function.
    ///
    /// The denominator may not be zero.
    ///
    /// The input `Natural`s may have common factors; this function reduces them.
    ///
    /// # Worst-case complexity
    /// TODO
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

    /// Converts two `Natural`s to a `Rational`, taking the `Natural`s by referece. The `Natural`s
    /// become the `Rational`'s numerator and denominator. Only non-negative `Rational`s can be
    /// produced with this function.
    ///
    /// The denominator may not be zero.
    ///
    /// The input `Natural`s may have common factors; this function reduces them.
    ///
    /// # Worst-case complexity
    /// TODO
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

    /// Converts two unsigned integers to a `Rational`. The integers become the `Rational`'s
    /// numerator and denominator. Only non-negative `Rational`s can be produced with this
    /// function.
    ///
    /// The denominator may not be zero.
    ///
    /// The input integers may have common factors; this function reduces them.
    ///
    /// # Worst-case complexity
    /// TODO
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

    /// Converts two `Integer`s to a `Rational`, taking the `Integer`s by value. The absolute
    /// values of the `Integer`s become the `Rational`'s numerator and denominator. The sign of the
    /// `Rational` is the sign of the `Integer`s' quotient.
    ///
    /// The denominator may not be zero.
    ///
    /// The input `Integer`s may have common factors; this function reduces them.
    ///
    /// # Worst-case complexity
    /// TODO
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

    /// Converts two `Integer`s to a `Rational`, taking the `Integer`s by reference. The absolute
    /// values of the `Integer`s become the `Rational`'s numerator and denominator. The sign of
    /// the `Rational` is the sign of the `Integer`s' quotient.
    ///
    /// The denominator may not be zero.
    ///
    /// The input `Integer`s may have common factors; this function reduces them.
    ///
    /// # Worst-case complexity
    /// TODO
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

    /// Converts two signed integers to a `Rational`. The absolute values of the integers become
    /// the `Rational`'s numerator and denominator. The sign of the `Rational` is the sign of the
    /// integers' quotient.
    ///
    /// The denominator may not be zero.
    ///
    /// The input integers may have common factors; this function reduces them.
    ///
    /// # Worst-case complexity
    /// TODO
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

    /// Converts a sign and two `Natural`s to a `Rational`, taking the `Natural`s by value. The
    /// `Natural`s become the `Rational`'s numerator and denominator, and the sign indicates
    /// whether the `Rational` should be non-negative. If the numerator is zero, then the
    /// `Rational` will be non-negative regardless of the sign.
    ///
    /// The denominator may not be zero.
    ///
    /// The input `Natural`s may have common factors; this function reduces them.
    ///
    /// # Worst-case complexity
    /// TODO
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

    /// Converts a sign and two `Natural`s to a `Rational`, taking the `Natural`s by reference. The
    /// `Natural`s become the `Rational`'s numerator and denominator, and the sign indicates
    /// whether the `Rational` should be non-negative. If the numerator is zero, then the
    /// `Rational` will be non-negative regardless of the sign.
    ///
    /// The denominator may not be zero.
    ///
    /// The input `Natural`s may have common factors; this function reduces them.
    ///
    /// # Worst-case complexity
    /// TODO
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

    /// Converts a sign and two unsigned integers to a `Rational`, taking the `Natural`s by value.
    /// The integers become the `Rational`'s numerator and denominator, and the sign indicates
    /// whether the `Rational` should be non-negative. If the numerator is zero, then the
    /// `Rational` will be non-negative regardless of the sign.
    ///
    /// The denominator may not be zero.
    ///
    /// The input integers may have common factors; this function reduces them.
    ///
    /// # Worst-case complexity
    /// TODO
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

    /// Extracts the numerator of a `Rational`, taking the `Rational` by reference and cloning.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::from_str("2/3").unwrap().to_numerator(), 2);
    /// assert_eq!(Rational::from_str("0").unwrap().to_numerator(), 0);
    /// ```
    #[inline]
    pub fn to_numerator(&self) -> Natural {
        self.numerator.clone()
    }

    /// Extracts the denominator of a `Rational`, taking the `Rational` by reference and cloning.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::from_str("2/3").unwrap().to_denominator(), 3);
    /// assert_eq!(Rational::from_str("0").unwrap().to_denominator(), 1);
    /// ```
    #[inline]
    pub fn to_denominator(&self) -> Natural {
        self.denominator.clone()
    }

    /// Extracts the numerator and denominator of a `Rational`, taking the `Rational` by reference
    /// and cloning.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Rational::from_str("2/3").unwrap().to_numerator_and_denominator().to_debug_string(),
    ///     "(2, 3)"
    /// );
    /// assert_eq!(
    ///     Rational::from_str("0").unwrap().to_numerator_and_denominator().to_debug_string(),
    ///     "(0, 1)"
    /// );
    /// ```
    #[inline]
    pub fn to_numerator_and_denominator(&self) -> (Natural, Natural) {
        (self.numerator.clone(), self.denominator.clone())
    }

    /// Extracts the numerator of a `Rational`, taking the `Rational` by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::from_str("2/3").unwrap().into_numerator(), 2);
    /// assert_eq!(Rational::from_str("0").unwrap().into_numerator(), 0);
    /// ```
    #[inline]
    pub fn into_numerator(self) -> Natural {
        self.numerator
    }

    /// Extracts the denominator of a `Rational`, taking the `Rational` by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::from_str("2/3").unwrap().into_denominator(), 3);
    /// assert_eq!(Rational::from_str("0").unwrap().into_denominator(), 1);
    /// ```
    #[inline]
    pub fn into_denominator(self) -> Natural {
        self.denominator
    }

    /// Extracts the numerator and denominator of a `Rational`, taking the `Rational` by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Rational::from_str("2/3").unwrap().into_numerator_and_denominator().to_debug_string(),
    ///     "(2, 3)"
    /// );
    /// assert_eq!(
    ///     Rational::from_str("0").unwrap().into_numerator_and_denominator().to_debug_string(),
    ///     "(0, 1)"
    /// );
    /// ```
    #[inline]
    pub fn into_numerator_and_denominator(self) -> (Natural, Natural) {
        (self.numerator, self.denominator)
    }

    /// Returns a reference to the numerator of a `Rational`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(*Rational::from_str("2/3").unwrap().numerator_ref(), 2);
    /// assert_eq!(*Rational::from_str("0").unwrap().numerator_ref(), 0);
    /// ```
    #[inline]
    pub fn numerator_ref(&self) -> &Natural {
        &self.numerator
    }

    /// Returns a reference to the denominator of a `Rational`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(*Rational::from_str("2/3").unwrap().denominator_ref(), 3);
    /// assert_eq!(*Rational::from_str("0").unwrap().denominator_ref(), 1);
    /// ```
    #[inline]
    pub fn denominator_ref(&self) -> &Natural {
        &self.denominator
    }

    /// Returns references to the numeraror and denominator of a `Rational`, taking the `Rational`
    /// by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Rational::from_str("2/3").unwrap().numerator_and_denominator_ref().to_debug_string(),
    ///     "(2, 3)"
    /// );
    /// assert_eq!(
    ///     Rational::from_str("0").unwrap().numerator_and_denominator_ref().to_debug_string(),
    ///     "(0, 1)"
    /// );
    /// ```
    #[inline]
    pub fn numerator_and_denominator_ref(&self) -> (&Natural, &Natural) {
        (&self.numerator, &self.denominator)
    }

    /// Mutates the numerator of a `Rational` using a provided closure, and then returns
    /// whatever the closure returns.
    ///
    /// After the closure executes, this function reduces the `Rational`.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// let mut q = Rational::from_str("22/7").unwrap();
    /// let ret = q.mutate_numerator(|x| {
    ///     *x -= Natural::ONE;
    ///     true
    /// });
    /// assert_eq!(q, 3);
    /// assert_eq!(ret, true);
    /// ```
    pub fn mutate_numerator<F: FnOnce(&mut Natural) -> T, T>(&mut self, f: F) -> T {
        let out = f(&mut self.numerator);
        let gcd = (&self.numerator).gcd(&self.denominator);
        self.numerator.div_exact_assign(&gcd);
        self.denominator.div_exact_assign(gcd);
        if !self.sign && self.numerator == 0 {
            self.sign = true;
        }
        out
    }

    /// Mutates the denominator of a `Rational` using a provided closure.
    ///
    /// After the closure executes, this function reduces the `Rational`.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if the closure sets the denominator to zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// let mut q = Rational::from_str("22/7").unwrap();
    /// let ret = q.mutate_denominator(|x| {
    ///     *x -= Natural::ONE;
    ///     true
    /// });
    /// assert_eq!(q.to_string(), "11/3");
    /// assert_eq!(ret, true);
    /// ```
    pub fn mutate_denominator<F: FnOnce(&mut Natural) -> T, T>(&mut self, f: F) -> T {
        let out = f(&mut self.denominator);
        assert_ne!(self.denominator, 0);
        let gcd = (&self.numerator).gcd(&self.denominator);
        self.numerator.div_exact_assign(&gcd);
        self.denominator.div_exact_assign(gcd);
        out
    }

    /// Mutates the numerator and denominator of a `Rational` using a provided closure.
    ///
    /// After the closure executes, this function reduces the `Rational`.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if the closure sets the denominator to zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// let mut q = Rational::from_str("22/7").unwrap();
    /// let ret = q.mutate_numerator_and_denominator(|x, y| {
    ///     *x -= Natural::ONE;
    ///     *y -= Natural::ONE;
    ///     true
    /// });
    /// assert_eq!(q.to_string(), "7/2");
    /// assert_eq!(ret, true);
    /// ```
    pub fn mutate_numerator_and_denominator<F: FnOnce(&mut Natural, &mut Natural) -> T, T>(
        &mut self,
        f: F,
    ) -> T {
        let out = f(&mut self.numerator, &mut self.denominator);
        assert_ne!(self.denominator, 0);
        let gcd = (&self.numerator).gcd(&self.denominator);
        self.numerator.div_exact_assign(&gcd);
        self.denominator.div_exact_assign(gcd);
        if !self.sign && self.numerator == 0 {
            self.sign = true;
        }
        out
    }
}

impl<'a> SignificantBits for &'a Rational {
    /// Returns the sum of the bits needed to represent the numerator and denominator.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::logic::traits::SignificantBits;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::ZERO.significant_bits(), 1);
    /// assert_eq!(Rational::from_str("-100/101").unwrap().significant_bits(), 14);
    /// ```
    fn significant_bits(self) -> u64 {
        self.numerator.significant_bits() + self.denominator.significant_bits()
    }
}

/// The constant 0.
///
/// # Worst-case complexity
/// Constant time and additional memory.
impl Zero for Rational {
    const ZERO: Rational = Rational {
        sign: true,
        numerator: Natural::ZERO,
        denominator: Natural::ONE,
    };
}

/// The constant 1.
///
/// # Worst-case complexity
/// Constant time and additional memory.
impl One for Rational {
    const ONE: Rational = Rational {
        sign: true,
        numerator: Natural::ONE,
        denominator: Natural::ONE,
    };
}

/// The constant 2.
///
/// # Worst-case complexity
/// Constant time and additional memory.
impl Two for Rational {
    const TWO: Rational = Rational {
        sign: true,
        numerator: Natural::TWO,
        denominator: Natural::ONE,
    };
}

/// The constant -1.
///
///
/// # Worst-case complexity
/// Constant time and additional memory.
impl NegativeOne for Rational {
    const NEGATIVE_ONE: Rational = Rational {
        sign: false,
        numerator: Natural::ONE,
        denominator: Natural::ONE,
    };
}

/// The constant 1/2.
///
///
/// # Worst-case complexity
/// Constant time and additional memory.
impl OneHalf for Rational {
    const ONE_HALF: Rational = Rational {
        sign: true,
        numerator: Natural::ONE,
        denominator: Natural::TWO,
    };
}

impl Default for Rational {
    /// The default value of a `Rational`, 0.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    fn default() -> Rational {
        Rational::ZERO
    }
}

// Implement `Named` for `Rational`.
impl_named!(Rational);

pub mod arithmetic;
pub mod comparison;
pub mod conversion;
pub mod exhaustive;
pub mod random;

#[cfg(feature = "test_build")]
pub mod test_util;

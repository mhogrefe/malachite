use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    DoubleFactorial, Factorial, Multifactorial, SaturatingSubAssign, Subfactorial,
};
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::NotAssign;

pub fn factorial_naive(mut n: u64) -> Natural {
    let mut f = Natural::ONE;
    while n != 0 {
        f *= Natural::from(n);
        n -= 1;
    }
    f
}

pub fn double_factorial_naive(mut n: u64) -> Natural {
    let mut f = Natural::ONE;
    while n != 0 {
        f *= Natural::from(n);
        n.saturating_sub_assign(2);
    }
    f
}

pub fn multifactorial_naive(mut n: u64, m: u64) -> Natural {
    assert_ne!(m, 0);
    let mut f = Natural::ONE;
    while n != 0 {
        f *= Natural::from(n);
        n.saturating_sub_assign(m);
    }
    f
}

pub fn subfactorial_naive(n: u64) -> Natural {
    let mut f = Natural::ONE;
    let mut b = true;
    for i in 1..=n {
        f *= Natural::from(i);
        if b {
            f -= Natural::ONE;
        } else {
            f += Natural::ONE;
        }
        b.not_assign();
    }
    f
}

impl Factorial for Natural {
    /// Computes the factorial of a number.
    ///
    /// $$
    /// f(n) = n! = 1 \times 2 \times 3 \times \cdots \times n.
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Factorial;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::factorial(0), 1);
    /// assert_eq!(Natural::factorial(1), 1);
    /// assert_eq!(Natural::factorial(2), 2);
    /// assert_eq!(Natural::factorial(3), 6);
    /// assert_eq!(Natural::factorial(4), 24);
    /// assert_eq!(Natural::factorial(5), 120);
    /// assert_eq!(
    ///     Natural::factorial(100).to_string(),
    ///     "9332621544394415268169923885626670049071596826438162146859296389521759999322991560894\
    ///     1463976156518286253697920827223758251185210916864000000000000000000000000"
    /// );
    /// ```
    #[inline]
    fn factorial(n: u64) -> Natural {
        factorial_naive(n)
    }
}

impl DoubleFactorial for Natural {
    /// Computes the double factorial of a number.
    ///
    /// $$
    /// f(n) = n!! = n \times (n - 2) \times (n - 4) \times \cdots \times i,
    /// $$
    /// where $i$ is 1 if $n$ is odd and $2$ if $n$ is even.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DoubleFactorial;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::double_factorial(0), 1);
    /// assert_eq!(Natural::double_factorial(1), 1);
    /// assert_eq!(Natural::double_factorial(2), 2);
    /// assert_eq!(Natural::double_factorial(3), 3);
    /// assert_eq!(Natural::double_factorial(4), 8);
    /// assert_eq!(Natural::double_factorial(5), 15);
    /// assert_eq!(Natural::double_factorial(6), 48);
    /// assert_eq!(Natural::double_factorial(7), 105);
    /// assert_eq!(
    ///     Natural::double_factorial(99).to_string(),
    ///     "2725392139750729502980713245400918633290796330545803413734328823443106201171875"
    /// );
    /// assert_eq!(
    ///     Natural::double_factorial(100).to_string(),
    ///     "34243224702511976248246432895208185975118675053719198827915654463488000000000000"
    /// );
    /// ```
    #[inline]
    fn double_factorial(n: u64) -> Natural {
        double_factorial_naive(n)
    }
}

impl Multifactorial for Natural {
    /// Computes a multifactorial of a number.
    ///
    /// $$
    /// f(n, m) = n!^{(m)} = n \times (n - m) \times (n - 2m) \times \cdots \times i.
    /// $$
    /// If $n$ is divisible by $m$, then $i$ is $m$; otherwise, $i$ is the remainder when $n$ is
    /// divided by $m$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Multifactorial;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::multifactorial(0, 1), 1);
    /// assert_eq!(Natural::multifactorial(1, 1), 1);
    /// assert_eq!(Natural::multifactorial(2, 1), 2);
    /// assert_eq!(Natural::multifactorial(3, 1), 6);
    /// assert_eq!(Natural::multifactorial(4, 1), 24);
    /// assert_eq!(Natural::multifactorial(5, 1), 120);
    /// 
    /// assert_eq!(Natural::multifactorial(0, 2), 1);
    /// assert_eq!(Natural::multifactorial(1, 2), 1);
    /// assert_eq!(Natural::multifactorial(2, 2), 2);
    /// assert_eq!(Natural::multifactorial(3, 2), 3);
    /// assert_eq!(Natural::multifactorial(4, 2), 8);
    /// assert_eq!(Natural::multifactorial(5, 2), 15);
    /// assert_eq!(Natural::multifactorial(6, 2), 48);
    /// assert_eq!(Natural::multifactorial(7, 2), 105);
    /// 
    /// assert_eq!(Natural::multifactorial(0, 3), 1);
    /// assert_eq!(Natural::multifactorial(1, 3), 1);
    /// assert_eq!(Natural::multifactorial(2, 3), 2);
    /// assert_eq!(Natural::multifactorial(3, 3), 3);
    /// assert_eq!(Natural::multifactorial(4, 3), 4);
    /// assert_eq!(Natural::multifactorial(5, 3), 10);
    /// assert_eq!(Natural::multifactorial(6, 3), 18);
    /// assert_eq!(Natural::multifactorial(7, 3), 28);
    /// assert_eq!(Natural::multifactorial(8, 3), 80);
    /// assert_eq!(Natural::multifactorial(9, 3), 162);
    /// 
    /// assert_eq!(
    ///     Natural::multifactorial(100, 3).to_string(),
    ///     "174548867015437739741494347897360069928419328000000000"
    /// );
    /// ```
    #[inline]
    fn multifactorial(n: u64, m: u64) -> Natural {
        multifactorial_naive(n, m)
    }
}

impl Subfactorial for Natural {
    /// Computes the subfactorial of a number.
    ///
    /// The subfactorial of $n$ counts the number of derangements of a set of size $n$; a
    /// derangement is a permutation with no fixed points.
    ///
    /// $$
    /// f(n) = \\ !n = \lfloor n!/e \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Subfactorial;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::subfactorial(0), 1);
    /// assert_eq!(Natural::subfactorial(1), 0);
    /// assert_eq!(Natural::subfactorial(2), 1);
    /// assert_eq!(Natural::subfactorial(3), 2);
    /// assert_eq!(Natural::subfactorial(4), 9);
    /// assert_eq!(Natural::subfactorial(5), 44);
    /// assert_eq!(
    ///     Natural::subfactorial(100).to_string(),
    ///     "3433279598416380476519597752677614203236578380537578498354340028268518079332763243279\
    ///     1396429850988990237345920155783984828001486412574060553756854137069878601"
    /// );
    /// ```
    #[inline]
    fn subfactorial(n: u64) -> Natural {
        subfactorial_naive(n)
    }
}

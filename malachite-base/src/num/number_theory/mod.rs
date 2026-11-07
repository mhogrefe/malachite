// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// [`BellNumber`](traits::BellNumber) and [`CheckedBellNumber`](traits::CheckedBellNumber), traits
/// for computing Bell numbers. The first panics if the result cannot be represented; the second
/// returns `None` in that case.
///
/// # bell_number
/// ```
/// use malachite_base::num::number_theory::traits::BellNumber;
///
/// assert_eq!(u8::bell_number(0), 1);
/// assert_eq!(u8::bell_number(4), 15);
/// assert_eq!(u8::bell_number(6), 203);
/// assert_eq!(u32::bell_number(10), 115975);
/// assert_eq!(u64::bell_number(20), 51724158235372);
/// ```
///
/// # checked_bell_number
/// ```
/// use malachite_base::num::number_theory::traits::CheckedBellNumber;
///
/// assert_eq!(u8::checked_bell_number(0), Some(1));
/// assert_eq!(u8::checked_bell_number(6), Some(203));
/// assert_eq!(u8::checked_bell_number(7), None);
/// assert_eq!(u32::checked_bell_number(10), Some(115975));
/// assert_eq!(u32::checked_bell_number(100), None);
/// ```
pub mod bell_number;
/// Traits for computing the binomial coefficient of two numbers. There is a trait whose
/// implementations panic if the result cannot be represented, and a checked trait whose
/// implementations return `None` in that case: [`BinomialCoefficient`](traits::BinomialCoefficient)
/// and [`CheckedBinomialCoefficient`](traits::CheckedBinomialCoefficient).
///
/// # binomial_coefficient
/// ```
/// use malachite_base::num::number_theory::traits::BinomialCoefficient;
///
/// assert_eq!(u8::binomial_coefficient(3, 0), 1);
/// assert_eq!(u8::binomial_coefficient(3, 1), 3);
/// assert_eq!(u8::binomial_coefficient(3, 2), 3);
/// assert_eq!(u8::binomial_coefficient(3, 3), 1);
/// assert_eq!(u8::binomial_coefficient(10, 5), 252);
///
/// assert_eq!(i8::binomial_coefficient(-3, 0), 1);
/// assert_eq!(i8::binomial_coefficient(-3, 1), -3);
/// assert_eq!(i8::binomial_coefficient(-3, 2), 6);
/// assert_eq!(i8::binomial_coefficient(-3, 3), -10);
/// ```
///
/// # checked_binomial_coefficient
/// ```
/// use malachite_base::num::number_theory::traits::CheckedBinomialCoefficient;
///
/// assert_eq!(u8::checked_binomial_coefficient(3, 0), Some(1));
/// assert_eq!(u8::checked_binomial_coefficient(3, 1), Some(3));
/// assert_eq!(u8::checked_binomial_coefficient(3, 2), Some(3));
/// assert_eq!(u8::checked_binomial_coefficient(3, 3), Some(1));
/// assert_eq!(u8::checked_binomial_coefficient(10, 5), Some(252));
/// assert_eq!(u8::checked_binomial_coefficient(11, 5), None);
///
/// assert_eq!(i8::checked_binomial_coefficient(-3, 0), Some(1));
/// assert_eq!(i8::checked_binomial_coefficient(-3, 1), Some(-3));
/// assert_eq!(i8::checked_binomial_coefficient(-3, 2), Some(6));
/// assert_eq!(i8::checked_binomial_coefficient(-3, 3), Some(-10));
/// assert_eq!(i8::checked_binomial_coefficient(-3, -3), None);
/// assert_eq!(i8::checked_binomial_coefficient(11, 5), None);
/// ```
pub mod binomial_coefficient;
/// [`CoprimeWith`](traits::CoprimeWith), a trait for determining whether two numbers are coprime.
///
/// # coprime_with
/// ```
/// use malachite_base::num::number_theory::traits::CoprimeWith;
///
/// assert_eq!(0u8.coprime_with(0), false);
/// assert_eq!(0u8.coprime_with(1), true);
/// assert_eq!(6u8.coprime_with(1), true);
/// assert_eq!(3u8.coprime_with(5), true);
/// assert_eq!(6u8.coprime_with(4), false);
/// assert_eq!(6u8.coprime_with(35), true);
/// ```
pub mod coprime_with;
/// [`ExtendedGcd`](traits::ExtendedGcd), a trait for computing the GCD (greatest common divisor) of
/// two numbers as well as the coefficients of Bézout's identity $ax+by=\gcd(a,b)$.
///
/// # extended_gcd
/// ```
/// use malachite_base::num::number_theory::traits::ExtendedGcd;
///
/// assert_eq!(3u8.extended_gcd(5), (1, 2, -1));
/// assert_eq!(240u16.extended_gcd(46), (2, -9, 47));
/// assert_eq!((-111i16).extended_gcd(300), (3, 27, 10));
/// ```
pub mod extended_gcd;
/// Traits for computing the factorial, double factorial, multifactorial, and subfactorial. Each
/// function has a trait whose implementations panic if the result cannot be represented, and a
/// checked trait whose implementations return `None` in that case. The traits are
/// [`Factorial`](traits::Factorial), [`DoubleFactorial`](traits::DoubleFactorial),
/// [`Multifactorial`](traits::Multifactorial), [`Subfactorial`](traits::Subfactorial),
/// [`CheckedFactorial`](traits::CheckedFactorial),
/// [`CheckedDoubleFactorial`](traits::CheckedDoubleFactorial),
/// [`CheckedMultifactorial`](traits::CheckedMultifactorial), and
/// [`CheckedSubfactorial`](traits::CheckedSubfactorial).
///
/// # factorial
/// ```
/// use malachite_base::num::number_theory::traits::Factorial;
///
/// assert_eq!(u8::factorial(0), 1);
/// assert_eq!(u8::factorial(1), 1);
/// assert_eq!(u8::factorial(2), 2);
/// assert_eq!(u8::factorial(3), 6);
/// assert_eq!(u8::factorial(4), 24);
/// assert_eq!(u8::factorial(5), 120);
/// assert_eq!(u32::factorial(10), 3628800);
/// ```
///
/// # checked_factorial
/// ```
/// use malachite_base::num::number_theory::traits::CheckedFactorial;
///
/// assert_eq!(u8::checked_factorial(0), Some(1));
/// assert_eq!(u8::checked_factorial(1), Some(1));
/// assert_eq!(u8::checked_factorial(2), Some(2));
/// assert_eq!(u8::checked_factorial(3), Some(6));
/// assert_eq!(u8::checked_factorial(4), Some(24));
/// assert_eq!(u8::checked_factorial(5), Some(120));
/// assert_eq!(u8::checked_factorial(6), None);
/// assert_eq!(u32::checked_factorial(10), Some(3628800));
/// assert_eq!(u32::checked_factorial(100), None);
/// ```
///
/// # double_factorial
/// ```
/// use malachite_base::num::number_theory::traits::DoubleFactorial;
///
/// assert_eq!(u8::double_factorial(0), 1);
/// assert_eq!(u8::double_factorial(1), 1);
/// assert_eq!(u8::double_factorial(2), 2);
/// assert_eq!(u8::double_factorial(3), 3);
/// assert_eq!(u8::double_factorial(4), 8);
/// assert_eq!(u8::double_factorial(5), 15);
/// assert_eq!(u8::double_factorial(6), 48);
/// assert_eq!(u8::double_factorial(7), 105);
/// assert_eq!(u32::double_factorial(19), 654729075);
/// assert_eq!(u32::double_factorial(20), 3715891200);
/// ```
///
/// # checked_double_factorial
/// ```
/// use malachite_base::num::number_theory::traits::CheckedDoubleFactorial;
///
/// assert_eq!(u8::checked_double_factorial(0), Some(1));
/// assert_eq!(u8::checked_double_factorial(1), Some(1));
/// assert_eq!(u8::checked_double_factorial(2), Some(2));
/// assert_eq!(u8::checked_double_factorial(3), Some(3));
/// assert_eq!(u8::checked_double_factorial(4), Some(8));
/// assert_eq!(u8::checked_double_factorial(5), Some(15));
/// assert_eq!(u8::checked_double_factorial(6), Some(48));
/// assert_eq!(u8::checked_double_factorial(7), Some(105));
/// assert_eq!(u8::checked_double_factorial(8), None);
/// assert_eq!(u32::checked_double_factorial(19), Some(654729075));
/// assert_eq!(u32::checked_double_factorial(20), Some(3715891200));
/// assert_eq!(u32::checked_double_factorial(100), None);
/// ```
///
/// # multifactorial
/// ```
/// use malachite_base::num::number_theory::traits::Multifactorial;
///
/// assert_eq!(u8::multifactorial(0, 1), 1);
/// assert_eq!(u8::multifactorial(1, 1), 1);
/// assert_eq!(u8::multifactorial(2, 1), 2);
/// assert_eq!(u8::multifactorial(3, 1), 6);
/// assert_eq!(u8::multifactorial(4, 1), 24);
/// assert_eq!(u8::multifactorial(5, 1), 120);
///
/// assert_eq!(u8::multifactorial(0, 2), 1);
/// assert_eq!(u8::multifactorial(1, 2), 1);
/// assert_eq!(u8::multifactorial(2, 2), 2);
/// assert_eq!(u8::multifactorial(3, 2), 3);
/// assert_eq!(u8::multifactorial(4, 2), 8);
/// assert_eq!(u8::multifactorial(5, 2), 15);
/// assert_eq!(u8::multifactorial(6, 2), 48);
/// assert_eq!(u8::multifactorial(7, 2), 105);
///
/// assert_eq!(u8::multifactorial(0, 3), 1);
/// assert_eq!(u8::multifactorial(1, 3), 1);
/// assert_eq!(u8::multifactorial(2, 3), 2);
/// assert_eq!(u8::multifactorial(3, 3), 3);
/// assert_eq!(u8::multifactorial(4, 3), 4);
/// assert_eq!(u8::multifactorial(5, 3), 10);
/// assert_eq!(u8::multifactorial(6, 3), 18);
/// assert_eq!(u8::multifactorial(7, 3), 28);
/// assert_eq!(u8::multifactorial(8, 3), 80);
/// assert_eq!(u8::multifactorial(9, 3), 162);
///
/// assert_eq!(u32::multifactorial(10, 1), 3628800);
/// assert_eq!(u32::multifactorial(20, 2), 3715891200);
/// assert_eq!(u32::multifactorial(25, 3), 608608000);
/// ```
///
/// # checked_multifactorial
/// ```
/// use malachite_base::num::number_theory::traits::CheckedMultifactorial;
///
/// assert_eq!(u8::checked_multifactorial(0, 1), Some(1));
/// assert_eq!(u8::checked_multifactorial(1, 1), Some(1));
/// assert_eq!(u8::checked_multifactorial(2, 1), Some(2));
/// assert_eq!(u8::checked_multifactorial(3, 1), Some(6));
/// assert_eq!(u8::checked_multifactorial(4, 1), Some(24));
/// assert_eq!(u8::checked_multifactorial(5, 1), Some(120));
/// assert_eq!(u8::checked_multifactorial(6, 1), None);
///
/// assert_eq!(u8::checked_multifactorial(0, 2), Some(1));
/// assert_eq!(u8::checked_multifactorial(1, 2), Some(1));
/// assert_eq!(u8::checked_multifactorial(2, 2), Some(2));
/// assert_eq!(u8::checked_multifactorial(3, 2), Some(3));
/// assert_eq!(u8::checked_multifactorial(4, 2), Some(8));
/// assert_eq!(u8::checked_multifactorial(5, 2), Some(15));
/// assert_eq!(u8::checked_multifactorial(6, 2), Some(48));
/// assert_eq!(u8::checked_multifactorial(7, 2), Some(105));
/// assert_eq!(u8::checked_multifactorial(8, 2), None);
///
/// assert_eq!(u8::checked_multifactorial(0, 3), Some(1));
/// assert_eq!(u8::checked_multifactorial(1, 3), Some(1));
/// assert_eq!(u8::checked_multifactorial(2, 3), Some(2));
/// assert_eq!(u8::checked_multifactorial(3, 3), Some(3));
/// assert_eq!(u8::checked_multifactorial(4, 3), Some(4));
/// assert_eq!(u8::checked_multifactorial(5, 3), Some(10));
/// assert_eq!(u8::checked_multifactorial(6, 3), Some(18));
/// assert_eq!(u8::checked_multifactorial(7, 3), Some(28));
/// assert_eq!(u8::checked_multifactorial(8, 3), Some(80));
/// assert_eq!(u8::checked_multifactorial(9, 3), Some(162));
/// assert_eq!(u8::checked_multifactorial(10, 3), None);
///
/// assert_eq!(u32::checked_multifactorial(10, 1), Some(3628800));
/// assert_eq!(u32::checked_multifactorial(20, 2), Some(3715891200));
/// assert_eq!(u32::checked_multifactorial(25, 3), Some(608608000));
/// assert_eq!(u32::checked_multifactorial(100, 1), None);
/// assert_eq!(u32::checked_multifactorial(100, 2), None);
/// assert_eq!(u32::checked_multifactorial(100, 3), None);
/// ```
///
/// # subfactorial
/// ```
/// use malachite_base::num::number_theory::traits::Subfactorial;
///
/// assert_eq!(u8::subfactorial(0), 1);
/// assert_eq!(u8::subfactorial(1), 0);
/// assert_eq!(u8::subfactorial(2), 1);
/// assert_eq!(u8::subfactorial(3), 2);
/// assert_eq!(u8::subfactorial(4), 9);
/// assert_eq!(u8::subfactorial(5), 44);
/// assert_eq!(u32::subfactorial(10), 1334961);
/// ```
///
/// # checked_subfactorial
/// ```
/// use malachite_base::num::number_theory::traits::CheckedSubfactorial;
///
/// assert_eq!(u8::checked_subfactorial(0), Some(1));
/// assert_eq!(u8::checked_subfactorial(1), Some(0));
/// assert_eq!(u8::checked_subfactorial(2), Some(1));
/// assert_eq!(u8::checked_subfactorial(3), Some(2));
/// assert_eq!(u8::checked_subfactorial(4), Some(9));
/// assert_eq!(u8::checked_subfactorial(5), Some(44));
/// assert_eq!(u8::checked_subfactorial(6), None);
/// assert_eq!(u32::checked_subfactorial(10), Some(1334961));
/// assert_eq!(u32::checked_subfactorial(100), None);
/// ```
pub mod factorial;
/// [`FallingFactorial`](traits::FallingFactorial) and
/// [`CheckedFallingFactorial`](traits::CheckedFallingFactorial), traits for computing the falling
/// factorial of a number.
///
/// # falling_factorial
/// ```
/// use malachite_base::num::number_theory::traits::FallingFactorial;
///
/// assert_eq!(3u8.falling_factorial(0), 1);
/// assert_eq!(6u16.falling_factorial(4), 360);
/// assert_eq!(6u32.falling_factorial(6), 720);
/// assert_eq!(3u64.falling_factorial(5), 0);
/// ```
///
/// # checked_falling_factorial
/// ```
/// use malachite_base::num::number_theory::traits::CheckedFallingFactorial;
///
/// assert_eq!(6u16.checked_falling_factorial(4), Some(360));
/// assert_eq!(6u8.checked_falling_factorial(4), None);
/// assert_eq!(3u8.checked_falling_factorial(300), Some(0));
/// ```
pub mod falling_factorial;
/// Traits for computing Fibonacci and Lucas numbers, either alone or paired with their
/// predecessors. Each function has a trait whose implementations panic if the result cannot be
/// represented, and a checked trait whose implementations return `None` in that case. The traits
/// are [`Fibonacci`](traits::Fibonacci), [`LucasNumber`](traits::LucasNumber),
/// [`CheckedFibonacci`](traits::CheckedFibonacci), and
/// [`CheckedLucasNumber`](traits::CheckedLucasNumber).
///
/// # fibonacci
/// ```
/// use malachite_base::num::number_theory::traits::Fibonacci;
///
/// assert_eq!(u8::fibonacci(0), 0);
/// assert_eq!(u8::fibonacci(1), 1);
/// assert_eq!(u8::fibonacci(2), 1);
/// assert_eq!(u8::fibonacci(3), 2);
/// assert_eq!(u8::fibonacci(10), 55);
/// assert_eq!(u32::fibonacci(30), 832040);
/// ```
///
/// # fibonacci_pair
/// ```
/// use malachite_base::num::number_theory::traits::Fibonacci;
///
/// assert_eq!(u8::fibonacci_pair(0), (0, 1));
/// assert_eq!(u8::fibonacci_pair(1), (1, 0));
/// assert_eq!(u8::fibonacci_pair(2), (1, 1));
/// assert_eq!(u8::fibonacci_pair(10), (55, 34));
/// assert_eq!(u32::fibonacci_pair(30), (832040, 514229));
/// ```
///
/// # checked_fibonacci
/// ```
/// use malachite_base::num::number_theory::traits::CheckedFibonacci;
///
/// assert_eq!(u8::checked_fibonacci(0), Some(0));
/// assert_eq!(u8::checked_fibonacci(10), Some(55));
/// assert_eq!(u8::checked_fibonacci(13), Some(233));
/// assert_eq!(u8::checked_fibonacci(14), None);
/// assert_eq!(u32::checked_fibonacci(100), None);
/// ```
///
/// # checked_fibonacci_pair
/// ```
/// use malachite_base::num::number_theory::traits::CheckedFibonacci;
///
/// assert_eq!(u8::checked_fibonacci_pair(0), Some((0, 1)));
/// assert_eq!(u8::checked_fibonacci_pair(10), Some((55, 34)));
/// assert_eq!(u8::checked_fibonacci_pair(13), Some((233, 144)));
/// assert_eq!(u8::checked_fibonacci_pair(14), None);
/// ```
///
/// # lucas_number
/// ```
/// use malachite_base::num::number_theory::traits::LucasNumber;
///
/// assert_eq!(u8::lucas_number(0), 2);
/// assert_eq!(u8::lucas_number(1), 1);
/// assert_eq!(u8::lucas_number(2), 3);
/// assert_eq!(u8::lucas_number(10), 123);
/// assert_eq!(u32::lucas_number(30), 1860498);
/// ```
///
/// # lucas_number_pair
/// ```
/// use malachite_base::num::number_theory::traits::LucasNumber;
///
/// assert_eq!(u8::lucas_number_pair(1), (1, 2));
/// assert_eq!(u8::lucas_number_pair(2), (3, 1));
/// assert_eq!(u8::lucas_number_pair(10), (123, 76));
/// assert_eq!(u32::lucas_number_pair(30), (1860498, 1149851));
/// ```
///
/// # checked_lucas_number
/// ```
/// use malachite_base::num::number_theory::traits::CheckedLucasNumber;
///
/// assert_eq!(u8::checked_lucas_number(0), Some(2));
/// assert_eq!(u8::checked_lucas_number(10), Some(123));
/// assert_eq!(u8::checked_lucas_number(11), Some(199));
/// assert_eq!(u8::checked_lucas_number(12), None);
/// assert_eq!(u32::checked_lucas_number(100), None);
/// ```
///
/// # checked_lucas_number_pair
/// ```
/// use malachite_base::num::number_theory::traits::CheckedLucasNumber;
///
/// assert_eq!(u8::checked_lucas_number_pair(0), None);
/// assert_eq!(u8::checked_lucas_number_pair(1), Some((1, 2)));
/// assert_eq!(u8::checked_lucas_number_pair(10), Some((123, 76)));
/// assert_eq!(u8::checked_lucas_number_pair(12), None);
/// ```
pub mod fibonacci;
/// [`Gcd`](traits::Gcd) and [`GcdAssign`](traits::GcdAssign), traits for computing the GCD
/// (greatest common divisor) of two numbers.
///
/// # gcd
/// ```
/// use malachite_base::num::number_theory::traits::Gcd;
///
/// assert_eq!(3u8.gcd(5), 1);
/// assert_eq!(12u16.gcd(90), 6);
/// ```
///
/// # gcd_assign
/// ```
/// use malachite_base::num::number_theory::traits::GcdAssign;
///
/// let mut x = 3u8;
/// x.gcd_assign(5);
/// assert_eq!(x, 1);
///
/// let mut x = 12u16;
/// x.gcd_assign(90);
/// assert_eq!(x, 6);
/// ```
pub mod gcd;
/// [`LegendreSymbol`](traits::LegendreSymbol), [`JacobiSymbol`](traits::JacobiSymbol), and
/// [`KroneckerSymbol`](traits::KroneckerSymbol), traits for computing the Legendre, Jacobi, and
/// Kronecker symbols of two numbers.
///
/// # legendre_symbol
/// ```
/// use malachite_base::num::number_theory::traits::LegendreSymbol;
///
/// assert_eq!(10u8.legendre_symbol(5), 0);
/// assert_eq!(7u8.legendre_symbol(5), -1);
/// assert_eq!(11u8.legendre_symbol(5), 1);
///
/// assert_eq!((-7i8).legendre_symbol(5), -1);
/// assert_eq!((-11i8).legendre_symbol(5), 1);
/// ```
///
/// # jacobi_symbol
/// ```
/// use malachite_base::num::number_theory::traits::JacobiSymbol;
///
/// assert_eq!(10u8.jacobi_symbol(5), 0);
/// assert_eq!(7u8.jacobi_symbol(5), -1);
/// assert_eq!(11u8.jacobi_symbol(5), 1);
/// assert_eq!(11u8.jacobi_symbol(9), 1);
///
/// assert_eq!((-7i8).jacobi_symbol(5), -1);
/// assert_eq!((-11i8).jacobi_symbol(5), 1);
/// assert_eq!((-11i8).jacobi_symbol(9), 1);
/// ```
///
/// # kronecker_symbol
/// ```
/// use malachite_base::num::number_theory::traits::KroneckerSymbol;
///
/// assert_eq!(10u8.kronecker_symbol(5), 0);
/// assert_eq!(7u8.kronecker_symbol(5), -1);
/// assert_eq!(11u8.kronecker_symbol(5), 1);
/// assert_eq!(11u8.kronecker_symbol(9), 1);
/// assert_eq!(11u8.kronecker_symbol(8), -1);
///
/// assert_eq!((-7i8).kronecker_symbol(5), -1);
/// assert_eq!((-11i8).kronecker_symbol(5), 1);
/// assert_eq!((-11i8).kronecker_symbol(9), 1);
/// assert_eq!((-11i8).kronecker_symbol(8), -1);
/// assert_eq!((-11i8).kronecker_symbol(-8), 1);
/// ```
pub mod kronecker_symbol;
/// [`Lcm`](traits::Lcm), [`LcmAssign`](traits::LcmAssign), and [`CheckedLcm`](traits::CheckedLcm),
/// traits for computing the LCM (least common multiple) of two numbers.
///
/// # lcm
/// ```
/// use malachite_base::num::number_theory::traits::Lcm;
///
/// assert_eq!(3u8.lcm(5), 15);
/// assert_eq!(12u16.lcm(90), 180);
/// ```
///
/// # lcm_assign
/// ```
/// use malachite_base::num::number_theory::traits::LcmAssign;
///
/// let mut x = 3u8;
/// x.lcm_assign(5);
/// assert_eq!(x, 15);
///
/// let mut x = 12u16;
/// x.lcm_assign(90);
/// assert_eq!(x, 180);
/// ```
///
/// # checked_lcm
/// ```
/// use malachite_base::num::number_theory::traits::CheckedLcm;
///
/// assert_eq!(3u8.checked_lcm(5), Some(15));
/// assert_eq!(12u16.checked_lcm(90), Some(180));
/// assert_eq!(120u8.checked_lcm(90), None);
/// ```
pub mod lcm;
/// Traits for computing the primorial and the product of the first $n$ primes. There is a trait
/// whose implementations panic if the result cannot be represented, and a checked trait whose
/// implementations return `None` in that case: [`Primorial`](traits::Primorial) and
/// [`CheckedPrimorial`](traits::CheckedPrimorial).
///
/// # primorial
/// ```
/// use malachite_base::num::number_theory::traits::Primorial;
///
/// assert_eq!(u8::primorial(0), 1);
/// assert_eq!(u8::primorial(1), 1);
/// assert_eq!(u8::primorial(2), 2);
/// assert_eq!(u8::primorial(3), 6);
/// assert_eq!(u8::primorial(4), 6);
/// assert_eq!(u8::primorial(5), 30);
/// assert_eq!(u32::primorial(20), 9699690);
/// ```
///
/// # product_of_first_n_primes
/// ```
/// use malachite_base::num::number_theory::traits::Primorial;
///
/// assert_eq!(u8::product_of_first_n_primes(0), 1);
/// assert_eq!(u8::product_of_first_n_primes(1), 2);
/// assert_eq!(u8::product_of_first_n_primes(2), 6);
/// assert_eq!(u8::product_of_first_n_primes(3), 30);
/// assert_eq!(u8::product_of_first_n_primes(4), 210);
/// assert_eq!(u32::product_of_first_n_primes(9), 223092870);
/// ```
///
/// # checked_primorial
/// ```
/// use malachite_base::num::number_theory::traits::CheckedPrimorial;
///
/// assert_eq!(u8::checked_primorial(0), Some(1));
/// assert_eq!(u8::checked_primorial(1), Some(1));
/// assert_eq!(u8::checked_primorial(2), Some(2));
/// assert_eq!(u8::checked_primorial(3), Some(6));
/// assert_eq!(u8::checked_primorial(4), Some(6));
/// assert_eq!(u8::checked_primorial(5), Some(30));
///
/// assert_eq!(u8::checked_primorial(11), None);
/// assert_eq!(u32::checked_primorial(20), Some(9699690));
/// assert_eq!(u32::checked_primorial(100), None);
/// ```
///
/// # checked_product_of_first_n_primes
/// ```
/// use malachite_base::num::number_theory::traits::CheckedPrimorial;
///
/// assert_eq!(u8::checked_product_of_first_n_primes(0), Some(1));
/// assert_eq!(u8::checked_product_of_first_n_primes(1), Some(2));
/// assert_eq!(u8::checked_product_of_first_n_primes(2), Some(6));
/// assert_eq!(u8::checked_product_of_first_n_primes(3), Some(30));
/// assert_eq!(u8::checked_product_of_first_n_primes(4), Some(210));
/// assert_eq!(u32::checked_product_of_first_n_primes(9), Some(223092870));
///
/// assert_eq!(u8::checked_product_of_first_n_primes(5), None);
/// assert_eq!(u32::checked_product_of_first_n_primes(100), None);
/// ```
pub mod primorial;
/// [`RisingFactorial`](traits::RisingFactorial) and
/// [`CheckedRisingFactorial`](traits::CheckedRisingFactorial), traits for computing the rising
/// factorial of a number.
///
/// # rising_factorial
/// ```
/// use malachite_base::num::number_theory::traits::RisingFactorial;
///
/// assert_eq!(3u8.rising_factorial(0), 1);
/// assert_eq!(3u16.rising_factorial(4), 360);
/// assert_eq!((-5i32).rising_factorial(3), -60);
/// assert_eq!((-2i64).rising_factorial(5), 0);
/// ```
///
/// # checked_rising_factorial
/// ```
/// use malachite_base::num::number_theory::traits::CheckedRisingFactorial;
///
/// assert_eq!(3u16.checked_rising_factorial(4), Some(360));
/// assert_eq!(3u8.checked_rising_factorial(4), None);
/// assert_eq!((-5i32).checked_rising_factorial(3), Some(-60));
/// ```
pub mod rising_factorial;
/// [`StirlingNumberOfTheSecondKind`](traits::StirlingNumberOfTheSecondKind) and
/// [`CheckedStirlingNumberOfTheSecondKind`](traits::CheckedStirlingNumberOfTheSecondKind), traits
/// for computing Stirling numbers of the second kind. The first panics if the result cannot be
/// represented; the second returns `None` in that case.
///
/// # stirling_number_of_the_second_kind
/// ```
/// use malachite_base::num::number_theory::traits::StirlingNumberOfTheSecondKind;
///
/// assert_eq!(u8::stirling_number_of_the_second_kind(0, 0), 1);
/// assert_eq!(u8::stirling_number_of_the_second_kind(5, 0), 0);
/// assert_eq!(u8::stirling_number_of_the_second_kind(3, 5), 0);
/// assert_eq!(u8::stirling_number_of_the_second_kind(5, 2), 15);
/// assert_eq!(u8::stirling_number_of_the_second_kind(6, 3), 90);
/// assert_eq!(u32::stirling_number_of_the_second_kind(10, 5), 42525);
/// assert_eq!(
///     u64::stirling_number_of_the_second_kind(30, 5),
///     7713000216608565075
/// );
/// ```
///
/// # checked_stirling_number_of_the_second_kind
/// ```
/// use malachite_base::num::number_theory::traits::CheckedStirlingNumberOfTheSecondKind;
///
/// assert_eq!(
///     u8::checked_stirling_number_of_the_second_kind(6, 3),
///     Some(90)
/// );
/// assert_eq!(u8::checked_stirling_number_of_the_second_kind(7, 3), None);
/// assert_eq!(
///     u32::checked_stirling_number_of_the_second_kind(16, 8),
///     Some(2141764053)
/// );
/// assert_eq!(u32::checked_stirling_number_of_the_second_kind(18, 9), None);
/// assert_eq!(
///     u64::checked_stirling_number_of_the_second_kind(u64::MAX, u64::MAX),
///     Some(1)
/// );
/// ```
pub mod stirling_number_of_the_second_kind;
/// Various traits for number-theoretic functions.
pub mod traits;

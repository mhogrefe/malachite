---
layout: default
title: "Malachite for FLINT Users: Arithmetic Functions"
permalink: /mapping/flint-arithmetic-functions/
theme: jekyll-theme-slate
---

# Malachite for FLINT Users: Arithmetic Functions

This page maps the functions of FLINT's `arith.h` module, which computes number-theoretic and
combinatorial sequences, onto their Malachite counterparts. It follows the organization of the
[arith.h chapter](https://flintlib.org/doc/arith.html) of the FLINT manual, as of FLINT 3.6.0,
and is a companion to [Malachite for FLINT Users: Integers](/mapping/flint-integers/); the
[mapping index](/mapping/) lists the whole family. The
[Conventions](/mapping/flint-integers/#conventions) of that page apply here unchanged.

Functions whose names begin with an underscore are FLINT-internal entry points, and are mapped
only where they expose a numerator-and-denominator form that Malachite would spell differently.

Each function falls into one of four categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## What this chapter needs {#what-this-chapter-needs}

Malachite has counterparts for the harmonic numbers, the Bell numbers, and Landau's function.
The other sequences in this chapter have no counterpart.

## [Harmonic numbers](https://flintlib.org/doc/arith.html#harmonic-numbers) {#harmonic-numbers}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _arith_harmonic_number (fmpz_t num, fmpz_t den, slong n)` | `Rational::harmonic_number(n).into_numerator_and_denominator()` |
| ✓ | `void arith_harmonic_number (fmpq_t x, slong n)` | `Rational::harmonic_number(n)` |

The underscore form returns the numerator and denominator separately; Malachite returns a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html),
whose parts are available through
[`into_numerator_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.into_numerator_and_denominator).
`harmonic_number` is also the counterpart of
[`fmpq_harmonic_ui`](/mapping/flint-rationals/#special-functions), which
`arith_harmonic_number` wraps. FLINT accepts a signed `n` and returns zero when it is negative,
while `harmonic_number` takes a `u64`, so map negative values to zero before the call.

## [Stirling numbers](https://flintlib.org/doc/arith.html#stirling-numbers) {#stirling-numbers}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void arith_stirling_number_1u (fmpz_t s, ulong n, ulong k)` | `Natural::unsigned_stirling_number_of_the_first_kind(n, k)` |
| ✓ | `void arith_stirling_number_1 (fmpz_t s, ulong n, ulong k)` | `Integer::stirling_number_of_the_first_kind(n, k)` |
| ✓ | `void arith_stirling_number_2 (fmpz_t s, ulong n, ulong k)` | `Natural::stirling_number_of_the_second_kind(n, k)` |
| ≈ | `void arith_stirling_number_1u_vec (fmpz * row, ulong n, slong klen)` | [`rising_factorial_polynomial_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/special/traits/trait.RisingFactorialPolynomialTruncated.html#tymethod.rising_factorial_polynomial_truncated), [`rising_factorial_polynomial`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/special/traits/trait.RisingFactorialPolynomial.html#tymethod.rising_factorial_polynomial) |
| ≈ | `void arith_stirling_number_1_vec (fmpz * row, ulong n, slong klen)` | [`falling_factorial_polynomial_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/special/traits/trait.FallingFactorialPolynomialTruncated.html#tymethod.falling_factorial_polynomial_truncated), [`falling_factorial_polynomial`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/special/traits/trait.FallingFactorialPolynomial.html#tymethod.falling_factorial_polynomial) |
| ≈ | `void arith_stirling_number_2_vec (fmpz * row, ulong n, slong klen)` | [`touchard_polynomial_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/special/traits/trait.TouchardPolynomialTruncated.html#tymethod.touchard_polynomial_truncated), [`touchard_polynomial`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/special/traits/trait.TouchardPolynomial.html#tymethod.touchard_polynomial) |
| ✓ | `void arith_stirling_matrix_1u (fmpz_mat_t mat)` | `NaturalMatrix::unsigned_stirling_matrix_of_the_first_kind(height, width)` |
| ✓ | `void arith_stirling_matrix_1 (fmpz_mat_t mat)` | `IntegerMatrix::stirling_matrix_of_the_first_kind(height, width)` |
| ✓ | `void arith_stirling_matrix_2 (fmpz_mat_t mat)` | `NaturalMatrix::stirling_matrix_of_the_second_kind(height, width)` |

A row of Stirling numbers of the second kind is the coefficient list of a Touchard polynomial,
$$T_n(x) = \sum_k S(n, k) x^k$$, so `arith_stirling_number_2_vec` corresponds to [`touchard_polynomial_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/special/traits/trait.TouchardPolynomialTruncated.html#tymethod.touchard_polynomial_truncated) on
[`NaturalPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html),
which returns the first `klen` coefficients as a polynomial rather than writing them to `row`;
[`touchard_polynomial`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/special/traits/trait.TouchardPolynomial.html#tymethod.touchard_polynomial) gives the whole row. Malachite chooses among FLINT's three algorithms with its own thresholds,
using the recurrence for longer rows than FLINT does. For single Stirling numbers of the second
kind, Malachite uses FLINT's closed forms, recurrences, alternating power sum, and multimodular
computation, but replaces its exponential generating function with a sum over the second-order
Eulerian numbers, whose cost depends on $$n - k$$ alone, so values such as
$$S(2^{64} - 1, 2^{64} - 3)$$ are fast; the primitive unsigned types also implement
[`StirlingNumberOfTheSecondKind`](https://docs.rs/malachite-base/latest/malachite_base/num/number_theory/traits/trait.StirlingNumberOfTheSecondKind.html),
with [`CheckedStirlingNumberOfTheSecondKind`](https://docs.rs/malachite-base/latest/malachite_base/num/number_theory/traits/trait.CheckedStirlingNumberOfTheSecondKind.html)
returning `None` on overflow.

Similarly, a row of unsigned Stirling numbers of the first kind is the coefficient list of the
rising factorial polynomial $$x(x + 1) \cdots (x + n - 1)$$, so `arith_stirling_number_1u_vec`
corresponds to [`rising_factorial_polynomial_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/special/traits/trait.RisingFactorialPolynomialTruncated.html#tymethod.rising_factorial_polynomial_truncated)
on `NaturalPolynomial`, with [`rising_factorial_polynomial`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/special/traits/trait.RisingFactorialPolynomial.html#tymethod.rising_factorial_polynomial)
giving the whole row. Malachite uses FLINT's binary splitting, also expanding groups of factors in
double words. `arith_stirling_number_1u` corresponds to
`Natural::unsigned_stirling_number_of_the_first_kind`, which uses the same binary splitting and,
in place of FLINT's exponential generating function, the second-order Eulerian numbers, as for the
second kind; the primitive unsigned types also implement
[`UnsignedStirlingNumberOfTheFirstKind`](https://docs.rs/malachite-base/latest/malachite_base/num/number_theory/traits/trait.UnsignedStirlingNumberOfTheFirstKind.html),
with [`CheckedUnsignedStirlingNumberOfTheFirstKind`](https://docs.rs/malachite-base/latest/malachite_base/num/number_theory/traits/trait.CheckedUnsignedStirlingNumberOfTheFirstKind.html)
returning `None` on overflow. The signed Stirling numbers of the first kind are the coefficients
of the falling factorial polynomial $$x(x - 1) \cdots (x - n + 1)$$: `arith_stirling_number_1_vec`
corresponds to [`falling_factorial_polynomial_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/special/traits/trait.FallingFactorialPolynomialTruncated.html#tymethod.falling_factorial_polynomial_truncated)
on [`IntegerPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html),
and `arith_stirling_number_1` to `Integer::stirling_number_of_the_first_kind`; the primitive signed
types also implement
[`StirlingNumberOfTheFirstKind`](https://docs.rs/malachite-base/latest/malachite_base/num/number_theory/traits/trait.StirlingNumberOfTheFirstKind.html),
with [`CheckedStirlingNumberOfTheFirstKind`](https://docs.rs/malachite-base/latest/malachite_base/num/number_theory/traits/trait.CheckedStirlingNumberOfTheFirstKind.html)
returning `None` on overflow. The Stirling matrices are constructed with their dimensions,
`NaturalMatrix::unsigned_stirling_matrix_of_the_first_kind(height, width)` and so on, rather than
filling a given matrix, and the unsigned ones are `NaturalMatrix`es.

## [Bell numbers](https://flintlib.org/doc/arith.html#bell-numbers) {#bell-numbers}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void arith_bell_number (fmpz_t b, ulong n)` | `Natural::bell_number(n)` |
| — | `void arith_bell_number_dobinski (fmpz_t res, ulong n)` | |
| ✓ | `void arith_bell_number_multi_mod (fmpz_t res, ulong n)` | `Natural::bell_number(n)` |
| ✓ | `void arith_bell_number_vec (fmpz * b, slong n)` | `bell_numbers_prefix(n)` |
| ✓ | `void arith_bell_number_vec_recursive (fmpz * b, slong n)` | `exhaustive_bell_numbers().take(n)` |
| ✓ | `void arith_bell_number_vec_multi_mod (fmpz * b, slong n)` | `bell_numbers_prefix(n)` |
| — | `double arith_bell_number_size (ulong n)` | |
| — | `ulong arith_bell_number_nmod (ulong n, nmod_t mod)` | |
| — | `void arith_bell_number_nmod_vec (nn_ptr b, slong n, nmod_t mod)` | |
| — | `void arith_bell_number_nmod_vec_recursive (nn_ptr b, slong n, nmod_t mod)` | |
| — | `void arith_bell_number_nmod_vec_ogf (nn_ptr b, slong n, nmod_t mod)` | |
| — | `int arith_bell_number_nmod_vec_series (nn_ptr b, slong n, nmod_t mod)` | |

`Natural::bell_number` returns the same values as both `arith_bell_number` and
`arith_bell_number_multi_mod`. `bell_numbers_prefix(n)` returns $$B_0, \ldots, B_{n-1}$$ as a
`Vec<Natural>`, and `exhaustive_bell_numbers()` yields the same sequence as an unbounded
iterator; both take a `u64` count where FLINT takes an `slong`.

`arith_bell_number_dobinski` is an alternative algorithm for the same value, and
`arith_bell_number_size` is a bit-size estimate used internally by the multimodular routine, so
neither needs a counterpart. The `_nmod_` rows compute Bell numbers modulo a single-word
modulus, as a building block for the multimodular algorithms; Malachite has no vector-filling
modular routines, so reduce the full values instead, using the `Mod*` traits described on the
[integers mod n page](/mapping/flint-integers-mod-n/).

## [Bernoulli numbers and polynomials](https://flintlib.org/doc/arith.html#bernoulli-numbers-and-polynomials) {#bernoulli-numbers-and-polynomials}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _arith_bernoulli_number (fmpz_t num, fmpz_t den, ulong n)` | `Rational::bernoulli_number(n).into_numerator_and_denominator()` |
| ✓ | `void arith_bernoulli_number (fmpq_t x, ulong n)` | `Rational::bernoulli_number(n)` |
| ✓ | `void _arith_bernoulli_number_vec (fmpz * num, fmpz * den, slong n)` | `bernoulli_numbers_prefix(n)` |
| ✓ | `void arith_bernoulli_number_vec (fmpq * x, slong n)` | `bernoulli_numbers_prefix(n)` |
| ✓ | `void arith_bernoulli_number_denom (fmpz_t den, ulong n)` | `Natural::bernoulli_number_denominator(n)` |
| — | `double arith_bernoulli_number_size (ulong n)` | |
| ✓ | `void arith_bernoulli_polynomial (fmpq_poly_t poly, ulong n)` | `RationalPolynomial::bernoulli_polynomial(n)` |
| ✓ | `void _arith_bernoulli_number_vec_recursive (fmpz * num, fmpz * den, slong n)` | `bernoulli_numbers_prefix(n)` |
| ✓ | `void _arith_bernoulli_number_vec_multi_mod (fmpz * num, fmpz * den, slong n)` | `bernoulli_numbers_prefix(n)` |

The Bernoulli numbers use the same convention as FLINT, $$B_1 = -1/2$$. `bernoulli_numbers_prefix(n)`
returns $$B_0, \ldots, B_{n-1}$$ as a `Vec<Rational>`, and takes a `u64` where FLINT takes an
`slong`; it chooses between the recursive and multimodular algorithms itself, so it stands for all
three vector functions. To get separate numerators and denominators, call
`into_numerator_and_denominator` on each entry. `even_bernoulli_numbers_prefix(n)` returns just the
nonzero numbers after $$B_1$$, $$B_2, B_4, \ldots, B_{2n}$$. Both functions are in
`malachite_q::rational::number_theory::bernoulli_number`.

`arith_bernoulli_number_size` is a bit-size estimate used internally by the multimodular routine,
so it needs no counterpart.

## [Euler numbers and polynomials](https://flintlib.org/doc/arith.html#euler-numbers-and-polynomials) {#euler-numbers-and-polynomials}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void arith_euler_number (fmpz_t res, ulong n)` | `Integer::euler_number(n)` |
| ✓ | `void arith_euler_number_vec (fmpz * res, slong n)` | `euler_numbers_prefix(n)` |
| — | `double arith_euler_number_size (ulong n)` | |
| ✓ | `void arith_euler_polynomial (fmpq_poly_t poly, ulong n)` | `RationalPolynomial::euler_polynomial(n)` |

`euler_numbers_prefix(n)`, in `malachite_nz::integer::number_theory::euler_number`, returns
$$E_0, \ldots, E_{n-1}$$ as a `Vec<Integer>`. `arith_euler_number_size` is a bit-size estimate
used internally, so it needs no counterpart.

## [Multiplicative functions](https://flintlib.org/doc/arith.html#multiplicative-functions) {#multiplicative-functions}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void arith_divisors (fmpz_poly_t res, const fmpz_t n)` | |
| ✗ | `void arith_ramanujan_tau (fmpz_t res, const fmpz_t n)` | |
| ✗ | `void arith_ramanujan_tau_series (fmpz_poly_t res, slong n)` | |

Malachite has no counterparts for these functions.

## [Landau's function](https://flintlib.org/doc/arith.html#landau-s-function) {#landau-s-function}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void arith_landau_function_vec (fmpz * res, slong len)` | `landau_function_prefix(len)` |

`landau_function_prefix(len)` returns $$g(0), \ldots, g(\mathrm{len}-1)$$ as a
`Vec<Natural>`, taking a `u64` where FLINT takes an `slong`.

## [Number of partitions](https://flintlib.org/doc/arith.html#number-of-partitions) {#number-of-partitions}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void arith_number_of_partitions_vec (fmpz * res, slong len)` | |
| ✗ | `void arith_number_of_partitions (fmpz_t x, ulong n)` | |
| ✗ | `void arith_number_of_partitions_mpfr (mpfr_t x, ulong n)` | |
| — | `void arith_number_of_partitions_nmod_vec (nn_ptr res, slong len, nmod_t mod)` | |
| — | `void trig_prod_init (trig_prod_t prod)` | |
| — | `void arith_hrr_expsum_factored (trig_prod_t prod, ulong k, ulong n)` | |

Malachite has no partition-number functions. The last three rows are marked — because they
are internal pieces of FLINT's evaluation of $$p(n)$$, a modular table used to check the result
and the structure and routine that evaluate the exponential sums, rather than independently
useful functions.

## [Sums of squares](https://flintlib.org/doc/arith.html#sums-of-squares) {#sums-of-squares}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void arith_sum_of_squares (fmpz_t r, ulong k, const fmpz_t n)` | |
| ✗ | `void arith_sum_of_squares_vec (fmpz * r, ulong k, slong n)` | |

Malachite has no sums-of-squares functions.

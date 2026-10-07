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
| ✗ | `void arith_stirling_number_1u (fmpz_t s, ulong n, ulong k)` | |
| ✗ | `void arith_stirling_number_1 (fmpz_t s, ulong n, ulong k)` | |
| ✗ | `void arith_stirling_number_2 (fmpz_t s, ulong n, ulong k)` | |
| ✗ | `void arith_stirling_number_1u_vec (fmpz * row, ulong n, slong klen)` | |
| ✗ | `void arith_stirling_number_1_vec (fmpz * row, ulong n, slong klen)` | |
| ✗ | `void arith_stirling_number_2_vec (fmpz * row, ulong n, slong klen)` | |
| ✗ | `void arith_stirling_matrix_1u (fmpz_mat_t mat)` | |
| ✗ | `void arith_stirling_matrix_1 (fmpz_mat_t mat)` | |
| ✗ | `void arith_stirling_matrix_2 (fmpz_mat_t mat)` | |

Malachite has no Stirling numbers.

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
| ✗ | `void _arith_bernoulli_number (fmpz_t num, fmpz_t den, ulong n)` | |
| ✗ | `void arith_bernoulli_number (fmpq_t x, ulong n)` | |
| ✗ | `void _arith_bernoulli_number_vec (fmpz * num, fmpz * den, slong n)` | |
| ✗ | `void arith_bernoulli_number_vec (fmpq * x, slong n)` | |
| ✗ | `void arith_bernoulli_number_denom (fmpz_t den, ulong n)` | |
| ✗ | `double arith_bernoulli_number_size (ulong n)` | |
| ✗ | `void arith_bernoulli_polynomial (fmpq_poly_t poly, ulong n)` | |
| ✗ | `void _arith_bernoulli_number_vec_recursive (fmpz * num, fmpz * den, slong n)` | |
| ✗ | `void _arith_bernoulli_number_vec_multi_mod (fmpz * num, fmpz * den, slong n)` | |

Malachite has no Bernoulli numbers or polynomials.

## [Euler numbers and polynomials](https://flintlib.org/doc/arith.html#euler-numbers-and-polynomials) {#euler-numbers-and-polynomials}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void arith_euler_number (fmpz_t res, ulong n)` | |
| ✗ | `void arith_euler_number_vec (fmpz * res, slong n)` | |
| ✗ | `double arith_euler_number_size (ulong n)` | |
| ✗ | `void arith_euler_polynomial (fmpq_poly_t poly, ulong n)` | |

Malachite has no Euler numbers or polynomials.

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

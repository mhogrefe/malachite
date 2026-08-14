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

Every row on this page is currently ✗ or —: Malachite computes
[factorials](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.factorial),
[binomial coefficients](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.binomial_coefficient),
[primorials](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.primorial),
and [Fibonacci numbers](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.fibonacci),
but none of the sequences in this chapter yet. The page is therefore best read as a list of what
is still missing, and the section below sorts that list by what each entry would require.

## What this chapter needs {#what-this-chapter-needs}

The functions here divide cleanly by the machinery they rest on, which is worth stating up front
because it explains why the ✗ rows below are not equally far away.

The dividing question is whether FLINT has *any* route to a value that avoids machinery
Malachite has not built yet. Where it does, that route is a complete algorithm and porting it
finishes the job. Where every route runs through polynomials, single-word modular vectors,
matrices, or factorization, writing a bespoke substitute now would be a stand-in for machinery
that is coming anyway, so those entries wait rather than acquire an implementation that would
have to be replaced.

One group has such a route, and the sharpest form of the question is whether that route is a
whole function or one branch inside a dispatcher. Harmonic numbers are a table plus a balanced
sum, and that is the entire algorithm. Landau's function needs only prime generation. Bell
numbers have Dobinski's formula, which FLINT exposes as a function of its own and which never
leaves exact integer arithmetic, alongside the triangle. Bernoulli numbers have von
Staudt–Clausen for the denominator and a recursive vector routine, also a public function of its
own, that computes the numerators without leaving `fmpz`. The `_size` functions, which return
`double` bit bounds, are self-contained floating-point estimates. These are the reachable
entries: in each case a complete published algorithm can be ported as it stands.

The rest wait, and it is worth being precise about what each waits on. Polynomials: the
Bernoulli and Euler polynomials, the Ramanujan tau function and its series, and the
generating-function routes that FLINT prefers for parts of the Stirling and Bell ranges.
Matrices: the whole Stirling matrix interface. Single-word modular arithmetic over vectors: every
`_nmod_` entry point, together with the transforms that make multimodular routes fast.
Factorization: `arith_divisors` and the sums-of-squares functions, which are factorization
problems wearing different hats, and which Malachite cannot yet take on because
[`Factor`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.Factor.html)
covers primitive integers rather than `Natural`.

Three entries deserve naming because they look reachable and are not. The **partition-number
table** can be produced by Euler's pentagonal-number recurrence using nothing but `Natural`
addition and subtraction, but FLINT computes it by inverting a power series, and so should
Malachite once it has them; a recurrence written now would be a placeholder for that. **Euler
numbers** are in the same position from the other direction: FLINT reaches a single value
through Arb, its ball-arithmetic library, and a vector through modular arithmetic and series, so
neither of its routes is currently open, and the classical recurrence would be a substitute
rather than a port.

**Stirling numbers** are the case that turns on the distinction above. Their reachable part is
real — closed forms at the extremes of $$k$$, triangular recurrences while the values are
narrow, and an explicit power sum over integer vectors for $$k$$ well below $$n$$ — but that
power sum is one branch of a dispatcher, not a function anyone calls directly, and the branches
beside it are a generating function and a multimodular evaluation. Porting the reachable branch
alone would produce something that answers correctly everywhere and quickly only in part of the
range, which is the shape of a partial implementation rather than a finished one, so the whole
family waits.

The partition function for a single argument stands apart. FLINT evaluates the
Hardy–Ramanujan–Rademacher formula in arbitrary-precision floating-point, and Malachite has the
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html) type
that this needs, so the dependency is satisfied; what remains is the formula itself, including
the Dedekind sums and the factored exponential sums, which is a larger undertaking than anything
else in the first group.

## [Harmonic numbers](https://flintlib.org/doc/arith.html#harmonic-numbers) {#harmonic-numbers}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _arith_harmonic_number (fmpz_t num, fmpz_t den, slong n)` | `Rational::harmonic_number(n).into_numerator_and_denominator()` |
| ✓ | `void arith_harmonic_number (fmpq_t x, slong n)` | `Rational::harmonic_number(n)` |

$$H_n = \sum_{k=1}^n 1/k$$, as an exact rational. FLINT stores a table of the first several
values and computes larger ones with a balanced sum, halving the work by summing only over odd
$$k$$ and recurring on $$H_{\lfloor n/2 \rfloor}$$. That is FLINT's whole algorithm, and it needs
only `Natural` and
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html).
The underscore form returns the numerator and denominator separately; Malachite would return a
`Rational`, whose parts are available through
[`into_numerator_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.into_numerator_and_denominator).

These rows are closed by the same port as
[`fmpq_harmonic_ui`](/mapping/flint-rationals/#special-functions) on the rationals page, which
FLINT's `arith_harmonic_number` wraps. The one difference is the argument type: FLINT accepts a
signed `n` and returns zero when it is negative, while `harmonic_number` takes a `u64`, so a
caller with a signed quantity maps negative values to zero before the call.

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

The unsigned first-kind numbers count permutations of $$n$$ elements with $$k$$ cycles; the
signed ones differ by $$(-1)^{n-k}$$, so the first kind is an `Integer` result and the second
kind a `Natural` one. FLINT special-cases the closed forms — $$S(n,2) = 2^{n-1}-1$$ and
$$S(n,n-1) = \binom{n}{2}$$ among them — then uses a triangular recurrence while the values
still fit in one or two words, and past that picks among three routes by where $$k$$ falls
relative to $$n$$. The route covering the widest range, for $$k$$ well below $$n$$, is the
explicit power sum $$S(n,k) = \frac{1}{k!}\sum_j (-1)^j \binom{k}{j} (k-j)^n$$, evaluated over
integer vectors with the binomial coefficients built up as it goes; it needs nothing Malachite
lacks, and neither do the closed forms or the recurrences
$$c(n,k) = c(n-1,k-1) + (n-1)\,c(n-1,k)$$ and $$S(n,k) = S(n-1,k-1) + k\,S(n-1,k)$$. The other
two routes, an exponential generating function for $$k$$ near $$n$$ and a multimodular
evaluation between them, need machinery Malachite does not have. Because the reachable part is a
branch rather than a callable algorithm, these rows wait for the other two routes rather than
landing in pieces; the row functions would return a `Vec`, and the matrix functions wait on a
matrix type regardless.

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

$$B_n$$ counts the partitions of a set of $$n$$ elements. `Natural::bell_number` follows
FLINT's default routine exactly: a table of the word-sized values, the Bell triangle in one-,
two-, and three-word accumulators while the entries still fit, and beyond that the multimodular
algorithm — the Dobinski-style sum modulo enough 61-bit primes, recombined with
`Natural::multi_crt` — so the `arith_bell_number` and `arith_bell_number_multi_mod` rows are the
same function here. `arith_bell_number_dobinski`, an alternative evaluation FLINT keeps
alongside the default, and `arith_bell_number_size`, the de Bruijn bit-size bound the
multimodular routine uses internally, are marked — as internal algorithm choices rather than
gaps. The `_vec` rows are covered twice over: `exhaustive_bell_numbers()` is the bignum Bell
triangle as an iterator, in the style of Malachite's other exhaustive generators, and
`bell_numbers_prefix` is FLINT's `_vec` dispatch, collecting the iterator for short prefixes
and switching to the multimodular batch at the same threshold FLINT uses. The batch recombines
each entry over only the primes its size needs, replacing FLINT's graded combs with sliced
calls to `Natural::multi_crt`; its per-prime routine is the word-sized triangle, so very long
prefixes lack the `nmod_poly`-based inner loops FLINT can select, which is the remaining
performance gap until Malachite grows polynomials.

The `_nmod_` rows are marked — rather than ✗ because they compute Bell numbers modulo a
single-word modulus as a means to the multimodular algorithms above, not as an end. Malachite's
modular arithmetic is the `Mod*` trait family described on the
[integers mod n page](/mapping/flint-integers-mod-n/), and it works on individual values rather
than on the vectors these routines fill; a Malachite implementation would reach the same results
by a different route.

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

The denominator is the most approachable entry in the chapter: by von Staudt–Clausen, the
denominator of $$B_n$$ for even $$n$$ is the product of the primes $$p$$ with $$(p-1) \mid n$$,
which Malachite's
[`Primes`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.Primes.html)
iterator supplies directly. Knowing the denominator turns the numerator into an integer
problem, which is how FLINT's vector routines proceed, and the recursive one of those —
Ramanujan's congruences, applied to a whole table at once — stays inside `fmpz` throughout. That
gives a complete route to both the numbers and the denominators; the multimodular vector routine
is the faster alternative and would follow later. The polynomial row waits on a polynomial type;
note that FLINT's polynomial here is over $$\mathbb{Q}$$, so it would want a
`Rational`-coefficient polynomial rather than an integer one.

## [Euler numbers and polynomials](https://flintlib.org/doc/arith.html#euler-numbers-and-polynomials) {#euler-numbers-and-polynomials}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void arith_euler_number (fmpz_t res, ulong n)` | |
| ✗ | `void arith_euler_number_vec (fmpz * res, slong n)` | |
| ✗ | `double arith_euler_number_size (ulong n)` | |
| ✗ | `void arith_euler_polynomial (fmpq_poly_t poly, ulong n)` | |

The Euler numbers are integers, zero at odd indices and alternating in sign at even ones, so the
result type is `Integer`. Neither of FLINT's routes is open to Malachite today: a single value
comes from Arb, its ball-arithmetic library, and a vector from modular arithmetic over a series.
The classical recurrence would reach the same values using only integers, but it would be a
substitute for machinery Malachite intends to build rather than a port of either route, so these
rows wait. The polynomial row waits on rational-coefficient polynomials in any case.

## [Multiplicative functions](https://flintlib.org/doc/arith.html#multiplicative-functions) {#multiplicative-functions}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void arith_divisors (fmpz_poly_t res, const fmpz_t n)` | |
| ✗ | `void arith_ramanujan_tau (fmpz_t res, const fmpz_t n)` | |
| ✗ | `void arith_ramanujan_tau_series (fmpz_poly_t res, slong n)` | |

`arith_divisors` returns the divisors of $$n$$ in ascending order, using a polynomial only as a
convenient array; a Malachite version would return a `Vec<Natural>`. It is a factorization
problem, and Malachite's
[`Factor`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.Factor.html)
trait currently covers primitive integers rather than `Natural`, so this waits on that being
extended.

The Ramanujan tau function is defined by the coefficients of $$q\prod_{k\geq 1}(1-q^k)^{24}$$,
and FLINT computes a single value from the series, so both rows wait on power series.

## [Landau's function](https://flintlib.org/doc/arith.html#landau-s-function) {#landau-s-function}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void arith_landau_function_vec (fmpz * res, slong len)` | `landau_function_prefix(len)` |

$$g(n)$$ is the largest order of an element of the symmetric group $$S_n$$, obtained by
maximizing a product of prime powers whose sum is at most $$n$$. `landau_function_prefix`
follows FLINT exactly: a knapsack over prime powers, offered in descending index order so each
prime contributes at most one power per value, with primes capped at FLINT's
$$1.328\sqrt{n \ln n}$$ bound on the largest useful prime.

## [Number of partitions](https://flintlib.org/doc/arith.html#number-of-partitions) {#number-of-partitions}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void arith_number_of_partitions_vec (fmpz * res, slong len)` | |
| ✗ | `void arith_number_of_partitions (fmpz_t x, ulong n)` | |
| ✗ | `void arith_number_of_partitions_mpfr (mpfr_t x, ulong n)` | |
| — | `void arith_number_of_partitions_nmod_vec (nn_ptr res, slong len, nmod_t mod)` | |
| — | `void trig_prod_init (trig_prod_t prod)` | |
| — | `void arith_hrr_expsum_factored (trig_prod_t prod, ulong k, ulong n)` | |

FLINT builds the table of $$p(0), \ldots, p(\mathrm{len}-1)$$ by writing down the sparse
pentagonal-number series and inverting it as a power series. Euler's pentagonal-number
recurrence would produce the same values using only `Natural` addition and subtraction, but that
is the same computation carried out by hand, and it is exactly what a power-series type will do
once Malachite has one; the row therefore waits for that rather than for a workaround.

A single $$p(n)$$ is the harder case. FLINT evaluates the Hardy–Ramanujan–Rademacher series to
just enough precision to round to the correct integer, which is why the `mpfr` form is part of
the public interface rather than an implementation detail. Malachite's `Float` supplies the
arbitrary-precision arithmetic, so what is missing is the series itself. The last three rows are
marked — because they are pieces of that machinery — a modular table used to check the result,
and the structure and routine that evaluate the exponential sums — rather than independently
useful functions; Malachite would not expose them separately.

## [Sums of squares](https://flintlib.org/doc/arith.html#sums-of-squares) {#sums-of-squares}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void arith_sum_of_squares (fmpz_t r, ulong k, const fmpz_t n)` | |
| ✗ | `void arith_sum_of_squares_vec (fmpz * r, ulong k, slong n)` | |

$$r_k(n)$$ counts the representations of $$n$$ as an ordered sum of $$k$$ squares, counting
signs and order. FLINT dispatches on $$k$$: one square is a
[square test](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.IsSquare.html),
$$k = 2$$ and $$k = 4$$ have closed forms that read the factorization of $$n$$, $$k = 3$$ and
$$k = 5$$ recurse onto those, and larger $$k$$ comes from a power of a theta series. Every
branch past the first therefore rests on factoring $$n$$, and the general one additionally on
power series.

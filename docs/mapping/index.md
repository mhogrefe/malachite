---
layout: default
title: "Malachite for Users of Other Libraries"
permalink: /mapping/
theme: jekyll-theme-slate
---

# Malachite for Users of Other Libraries

These pages map the functions of established arithmetic libraries onto their Malachite
counterparts, section by section, following the organization of each library's manual. They are
meant to be read in two directions: if you are porting code, look up the function you are using
and find what to write instead; if you are wondering what Malachite is still missing, look for
the rows marked ✗, which are the ones it is committed to filling in.

## [GMP](https://gmplib.org/)

- [Integers](/mapping/gmp-integers/): the `mpz_t` type, mapped onto
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) and
  [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html).
- [Rationals](/mapping/gmp-rationals/): the `mpq_t` type, mapped onto
  [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html).

GMP's floating-point type, `mpf_t`, will not get a page of its own. GMP's manual steers new
projects toward [MPFR](https://www.mpfr.org/), and Malachite's
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html) follows
MPFR, so the float mapping belongs with
[the MPFR page](/mapping/mpfr-floats/#compatibility-with-mpf).

## [FLINT](https://flintlib.org/)

- [Integers](/mapping/flint-integers/): the `fmpz_t` type, mapped onto
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) and
  [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html).
- [Integers mod n](/mapping/flint-integers-mod-n/): the `fmpz_mod.h` module, mapped onto the
  `Mod*` traits over
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
  residues.
- [Rationals](/mapping/flint-rationals/): the `fmpq_t` type, mapped onto
  [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html).
- [Gaussian integers](/mapping/flint-gaussian-integers/): the `fmpzi_t` type, mapped onto
  [`GaussianInteger`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/struct.GaussianInteger.html).
- [Integer polynomials](/mapping/flint-integer-polynomials/): the `fmpz_poly_t` type, mapped onto
  [`IntegerPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html).
  All 48 sections of `fmpz_poly.h` are mapped.
- [Rational polynomials](/mapping/flint-rational-polynomials/): the `fmpq_poly_t` type, mapped onto
  [`RationalPolynomial`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html).
  All 32 sections of `fmpq_poly.h` are mapped.
- [Modular polynomials](/mapping/flint-modular-polynomials/): the `fmpz_mod_poly_t` type —
  polynomials over the integers mod $$n$$ — mapped onto
  [`NaturalPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html).
  All 34 sections of `fmpz_mod_poly.h` are mapped.
- [Word-sized modular polynomials](/mapping/flint-word-sized-modular-polynomials/): the
  `nmod_poly_t` type — polynomials over the integers mod $$n$$, for an $$n$$ that fits in a
  machine word — mapped onto
  [`UnsignedPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html).
  All 42 sections of `nmod_poly.h` are mapped.
- [Arithmetic functions](/mapping/flint-arithmetic-functions/): the `arith.h` module, which
  computes number-theoretic and combinatorial sequences.

FLINT modules not listed here have no page.

## [MPFR](https://www.mpfr.org/)

- [Floats](/mapping/mpfr-floats/): the `mpfr_t` type, mapped onto
  [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html).

## [num](https://docs.rs/num/latest/num/)

- [Integers](/mapping/num-integers/): num-bigint's `BigUint` and `BigInt`, mapped onto
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) and
  [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html).
- [Rationals](/mapping/num-rationals/): num-rational's `Ratio<T>` and `BigRational`, mapped
  onto [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html).
- [Traits](/mapping/num-traits/): the num-traits and num-integer trait crates, with num-iter's
  ranges, mapped onto [malachite-base](https://docs.rs/malachite-base/latest/malachite_base/)'s
  generic vocabulary.

num-complex has no page: Malachite has no complex type for it to map onto.

## [Azurite](https://github.com/mhogrefe/azurite)

- [Naturals](/mapping/azurite-naturals/): the `AzNat` type, Azurite's formally verified
  multi-limb natural number, mapped onto
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html).
- [Integers](/mapping/azurite-integers/): the `AzInt` type, mapped onto
  [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html).
- [Integers modulo a power of 2](/mapping/azurite-mod-power-of-2/): the `AzZModPow2 k` type,
  mapped onto the `mod_power_of_2_*` operations of
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html).
- [Integers modulo a natural](/mapping/azurite-mod/): the `AzZMod m` type, mapped onto the
  `mod_*` operations of
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html).
- [Rationals](/mapping/azurite-rationals/): the `AzRat` type, mapped onto
  [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html).
- [Floats](/mapping/azurite-floats/): the `AzFloat` type, mapped onto
  [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html).

Azurite's polynomial and matrix types will get pages as they are mapped.

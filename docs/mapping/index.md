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

FLINT is much larger than GMP, and its pages will arrive module by module.

## [MPFR](https://www.mpfr.org/)

- [Floats](/mapping/mpfr-floats/): the `mpfr_t` type, mapped onto
  [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html).

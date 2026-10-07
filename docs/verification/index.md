---
layout: default
title: "How Malachite Is Tested"
permalink: /verification/
theme: jekyll-theme-slate
---

# How Malachite Is Tested

Arbitrary-precision arithmetic is easy to get subtly wrong and hard to notice when it is: a carry
dropped once in a million limbs, a rounding tie resolved the wrong way at one exponent, a
threshold crossed into an algorithm that was never exercised. Malachite's answer is differential
testing at scale. Every public function is run against independent implementations of the same
mathematics, over millions of inputs drawn from several generators, and any disagreement fails the
build.

These pages record, function by function, which independent checks each operation has. They are
the inverse of the [mapping pages](/mapping/): those tell you what to call in Malachite for a
function of another library; these tell you what else has computed the same answers as the
Malachite function you are calling. A row with no checks is a row we intend to fill.

## The oracles

- **Azurite** — [Azurite](https://github.com/mhogrefe/azurite) is a bignum library written in
  Lean 4 whose operations are proven correct against Lean's and Mathlib's own `Nat`, `Int`, `ℚ`,
  and `ZMod`. Its oracle executable reads the output of Malachite's demos, recomputes every line,
  and reports the first disagreement; the comparison is exact and the proofs are machine-checked,
  so an agreement with Azurite is the strongest check on this page. Azurite is young, and the
  coverage it gives grows as it gains operations. The Azurite oracle is run in the Malachite
  repository's `oracle-test/` driver, three generator modes at ten thousand lines each per demo,
  together with the edge-case rows of the unit tests.
- **FLINT** — [FLINT](https://flintlib.org/)'s C implementation, run through the same driver
  against a small C program that reads the demo output. Where FLINT's conventions differ from
  Malachite's, the C side adapts; where FLINT has a known bug, the driver checks Malachite against
  the corrected behavior and the page says so.
- **GMP** — [GMP](https://gmplib.org/), through the [`rug`](https://crates.io/crates/rug) crate,
  inside Malachite's own test suite.
- **num** — the [`num`](https://crates.io/crates/num) crates, inside the test suite.
- **MPFR** — [MPFR](https://www.mpfr.org/), the reference implementation of correctly rounded
  arbitrary-precision floating-point arithmetic, through `rug`, inside the test suite. It is the
  oracle for `Float` that GMP is for the integer types.
- **Reference implementation** — a deliberately simple algorithm for the same function (schoolbook
  multiplication, bit-by-bit shifts, a textbook GCD), kept in Malachite's `test_util` and compared
  against the production algorithm in the property tests. It is not independent of Malachite, but
  it is independent of the optimized code, which is where the bugs are.

Every function also has unit tests with hand-picked edge cases and property tests of its algebraic
identities (`(a + b) - b = a`, `gcd(a, b) | a`, round-trips through strings), run over exhaustive,
random, and "special" random generators that favor the limb boundaries and extreme values where
implementations fail. Those are not listed per function, because every function has them; the
columns record only what is independent of Malachite.

## The pages

- [Naturals](/verification/naturals/): every function of
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html).
- [Integers](/verification/integers/): every function of
  [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html).
- [Rationals](/verification/rationals/): every function of
  [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html).
- [Floats](/verification/floats/): every function of
  [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html).

Pages for the polynomial types will follow.

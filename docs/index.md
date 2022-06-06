<p align="center">
  <img width="650" src="/assets/logo-and-name.svg" alt="Logo">
</p>

Malachite is an arbitrary-precision arithmetic library for [Rust](https://www.rust-lang.org/). It
achieves high performance in part by using algorithms derived from [GMP](https://gmplib.org/) and
[FLINT](https://www.flintlib.org/).

See [here](/performance) for a performance comparison against other libraries.

Malachite comes in three crates.
- **malachite-base** ([crates.io](https://crates.io/crates/malachite-base),
  [docs.rs](https://docs.rs/malachite-base/latest/malachite_base/)) is a collection of utilities
  supporting the other crates. It includes
  - Traits that wrap functions from the standard library, like
  [`CheckedAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedAdd.html);
  - Traits that give extra functionality to primitive types, like
    [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html),
    [`FloorSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorSqrt.html),
    and
    [`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html);
  - Iterator-producing functions that let you generate values for testing.
- **malachite-nz** ([crates.io](https://crates.io/crates/malachite-nz),
  [docs.rs](https://docs.rs/malachite-nz/latest/malachite_nz/)) defines two bignum types,
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s and
  [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)s. The
  functions defined on these types include
  - All the ones you'd expect, like addition, subtraction, multiplication, and integer division;
  - Implementations of
    [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html),
    which provides division that rounds according to a specified
    [`RoundingMode`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html);
  - Various mathematical functions, like implementations of
    [`FloorSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorSqrt.html)
    and
    [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html);
  - Modular arithmetic functions, like implementations of
    [`ModAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAdd.html)
    and
    [`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html),
    and of traits for arithmetic modulo a power of 2, like
    [`ModPowerOf2Add`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Add.html)
    and
    [`ModPowerOf2Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Pow.html);
  - Various functions for logic and bit manipulation, like
    [`BitAnd`](https://doc.rust-lang.org/nightly/core/ops/trait.BitAnd.html) and
    [`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html).
- **malachite-q** ([crates.io](https://crates.io/crates/malachite-q),
  [docs.rs](https://docs.rs/malachite-q/latest/malachite_q/)) defines
  [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/struct.Rational.html)s. The
  functions defined on this type include
  - All the ones you'd expect, like addition, subtraction, multiplication, and division;
  - Functions related to conversion between
    [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/struct.Rational.html)s and other
    kinds of numbers, including primitive floats;
  - Functions for Diophantine approximation;
  - Functions for expressing
    [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/struct.Rational.html)s in
    scientific notation.

Malachite is under active development, with many more types and features planned for the future.
Nonetheless, it is extensively tested and documented, and ready for use today. Just be aware that
its API is not stable yet, and that it is licensed under LGPL 3.0.

<ul>
  {% for post in site.posts %}
    <li>
      <a href="{{ post.url }}">{{ post.title }}</a>
    </li>
  {% endfor %}
</ul>

Copyright © 2022 Mikhail Hogrefe

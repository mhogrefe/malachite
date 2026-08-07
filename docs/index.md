<p align="center">
  <img width="650" src="/assets/logo-and-name.svg" alt="Logo">
</p>

Malachite is an arbitrary-precision arithmetic library for [Rust](https://www.rust-lang.org/). It
achieves high performance in part by using algorithms derived from [GMP](https://gmplib.org/),
[FLINT](https://www.flintlib.org/), and [MPFR](https://www.mpfr.org/).

The documentation for Malachite is [here](https://docs.rs/malachite/latest/malachite/), and its crate is [here](https://crates.io/crates/malachite).

**Coming from another arithmetic library?** The [mapping pages](/mapping/) list each library's functions next
to their Malachite counterparts, section by section, and mark the ones Malachite does not have
yet: GMP's [integers](/mapping/gmp-integers/) and [rationals](/mapping/gmp-rationals/),
FLINT's [integers](/mapping/flint-integers/),
[integers mod n](/mapping/flint-integers-mod-n/), and [rationals](/mapping/flint-rationals/),
MPFR's [floats](/mapping/mpfr-floats/), and num's [integers](/mapping/num-integers/),
[rationals](/mapping/num-rationals/), and [traits](/mapping/num-traits/) are covered today,
with more FLINT pages to follow.

```rust
use malachite::base::num::arithmetic::traits::Factorial;
use malachite::Natural;

fn main() {
    println!("{}", Natural::factorial(100));
}
```
The code above outputs the following:
```
93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000
```
You have to scroll to see the entire output.

Here is [Ramanujan's constant](https://en.wikipedia.org/wiki/Heegner_number#Almost_integers_and_Ramanujan's_constant),
$$e^{\pi \sqrt{163}}$$, which is famously, and not by coincidence, within a trillionth of an
integer. Computing it takes real arbitrary-precision machinery: a transcendental constant, a
square root, and an exponential, each correctly rounded to 200 bits. Only the starting values
name a precision; every operation after that inherits the precision of its input, or the larger
precision of its two inputs. (`Float` support is enabled by the `floats` feature.)

```rust
use malachite::base::num::arithmetic::traits::{Exp, Sqrt};
use malachite::base::num::conversion::string::options::ToSciOptions;
use malachite::base::num::conversion::traits::ToSci;
use malachite::Float;

fn main() {
    let prec = 200;
    let pi = Float::pi_prec(prec).0;
    let sqrt_163 = Float::from_unsigned_prec(163u32, prec).0.sqrt();
    let almost_integer = (pi * sqrt_163).exp();
    let mut options = ToSciOptions::default();
    options.set_precision(45);
    println!("{}", almost_integer.to_sci_with_options(options));
}
```
The output is this, with twelve nines after the decimal point:
```
262537412640768743.999999999999250072597198186
```
Every digit is correct. Each operation returns its result along with an
[`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html) reporting whether that
result is below, equal to, or above the exact value; the example discards them with `.0`, but they
are how you track exactness through a computation.

Exactness is sometimes the whole story. The polynomial below is
[Rump's example](https://en.wikipedia.org/wiki/Rump%27s_example): evaluated at $$x = 77617$$ and
$$y = 33096$$ in `f64` arithmetic, the rounding errors prevent the two enormous terms from cancelling almost exactly. `Rational`
arithmetic is exact, so cancellation works.

```rust
use malachite::base::num::basic::traits::Two;
use malachite::base::num::conversion::string::options::ToSciOptions;
use malachite::base::num::conversion::traits::ToSci;
use malachite::Rational;

fn main() {
    let (x, y) = (Rational::from(77617), Rational::from(33096));
    let x2 = &x * &x;
    let y2 = &y * &y;
    let y4 = &y2 * &y2;
    let y6 = &y4 * &y2;
    let y8 = &y4 * &y4;
    let inner =
        Rational::from(11) * &x2 * y2 - &y6 - Rational::from(121) * y4 - Rational::TWO;
    let exact = Rational::from_signeds(1335, 4) * y6
        + x2 * inner
        + Rational::from_signeds(11, 2) * y8
        + x / (Rational::TWO * y);
    let mut options = ToSciOptions::default();
    options.set_precision(20);
    println!("{} ~ {}", exact, exact.to_sci_with_options(options));
}
```
The output is this:
```
-54767/66192 ≈ -0.82739605994682136814
```
The same formula evaluated in `f64` arithmetic gives roughly $$-1.18 \times 10^{21}$$: off by
twenty-one orders of magnitude.

Malachite is designed to work with very large numbers efficiently. See [here](/performance) for a
performance comparison against other libraries.

Malachite uses `no_std`, unless the `random`, `test_build`, or `bin_build` features are enabled.

To use Malachite, add the following to your project's `Cargo.toml` file:
```yaml
[dependencies.malachite]
version = "0.10.0"
```

By default, Malachite includes `Natural`, `Integer`, and `Rational`. `Float` support is opt-in:
```yaml
[dependencies.malachite]
version = "0.10.0"
features = [ "floats" ]
```
You can also opt out of the types you don't need. For example, if you want to use `Natural` and
`Integer` but not `Rational`, you can use
```yaml
[dependencies.malachite]
version = "0.10.0"
default-features = false
features = [ "naturals_and_integers" ]
```

The `malachite` crate re-exports four sub-crates.
- **malachite-base** ([crates.io](https://crates.io/crates/malachite-base)) is a collection of utilities
  supporting the other crates. It includes
  - Traits that wrap functions from the standard library, like
  [`CheckedAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedAdd.html);
  - Traits that give extra functionality to primitive types, like
    [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html),
    [`FloorSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorSqrt.html),
    and
    [`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html);
  - Iterator-producing functions that let you generate values for testing.
- **malachite-nz** ([crates.io](https://crates.io/crates/malachite-nz)) defines two bignum types,
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

  If you need to explicitly include this crate as a dependency of the `malachite` crate, use the
  `naturals_and_integers` or `malachite-nz` feature.
- **malachite-q** ([crates.io](https://crates.io/crates/malachite-q)) defines
  [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)s. The
  functions defined on this type include
  - All the ones you'd expect, like addition, subtraction, multiplication, and division;
  - Functions related to conversion between
    [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)s and other
    kinds of numbers, including primitive floats;
  - Functions for Diophantine approximation;
  - Functions for expressing
    [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)s in
    scientific notation.

  If you need to explicitly include this crate as a dependency of the `malachite` crate, use the
  `rationals` or `malachite-q` feature.

- **malachite-float** ([crates.io](https://crates.io/crates/malachite-float)) defines
  [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html)s,
  arbitrary-precision floating-point numbers whose semantics follow
  [MPFR](https://www.mpfr.org/)'s: the precision is chosen per value, every operation is correctly
  rounded to a requested precision and rounding mode, and every rounding operation reports whether
  its result is below, equal to, or above the exact value. The functions defined on this type
  include
  - All the ones you'd expect, like addition, subtraction, multiplication, and division;
  - Square roots, cube roots, and kth roots, along with their reciprocals;
  - Exponentials, logarithms, and powers, in an arbitrary base as well as base 2, base 10, and
    base e;
  - Around three dozen mathematical constants, computed to any requested precision;
  - Functions related to conversion between
    [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html)s and
    other kinds of numbers, and to and from strings in any base from 2 to 36 (or up to 62 through
    the lower-level MPFR-style entry points).

  These are not yet feature-complete, but the functions that are implemented are thoroughly tested
  and documented.

  If you need to explicitly include this crate as a dependency of the `malachite` crate, use the
  `floats` or `malachite-float` feature.

Malachite is under active development, with many more types and features planned for the future.
Nonetheless, it is extensively tested and documented, and ready for use today. Just be aware that
its API is not stable yet, and that Malachite is licensed under LGPL 3.0.

Malachite is developed by Mikhail Hogrefe. `malachite-bigint`, a drop-in num-bigint replacement based on Malachite, was created by Steve Shi and is now maintained by Mikhail Hogrefe. Thanks to 43615, b4D8, Romain Billot, Maxim Biryukov, coolreader18, Dasaav-dsv, Duncan Freeman, florian1345, konstin, Rowan Hart, YunWon Jeong, Park Joon-Kyu, Antonio Mamić, OliverNChalk, Kevin Phoenix, probablykasper, shekohex, skycloudd, John Vandenberg, Brandon Weeks, and Will Youmans for additional contributions.

# FAQ
**How is "Malachite" pronounced, and what does it mean?**
"Malachite" is pronounced MA-luh-kite, or /ˈmæl.əˌkaɪt/. It is the name of
[a green gemstone](https://en.wikipedia.org/wiki/Malachite). Unfortunately, malachite does not
contain iron, which would have made it a particularly good namesake for a Rust library.

Malachite's logo is an image of a [snub cube](https://en.wikipedia.org/wiki/Snub_cube).

**When does Malachite allocate memory?**
Any `Natural` less than $$2^{64}$$ is represented inline, without allocating memory. Any `Integer`
whose absolute value is less than $$2^{64}$$ doesn't allocate either, and neither does any
`Rational` whose absolute numerator and denominator are both less than $$2^{64}$$. If you're using
a build with `--features 32_bit_limbs`, then the threshold is $$2^{32}$$ instead.

**Can I build Malachite for WebAssembly?**
Yes. If, in the future, Malachite includes code incompatible with Wasm (for example, code that uses
[rayon](https://docs.rs/rayon/latest/rayon/)), it will be possible to disable that code with cargo
flags.

# Blog Posts
<ul>
  {% for post in site.posts %}
    <li>
      <a href="{{ post.url }}">{{ post.title }}</a>
    </li>
  {% endfor %}
</ul>

Copyright © 2026 Mikhail Hogrefe

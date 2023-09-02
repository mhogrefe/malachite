<p align="center">
  <img width="650" src="/assets/logo-and-name.svg" alt="Logo">
</p>

Malachite is an arbitrary-precision arithmetic library for [Rust](https://www.rust-lang.org/). It
achieves high performance in part by using algorithms derived from [GMP](https://gmplib.org/) and
[FLINT](https://www.flintlib.org/).

```rust
use malachite::num::arithmetic::traits::Factorial;
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

Here's a more complex example, calculating the negative-one-millionth power of 3 and displaying the
result with 30 digits of precision.

```rust
use malachite::num::arithmetic::traits::Pow;
use malachite::num::conversion::string::options::ToSciOptions;
use malachite::num::conversion::traits::ToSci;
use malachite::Rational;

fn main() {
    let mut options = ToSciOptions::default();
    options.set_precision(30);
    println!("{}", Rational::from(3).pow(-1_000_000i64).to_sci_with_options(options));
}
```
The output is this:
```
5.56263209915712886588211486263e-477122
```
Every digit is correct, except that the least-significant digit was rounded up from 2. The default
rounding mode,
[`Nearest`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html#variant.Nearest),
uses [bankers' rounding](https://en.wikipedia.org/wiki/Rounding#Round_half_to_even), but you may
specify different rounding behavior via the options parameter.

Malachite is designed to work with very large numbers efficiently. See [here](/performance) for a
performance comparison against other libraries.

To use Malachite, add the following to your project's `Cargo.toml` file:
```yaml
[dependencies.malachite]
version = "0.3.0"
```

By default, all of Malachite's features are included, but you can opt out of some of them. For
example, if you want to use `Natural` and `Integer` but not `Rational`, you can instead use
```yaml
[dependencies.malachite]
version = "0.3.0"
default-features = false
features = [ "naturals_and_integers" ]
```

The `malachite` crate re-exports three sub-crates.
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

  If you need to explicitly include this crate as a dependency of the `malachite` crate, use the
  `naturals_and_integers` or `malachite-nz` feature.
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

  If you need to explicitly include this crate as a dependency of the `malachite` crate, use the
  `rationals` or `malachite-q` feature.

- **malachite-float** Arbitrary-precision floating-point numbers. These are in development, and
  most features are missing.

Malachite is under active development, with many more types and features planned for the future.
Nonetheless, it is extensively tested and documented, and ready for use today. Just be aware that
its API is not stable yet, and that it is licensed under LGPL 3.0.

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

Copyright © 2023 Mikhail Hogrefe

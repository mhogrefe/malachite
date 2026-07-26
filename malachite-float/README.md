- [crates.io](https://crates.io/crates/malachite-float)
- [docs.rs](https://docs.rs/malachite-float/latest/malachite_float/)

Rather than using this crate directly, use the
[`malachite`](https://crates.io/crates/malachite) meta-crate. It re-exports all of this crate's
public members.

In `malachite-float`'s doctests you will frequently see import paths beginning with
`malachite_float::`. When using the `malachite` crate, replace this part of the paths with
`malachite::`.

The import path of the `Float` type is shortened to `malachite::Float`.

# malachite-float
This crate defines
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html)s, which
are arbitrary-precision floating-point numbers. They are not yet feature-complete, but the
functions that are implemented are thoroughly tested and documented.
- A [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html)
  behaves much like a primitive float, except that its precision is chosen per value rather than
  fixed by the type. Like the floats of the IEEE 754 standard, they include NaN, ∞ and -∞,
  and positive and negative zero; unlike them, there is only one NaN, with no payload.
- Their semantics follow [MPFR](https://www.mpfr.org/)'s, so most functions come in a family:
  - `f(x)`, which uses the precision of the input (or the maximum of the inputs);
  - `f_prec(x, prec)`, which rounds to `prec` bits, to the nearest;
  - `f_prec_round(x, prec, rm)`, which rounds to `prec` bits using a specified
    [`RoundingMode`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html);

  along with `_assign` and reference-taking variants. Every function that rounds also returns an
  [`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html), saying whether the
  value returned is less than, equal to, or greater than the exact value. Results are correctly
  rounded: each is the exact result rounded once, never twice.
- There are many functions defined on
  [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html)s. These
  include
  - All the ones you'd expect, like addition, subtraction, multiplication, and division;
  - Square roots, cube roots, and kth roots, along with their reciprocals;
  - Exponentials and logarithms, in an arbitrary base as well as base 2, base 10, and base e,
    including the "1 plus x" and "x minus 1" variants that stay accurate near zero;
  - Powers, with an integer,
    [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html), or
    [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html)
    exponent, and the arithmetic-geometric mean;
  - Around three dozen mathematical constants, computed to any requested precision, from π and e
    to the Gauss, lemniscate, and Prouhet-Thue-Morse constants;
  - Functions for examining and manipulating a
    [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html)'s
    precision, exponent, significand, and ulp.
- Conversion is supported to and from
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s,
  [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)s,
  [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)s,
  primitive integers, and primitive floats, in correctly-rounded and exact flavors.
- String conversion follows MPFR's too. A
  [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html) can be
  written in any base from 2 to 36, or up to 62 through the lower-level MPFR-style entry points,
  with MPFR's `printf`-style formatting available where field widths, flags, and per-call rounding
  are wanted, and read back through
  [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html),
  [`FromStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromStringBase.html),
  and
  [`FromSciString`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromSciString.html).
- The significands of
  [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html)s are
  stored as
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s, so
  [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html)s of
  small enough precision can be stored entirely on the stack.
- Because NaN is not equal to itself, and the two zeros are equal to each other,
  [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html) does
  not implement [`Eq`](https://doc.rust-lang.org/nightly/std/cmp/trait.Eq.html) or
  [`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html). Where a total order is needed,
  for a sort or to use a
  [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html) as a
  hash map key,
  [`ComparableFloat`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.ComparableFloat.html)
  and
  [`ComparableFloatRef`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.ComparableFloatRef.html)
  provide one, distinguishing the zeros, treating NaN as equal to itself, and counting precision as
  part of the value. The digits of a
  [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html) do not
  determine its precision, so these wrappers are also what make a string round trip preserve
  precision as well as value: their output carries a `#` and the precision.

# Demos and benchmarks
This crate comes with a `bin` target that can be used for running demos and benchmarks.
- Almost all of the public functions in this crate have an associated demo. Running a demo
  shows you a function's behavior on a large number of inputs. For example, to demo
  [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html)
  addition, you can use the following command:
  ```text
  cargo run --features bin_build --release -- -l 10000 -m exhaustive -d demo_float_add
  ```
  This command uses the `exhaustive` mode, which generates every possible input, generally
  starting with the simplest input and progressing to more complex ones. Another mode is
  `random`. The `-l` flag specifies how many inputs should be generated.
- You can use a similar command to run benchmarks. The following command benchmarks various
  addition algorithms:
  ```text
  cargo run --features bin_build --release -- -l 1000000 -m random -b \
      benchmark_float_add_algorithms -o add-bench.gp
  ```
  or the addition implementations of other libraries:
  ```text
  cargo run --features bin_build --release -- -l 1000000 -m random -b \
      benchmark_float_add_library_comparison -o add-bench.gp
  ```
  This creates a file called add-bench.gp. You can use gnuplot to create an SVG from it like
  so:
  ```text
  gnuplot -e "set terminal svg; l \"add-bench.gp\"" > add-bench.svg
  ```

The list of available demos and benchmarks is not documented anywhere; you must find them by
browsing through
[`bin_util/demo_and_bench`](https://github.com/mhogrefe/malachite/tree/master/malachite-float/src/bin_util/demo_and_bench).

# Features
- `32_bit_limbs`: Sets the type of `Limb` to
  [`u32`](https://doc.rust-lang.org/nightly/std/primitive.u32.html) instead of the default,
  [`u64`](https://doc.rust-lang.org/nightly/std/primitive.u64.html).
- `random`: This feature provides some functions for randomly generating values. It is off by
  default to avoid pulling in some extra dependencies.
- `enable_serde`: Enables serialization and deserialization using [serde](`https://serde.rs/`).
- `test_build`: A large proportion of the code in this crate is only used for testing. For a
  typical user, building this code would result in an unnecessarily long compilation time and
  an unnecessarily large binary. My solution is to only build this code when the `test_build`
  feature is enabled. If you want to run unit tests, you must enable `test_build`. However,
  doctests don't require it, since they only test the public interface. Enabling this feature also
  enables `random`.
- `bin_build`: This feature is used to build the code for demos and benchmarks, which also
  takes a long time to build. Enabling this feature also enables `test_build` and `random`.

Malachite is developed by Mikhail Hogrefe. Thanks to 43615, b4D8, Romain Billot, Maxim Biryukov, coolreader18, Dasaav-dsv, Duncan Freeman, florian1345, konstin, Rowan Hart, YunWon Jeong, Park Joon-Kyu, Antonio Mamić, OliverNChalk, Kevin Phoenix, probablykasper, shekohex, skycloudd, John Vandenberg, Brandon Weeks, and Will Youmans for additional contributions.

Copyright © 2026 Mikhail Hogrefe

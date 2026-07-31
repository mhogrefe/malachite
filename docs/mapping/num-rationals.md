---
layout: default
title: "Malachite for num Users: Rationals"
permalink: /mapping/num-rationals/
theme: jekyll-theme-slate
---

# Malachite for num Users: Rationals

This page maps the type of the
[num-rational](https://docs.rs/num-rational/latest/num_rational/) crate, `Ratio<T>`, and
chiefly its bignum instantiation `BigRational`, onto its Malachite counterpart:
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html),
from the `malachite-q` crate. It covers num-rational 0.4.2 as re-exported by the
[num](https://docs.rs/num/latest/num/) umbrella crate. It is a companion to
[Malachite for num Users: Integers](/mapping/num-integers/) on the num side, the components of
a `BigRational` being that page's `BigInt`s, and to
[the GMP](/mapping/gmp-rationals/) and [FLINT](/mapping/flint-rationals/) rationals pages on
the type's side, since all three map onto the same `Rational`; the
[mapping index](/mapping/) lists the whole family. As on the integers page, num-rational has
no manual to follow, so the sections below are organized by theme and cover the type's full
public surface, inherent methods and trait implementations together.

One note on routes: the drop-in crate
[malachite-bigint](https://crates.io/crates/malachite-bigint) reimplements the num-bigint API
only, so for rationals the native route described here is the route.

## Conventions {#conventions}

### The types

`Ratio<T>` is generic over its component type, and num-rational names four instantiations:
`Rational32` and `Rational64` over 32- and 64-bit components, the deprecated `Rational` over
`isize`, and `BigRational` over `BigInt`, available behind the `num-bigint` feature, which is
on by default. Malachite has one rational type, with arbitrary-precision components. The
mapping below reads most directly against `BigRational`, but every `Ratio` value embeds in a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html),
and one behavioral difference follows from the widths: arithmetic on the fixed-size
instantiations can overflow its component type, while `Rational` arithmetic always produces
the exact value. Code ported from `Rational64` sheds those overflow cases rather than
translating them.

### Canonical form

The [spectrum described on the FLINT rationals page](/mapping/flint-rationals/#conventions)
gets a third act, and num sits where `fmpq_t` does: the library maintains canonical form,
with one documented door out of it. `Ratio::new` reduces, normalizes the sign so that the
denominator stays positive, gives zero the denominator 1, and panics on a zero denominator;
arithmetic keeps results reduced; and num states that "the only method of procuring a
non-reduced fraction is through `new_raw`", which creates a `Ratio` "without checking for
`denom == 0` or reducing", with the library's own bold warning that "there are several
methods that will panic if used on a `Ratio` with `denom == 0`". The `reduced` method exists
to repair what `new_raw` may have built. Malachite closes that door too:
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)'s
canonical form, the same form num maintains, is
[unbreakable](/mapping/gmp-rationals/#applying-integer-functions-to-rationals), component
access being read-only or mediated by reducing closures, so the `new_raw`/`reduced` pair and
the hazard they bracket have no counterparts here.

### The representation

A `Ratio<T>` is a numerator and a denominator, each a `T`, so a `BigRational` keeps its sign
inside the numerator `BigInt`; a `Rational` is a sign and two
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
magnitudes. This is the same placement difference discussed on
[the GMP rationals page](/mapping/gmp-rationals/#applying-integer-functions-to-rationals),
and it surfaces wherever components are taken apart or reassembled.

### Idiom and features

The [conventions of the integers page](/mapping/num-integers/#conventions) carry over:
operators on owned and borrowed operands with `*Assign` forms, values rather than initialized
variables, exact types with no rounding modes. The `serde` features correspond, with the wire
formats differing as described under serialization below. num-rational has no random-number
support; Malachite's `random` feature covers
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)
as it does the other types. Both libraries work without `std`.

### Categories

Each item falls into one of four categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| — | No counterpart is needed; the notes say why. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## Construction and components {#construction-and-components}

| | num | Malachite |
| :---: | --- | --- |
| ✓ | `Ratio::new (numer: T, denom: T) -> Ratio<T>` | [`from_integers`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_integers), [`from_signeds`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_signeds) |
| — | `Ratio::new_raw (numer: T, denom: T) -> Ratio<T>` | |
| ≈ | `numer (&self) -> &T` | [`numerator_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.numerator_ref) |
| ✓ | `denom (&self) -> &T` | [`denominator_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.denominator_ref) |
| ≈ | `into_raw (self) -> (T, T)` | [`into_numerator_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.into_numerator_and_denominator) |
| ✓ | `Ratio::from_integer (t: T) -> Ratio<T>` | [`From`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#impl-From%3CInteger%3E-for-Rational) |
| ✓ | `From<T>`, `From<(T, T)>` | [`From`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#impl-From%3CInteger%3E-for-Rational), [`from_integers`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_integers) |
| — | `reduced (&self) -> Ratio<T>` | |
| ✓ | `Default` | [`Default`](https://doc.rust-lang.org/nightly/std/default/trait.Default.html) |
| ✓ | `Zero`, `ConstZero` (`zero()`, `ZERO`, `is_zero`) | [`Zero`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| ✓ | `One`, `ConstOne` (`one()`, `ONE`, `is_one`) | [`One`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.One.html) |

**The constructors.** `Ratio::new(n, d)` and
[`Rational::from_integers(n, d)`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_integers)
agree on everything: both reduce, both normalize the sign into the numerator, and both panic
on a zero denominator. The word-component instantiations construct through
[`from_signeds`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_signeds)
and `from_unsigneds`, so `Rational64::new(a, b)` ports as `Rational::from_signeds(a, b)`, and
`from_sign_and_naturals` completes the set for callers holding a sign and two magnitudes.
`from_integer`, `From<T>`, and `From<(T, T)>` are the one-argument spellings,
`Rational::from(n)` for integers of every size, with the pair form behaving as `new` does.

**The raw door.** `new_raw` and `reduced` are the two ends of the hazard described under
[Conventions](#conventions): one builds an unchecked value, the other repairs it. Neither has
a counterpart, because unreduced
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)s
cannot be built; the constructors reduce, and that is the whole story.

**The components.** `denom` and
[`denominator_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.denominator_ref)
both lend a positive denominator for free. `numer` is the ≈ of the
[representation difference](#conventions): num lends a signed `&BigInt`, while
[`numerator_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.numerator_ref)
lends the magnitude as a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html),
with the sign asked for separately. `into_raw` deconstructs by value, and
`into_numerator_and_denominator` does the same with the sign read first,
`let sign = x >= 0u32;` before the components are moved out. Malachite also has a write path
the read-only accessors do not open:
[`mutate_numerator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.mutate_numerator)
and `mutate_denominator` admit a closure to a component and re-reduce afterward, the mediated
access described on
[the GMP rationals page](/mapping/gmp-rationals/#applying-integer-functions-to-rationals).

**The constants.** `Default` "returns zero" on both sides. `Zero` and `One`, with their
`ConstZero` and `ConstOne` refinements, map to the
[`Zero`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html)
and `One` associated constants, `const` from the start; Malachite's shelf continues with
`Two`, `NegativeOne`, and `OneHalf`, so `Rational::ONE_HALF` is a constant rather than a
construction.

## Comparison {#comparison}

| | num | Malachite |
| :---: | --- | --- |
| ✓ | `PartialEq`, `Eq` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html), [`Eq`](https://doc.rust-lang.org/nightly/std/cmp/trait.Eq.html) |
| ✓ | `PartialOrd`, `Ord` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html), [`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html) |
| ✓ | `Hash` | [`Hash`](https://doc.rust-lang.org/nightly/std/hash/trait.Hash.html) |
| ✓ | `Signed::is_positive`, `is_negative` | [`PartialOrd`](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/partial_cmp_primitive_int/index.html) |

**The standard traits.** Exact types, total numeric order, on both sides. The
implementations reach the same answers from opposite directions, in a way worth understanding
when porting `new_raw`-using code: because an unreduced `Ratio` can exist, num implements
equality through comparison and hashes by walking the continued-fraction expansion, its
source noting that a derived hash "needs to agree with `Eq` even for non-reduced ratios";
Malachite's equality and hash are structural, and agree with value semantics because every
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) is
canonical. On values built by ordinary constructors the two libraries are observably
identical. The `Signed` predicates are the mixed comparisons `x > 0u32` and `x < 0u32`, false
for zero on both sides; the rest of `Signed` appears with the arithmetic.

**Mixed comparisons and the magnitude order.** num-rational compares `Ratio`s against
`Ratio`s; Malachite's mixed implementations extend to
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html),
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html),
every primitive integer, and every primitive float, the float comparisons exact and
allocation-free, in both argument orders. The magnitude order comes along as it did
[on the integers page](/mapping/num-integers/#comparison):
[`OrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.OrdAbs.html),
[`PartialOrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.PartialOrdAbs.html),
and
[`EqAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.EqAbs.html)
compare `|x|` with `|y|` directly. One further order exists here:
[`cmp_complexity`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.cmp_complexity),
the simplicity well-order behind Malachite's simplest-in-interval machinery, described on
[the FLINT rationals page](/mapping/flint-rationals/#rational-enumeration).

## Arithmetic {#arithmetic}

| | num | Malachite |
| :---: | --- | --- |
| ✓ | `Add`, `Sub`, `Mul`, `Div`, `*Assign` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html), [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html), [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html), [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html), `*Assign` |
| ≈ | `Rem`, `RemAssign` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html), [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html) |
| ✓ | `Neg` | [`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html), [`NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegAssign.html) |
| ✓ | `recip (&self)`; `Inv` | [`Reciprocal`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Reciprocal.html), [`ReciprocalAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ReciprocalAssign.html) |
| ✓ | `pow (&self, expon: i32)`; `Pow` (primitive and bignum exponents) | [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html), [`PowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowAssign.html) |
| — | `CheckedAdd`, `CheckedSub`, `CheckedMul` | |
| ≈ | `CheckedDiv` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html) |
| ✓ | `Sum`, `Product` | [`Sum`](https://doc.rust-lang.org/nightly/std/iter/trait.Sum.html), [`Product`](https://doc.rust-lang.org/nightly/std/iter/trait.Product.html) |
| ✓ | `Signed::abs` | [`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html), [`AbsAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsAssign.html) |
| ≈ | `Signed::abs_sub` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html) |
| ✓ | `Signed::signum` | [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html) |

**The operators.** The four operations and their `*Assign` forms exist in all owned and
borrowed combinations on both sides, with division by zero a panic on both. Neither library
implements operators between a rational and a scalar, so the asymmetry noted on
[the integers page](/mapping/num-integers/#arithmetic-operators) does not recur here: mixed
operands lift on both sides, `r + Ratio::from_integer(c)` and `r + Rational::from(c)`.

**`Rem`.** num-rational carries `%` over from the primitives: `a % b` is what remains after
removing the truncated quotient, $$a - \operatorname{trunc}(a/b) \cdot b$$. Malachite spells
the definition out, a division, a truncation as in
[the next section](#rounding-and-conversion), a multiplication, and a subtraction; rational
division being exact, nothing else hides in the operator.

**Reciprocals and powers.** `recip` and `Inv` are
[`Reciprocal`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Reciprocal.html)
and its assign form, and the reciprocal of zero panics on both sides. `pow` takes an `i32`
inherently and any primitive or bignum exponent through `Pow`; Malachite's
[`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html)
takes `i64` and `u64`, the bignum-exponent forms being
[wider in type only](/mapping/num-integers/#powers-and-roots) as on the integers page. A
negative exponent inverts first, so zero under a negative exponent panics on both sides.

**The checked family.** The same sorting as on
[the integers page](/mapping/num-integers/#arithmetic-operators): on `BigRational`,
`checked_add`, `checked_sub`, and `checked_mul` cannot fail and exist for generic-code
uniformity, [the traits page](/mapping/num-traits/#operator-refinements)'s subject, while
`checked_div` guards the zero divisor, spelled
`(y != 0u32).then(|| x / y)`. On the fixed-size instantiations the first three are real
overflow guards, which is part of the
[width difference](#conventions) described under Conventions rather than a mappable
operation.

**The `Signed` arithmetic.** `abs` is
[`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html)
with `AbsAssign`; `abs_sub` is the positive difference again,
`if a > b { a - b } else { Rational::ZERO }`; and `signum` reads
[`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html):
`Greater`, `Equal`, and `Less` become `1`, `0`, and `-1` as `Rational`s. `Sum` and `Product`
fold iterators of values or references on both sides.

**Beyond the dictionary.** Malachite's
[`Shl`](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/shl/index.html)
and `Shr` scale a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) by
powers of two exactly, in either direction and by signed or unsigned amounts, and
[`Square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html)
with `SquareAssign` is the dedicated squaring; num-rational spells these `r * &two_pow` and
`&r * &r`.
[`Average`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Average.html),
with `AverageAssign`, is the midpoint of two values, exact on rationals and so free of the
rounding modes its integer counterpart carries; num-integer's `Average` trait is bounded by
`Integer` and does not reach `Ratio`, where the spelling is `(a + b) / two`.

## Rounding and conversion {#rounding-and-conversion}

| | num | Malachite |
| :---: | --- | --- |
| ✓ | `floor (&self) -> Ratio<T>` | [`Floor`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Floor.html) |
| ✓ | `ceil (&self) -> Ratio<T>` | [`Ceiling`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Ceiling.html) |
| ≈ | `round (&self) -> Ratio<T>` | [`RoundingFrom`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/integer_from_rational/index.html) |
| ✓ | `trunc (&self) -> Ratio<T>` | [`RoundingFrom`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/integer_from_rational/index.html) |
| ✓ | `fract (&self) -> Ratio<T>` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html) |
| ✓ | `to_integer (&self) -> T` | [`RoundingFrom`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/integer_from_rational/index.html) |
| ✓ | `is_integer (&self) -> bool` | [`is_integer`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.is_integer) |
| ✓ | `BigRational::from_float (f: T) -> Option<BigRational>` | [`TryFrom`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_primitive_float/index.html) |
| ≈ | `approximate_float`, `approximate_float_unsigned` | [`Approximate`](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/traits/trait.Approximate.html) |
| ≈ | `ToPrimitive` (`to_i64`, `to_u64`, `to_f64`, ...) | [`RoundingFrom`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/primitive_float_from_rational/index.html), [`TryFrom`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/primitive_int_from_rational/index.html) |
| ✓ | `FromPrimitive for BigRational` (`from_i64`, `from_f64`, ...) | [`From`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_primitive_int/index.html), [`TryFrom`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_primitive_float/index.html) |

**Rounding to integers.** num's quartet returns integer-valued `Ratio`s; Malachite's rounding
returns
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)s,
with `Rational::from(i)` rewrapping when the rational shape is wanted. `floor` and `ceil` are
the [`Floor`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Floor.html)
and `Ceiling` traits; `trunc` and `to_integer`, which differ only in packaging, are
`Integer::rounding_from(&q, Down)`; and `fract` is the subtraction its own documentation
defines it by, "satisfies `self == self.trunc() + self.fract()`". The ≈ is `round`: num
"rounds half-way cases away from zero", while `Integer::rounding_from(&q, Nearest)` follows
the bankers' rule, ties to even, and reports the rounding direction as an
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html). Ties-away is a
short spelling when compatibility matters:
`if q >= 0 { (q + Rational::ONE_HALF).floor() } else { (q - Rational::ONE_HALF).ceiling() }`.
`is_integer` maps to
[`is_integer`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.is_integer)
directly.

**Floats, exactly.** `BigRational::from_float` converts a float's exact binary value,
answering `None` for NaN and infinities, and `Rational::try_from(f)` is
[the same conversion](/mapping/gmp-rationals/#conversion-functions) with an `Err` in those
places; `FromPrimitive::from_f64` on `BigRational` delegates to it, so the exact semantics
carry over. On the fixed-size instantiations `from_f64` instead approximates, which belongs
to the [width story](#conventions) under Conventions.

**Floats, approximately.** `approximate_float` walks a continued fraction toward the float,
bounded by the component type and steered by default tolerances; it exists on the fixed-size
instantiations, `Bounded` excluding `BigInt`. Malachite's relatives parameterize the goal
explicitly:
[`approximate`](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/traits/trait.Approximate.html)
finds the best approximation with the denominator bounded by a chosen limit, and
[`from_float_simplest`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_float_simplest)
finds the simplest
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)
that rounds to the given float. The surrounding machinery, continued fractions, convergents,
and `simplest_rational_in_interval`, is mapped in detail on
[the FLINT rationals page](/mapping/flint-rationals/#continued-fractions).

**`ToPrimitive`.** The integer targets truncate and then fit, which is
`Integer::rounding_from(&q, Down)` followed by the integer conversions of
[the integers page](/mapping/num-integers/#conversion), with the usual `Option`-against-`Err`
shape. For `to_f64`, the implementations differ by instantiation: with fixed-size components
num divides the two components as `f64`s, rounding each before the quotient, while
`BigRational` converts through a scaled division built for the purpose. Malachite's
`f64::rounding_from(&q, rm)` rounds the exact value once, in the chosen mode, and reports the
direction.

## Strings and serde {#strings-and-serde}

| | num | Malachite |
| :---: | --- | --- |
| ✓ | `Display` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |
| ≈ | `Debug` | [`Debug`](https://doc.rust-lang.org/nightly/std/fmt/trait.Debug.html) |
| ✓ | `Binary`, `Octal`, `LowerHex`, `UpperHex` | [`Binary`](https://doc.rust-lang.org/nightly/std/fmt/trait.Binary.html), [`Octal`](https://doc.rust-lang.org/nightly/std/fmt/trait.Octal.html), [`LowerHex`](https://doc.rust-lang.org/nightly/std/fmt/trait.LowerHex.html), [`UpperHex`](https://doc.rust-lang.org/nightly/std/fmt/trait.UpperHex.html) |
| ≈ | `LowerExp`, `UpperExp` | [`ToSci`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToSci.html) |
| ✓ | `FromStr` (error: `ParseRatioError`) | [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) |
| ✓ | `Num::from_str_radix (s: &str, radix: u32) -> Result` | [`FromStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromStringBase.html) |
| ≈ | `Serialize`, `Deserialize` | [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html), [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) |

**`Display` and `Debug`.** The display convention converges exactly, and it is the same
convergence [the FLINT page found](/mapping/flint-rationals/#input-and-output) in
`fmpq_print`: both libraries print `numer/denom` and drop the `/1`, so an integral value
prints as a bare integer. `Debug` differs: num derives it, printing the struct form with
field names, while Malachite's `Debug` prints what `Display` prints.

**Radix strings.** num parses any radix from 2 to 36 through `Num::from_str_radix`, and
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)'s
`from_string_base` covers it, reaching 62 with
[the GMP digit alphabet](/mapping/gmp-rationals/#conversion-functions). The grammars differ at
the edges: num requires the `numer/denom` shape, rejecting a bare integer, and hands each
component to the component type's parser, so `"1/-2"` is accepted and canonicalized;
Malachite's slash is optional and its sign belongs only at the front of the string. Both
reduce, and both refuse a zero denominator. In the other direction, `to_string_base` produces
the value in any base as `numer/denom`, and the `Binary`, `Octal`, `LowerHex`, and `UpperHex`
rows converge the way `Display` does: both libraries print the components in the radix,
separated by `'/'`, dropping the `/1` of an integral value, and both expand the `#` flag to a
prefix on each component, `0xff/0x7`. Only the padded forms differ, and in the same way as
`Display`: num sizes width and fill against the whole composite string through
[`pad_integral`](https://doc.rust-lang.org/nightly/std/fmt/struct.Formatter.html#method.pad_integral),
while Malachite hands the formatter flags to each component.

**Scientific notation.** num's `LowerExp` and `UpperExp` render each component in exponential
notation, `numer`'s over `denom`'s. Malachite's
[`ToSci`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToSci.html)
renders the *value*: correctly rounded scientific digits under
[`ToSciOptions`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/string/options/struct.ToSciOptions.html)
control of precision and rounding mode, with `from_sci_string` parsing the notation back in.
The two serve presentation differently enough that the row is ≈ rather than a rename.

**`FromStr`.** The grammars agree in detail: num "parses `numer/denom` or just `numer`", and
Malachite accepts the same two shapes, where "the numerator and denominator do not need to be
in lowest terms, but the denominator must be nonzero". Both reduce unreduced text, and both
return an error rather than panicking on a zero denominator, num's a typed
`ParseRatioError`, Malachite's a unit error.

**serde.** The wire formats differ as they did
[on the integers page](/mapping/num-integers/#strings-formatting-and-serde): num serializes
the pair `(numer, denom)`, each component in num-bigint's digit-sequence format, while
Malachite writes a three-field struct, the sign and two hexadecimal-string magnitudes. Both
reject a zero denominator when deserializing. One difference runs deeper than format: num
deserializes through `new_raw`, so unreduced wire data enters as an unreduced `Ratio`, which
its value-based equality and hash
[absorb by design](#comparison); Malachite's deserialization checks canonical form outright,
rejecting what does not satisfy it, the validation described on
[the GMP rationals page](/mapping/gmp-rationals/#input-and-output-functions).
As before, data migrates between the libraries through explicit components rather than
directly.

That completes the type-level mapping; the generic traits a `Ratio<T>` answers to are
[the traits page](/mapping/num-traits/)'s subject.

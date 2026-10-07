---
layout: default
title: "Malachite for Azurite Users: Rationals"
permalink: /mapping/azurite-rationals/
theme: jekyll-theme-slate
---

# Malachite for Azurite Users: Rationals

This page maps the operations of [Azurite](https://github.com/mhogrefe/azurite)'s `AzRat` type,
its rational number in lowest terms, onto their Malachite counterparts on
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html), from
the `malachite-q` crate. It is a companion of
[Malachite for Azurite Users: Naturals](/mapping/azurite-naturals/) and
[Integers](/mapping/azurite-integers/), whose
[Conventions](/mapping/azurite-naturals/#conventions) carry over; only what is new for rationals
is repeated here. The page covers Azurite as of commit `4932438` (2026-10-04). The
[mapping index](/mapping/) lists the whole family of pages.

## Conventions {#conventions}

### The types

An `AzRat` is a sign, an `AzNat` numerator magnitude, and an `AzNat` denominator, with proofs
that the denominator is nonzero, that the numerator and denominator are coprime, and that zero
carries the positive sign. A
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) is the
same construction, a sign and two
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s in
lowest terms with a single zero, so both representations are canonical, equality is structural on
both sides, and the two print identically: `"0"`, `"4"`, `"1/3"`, `"-1/3"`.

### Total functions

As on the other pages, Lean's totality shows up where Malachite panics: `x / 0 = 0`, `0⁻¹ = 0`
(the `GroupWithZero` convention), `0 ^ (-k) = 0`, and a zero denominator passed to a constructor
gives `0` (Lean's `mkRat` convention), while Malachite panics on division by zero, on the
reciprocal of zero, on `0.pow(-k)`, and on a zero denominator. Rows for such pairs are ✓, the two
agreeing wherever Azurite defines the result; Malachite's
[`checked_div`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedDiv.html)
is the `Option` form of division.

### Typeclasses and traits

`AzRat` is a verified Mathlib `Field` and `IsStrictOrderedRing` with a `LinearOrder`, so the
whole linearly-ordered-field API applies to it, including `Neg`, `Inv`, `Div`, `HShiftLeft` and
`HShiftRight` by a `Nat`, `OfNat` literals, and `NatCast`/`IntCast`/`RatCast`. Malachite has no
field abstraction; the operators and the
[`malachite_base`](https://docs.rs/malachite-base/latest/malachite_base/) traits stand in.

### Categories

Each definition falls into one of five categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| ⚙ | Malachite does not expose this algorithm or helper; the Malachite column or the notes say what to call instead. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## Construction and conversion {#construction-and-conversion}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `instance : OfNat AzRat 0`, `instance : Zero AzRat` | [`Rational::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| ✓ | `instance : OfNat AzRat 1`, `instance : One AzRat` | [`Rational::ONE`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.One.html) |
| ✓ | `ofSignAzNats (s : Bool) (num den : AzNat) : AzRat` | [`from_sign_and_naturals`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_sign_and_naturals) |
| ✓ | `ofAzNats (num den : AzNat) : AzRat` | [`from_naturals`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_naturals) |
| ✓ | `ofAzInts (num den : AzInt) : AzRat` | [`from_integers`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_integers) |
| ✓ | `AzNat.toAzRat (n : AzNat) : AzRat`, `instance : NatCast AzRat` | [`From<Natural>`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_natural/index.html) |
| ✓ | `AzInt.toAzRat (z : AzInt) : AzRat`, `instance : IntCast AzRat` | [`From<Integer>`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_integer/index.html) |
| ✓ | `UInt64.toAzRat`, `UInt32.toAzRat`, `UInt16.toAzRat`, `UInt8.toAzRat`, `USize.toAzRat`, `Int64.toAzRat`, `Int32.toAzRat`, `Int16.toAzRat`, `Int8.toAzRat`, `ISize.toAzRat` | [`From`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_primitive_int/index.html) |
| ✓ | `AzRat.num : AzNat`, `AzRat.den : AzNat` | [`to_numerator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.to_numerator), [`to_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.to_denominator) |
| ✓ | `AzRat.sign : Bool` | [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html) |
| ⚙ | `numInt (q : AzRat) : AzInt` | [`to_numerator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.to_numerator), [`Integer::from_sign_and_abs`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_sign_and_abs) |
| — | `ofRat (r : ℚ) : AzRat`, `toRat (q : AzRat) : ℚ`, `instance : RatCast AzRat`, `instance : NNRatCast AzRat` | |

**Constructors.** `ofSignAzNats s n d` reduces `n / d`, canonicalizes zero, and gives it sign `s`,
which is `Rational::from_sign_and_naturals(s, n, d)`; `ofAzNats` and `ofAzInts` are
`from_naturals` and `from_integers`, the latter taking its sign from the two integers' signs. The
difference is a zero denominator, `0` in Azurite and a panic in Malachite. The ten machine-integer
conversions and the two casts are `Rational::from(x)`, and `Rational` has `const_from_unsigneds`
and `const_from_signeds` for compile-time constants, which need no Azurite counterpart.

**Fields.** `q.num` and `q.den` are the magnitudes `q.to_numerator()` and `q.to_denominator()`
(or `numerator_ref()` and `denominator_ref()` to borrow); `q.sign` is `q >= 0`, and `sign()` is
the `Ordering` against zero. `numInt` is the signed numerator, which Malachite does not expose
directly: `Integer::from_sign_and_abs(q >= 0, q.to_numerator())`. Malachite's
[`mutate_numerator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.mutate_numerator)
family edits a rational in place; an `AzRat` is rebuilt with a constructor.

**`ofRat`, `toRat`.** The bridge to Mathlib's `ℚ`, the specification side of Azurite's proofs,
as `toNat` is [on the naturals page](/mapping/azurite-naturals/#proofs); it has no counterpart.

## Comparison {#comparison}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `cmp (x y : AzRat) : Ordering`, `instance : Ord AzRat`, `LE`, `LT`, `LinearOrder` | [`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html), [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `deriving DecidableEq` | [`Eq`](https://doc.rust-lang.org/nightly/std/cmp/trait.Eq.html), [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ✓ | `instance : Max AzRat`, `instance : Min AzRat` | [`Ord::max`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html#method.max), [`Ord::min`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html#method.min) |
| ✓ | `signOrd (x : AzRat) : Ordering` | [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html) |

**Comparison.** Both compare exactly, Azurite by a staged algorithm (signs, then the position
relative to 1, then bit lengths, then a cross-multiplication) and Malachite similarly without
forming the cross products when it can avoid them. Malachite also compares a `Rational` against
`Natural`, `Integer`, and every primitive integer and float
([`PartialOrd<Natural>`](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/partial_cmp_natural/index.html)
and siblings), and by magnitude
([`OrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.OrdAbs.html));
Azurite converts the other operand with `toAzRat` and compares, and compares `abs` values.

## Arithmetic {#arithmetic}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `add (x y : AzRat) : AzRat`, `instance : Add AzRat` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html), [`AddAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.AddAssign.html) |
| ✓ | `sub (x y : AzRat) : AzRat`, `instance : Sub AzRat` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html), [`SubAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.SubAssign.html) |
| ✓ | `mul (x y : AzRat) : AzRat`, `instance : Mul AzRat` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.MulAssign.html) |
| ✓ | `div (x y : AzRat) : AzRat`, `instance : Div AzRat` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html), [`DivAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.DivAssign.html) |
| ✓ | `neg (q : AzRat) : AzRat`, `instance : Neg AzRat` | [`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html), [`NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegAssign.html) |
| ✓ | `AzRat.abs (q : AzRat) : AzRat` | [`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html), [`AbsAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsAssign.html) |
| ✓ | `inv (q : AzRat) : AzRat`, `instance : Inv AzRat` | [`Reciprocal`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Reciprocal.html), [`ReciprocalAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ReciprocalAssign.html) |
| ✓ | `AzRat.pow (q : AzRat) (n : ℕ) : AzRat`, the `Field` `npow` | [`Pow<u64>`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html), [`PowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowAssign.html) |
| ✓ | `AzRat.zpow (q : AzRat) : ℤ → AzRat`, the `Field` `zpow` | [`Pow<i64>`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html) |
| ✓ | `AzRat.shiftLeft (x : AzRat) (n : Nat) : AzRat`, `instance : HShiftLeft AzRat Nat AzRat` | [`Shl`](https://doc.rust-lang.org/nightly/std/ops/trait.Shl.html), [`ShlAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.ShlAssign.html) |
| ✓ | `AzRat.shiftRight (x : AzRat) (n : Nat) : AzRat`, `instance : HShiftRight AzRat Nat AzRat` | [`Shr`](https://doc.rust-lang.org/nightly/std/ops/trait.Shr.html), [`ShrAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.ShrAssign.html) |
| — | `combineSigned (sx sy : Bool) (u v : AzNat) : Bool × AzNat` | |
| — | `instance : Field AzRat`, `instance : IsStrictOrderedRing AzRat` | |

**Field operations.** The four operations agree, with `x / 0 = 0` against Malachite's panic (or
`checked_div`'s `None`); both keep results in lowest terms by reducing the cross pairs rather than
the products. `inv` is `reciprocal()`, with `0⁻¹ = 0` against a panic. `pow` is `pow(e)` with a
`u64` exponent and `zpow` with an `i64` one, where `0^(-k)` is `0` in Azurite and a panic in
Malachite. Shifts multiply or divide by $$2^n$$ without a GCD on either side; Malachite's shift
count may be any primitive integer, a negative count reversing the direction. Malachite also has
[`Square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html),
[`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html),
and the other derived operations of the GMP and FLINT pages, which Azurite spells with the ring
operations, and functions with no Azurite counterpart at all
([`approximate`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Approximate.html),
[`simplest_rational_in_interval`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.simplest_rational_in_interval),
continued fractions, roots). `combineSigned` is the sign bookkeeping shared by `add` and `sub`.

## Rounding and logarithms {#rounding-and-logarithms}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `round (q : AzRat) (mode : RoundingMode) : AzInt × Ordering` | [`RoundingFrom<Rational>`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/integer_from_rational/index.html) for `Integer` |
| ✓ | `floorLogBase2Abs (q : AzRat) : ℤ`, `ceilingLogBase2Abs (q : AzRat) : ℤ` | [`floor_log_base_2_abs`](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/log_base_2/index.html), [`ceiling_log_base_2_abs`](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/log_base_2/index.html) |
| ✓ | `floorLogBaseAbs (b : UInt64) (q : AzRat) : ℤ` | [`FloorLogBase<u64>`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorLogBase.html) |
| — | `cmpPowAbs (b : UInt64) (e : ℤ) (q : AzRat) : Ordering`, `floorLogSearch` | |

**Rounding to an integer.** `round q mode` is `Integer::rounding_from(q, mode)`: both return the
rounded integer and its `Ordering` against `q`, with ties to even under `Nearest`; Malachite's
`Exact` mode panics unless `q` is an integer. Malachite's
[`Floor`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Floor.html)
and
[`Ceiling`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Ceiling.html)
are the `Floor` and `Ceiling` modes with the `Ordering` dropped, and
[`RoundToMultiple`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.RoundToMultiple.html)
rounds to a multiple of another rational, which Azurite spells as `round (q / m) mode` scaled back.

**Logarithms.** The base-2 pair agrees exactly, both reading the exponent off the bit lengths of
the numerator and denominator. `floorLogBaseAbs b q` is `q.floor_log_base(b)` for a `u64` base,
where Malachite requires `q > 0` and Azurite takes `|q|` (returning `0` for `q = 0`); Azurite
brackets the exponent and binary-searches it, Malachite estimates it with floating-point
logarithms and corrects, and the results agree. Malachite's
[`CeilingLogBase`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingLogBase.html)
and
[`CheckedLogBase`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedLogBase.html)
follow from the floor and a comparison of $$b^e$$ with $$|q|$$ (`cmpPowAbs`), and its
`floor_log_base(&Rational)` family takes a rational base, which Azurite does not. The three
`FloorLogBase2`/`CeilingLogBase2`/`CheckedLogBase2` traits are the positive-only versions of the
`_abs` functions.

## Strings and scientific notation {#strings-and-scientific-notation}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `toString (q : AzRat) : String`, `toChars`, `instance : ToString AzRat` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |
| ≈ | `parse (s : String) : Option AzRat` | [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) |
| ✓ | `fromSci (s : String) (b : UInt64 := 10) : Option AzRat` | [`FromSciString`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromSciString.html) |
| ✓ | `toSci (q : AzRat) (o : SciOptions := {}) : Option String`, `toSciString (q : AzRat) : String` | [`ToSci`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToSci.html) |
| ✓ | `toSciExact (q : AzRat) (o : SciOptions) : Bool` | [`fmt_sci_valid`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToSci.html#tymethod.fmt_sci_valid) |
| ✓ | `SciOptions`, `SciSizeOptions`, `SciFormat` | [`ToSciOptions`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/string/options/struct.ToSciOptions.html), [`SciSizeOptions`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/string/options/enum.SciSizeOptions.html) |
| ✓ | `lengthAfterPoint (b : UInt64) (q : AzRat) : Option Nat` | [`length_after_point_in_small_base`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.length_after_point_in_small_base) |
| — | `toSciNumber`, `SciNumber`, `zeroScale`, `sizeScale`, `scaledRound`, `basePrimeFactors`, `countFactor`, the `fromSci` helpers (`splitLast`, `splitExponent`, `parseSignedDigits`, …) | |
| — | `instance : ParsableElement AzRat` | |

**Plain strings.** Both print `"-1/3"` style, the sign on the numerator and no `/1` for an
integer, so `toString` is `Display` (and Malachite's `Debug` is the same). `parse` and
`Rational::from_str` both read `num`, `-num`, `num/den` and reduce an unreduced fraction, but
differ at the edges: Malachite accepts a single leading `+` on the numerator and on the
denominator and rejects a zero denominator, while Azurite rejects `+`, accepts `0x`/`0o`/`0b`
prefixes through `AzNat.parse`, maps a zero denominator to `0`, and rejects `-0`. Malachite's
other bases are
[`FromStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromStringBase.html)
and
[`ToStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToStringBase.html).

**Scientific notation.** Azurite's `fromSci` and `toSci` are ports of Malachite's
`from_sci_string` and `to_sci`, so the accepted language and the output agree digit for digit:
`fromSci s b` is `Rational::from_sci_string_with_options(s, options)` with `options` set to base
`b` (the rounding mode in
[`FromSciStringOptions`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/string/options/struct.FromSciStringOptions.html)
is for integer targets and does not affect a `Rational`), and `toSci q o` is
`q.to_sci_with_options(o)`, with `SciOptions`' `base`, `mode`, and `size` the `ToSciOptions`
fields `base`, `rounding_mode`, and `size_options`, and `SciFormat`'s `negExpThreshold`,
`lowercase`, `eLowercase`, `forceExponentPlusSign`, and `includeTrailingZeros` the remaining five.
The defaults coincide (base 10, `Nearest`, 16 significant digits, exponent notation below
$$10^{-6}$$). Malachite's `Exact` rounding mode is Azurite's `toSciExact` predicate, which is
`fmt_sci_valid` for `Exact` options; `toSci` returns `none` where `to_sci_with_options` panics
(invalid options, or `Complete` for a non-terminating expansion). Malachite's
`from_sci_string_simplest` finds the simplest rational in the string's rounding interval and has
no Azurite counterpart. `lengthAfterPoint b q` is `q.length_after_point_in_small_base(b)`, the
number of digits after the point in a terminating base-`b` expansion.

## Exhaustive generation {#exhaustive-generation}

Azurite has no `ExhaustiveGenerator` instances for `AzRat` yet; Malachite's
[`exhaustive_rationals`](https://docs.rs/malachite-q/latest/malachite_q/rational/exhaustive/fn.exhaustive_rationals.html)
and its range variants are unmapped.

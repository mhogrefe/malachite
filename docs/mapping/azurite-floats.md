---
layout: default
title: "Malachite for Azurite Users: Floats"
permalink: /mapping/azurite-floats/
theme: jekyll-theme-slate
---

# Malachite for Azurite Users: Floats

This page maps the operations of [Azurite](https://github.com/mhogrefe/azurite)'s `AzFloat` type,
its arbitrary-precision binary floating-point number, onto their Malachite counterparts on
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html), from the
`malachite-float` crate. It is a companion of
[Malachite for Azurite Users: Naturals](/mapping/azurite-naturals/),
[Integers](/mapping/azurite-integers/), and [Rationals](/mapping/azurite-rationals/), whose
[Conventions](/mapping/azurite-naturals/#conventions) carry over; only what is new for floats is
repeated here. The page covers Azurite as of commit `3cd44c3` (2026-10-05). The
[mapping index](/mapping/) lists the whole family of pages.

## Conventions {#conventions}

### The types

An `AzFloat` is `NaN`, a signed infinity, zero, or a finite nonzero value with a sign, an `AzInt`
exponent, a precision, and an `AzNat` significand. A
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html) is the same
four-way classification with an `i32` exponent and a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
significand. The representations were designed to coincide: both use the exponent convention
$$2^{e-1} \leq |x| < 2^e$$, both keep the significand left-aligned at a 64-bit limb boundary with
exactly `precision` significant bits, and both are canonical, so the hexadecimal rendering of a
value with its precision (`0x1.8#2`) identifies it on either side.

Two things differ. Azurite's exponent is unbounded, so no `AzFloat` operation overflows or
underflows; Malachite's raw exponent lies in $$[-(2^{30}-1), 2^{30}-1]$$
([`MIN_EXPONENT`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#associatedconstant.MIN_EXPONENT),
[`MAX_EXPONENT`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#associatedconstant.MAX_EXPONENT)),
and a result beyond it becomes $$\pm\infty$$, the largest finite value of its precision, zero, or
the smallest positive value, by rules that depend on the rounding mode and that every
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html) operation
documents under "Overflow and underflow". And Azurite has a single zero where Malachite has `0.0`
and `-0.0`: with an unbounded exponent nothing underflows, so there is no sign of an underflowed
value to remember. A function whose result is zero on both sides may therefore differ in the sign of
that zero. These two differences are the whole content of ≈ on this page; everywhere else the two
agree bit for bit.

### Rounding

Both round to a target precision in `Floor`, `Ceiling`, `Down`, `Up`, or `Nearest` (ties to even)
and return the `Ordering` of the result against the exact value. Malachite's `Exact` mode panics
when rounding would be needed; on the Azurite side that is "the result in any mode is exact", as
[on the naturals page](/mapping/azurite-naturals/#conventions). The `*PrecRound` functions are the
`_prec_round` methods; Azurite's `Add`, `Sub`, `Mul`, and `Div` instances round to nearest at the
larger of the operands' precisions, exactly as Malachite's `+`, `-`, `*`, and `/` do, and its
`sqr`, `sqrt`, and `rsqrt` at the operand's precision, as `square()`, `sqrt()`, and
`reciprocal_sqrt()` do.

### Categories

Each definition falls into one of five categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| ⚙ | Malachite does not expose this algorithm or helper; the Malachite column or the notes say what to call instead. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## Construction and classification {#construction-and-classification}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `nan : AzFloat` | [`Float::NAN`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.NaN.html) |
| ✓ | `infinity (sign : Bool)`, `posInfinity`, `negInfinity` | [`Float::INFINITY`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Infinity.html), [`Float::NEGATIVE_INFINITY`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.NegativeInfinity.html) |
| ≈ | `zero : AzFloat`, `instance : Zero AzFloat` | [`Float::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html), [`Float::NEGATIVE_ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.NegativeZero.html) |
| ✓ | `one`, `negOne`, `two`, `oneHalf`, `instance : One AzFloat` | [`Float::ONE`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.One.html), [`NEGATIVE_ONE`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.NegativeOne.html), [`TWO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Two.html), [`ONE_HALF`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.OneHalf.html) |
| ✓ | `powerOf2 (e : AzInt) : AzFloat` | [`power_of_2`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_2), [`power_of_2_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_2_prec_round) |
| ⚙ | `mkFinite (sign : Bool) (exponent : AzInt) (p : Nat) (m : AzNat) : AzFloat` | [`from_integer_mantissa_and_exponent`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_integer_mantissa_and_exponent), [`from_raw_mantissa_and_exponent`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_raw_mantissa_and_exponent) |
| ✓ | `isNaN`, `isInfinite`, `isZero`, `isFinite`, `isNormal` | [`is_nan`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.is_nan), [`is_infinite`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.is_infinite), [`is_zero`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.is_zero), [`is_finite`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.is_finite), [`is_normal`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.is_normal) |
| ≈ | `isPositive`, `isNegative`, `sign? : AzFloat → Option Bool` | [`is_sign_positive`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.is_sign_positive), [`is_sign_negative`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.is_sign_negative), [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html) |
| ✓ | `exponent? : AzFloat → Option AzInt`, `precision? : AzFloat → Option Nat`, `significand? : AzFloat → Option AzNat` | [`get_exponent`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.get_exponent), [`get_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.get_prec), [`to_significand`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.to_significand) |
| ≈ | `ulp? : AzFloat → Option AzFloat` | [`ulp`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ulp) |
| — | `FiniteValid`, `alignedBits`, `instance : Inhabited AzFloat` | |

**Constants and construction.** The named constants coincide (Malachite's have precision 1, as
Azurite's do, and
[`one_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.one_prec)
and
[`two_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.two_prec)
are `setPrec one p` and `setPrec two p`). `powerOf2 e` is `Float::power_of_2(e)` at precision 1, and
`power_of_2_prec_round(e, p, rm)` is `setPrecRound (powerOf2 e) p rm`, exact until it leaves
Malachite's exponent range. `mkFinite` builds a value from its parts, left-aligning the significand;
Malachite's constructors take an integer mantissa and exponent, or the raw aligned pair, and are the
same operation under a different parametrization. Malachite's
[`min_positive_value_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.min_positive_value_prec)
and
[`max_finite_value_with_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.max_finite_value_with_prec)
are the ends of its exponent range and have no Azurite meaning.

**Classification.** The five predicates agree, `isNormal` being "finite and nonzero" on both sides.
`isPositive` and `isNegative` are the strict comparisons with zero, `false` for zero and `NaN`;
Malachite's `is_sign_positive` and `is_sign_negative` read the sign bit, so they are `true` for
`0.0` and `-0.0` respectively, and `Float`'s `sign()` returns `Greater` or `Less` for a zero by
the same bit, never `Equal`. `sign?` is `None` for zero, as the other two have no sign to report.
The three accessors agree, including the significand, which both libraries left-align at a limb
boundary. `ulp?` is $$2^{e-p}$$ at precision 1; `ulp()` is the same, except that it returns `None`
when that power of two is below Malachite's exponent range.

## Comparison {#comparison}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `partialCompare : AzFloat → AzFloat → Option Ordering` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `eqIEEE`, `lt`, `le`, `gt`, `ge` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html), [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ≈ | `deriving DecidableEq` | [`ComparableFloat`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.ComparableFloat.html)'s [`Eq`](https://doc.rust-lang.org/nightly/std/cmp/trait.Eq.html) and [`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html) |
| — | `compareMagnitude`, `compareSigns` | |

**Two equalities.** Both libraries distinguish IEEE comparison, where `NaN` is unordered and the
precision is ignored (`partialCompare`, `eqIEEE`; `Float`'s `PartialOrd` and `PartialEq`), from
structural equality, where the precision counts and `NaN` equals itself (`=` on `AzFloat`;
[`ComparableFloat`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.ComparableFloat.html)).
The structural one is ≈ because `ComparableFloat` also distinguishes `0.0` from `-0.0`, and because
it is a total order: equal values sort by precision and the zeros and `NaN` have fixed positions,
which Azurite does not define. Malachite's comparisons against
[`Natural`](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_natural/index.html),
`Integer`, `Rational`, and the primitive types, and its magnitude comparisons
([`PartialOrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.PartialOrdAbs.html)),
are spelled in Azurite by converting the other operand exactly (`ofAzNat`, `ofAzInt`) and comparing
`abs` values.

## Constants {#constants}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `sqrt2PrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sqrt2` | [`sqrt_2_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_2_prec_round), [`sqrt_2_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_2_prec) |
| ✓ | `sqrt3PrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sqrt3` | [`sqrt_3_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_3_prec_round), [`sqrt_3_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_3_prec) |
| ✓ | `sqrt5PrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sqrt5` | [`sqrt_5_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_5_prec_round), [`sqrt_5_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_5_prec) |
| ✓ | `sqrt2Over2PrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sqrt2Over2` | [`sqrt_2_over_2_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_2_over_2_prec_round), [`sqrt_2_over_2_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_2_over_2_prec) |
| ✓ | `sqrt3Over3PrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sqrt3Over3` | [`sqrt_3_over_3_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_3_over_3_prec_round), [`sqrt_3_over_3_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_3_over_3_prec) |
| ✓ | `sqrt5Over5PrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sqrt5Over5` | [`sqrt_5_over_5_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_5_over_5_prec_round), [`sqrt_5_over_5_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_5_over_5_prec) |
| ✓ | `phiPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `phi` | [`phi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.phi_prec_round), [`phi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.phi_prec) |
| ✓ | `primeConstantPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `primeConstant` | [`prime_constant_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.prime_constant_prec_round), [`prime_constant_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.prime_constant_prec) |
| ✓ | `prouhetThueMorsePrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `prouhetThueMorse` | [`prouhet_thue_morse_constant_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.prouhet_thue_morse_constant_prec_round), [`prouhet_thue_morse_constant_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.prouhet_thue_morse_constant_prec) |
| — | `phiApprox`, `wordOfBits`, `primeConstantLimbs`, `prouhetThueMorseSeq`, `prouhetThueMorseWord`, `prouhetThueMorseWordNot`, `prouhetThueMorseLimb`, `prouhetThueMorseLimbs` | |

**The constants.** Each is the constant rounded to precision `p` with `mode`, with the comparison
with the exact value, and the plain form rounds to nearest; Malachite's `_prec_round` and `_prec`
functions return the same pairs, bit for bit. The constants lie well inside Malachite's exponent
range and are never zero, so neither of the ≈ reasons in [Conventions](#conventions) arises.
$$\sqrt2$$, $$\sqrt3$$, $$\sqrt5$$, their reciprocals $$\sqrt2/2$$, $$\sqrt3/3$$, and $$\sqrt5/5$$,
and the golden ratio $$\varphi$$ are irrational, so no rounding is exact; the prime constant (bit
$$k$$ of the binary expansion is 1 iff $$k$$ is prime) and the Prouhet–Thue–Morse constant (bit
$$k$$ is the parity of the number of 1s in $$k$$) are also irrational, and Azurite builds their
expansions a limb at a time. Malachite has many more constants; see the [MPFR
page](/mapping/mpfr-floats/).

## Arithmetic {#arithmetic}

| | Azurite | Malachite |
| :---: | --- | --- |
| ≈ | `addPrecRound (x y : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `instance : Add AzFloat` | [`add_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.add_prec_round), [`add_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.add_prec), [`add_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.add_round), [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html) |
| ≈ | `subPrecRound (x y : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `instance : Sub AzFloat` | [`sub_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sub_prec_round), [`sub_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sub_prec), [`sub_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sub_round), [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html) |
| ≈ | `mulPrecRound (x y : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `instance : Mul AzFloat` | [`mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.mul_prec_round), [`mul_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.mul_prec), [`mul_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.mul_round), [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html) |
| ≈ | `divPrecRound (x y : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `instance : Div AzFloat` | [`div_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.div_prec_round), [`div_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.div_prec), [`div_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.div_round), [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html) |
| ≈ | `addRatPrecRound (x : AzFloat) (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `instance : HAdd AzFloat AzRat AzFloat`, `HAdd AzRat AzFloat AzFloat` | [`add_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.add_rational_prec_round), [`add_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.add_rational_prec), [`add_rational_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.add_rational_round), [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html) |
| ≈ | `subRatPrecRound (x : AzFloat) (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `instance : HSub AzFloat AzRat AzFloat` | [`sub_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sub_rational_prec_round), [`sub_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sub_rational_prec), [`sub_rational_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sub_rational_round), [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html) |
| ≈ | `ratSubPrecRound (q : AzRat) (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `instance : HSub AzRat AzFloat AzFloat` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html), [`sub_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sub_rational_prec_round) |
| ≈ | `mulRatPrecRound (x : AzFloat) (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `instance : HMul AzFloat AzRat AzFloat`, `HMul AzRat AzFloat AzFloat` | [`mul_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.mul_rational_prec_round), [`mul_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.mul_rational_prec), [`mul_rational_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.mul_rational_round), [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html) |
| ≈ | `divRatPrecRound (x : AzFloat) (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `instance : HDiv AzFloat AzRat AzFloat` | [`div_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.div_rational_prec_round), [`div_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.div_rational_prec), [`div_rational_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.div_rational_round), [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html) |
| ≈ | `ratDivPrecRound (q : AzRat) (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `instance : HDiv AzRat AzFloat AzFloat` | [`rational_div_float_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.rational_div_float_prec_round), [`rational_div_float_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.rational_div_float_prec), [`rational_div_float_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.rational_div_float_round), [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html) |
| ≈ | `sqrPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sqr` | [`square_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.square_prec_round), [`square_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.square_prec), [`square_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.square_round), [`Square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html) |
| ≈ | `sqrtPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sqrt` | [`sqrt_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_prec_round), [`sqrt_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_prec), [`sqrt_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_round), [`Sqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sqrt.html) |
| ≈ | `rsqrtPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `rsqrt` | [`reciprocal_sqrt_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.reciprocal_sqrt_prec_round), [`reciprocal_sqrt_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.reciprocal_sqrt_prec), [`reciprocal_sqrt_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.reciprocal_sqrt_round), [`ReciprocalSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ReciprocalSqrt.html) |
| ≈ | `neg : AzFloat → AzFloat`, `instance : Neg AzFloat` | [`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html), [`NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegAssign.html) |
| ≈ | `abs : AzFloat → AzFloat` | [`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html), [`AbsAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsAssign.html) |
| ≈ | `shiftLeft (x : AzFloat) (k : AzInt)`, `shiftRight`, `instance : HShiftLeft AzFloat AzInt AzFloat`, `HShiftRight` | [`Shl`](https://doc.rust-lang.org/nightly/std/ops/trait.Shl.html), [`Shr`](https://doc.rust-lang.org/nightly/std/ops/trait.Shr.html), [`ShlAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.ShlAssign.html), [`ShrAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.ShrAssign.html) |
| ≈ | `setPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `setPrec` | [`set_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.set_prec_round), [`set_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.set_prec), [`from_float_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_float_prec_round) |
| — | `roundScaled`, `addMagnitudes`, `subMagnitudes`, `alignShifts`, `divCores`, `normalizeCarry`, `sqrtCore`, `rsqrtCore`, `roundFromFloor`, `compareScaled`, `ofFractionRound`, `combinedPrecision`, `coreSignificand`, `zivStart`, `addRatApprox`, `addRatFuel`, `ratOpPrecision`, `Roundable`, `truncError`, `roundingPossible`, `zivLoop`, `zivGuardBits`, `zivFuel` | |

**The ≈ marks.** Every row here is ≈ for the two reasons in [Conventions](#conventions) and no
other: on inputs whose results stay inside Malachite's exponent range, the rounded values, their
precisions, and the `Ordering`s agree bit for bit, with the special values handled alike (`∞ - ∞`,
`0 · ∞`, `0 / 0`, and `√x` and `1/√x` for negative `x` are `NaN`; `x / 0` is `±∞`, `1/√0` is `+∞`,
and `1/√∞` is `0`). Where a result leaves the range, Malachite's documented rule applies and
Azurite's does not exist; where a result is zero, Malachite's may be `-0.0` (`-(0.0)`, `0.0 · -1`,
`x - x` under `Floor`) and Azurite's is `0`; and where an *input* is `-0.0`, Malachite's `x / -0.0`
(and `q / -0.0` for a rational `q`) is `∓∞` with the zero's sign, which Azurite, reading the zero as
unsigned, gives as `±∞`. The shifts are exact on both sides, Malachite's rounding the exponent into
its range under the `Nearest` rule; Azurite's count is an `AzInt`, Malachite's any primitive
integer. `setPrecRound` is `set_prec_round` (or `from_float_prec_round`, the same function returning
a new value), which can overflow when a carry at the maximum exponent occurs and never underflows.

**Mixed operations with a rational.** `addRatPrecRound`, `subRatPrecRound`, `mulRatPrecRound`, and
`divRatPrecRound` are `add_rational_prec_round` and its siblings: the exact result of the float and
the rational, rounded once, ≈ for the same reasons as the rows above. The operators and Malachite's
`_round` methods use the float's precision, which Azurite calls `ratOpPrecision`; a special float
counts as precision 1 on both sides, so `0 + 123` is `128`. `ratDivPrecRound`, the quotient `q / x`,
is `rational_div_float_prec_round`. Malachite has no method for `q - x` at an arbitrary precision,
only the `-` operator at the float's; `ratSubPrecRound q x p mode` is
`-(x.sub_rational_prec_round(q, p, -mode))` with the ordering reversed. Azurite computes a sum or
difference in a Ziv loop over brackets of the rational, with fuel proven sufficient (`zivLoop`); a
product or quotient needs no loop, because the float's exponent factors out as an exact shift.

Malachite has the many derived operations of the MPFR page (`add_mul`, `reciprocal`,
`reciprocal_sqrt_rational_prec`, `pow`, the transcendental functions, `round_to_integer`, …), none
of which `AzFloat` has yet.

## Conversion {#conversion}

| | Azurite | Malachite |
| :---: | --- | --- |
| ≈ | `ofAzNat (n : AzNat) : AzFloat`, `ofAzInt (z : AzInt) : AzFloat` | [`TryFrom<Natural>`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_natural/index.html), [`TryFrom<Integer>`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_integer/index.html), [`ExactFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ExactFrom.html) |
| ≈ | `ofAzRatRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `ofAzRat` | [`from_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_rational_prec_round), [`from_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_rational_prec) |
| ✓ | `toAzRat? : AzFloat → Option AzRat` | [`Rational::try_from`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/rational_from_float/index.html) |
| ✓ | `ofFloat64 (f : Float) : AzFloat` | [`From<f64>`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_primitive_float/index.html) |
| ≈ | `toFloat64 (x : AzFloat) (mode : RoundingMode := .Nearest) : Float` | [`RoundingFrom<Float>`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_float_from_float/index.html) for `f64` |
| ⚙ | `instance : OfNat AzFloat (n + 2)`, `instance : OfScientific AzFloat`, `literalPrecision` | [`From<u64>`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_primitive_int/index.html), [`from_sci_string`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromSciString.html) |
| — | `ofUnpacked`, `toUnpacked`, `finishUnpacked`, `maxFinite64`, `overflowToInfinity`, `azIntOfInt`, `smallToNat`, `signOfBool` | |

**From the exact types.** `ofAzNat` and `ofAzInt` convert exactly at the precision of the bit
length; `Float::try_from` also converts exactly, but at the precision of the bit length *without
the trailing zero bits*, so `1000` becomes `0x3e8.0#7` in Malachite and `0x3e8.0#10` in Azurite,
the same value at different precisions (`setPrec` moves between them exactly). It is a `TryFrom`
because a value beyond Malachite's exponent range is an error (`FloatConversionError::Overflow`);
`Float::from` for the machine integers uses the same minimal precision. The rounding
constructors `from_natural_prec_round` and `from_integer_prec_round` are `setPrecRound (ofAzNat n)
p mode`. `ofAzRatRound q p mode` is `from_rational_prec_round(q, p, mode)`, agreeing bit for bit
inside the range and leaving it only through Malachite's overflow and underflow rules.
`toAzRat?` is `Rational::try_from(&x)`, `None` and `Err` for the special values; it is exact and
can be large, since a float with exponent $$e$$ is a rational with a $$2^{|e|}$$ in it.

**Primitive floats.** `ofFloat64` is `Float::from(f64)`, exact at precision 53 (both signed zeros
become Azurite's `zero`). `toFloat64 x mode` rounds to binary64 with IEEE overflow and subnormal
rules; `f64::rounding_from(x, mode)` does the same, with one difference inside Malachite's `Nearest`
rule: a value closer to zero than to any `f64`, or tied with zero, becomes `±0.0` with the value's
sign. Lean's `Float` operations themselves are outside the mapping. Azurite's literals are exact
integers (`OfNat`) and nearest-rounded decimals at precision 53 (`OfScientific`); Malachite spells
the first as `Float::from(n)` for a machine integer or `Float::exact_from` for a `Natural` and the
second as `Float::from_sci_string`, whose precision comes from the digits.

## Strings {#strings}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `toHexString (x : AzFloat) (uppercase : Bool := false) : String`, `toHexChars`, `instance : Repr AzFloat` | [`LowerHex`](https://doc.rust-lang.org/nightly/std/fmt/trait.LowerHex.html), [`UpperHex`](https://doc.rust-lang.org/nightly/std/fmt/trait.UpperHex.html) for [`ComparableFloat`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.ComparableFloat.html) |
| ✓ | `ofHexString (s : String) : Option AzFloat`, `ofHexChars` | [`FromStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromStringBase.html) in base 16 for [`ComparableFloat`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.ComparableFloat.html) |
| ≈ | `toDecimalString (x : AzFloat) : String`, `instance : ToString AzFloat`, `toDecimalAt` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |
| ≈ | `ofDecimalStringRound (s : String) (p : Nat) (mode : RoundingMode) : Option (AzFloat × Ordering)`, `ofDecimalString` | [`FromSciString`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromSciString.html), [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) |
| ✓ | `toSci (x : AzFloat) (o : SciOptions := {}) : Option String`, `toSciNumber` | [`ToSci`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToSci.html) |
| — | `hexDigits`, `decDigits`, `hexDigitCount`, `hexLayout`, `splitHexExponent`, `shortestDecimalPrecision`, `decimalRoundTrips`, `searchLeast`, `searchLeastFromTop`, `expandUntil`, `ensurePoint`, `decimalOptions` | |

**The exact format.** `toHexString` writes Malachite's `{:#x}` rendering of a `ComparableFloat`:
the exact value in hexadecimal with the precision after `#` (`0x1.8#2`, `-0x0.0050df15a4acf314#53`,
`0x1.0E-250000000#1`, and `NaN`, `Infinity`, `-Infinity`, `0x0.0`), the one string that identifies
a float on both sides; `ofHexString` reads exactly that language back, as `ComparableFloat`'s
base-16 parser does. The one string Azurite cannot read or write is `-0x0.0`.

**Decimal.** `toDecimalString` prints the *shortest* decimal that reads back to the value at its
precision; Malachite's `Display` prints a digit count fixed by the precision,
$$1 + \lceil p \log_{10} 2 \rceil$$ significant digits, so the two strings denote the same value
but differ in length (`1.0` against `1.000000000000000000000000000000` at precision 100). The
layouts otherwise agree (a point always present, exponent notation far from zero, `0.0`, and the
special names). `toSci` with explicit
[`SciOptions`](/mapping/azurite-rationals/#strings-and-scientific-notation) is
`to_sci_with_options`, and reproduces `Display` when given its digit count. Reading decimals,
`ofDecimalStringRound s p mode` takes the precision and mode explicitly; `Float::from_str` and
`from_sci_string` infer the precision from the digits (or read a `#p` suffix) and round to
nearest, so the two agree when `p` is that inferred count.

## Not yet in Azurite {#not-yet-in-azurite}

`AzFloat` is the newest Azurite type, so the Malachite side of this page is far wider than the
Azurite side: the transcendental functions and the constants defined through them ($$\pi$$,
$$\ln 2$$, $$e$$, and the rest), `round_to_integer`, `reciprocal`, `pow`, the `ComparableFloat`
total order, the operations with a `Natural` or `Integer` operand, exhaustive and random
generation, and the rest of the [MPFR page](/mapping/mpfr-floats/) have no `AzFloat`
counterpart yet. They will be added here as
Azurite gains them.

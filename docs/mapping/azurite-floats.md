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
| ≈ | `minOf (x y : AzFloat) : AzFloat`, `maxOf` | [`min`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.min), [`max`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.max) |
| — | `compareMagnitude`, `compareSigns` | |

`minOf x y` and `maxOf x y` select one operand, like `x.min(y)` and `x.max(y)`, and are ≈ for
`NaN`: Azurite returns `x` whenever the comparison is undefined, so a `NaN` in `x` propagates and
one in `y` is ignored, where Malachite selects the other operand whenever exactly one is `NaN`.

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
| ✓ | `ln2PrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `ln2` | [`ln_2_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_2_prec_round), [`ln_2_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_2_prec) |
| ✓ | `log2EPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `log2E` | [`log_2_e_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_2_e_prec_round), [`log_2_e_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_2_e_prec) |
| ✓ | `ln10PrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `ln10` | [`ln_10_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_10_prec_round), [`ln_10_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_10_prec) |
| ✓ | `log10EPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `log10E` | [`log_10_e_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_10_e_prec_round), [`log_10_e_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_10_e_prec) |
| ✓ | `log2TenPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `log2Ten` | [`log_2_10_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_2_10_prec_round), [`log_2_10_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_2_10_prec) |
| ✓ | `log10TwoPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `log10Two` | [`log_10_2_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_10_2_prec_round), [`log_10_2_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_10_2_prec) |
| ✓ | `piPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `pi` | [`pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pi_prec_round), [`pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pi_prec) |
| ✓ | `tauPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `tau` | [`tau_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tau_prec_round), [`tau_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tau_prec) |
| ✓ | `ePrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `e` | [`e_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.e_prec_round), [`e_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.e_prec) |
| ✓ | `gaussConstantPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `gaussConstant` | [`gauss_constant_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.gauss_constant_prec_round), [`gauss_constant_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.gauss_constant_prec) |
| ✓ | `lemniscateConstantPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering` | [`lemniscate_constant_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.lemniscate_constant_prec_round), [`lemniscate_constant_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.lemniscate_constant_prec) |
| ✓ | `piDiv2PrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `piDiv2` | [`pi_over_2_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pi_over_2_prec_round), [`pi_over_2_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pi_over_2_prec) |
| ✓ | `piDiv3PrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `piDiv3` | [`pi_over_3_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pi_over_3_prec_round), [`pi_over_3_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pi_over_3_prec) |
| ✓ | `piDiv4PrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `piDiv4` | [`pi_over_4_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pi_over_4_prec_round), [`pi_over_4_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pi_over_4_prec) |
| ✓ | `piDiv6PrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `piDiv6` | [`pi_over_6_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pi_over_6_prec_round), [`pi_over_6_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pi_over_6_prec) |
| ✓ | `piDiv8PrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `piDiv8` | [`pi_over_8_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pi_over_8_prec_round), [`pi_over_8_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pi_over_8_prec) |
| ✓ | `oneDivPiPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `oneDivPi` | [`one_over_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.one_over_pi_prec_round), [`one_over_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.one_over_pi_prec) |
| ✓ | `twoDivPiPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `twoDivPi` | [`two_over_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.two_over_pi_prec_round), [`two_over_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.two_over_pi_prec) |
| ✓ | `sqrtPiPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sqrtPi` | [`sqrt_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_pi_prec_round), [`sqrt_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_pi_prec) |
| ✓ | `oneDivSqrtPiPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `oneDivSqrtPi` | [`one_over_sqrt_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.one_over_sqrt_pi_prec_round), [`one_over_sqrt_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.one_over_sqrt_pi_prec) |
| ✓ | `twoDivSqrtPiPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `twoDivSqrtPi` | [`two_over_sqrt_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.two_over_sqrt_pi_prec_round), [`two_over_sqrt_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.two_over_sqrt_pi_prec) |
| ✓ | `oneDivSqrtTauPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `oneDivSqrtTau` | [`one_over_sqrt_tau_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.one_over_sqrt_tau_prec_round), [`one_over_sqrt_tau_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.one_over_sqrt_tau_prec) |
| ✓ | `gelfondPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering` | [`gelfonds_constant_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.gelfonds_constant_prec_round), [`gelfonds_constant_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.gelfonds_constant_prec) |
| ✓ | `ramanujanPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering` | [`ramanujans_constant_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ramanujans_constant_prec_round), [`ramanujans_constant_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ramanujans_constant_prec) |
| ✓ | `gelfondSchneiderPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering` | [`gelfond_schneider_constant_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.gelfond_schneider_constant_prec_round), [`gelfond_schneider_constant_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.gelfond_schneider_constant_prec) |
| ✓ | `eulerGammaPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering` | [`eulers_constant_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.eulers_constant_prec_round), [`eulers_constant_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.eulers_constant_prec) |
| ✓ | `cbrt2PrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cbrt2` | [`cbrt_2_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cbrt_2_prec_round), [`cbrt_2_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cbrt_2_prec) |
| ✓ | `dottiePrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `dottie` | [`dottie_number_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.dottie_number_prec_round), [`dottie_number_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.dottie_number_prec) |
| ✓ | `catalanConstantPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `catalanConstant` | [`catalans_constant_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.catalans_constant_prec_round), [`catalans_constant_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.catalans_constant_prec) |
| — | `phiApprox`, `piDiv3Approx`, `gelfondApprox`, `ramanujanApprox`, `gelfondSchneiderApprox`, `eulerGammaApprox`, `oneDivPiApprox`, `sqrtPiApprox`, `oneDivSqrtPiApprox`, `oneDivSqrtTauApprox`, `ln2Approx`, `log2EApprox`, `ln10Approx`, `log10EApprox`, `log2TenApprox`, `log10TwoApprox`, `wordOfBits`, `primeConstantLimbs`, `prouhetThueMorseSeq`, `prouhetThueMorseWord`, `prouhetThueMorseWordNot`, `prouhetThueMorseLimb`, `prouhetThueMorseLimbs`, `catalanTerms`, `atanhSqrtThreeTerms`, `catalanBits`, `catalanBracket`, `catalanApprox` | |

**The constants.** Each is the constant rounded to precision `p` with `mode`, with the comparison
with the exact value, and the plain form rounds to nearest; Malachite's `_prec_round` and `_prec`
functions return the same pairs, bit for bit. The constants lie well inside Malachite's exponent
range and are never zero, so neither of the ≈ reasons in [Conventions](#conventions) arises.
$$\sqrt2$$, $$\sqrt3$$, $$\sqrt5$$, their reciprocals $$\sqrt2/2$$, $$\sqrt3/3$$, and $$\sqrt5/5$$,
and the golden ratio $$\varphi$$ are irrational, so no rounding is exact; the prime constant (bit
$$k$$ of the binary expansion is 1 iff $$k$$ is prime) and the Prouhet–Thue–Morse constant (bit
$$k$$ is the parity of the number of 1s in $$k$$) are also irrational, and Azurite builds their
expansions a limb at a time. The logarithmic constants $$\ln 2$$, $$\log_2 e = 1/\ln 2$$,
$$\ln 10$$, $$\log_{10} e = 1/\ln 10$$, $$\log_2 10$$, and $$\log_{10} 2$$ are irrational too;
Azurite computes each in a Ziv loop over certified brackets, built from the arithmetic-geometric
mean, and its proofs guarantee that the loop terminates, so it needs no iteration limit. $$\pi$$
is computed the same way, $$\tau = 2\pi$$, $$\pi/2$$, $$\pi/4$$, and $$\pi/8$$ are exact shifts of
it, $$\pi/3$$ has its own certified loop, $$\pi/6$$ is an exact shift of that; so do $$1/\pi$$,
$$\sqrt\pi$$, $$1/\sqrt\pi$$, and $$1/\sqrt{2\pi}$$, with $$2/\pi$$ and $$2/\sqrt\pi$$ exact shifts of
the reciprocals; and $$e$$ is Azurite's
correctly rounded `exp` at 1. Gauss's constant $$G = 1/\operatorname{agm}(1, \sqrt2)$$, the
lemniscate constant $$\varpi = \pi G$$, Gelfond's constant $$e^\pi$$, Ramanujan's constant
$$e^{\pi\sqrt{163}}$$, the Gelfond–Schneider constant $$2^{\sqrt2}$$, and Euler's constant
$$\gamma$$ use a Ziv loop with an iteration limit instead, and are
proven to return either the correctly rounded value or `NaN` if that limit is reached; Malachite
always returns a value, so a `NaN` would show up as a disagreement. Catalan's constant
$$G = \sum_{j \geq 0} (-1)^j/(2j+1)^2$$ needs no such limit: Azurite sums two rational series by
binary splitting, and since $$G$$ is proven irrational, its Ziv loop is proven to terminate with
the correctly rounded value. Malachite has many more constants; see the
[MPFR page](/mapping/mpfr-floats/).

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
| ✓ | `sqrtRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering` | [`sqrt_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_rational_prec_round), [`sqrt_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_rational_prec), [`sqrt_unsigned_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_unsigned_prec_round), [`sqrt_unsigned_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_unsigned_prec) |
| ✓ | `rsqrtRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering` | [`reciprocal_sqrt_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.reciprocal_sqrt_rational_prec_round), [`reciprocal_sqrt_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.reciprocal_sqrt_rational_prec) |
| ≈ | `cbrtPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cbrt` | [`cbrt_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cbrt_prec_round), [`cbrt_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cbrt_prec), [`cbrt_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cbrt_round), [`Cbrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Cbrt.html), [`CbrtAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CbrtAssign.html) |
| ✓ | `cbrtRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cbrtRat` | [`cbrt_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cbrt_rational_prec_round), [`cbrt_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cbrt_rational_prec) |
| ≈ | `powIntPrecRound (x : AzFloat) (n : AzInt) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `powInt` | [`pow_integer_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_integer_prec_round), [`pow_integer_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_integer_prec), [`pow_integer_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_integer_round), [`pow_s_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_s_prec_round), [`pow_s_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_s_prec), [`pow_s_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_s_round), [`pow_u_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_u_prec_round), [`pow_u_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_u_prec), [`pow_u_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_u_round), [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html), [`PowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowAssign.html) |
| ≈ | `powPrecRound (x y : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `pow` | [`pow_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_prec_round), [`pow_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_prec), [`pow_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_round), [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html), [`PowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowAssign.html) |
| ≈ | `powRatPrecRound (x : AzFloat) (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `powRat` | [`pow_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_rational_prec_round), [`pow_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_rational_prec), [`pow_rational_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_rational_round) |
| ✓ | `ratPowPrecRound (q : AzRat) (y : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `ratPowFloat` | [`rational_pow_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.rational_pow_prec_round), [`rational_pow_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.rational_pow_prec) |
| ✓ | `ratPowRatPrecRound (q r : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `ratPowRat` | [`rational_pow_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.rational_pow_rational_prec_round), [`rational_pow_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.rational_pow_rational_prec) |
| ≈ | `recipPrecRound (y : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `recip` | [`reciprocal_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.reciprocal_prec_round), [`reciprocal_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.reciprocal_prec), [`reciprocal_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.reciprocal_round), [`Reciprocal`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Reciprocal.html) |
| ≈ | `sumPrecRound (xs : List AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering` | [`sum_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sum_prec_round), [`sum_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sum_prec), [`sum_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sum_round), [`Sum`](https://doc.rust-lang.org/nightly/std/iter/trait.Sum.html) |
| ≈ | `productPrecRound (xs : List AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering` | [`product_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.product_prec_round), [`product_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.product_prec), [`product_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.product_round), [`Product`](https://doc.rust-lang.org/nightly/std/iter/trait.Product.html) |
| ≈ | `sumFloat64 (xs : List Float) : Float` | [`primitive_float_sum`](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sum/fn.primitive_float_sum.html) |
| ≈ | `productFloat64 (xs : List Float) : Float` | [`primitive_float_product`](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/product/fn.primitive_float_product.html) |
| ≈ | `addMulPrecRound (x y z : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `addMul` | [`add_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.add_mul_prec_round), [`add_mul_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.add_mul_prec), [`add_mul_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.add_mul_round), [`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html) |
| ≈ | `subMulPrecRound (x y z : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `subMul` | [`sub_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sub_mul_prec_round), [`sub_mul_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sub_mul_prec), [`sub_mul_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sub_mul_round), [`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html) |
| ≈ | `addMulFloat64 (x y z : Float) : Float` | [`primitive_float_add_mul`](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/add_mul/fn.primitive_float_add_mul.html) |
| ≈ | `subMulFloat64 (x y z : Float) : Float` | [`primitive_float_sub_mul`](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sub_mul/fn.primitive_float_sub_mul.html) |
| ≈ | `addMulRatPrecRound (x y : AzFloat) (z : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `addMulRat` | [`add_mul_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.add_mul_rational_prec_round), [`add_mul_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.add_mul_rational_prec), [`add_mul_rational_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.add_mul_rational_round), [`AddMul<Float, Rational>`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html) |
| ≈ | `subMulRatPrecRound (x y : AzFloat) (z : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `subMulRat` | [`sub_mul_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sub_mul_rational_prec_round), [`sub_mul_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sub_mul_rational_prec), [`sub_mul_rational_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sub_mul_rational_round), [`SubMul<Float, Rational>`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html) |
| ≈ | `addMulRatFloat64 (x y : Float) (z : AzRat) : Float` | [`primitive_float_add_mul_rational`](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/add_mul/fn.primitive_float_add_mul_rational.html) |
| ≈ | `subMulRatFloat64 (x y : Float) (z : AzRat) : Float` | [`primitive_float_sub_mul_rational`](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sub_mul/fn.primitive_float_sub_mul_rational.html) |
| — | `fusedMulRat` | |
| ≈ | `mulAddMulPrecRound (x y z w : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `mulAddMul` | [`mul_add_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.mul_add_mul_prec_round), [`mul_add_mul_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.mul_add_mul_prec), [`mul_add_mul_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.mul_add_mul_round), [`MulAddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulAddMul.html) |
| ≈ | `mulSubMulPrecRound (x y z w : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `mulSubMul` | [`mul_sub_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.mul_sub_mul_prec_round), [`mul_sub_mul_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.mul_sub_mul_prec), [`mul_sub_mul_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.mul_sub_mul_round), [`MulSubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulSubMul.html) |
| ≈ | `mulAddMulFloat64 (x y z w : Float) : Float` | [`primitive_float_mul_add_mul`](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul_add_mul/fn.primitive_float_mul_add_mul.html) |
| ≈ | `mulSubMulFloat64 (x y z w : Float) : Float` | [`primitive_float_mul_sub_mul`](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul_sub_mul/fn.primitive_float_mul_sub_mul.html) |
| — | `fusedPrecision₄` | |
| ≈ | `mulAddMulRatPrecRound (x y z : AzFloat) (w : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `mulAddMulRat` | [`mul_add_mul_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.mul_add_mul_rational_prec_round), [`mul_add_mul_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.mul_add_mul_rational_prec), [`mul_add_mul_rational_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.mul_add_mul_rational_round), [`MulAddMul<Float, Float, Rational>`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulAddMul.html) |
| ≈ | `mulSubMulRatPrecRound (x y z : AzFloat) (w : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `mulSubMulRat` | [`mul_sub_mul_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.mul_sub_mul_rational_prec_round), [`mul_sub_mul_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.mul_sub_mul_rational_prec), [`mul_sub_mul_rational_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.mul_sub_mul_rational_round), [`MulSubMul<Float, Float, Rational>`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulSubMul.html) |
| ≈ | `mulAddMulRatFloat64 (x y z : Float) (w : AzRat) : Float` | [`primitive_float_mul_add_mul_rational`](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul_add_mul/fn.primitive_float_mul_add_mul_rational.html) |
| ≈ | `mulSubMulRatFloat64 (x y z : Float) (w : AzRat) : Float` | [`primitive_float_mul_sub_mul_rational`](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul_sub_mul/fn.primitive_float_mul_sub_mul_rational.html) |
| — | `exactProductPrec`, `exactMul`, `fusedPrecision` | |
| ≈ | `remPrecRound (x y : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering` | [`rem_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.rem_prec_round), [`rem_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.rem_prec), [`rem_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.rem_round), [`Rem`](https://doc.rust-lang.org/nightly/std/ops/trait.Rem.html), [`RemAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.RemAssign.html) |
| — | `remExact`, `AzNat.shiftLeftMod` | |
| ≈ | `roundToIntegerPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering × Bool`, `roundToIntegerPrec`, `roundToIntegerRound`, `roundToInteger` | [`round_to_integer_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.round_to_integer_prec_round), [`round_to_integer_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.round_to_integer_prec), [`round_to_integer_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.round_to_integer_round), [`round_to_integer`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.round_to_integer) |
| ≈ | `roundToIntegerThenPrecRound (x : AzFloat) (irm : RoundingMode) (p : Nat) (rm : RoundingMode) : AzFloat × Ordering`, `roundToIntegerThenPrec` | [`round_to_integer_then_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.round_to_integer_then_prec_round) |
| ✓ | `isInteger : AzFloat → Bool` | [`IsInteger`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.IsInteger.html) |
| ≈ | `neg : AzFloat → AzFloat`, `instance : Neg AzFloat` | [`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html), [`NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegAssign.html) |
| ≈ | `abs : AzFloat → AzFloat` | [`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html), [`AbsAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsAssign.html) |
| ≈ | `shiftLeft (x : AzFloat) (k : AzInt)`, `shiftRight`, `instance : HShiftLeft AzFloat AzInt AzFloat`, `HShiftRight` | [`Shl`](https://doc.rust-lang.org/nightly/std/ops/trait.Shl.html), [`Shr`](https://doc.rust-lang.org/nightly/std/ops/trait.Shr.html), [`ShlAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.ShlAssign.html), [`ShrAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.ShrAssign.html) |
| ≈ | `setPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `setPrec` | [`set_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.set_prec_round), [`set_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.set_prec), [`from_float_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_float_prec_round) |
| — | `exactAdd`, `exactSub`, `exactSum`, `exactSumPrec`, `sumFinite`, `signSum`, `splitWindow`, `tailBound`, `maxExponent`, `lengthBits`, `coreParts`, `productSign` | |
| — | `roundScaled`, `addMagnitudes`, `subMagnitudes`, `alignShifts`, `divCores`, `normalizeCarry`, `sqrtCore`, `rsqrtCore`, `sqrtRatCore`, `roundFromFloor`, `compareScaled`, `ofFractionRound`, `combinedPrecision`, `coreSignificand`, `zivStart`, `addRatApprox`, `addRatFuel`, `ratOpPrecision`, `Roundable`, `truncError`, `roundingPossible`, `zivLoop`, `zivGuardBits`, `zivFuel` | |

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

**Sums and products.** `sumPrecRound xs p mode` and `productPrecRound xs p mode` are
`Float::sum_prec_round(&xs, p, mode)` and `Float::product_prec_round(&xs, p, mode)`: the exact sum
or product of the list, rounded once, ≈ for the same reasons as the rows above. `sum_prec` and
`product_prec` round to nearest; `sum_round`, `product_round`, and the `Sum` and `Product`
implementations use the largest precision among the inputs, a special value counting as 1, so the
empty product is `1` at precision 1 and the empty sum is `0`. The special values follow the
two-operand rules: any `NaN`, opposite infinities in a sum, or a zero times an infinity in a
product give `NaN`. Azurite never forms an enormous exact sum: it sums a window of about `p` bits
below the largest term exactly and bounds the rest, restarting when the window cancels. A product
of finite floats it forms exactly, as the product of the significands times a power of 2.

**Reciprocals.** `recipPrecRound y p mode` is `y.reciprocal_prec_round(p, mode)`, and `recip y`,
at `y`'s precision, is `y.reciprocal()`; Azurite computes it as the division of 1, ≈ for the
reasons of the conventions, among them that Malachite's `1/(-0.0)` is $$-\infty$$.

**Correctly rounded machine-float sums and products.** `sumFloat64 xs` and `productFloat64 xs` are
`primitive_float_sum(&xs)` and `primitive_float_product(&xs)` for `f64`s: the exact sum or product
of the list rounded once to binary64, nearest with ties to even, with IEEE 754's subnormals,
overflow to $$\pm\infty$$, and `NaN`. Azurite reaches binary64 through a 53-bit `AzFloat` result
and its `Ordering`, which `shiftRightNearestTernary` uses to round a subnormal result a second time
without double-rounding error. They are ≈ because Azurite returns `+0.0` for an exact zero, where
Malachite gives a sum of zeros that all have the same sign that sign, and a zero product the sign
of the product of the signs. Malachite also has the `f32` versions, and `primitive_float_*` forms of
most other operations, which Azurite does not.

**Fused multiply-add and multiply-subtract.** `addMulPrecRound x y z p mode` is
`x.add_mul_prec_round(y, z, p, mode)`, $$x + yz$$ rounded once, and `subMulPrecRound` is
`sub_mul_prec_round`, $$x - yz$$; `addMul` and `subMul` round to nearest at the largest of the
three precisions, as `x.add_mul(y, z)` and `x.sub_mul(y, z)` do. Azurite forms the product exactly
(`exactMul`, at the sum of the two precisions) and rounds the sum or difference once; they are ≈
for the reasons of the conventions. `addMulFloat64` and `subMulFloat64` are
`primitive_float_add_mul` and `primitive_float_sub_mul` for `f64`s, IEEE 754's correctly rounded
`fma` and its subtracting form, ≈ only for the sign of an exact zero: Azurite gives `+0.0`, where
Malachite follows IEEE 754 (a zero product has the sign of the product of the signs, `-0.0 + -0.0`
is `-0.0`, and other exact zeros are `+0.0`). With a `Rational` third operand, `addMulRatPrecRound x y z p mode` is
`x.add_mul_rational_prec_round(y, z, p, mode)`, and `addMulRat x y z`, at the larger of the two
floats' precisions, is `x.add_mul(y, z)` with a `Rational` `z`; likewise for `subMulRat`. Azurite
forms $$yz$$ exactly as a rational and rounds the float–rational sum once; an infinite $$y$$ times a
zero rational is `NaN`. `addMulRatFloat64` and `subMulRatFloat64` are
`primitive_float_add_mul_rational` and `primitive_float_sub_mul_rational` for `f64`s, ≈ for the sign
of an exact zero as above, a zero rational counting as positive in the sign of the product.

**Sums and differences of two products.** `mulAddMulPrecRound x y z w p mode` is
`x.mul_add_mul_prec_round(y, z, w, p, mode)`, $$xy + zw$$ rounded once, and `mulSubMulPrecRound`
is `mul_sub_mul_prec_round`, $$xy - zw$$; `mulAddMul` and `mulSubMul` round to nearest at the
largest of the four precisions, as `x.mul_add_mul(y, z, w)` and `x.mul_sub_mul(y, z, w)` do. Both
products are exact and only the sum or difference is rounded, ≈ for the reasons of the conventions.
`mulAddMulFloat64` and `mulSubMulFloat64` are `primitive_float_mul_add_mul` and
`primitive_float_mul_sub_mul` for `f64`s, ≈ only for the sign of an exact zero: when both
products are zero, Malachite follows IEEE 754 and gives `-0.0` exactly when both signed products
(the second negated for a difference) are `-0.0`, where Azurite gives `+0.0`. With a `Rational`
fourth operand, `mulAddMulRatPrecRound x y z w p mode` is
`x.mul_add_mul_rational_prec_round(y, z, w, p, mode)`, and `mulAddMulRat`, at the largest of the
three floats' precisions, is `x.mul_add_mul(y, z, w)` with a `Rational` `w`; likewise for
`mulSubMulRat`. Azurite forms $$xy$$ exactly and adds $$zw$$ with the fused float–rational
operation, rounding once. `mulAddMulRatFloat64` and `mulSubMulRatFloat64` are
`primitive_float_mul_add_mul_rational` and `primitive_float_mul_sub_mul_rational` for `f64`s, ≈ for
the sign of an exact zero, a zero rational counting as positive in the sign of its product.

`remPrecRound x y p mode` is `x.rem_prec_round(y, p, mode)`, the truncated remainder, whose sign
is that of `x`; `x % y` rounds at the larger of the two precisions. `roundToIntegerPrecRound x p mode`
is `x.round_to_integer_prec_round(p, mode)`, returning the rounded integer, its comparison with `x`,
and whether `x` was already an integer; `roundToIntegerPrec`, `roundToIntegerRound`, and
`roundToInteger` default to `x`'s precision and to `Nearest`, but return only the value, where
Malachite's forms return all three components. `roundToIntegerThenPrecRound x irm p rm` is
`x.round_to_integer_then_prec_round(irm, p, rm)`: the integer rounding in `irm`, then the rounding
to `p` bits in `rm`; Malachite has no `round_to_integer_then_prec`. All are ≈ for the sign of a
zero result, which Azurite does not track. `isInteger` is `x.is_integer()`: zeros of either sign are
integers, and `NaN` and the infinities are not.

`sqrtRatPrecRound q p mode` is `Float::sqrt_rational_prec_round(q, p, mode)` and
`rsqrtRatPrecRound q p mode` is `Float::reciprocal_sqrt_rational_prec_round(q, p, mode)`: a
negative `q` gives `NaN`, and a zero `q` gives `+0.0` and `+∞` respectively, as in Malachite.

`cbrtPrecRound x p mode` is `x.cbrt_prec_round(p, mode)`, ≈ for the sign of a zero result, and
`cbrtRatPrecRound q p mode` is `Float::cbrt_rational_prec_round(q, p, mode)`; a negative argument
has a negative cube root on both sides. `powIntPrecRound x n p mode` is
`x.pow_integer_prec_round(n, p, mode)`, and, for an `n` that fits, `x.pow_s_prec_round(n, p, mode)`
or `x.pow_u_prec_round(n, p, mode)`; `powInt x n` is `x.pow(n)`. It is ≈ for three reasons: the
sign of a zero result; $$(-0)^n$$ for a negative odd $$n$$, which is $$-\infty$$ in Malachite and
$$+\infty$$ in Azurite, with no signed zero; and $$\mathrm{NaN}^0$$, which is $$1$$ in Malachite,
following MPFR, and `NaN` in Azurite. Where $$n \log_2 |x|$$, bracketed by directed roundings, is
at least $$2^{30}$$ or at most $$-2^{31}$$, the oracle compares Malachite's result with Azurite's
at $$(\pm 2^{2^{30}})^1$$ or $$(\pm 2^{-2^{31}})^1$$, which Malachite rounds identically, rather
than have Azurite form an exact power with up to $$2^{63}$$ times the bits of the significand.

`powPrecRound x y p mode` is `x.pow_prec_round(y, p, mode)`, `powRatPrecRound x q p mode` is
`x.pow_rational_prec_round(q, p, mode)`, `ratPowPrecRound q y p mode` is
`Float::rational_pow_prec_round(q, y, p, mode)`, and `ratPowRatPrecRound q r p mode` is
`Float::rational_pow_rational_prec_round(q, r, p, mode)`. All four follow MPFR's special values,
among them $$x^0 = 1$$ and $$1^y = 1$$ for every $$x$$ and $$y$$, `NaN` included. `pow` and
`powRat` are ≈ for the sign of a zero result and for $$(-0)^y$$ with $$y$$ a negative odd integer,
which is $$-\infty$$ in Malachite and $$+\infty$$ in Azurite; `pow x y` rounds to the precision of
`x`, where `x.pow(y)` rounds to the larger of the two precisions. As for `powInt`, a power whose
$$y \log_2 |x|$$ is certainly at least $$2^{30}$$ or at most $$-2^{31}$$ is compared at
$$(\pm 2^{2^{30}})^1$$ or $$(\pm 2^{-2^{31}})^1$$, for every base and exponent type.

Malachite has more derived operations of the MPFR page than `AzFloat` does; the exponentials,
logarithms, and trigonometric functions that `AzFloat` has are mapped in the sections that follow,
and the rest are listed under [Not yet in Azurite](#not-yet-in-azurite).

## Exponentials and logarithms {#exponentials-and-logarithms}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `expPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `exp` | [`exp_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.exp_prec_round), [`exp_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.exp_prec), [`exp_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.exp_round), [`Exp`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.Exp.html), [`ExpAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.ExpAssign.html) |
| ✓ | `expRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `expRat` | [`exp_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.exp_rational_prec_round), [`exp_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.exp_rational_prec) |
| ≈ | `expM1PrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `expM1` | [`exp_x_minus_1_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.exp_x_minus_1_prec_round), [`exp_x_minus_1_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.exp_x_minus_1_prec), [`exp_x_minus_1_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.exp_x_minus_1_round), [`ExpXMinus1`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.ExpXMinus1.html), [`ExpXMinus1Assign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.ExpXMinus1Assign.html) |
| ✓ | `expM1RatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `expM1Rat` | [`exp_x_minus_1_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.exp_x_minus_1_rational_prec_round), [`exp_x_minus_1_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.exp_x_minus_1_rational_prec) |
| ✓ | `pow2PrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `pow2` | [`power_of_2_of_float_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_2_of_float_prec_round), [`power_of_2_of_float_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_2_of_float_prec), [`power_of_2_of_float_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_2_of_float_round), [`PowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowerOf2.html), [`PowerOf2Assign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowerOf2Assign.html) |
| ✓ | `pow2RatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `pow2Rat` | [`power_of_2_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_2_rational_prec_round), [`power_of_2_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_2_rational_prec) |
| ≈ | `pow2M1PrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `pow2M1` | [`power_of_2_x_minus_1_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_2_x_minus_1_prec_round), [`power_of_2_x_minus_1_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_2_x_minus_1_prec), [`power_of_2_x_minus_1_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_2_x_minus_1_round), [`PowerOf2XMinus1`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.PowerOf2XMinus1.html), [`PowerOf2XMinus1Assign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.PowerOf2XMinus1Assign.html) |
| ✓ | `pow2M1RatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `pow2M1Rat` | [`power_of_2_x_minus_1_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_2_x_minus_1_rational_prec_round), [`power_of_2_x_minus_1_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_2_x_minus_1_rational_prec) |
| ✓ | `powBasePrecRound (u : UInt64) (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `powBase` | [`unsigned_pow_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.unsigned_pow_prec_round), [`unsigned_pow_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.unsigned_pow_prec) |
| ✓ | `powBaseRatPrecRound (u : UInt64) (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `powBaseRat` | [`unsigned_pow_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.unsigned_pow_rational_prec_round), [`unsigned_pow_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.unsigned_pow_rational_prec), [`unsigned_pow_unsigned_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.unsigned_pow_unsigned_prec_round), [`unsigned_pow_unsigned_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.unsigned_pow_unsigned_prec) |
| ≈ | `powBaseM1PrecRound (u : UInt64) (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `powBaseM1` | [`power_of_10_x_minus_1_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_10_x_minus_1_prec_round), [`power_of_10_x_minus_1_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_10_x_minus_1_prec), [`power_of_10_x_minus_1_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_10_x_minus_1_round), [`PowerOf10XMinus1`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.PowerOf10XMinus1.html), [`PowerOf10XMinus1Assign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.PowerOf10XMinus1Assign.html) |
| ≈ | `powBaseM1RatPrecRound (u : UInt64) (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `powBaseM1Rat` | [`power_of_10_x_minus_1_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_10_x_minus_1_rational_prec_round), [`power_of_10_x_minus_1_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_10_x_minus_1_rational_prec) |
| ✓ | `pow10PrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `pow10` | [`power_of_10_of_float_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_10_of_float_prec_round), [`power_of_10_of_float_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_10_of_float_prec), [`power_of_10_of_float_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_10_of_float_round), [`PowerOf10`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.PowerOf10.html), [`PowerOf10Assign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.PowerOf10Assign.html) |
| ✓ | `pow10RatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `pow10Rat` | [`power_of_10_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_10_rational_prec_round), [`power_of_10_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_10_rational_prec) |
| ≈ | `pow10M1PrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `pow10M1` | [`power_of_10_x_minus_1_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_10_x_minus_1_prec_round), [`power_of_10_x_minus_1_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_10_x_minus_1_prec), [`power_of_10_x_minus_1_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_10_x_minus_1_round), [`PowerOf10XMinus1`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.PowerOf10XMinus1.html), [`PowerOf10XMinus1Assign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.PowerOf10XMinus1Assign.html) |
| ✓ | `pow10M1RatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `pow10M1Rat` | [`power_of_10_x_minus_1_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_10_x_minus_1_rational_prec_round), [`power_of_10_x_minus_1_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_10_x_minus_1_rational_prec) |
| ✓ | `lnPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `ln` | [`ln_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_prec_round), [`ln_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_prec), [`ln_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_round), [`Ln`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.Ln.html), [`LnAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LnAssign.html) |
| ✓ | `lnRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `lnRat` | [`ln_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_rational_prec_round), [`ln_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_rational_prec), [`ln_unsigned_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_unsigned_prec_round), [`ln_unsigned_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_unsigned_prec) |
| ≈ | `lnP1PrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `lnP1` | [`ln_1_plus_x_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_1_plus_x_prec_round), [`ln_1_plus_x_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_1_plus_x_prec), [`ln_1_plus_x_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_1_plus_x_round), [`Ln1PlusX`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.Ln1PlusX.html), [`Ln1PlusXAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.Ln1PlusXAssign.html) |
| ✓ | `log2PrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `log2` | [`log_base_2_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_2_prec_round), [`log_base_2_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_2_prec), [`log_base_2_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_2_round), [`LogBase2`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBase2.html), [`LogBase2Assign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBase2Assign.html) |
| ✓ | `log2RatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `log2Rat` | [`log_base_2_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_2_rational_prec_round), [`log_base_2_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_2_rational_prec) |
| ≈ | `log2P1PrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `log2P1` | [`log_base_2_1_plus_x_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_2_1_plus_x_prec_round), [`log_base_2_1_plus_x_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_2_1_plus_x_prec), [`log_base_2_1_plus_x_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_2_1_plus_x_round), [`LogBase2Of1PlusX`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBase2Of1PlusX.html), [`LogBase2Of1PlusXAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBase2Of1PlusXAssign.html) |
| ✓ | `log10PrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `log10` | [`log_base_10_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_10_prec_round), [`log_base_10_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_10_prec), [`log_base_10_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_10_round), [`LogBase10`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBase10.html), [`LogBase10Assign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBase10Assign.html) |
| ✓ | `log10RatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `log10Rat` | [`log_base_10_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_10_rational_prec_round), [`log_base_10_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_10_rational_prec) |
| ≈ | `log10P1PrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `log10P1` | [`log_base_10_1_plus_x_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_10_1_plus_x_prec_round), [`log_base_10_1_plus_x_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_10_1_plus_x_prec), [`log_base_10_1_plus_x_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_10_1_plus_x_round), [`LogBase10Of1PlusX`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBase10Of1PlusX.html), [`LogBase10Of1PlusXAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBase10Of1PlusXAssign.html) |
| ✓ | `logBasePow2PrecRound (x : AzFloat) (k : AzInt) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `logBasePow2` | [`log_base_power_of_2_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_power_of_2_prec_round), [`log_base_power_of_2_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_power_of_2_prec), [`log_base_power_of_2_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_power_of_2_round), [`LogBasePowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBasePowerOf2.html), [`LogBasePowerOf2Assign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBasePowerOf2Assign.html) |
| ✓ | `logBasePow2RatPrecRound (q : AzRat) (k : AzInt) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `logBasePow2Rat` | [`log_base_power_of_2_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_power_of_2_rational_prec_round), [`log_base_power_of_2_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_power_of_2_rational_prec) |
| ≈ | `logBasePow2P1PrecRound (x : AzFloat) (k : AzInt) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `logBasePow2P1` | [`log_base_power_of_2_1_plus_x_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_power_of_2_1_plus_x_prec_round), [`log_base_power_of_2_1_plus_x_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_power_of_2_1_plus_x_prec), [`log_base_power_of_2_1_plus_x_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_power_of_2_1_plus_x_round), [`LogBasePowerOf2Of1PlusX`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBasePowerOf2Of1PlusX.html), [`LogBasePowerOf2Of1PlusXAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBasePowerOf2Of1PlusXAssign.html) |
| ✓ | `logBasePrecRound (x : AzFloat) (b : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `logBase` | [`log_base_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_prec_round), [`log_base_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_prec), [`log_base_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_round), [`LogBase`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBase.html), [`LogBaseAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBaseAssign.html) |
| ✓ | `logBaseRatPrecRound (q : AzRat) (b : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `logBaseRat` | [`log_base_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_rational_prec_round), [`log_base_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_rational_prec) |
| ≈ | `logBaseP1PrecRound (x : AzFloat) (b : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `logBaseP1` | [`log_base_1_plus_x_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_1_plus_x_prec_round), [`log_base_1_plus_x_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_1_plus_x_prec), [`log_base_1_plus_x_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_1_plus_x_round), [`LogBaseOf1PlusX`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBaseOf1PlusX.html), [`LogBaseOf1PlusXAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBaseOf1PlusXAssign.html) |
| ≈ | `logBaseFloatPrecRound (x b : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `logBaseFloat` | [`log_base_float_base_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_float_base_prec_round), [`log_base_float_base_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_float_base_prec), [`log_base_float_base_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_float_base_round), [`LogBase`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBase.html), [`LogBaseAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBaseAssign.html) |
| ≈ | `logBaseFloatP1PrecRound (x b : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `logBaseFloatP1` | [`log_base_float_base_1_plus_x_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_float_base_1_plus_x_prec_round), [`log_base_float_base_1_plus_x_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_float_base_1_plus_x_prec), [`log_base_float_base_1_plus_x_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_float_base_1_plus_x_round), [`LogBaseOf1PlusX`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBaseOf1PlusX.html), [`LogBaseOf1PlusXAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBaseOf1PlusXAssign.html) |
| ≈ | `logBaseRatFloatPrecRound (q : AzRat) (b : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `logBaseRatFloat` | [`log_base_rational_float_base_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_rational_float_base_prec_round), [`log_base_rational_float_base_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_rational_float_base_prec) |
| ✓ | `logBaseRatBasePrecRound (x : AzFloat) (b : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `logBaseRatBase` | [`log_base_rational_base_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_rational_base_prec_round), [`log_base_rational_base_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_rational_base_prec), [`log_base_rational_base_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_rational_base_round), [`LogBase`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBase.html), [`LogBaseAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBaseAssign.html) |
| ≈ | `logBaseRatBaseP1PrecRound (x : AzFloat) (b : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `logBaseRatBaseP1` | [`log_base_rational_base_1_plus_x_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_rational_base_1_plus_x_prec_round), [`log_base_rational_base_1_plus_x_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_rational_base_1_plus_x_prec), [`log_base_rational_base_1_plus_x_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_rational_base_1_plus_x_round), [`LogBaseOf1PlusX`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBaseOf1PlusX.html), [`LogBaseOf1PlusXAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LogBaseOf1PlusXAssign.html) |
| ✓ | `logBaseRatRatPrecRound (q b : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `logBaseRatRat` | [`log_base_rational_rational_base_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_rational_rational_base_prec_round), [`log_base_rational_rational_base_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_rational_rational_base_prec) |
| ≈ | `lambertW0PrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `lambertW0Float` | [`lambert_w0_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.lambert_w0_prec_round), [`lambert_w0_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.lambert_w0_prec), [`lambert_w0_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.lambert_w0_round), [`LambertW0`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LambertW0.html), [`LambertW0Assign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LambertW0Assign.html) |
| ✓ | `lambertWm1PrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `lambertWm1Float` | [`lambert_w_minus_1_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.lambert_w_minus_1_prec_round), [`lambert_w_minus_1_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.lambert_w_minus_1_prec), [`lambert_w_minus_1_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.lambert_w_minus_1_round), [`LambertWMinus1`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LambertWMinus1.html), [`LambertWMinus1Assign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.LambertWMinus1Assign.html) |
| ✓ | `lambertW0RatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `lambertW0Rat` | [`lambert_w0_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.lambert_w0_rational_prec_round), [`lambert_w0_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.lambert_w0_rational_prec) |
| ✓ | `lambertWm1RatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `lambertWm1Rat` | [`lambert_w_minus_1_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.lambert_w_minus_1_rational_prec_round), [`lambert_w_minus_1_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.lambert_w_minus_1_rational_prec) |
| — | `lambertW0Pos`, `lambertW0Neg`, `lambertWm1Neg`, `floatAt`, `ratAt`, `findSome`, `mulExpBracket`, `sideAt`, `sideOf`, `qBounds`, `qAt`, `qEncl`, `compBits`, `qTight`, `lnBounds`, `aprioriW0Pos`, `aprioriW0Neg`, `aprioriWm1`, `mvtDelta`, `derivLowerExp`, `mvtRound`, `mvtRounds`, `gridBisectStep`, `bisectGrid`, `gridOf`, `gridFloats`, `lambertW0Tiny`, `lambertHalleyStep`, `halleyRefine`, `halleyFloat`, `seedFloat`, `logSeedStep`, `logSeed`, `seriesSeed`, `clampInto`, `halleyCandidate`, `lambertWork`, `lambertTarget`, `lambertBracket`, `lambertApprox` | |
| ≈ | `treeT0PrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `treeT0Float` | [`tree_t0_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tree_t0_prec_round), [`tree_t0_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tree_t0_prec), [`tree_t0_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tree_t0_round), [`TreeT0`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.TreeT0.html), [`TreeT0Assign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.TreeT0Assign.html) |
| ✓ | `treeTm1PrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `treeTm1Float` | [`tree_t_minus_1_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tree_t_minus_1_prec_round), [`tree_t_minus_1_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tree_t_minus_1_prec), [`tree_t_minus_1_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tree_t_minus_1_round), [`TreeTMinus1`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.TreeTMinus1.html), [`TreeTMinus1Assign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.TreeTMinus1Assign.html) |
| ✓ | `treeT0RatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `treeT0Rat` | [`tree_t0_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tree_t0_rational_prec_round), [`tree_t0_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tree_t0_rational_prec) |
| ✓ | `treeTm1RatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `treeTm1Rat` | [`tree_t_minus_1_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tree_t_minus_1_rational_prec_round), [`tree_t_minus_1_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tree_t_minus_1_rational_prec) |
| ✓ | `wrightOmegaPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `wrightOmegaFloat` | [`wright_omega_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.wright_omega_prec_round), [`wright_omega_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.wright_omega_prec), [`wright_omega_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.wright_omega_round), [`WrightOmega`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.WrightOmega.html), [`WrightOmegaAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/exponential/traits/trait.WrightOmegaAssign.html) |
| ✓ | `wrightOmegaRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `wrightOmegaRat` | [`wright_omega_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.wright_omega_rational_prec_round), [`wright_omega_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.wright_omega_rational_prec) |
| — | `wrightOmegaLoop`, `expAt`, `expRatAt`, `isOne`, `omegaFloat`, `omegaSeed`, `omegaRefine`, `omegaCandidate`, `omegaEncl`, `omegaWork`, `wrightBracket`, `wrightApprox`, `wrightOmegaHuge?` | |
| — | `LnConsts`, `LnConsts.make`, `LnConsts.forPrecision`, `LnConsts.ln2At`, `lnPrecRoundWith`, `lnWith`, `lnRatPrecRoundWith`, `lnRatWith`, `lnP1PrecRoundWith`, `lnP1With` | |
| — | `lnRatPrecRoundCore`, `lnP1PrecRoundCore`, `agmIterN`, `lnH`, `lnSteps`, `lnAgmDelta`, `lnAgmBracket`, `lnSeriesK`, `lnSeriesTerms`, `lnSqrtIter`, `lnSeriesLoop`, `lnSeriesLead`, `lnSeriesBracketK`, `lnSeriesBracket`, `lnSeriesThreshold`, `lnBracketCore`, `lnBracket`, `lnApprox`, `lnBracketWith`, `lnApproxWith`, `lnFloorBracketBy`, `lnFloorBracket`, `lnFloorApprox`, `lnRatApprox`, `lnP1Approx`, `lnFloorBracketWith`, `lnFloorApproxWith`, `lnRatApproxWith`, `lnP1ApproxWith`, `leadingZeroBits` | |

`expPrecRound x p mode` is `x.exp_prec_round(p, mode)`, and `exp x` is `x.exp()`, at `x`'s
precision; `expRatPrecRound q p mode` is `Float::exp_rational_prec_round(q, p, mode)`. Azurite's
exponent is unbounded, so its $$e^x$$ never overflows or underflows; Malachite's result is that
value rounded into Malachite's exponent range, overflowing to $$\infty$$ (or the largest finite
value, by the rounding mode) and underflowing to $$0$$ or the smallest positive value, and the two
agree once Azurite's result is brought into that range. For $$|x|$$ near $$2^{2^{28}}$$, $$e^x$$ has
a binary exponent of some $$2^{28}$$ bits, beyond practical computation, so the oracle compares
Malachite's result at such an $$x$$ with Azurite's at an input whose result Malachite rounds
identically: $$2^{30}$$ for $$x \ge 2^{30}$$ (both overflow), $$-2^{31}$$ for $$x \le -2^{31}$$
(both underflow far below half the smallest positive value), and, for $$e^x - 1$$ at
$$x \le -2^{31}$$, $$-(p + 2)$$ (both are $$-1 + \varepsilon$$ with $$\varepsilon < 2^{-p-1}$$).
`expM1PrecRound x p mode` is `x.exp_x_minus_1_prec_round(p, mode)` and `expM1RatPrecRound q p mode`
is `Float::exp_x_minus_1_rational_prec_round(q, p, mode)`; `expM1` is ≈ only for the sign of a zero:
Malachite's `exp_x_minus_1(-0.0)` is `-0.0`, where Azurite gives `0`.

`pow2PrecRound x p mode` is `Float::power_of_2_of_float_prec_round(x, p, mode)`, and `pow2 x` is
`Float::power_of_2(x)`, at `x`'s precision; `pow2RatPrecRound q p mode` is
`Float::power_of_2_rational_prec_round(q, p, mode)`; `pow2M1PrecRound x p mode` is
`x.power_of_2_x_minus_1_prec_round(p, mode)`; and `pow2M1RatPrecRound q p mode` is
`Float::power_of_2_x_minus_1_rational_prec_round(q, p, mode)`. The exponent range and the
substitution for huge $$|x|$$ are as for $$e^x$$, and `pow2M1` is ≈ for the sign of a zero, as
`expM1` is.

`powBasePrecRound u x p mode` is `Float::unsigned_pow_prec_round(u, x, p, mode)`, and
`powBaseRatPrecRound u q p mode` is `Float::unsigned_pow_rational_prec_round(u, q, p, mode)`, for an
integer `q` also `Float::unsigned_pow_unsigned_prec_round(u, q, p, mode)`. Both follow MPFR's
special values, among them $$1^x = 1$$ for every $$x$$, `NaN` and $$\pm\infty$$ included. Malachite
has $$u^x - 1$$ only for $$u = 10$$ and $$u = 2$$, so the `powBaseM1` rows are ≈. The base-10
functions are `powBase*` at $$u = 10$$: `pow10PrecRound x p mode` is
`Float::power_of_10_of_float_prec_round(x, p, mode)`, and `pow10 x` is `Float::power_of_10(x)`, at
`x`'s precision; `pow10RatPrecRound q p mode` is
`Float::power_of_10_rational_prec_round(q, p, mode)`; `pow10M1PrecRound x p mode` is
`x.power_of_10_x_minus_1_prec_round(p, mode)`; and `pow10M1RatPrecRound q p mode` is
`Float::power_of_10_x_minus_1_rational_prec_round(q, p, mode)`. `pow10M1` is ≈ for the sign of a
zero, as `expM1` is. Huge exponents are substituted as for $$e^x$$, for a `Rational` exponent too,
but with $$\pm(2^k + 2^{-7})$$ in place of $$\pm 2^k$$: for a base $$u = r^s$$ with $$r$$ its
primitive root, $$s \le 63 < 2^7$$, so $$u$$ to that power is irrational and Azurite reaches it by
its Ziv loop rather than by forming an exact power with billions of bits.

`lnPrecRound x p mode` is `x.ln_prec_round(p, mode)`, and `ln x` is `x.ln()`, at `x`'s
precision. `lnRatPrecRound q p mode` is `Float::ln_rational_prec_round(q, p, mode)`; for a
nonnegative integer `n`, it is also `Float::ln_unsigned_prec_round(n, p, mode)`. `lnP1PrecRound x p
mode` is `x.ln_1_plus_x_prec_round(p, mode)`, ≈ only for the sign of a zero: Malachite's
`ln_1_plus_x(-0.0)` is `-0.0`, where Azurite, with no signed zero, gives `0`. The special values
agree: $$\ln 0 = -\infty$$, $$\ln x$$ is `NaN` for $$x < 0$$, $$\ln 1 = 0$$ exactly, and
$$\ln(1 + x)$$ is $$-\infty$$ at $$x = -1$$ and `NaN` below it. The `*With` forms take the
brackets of $$\pi$$ and $$\ln 2$$ from a shared `LnConsts`, which speeds up many evaluations at one
precision and gives the same results; Malachite caches its constants internally instead, so it has
no such parameter.

`log2PrecRound x p mode` is `x.log_base_2_prec_round(p, mode)`, exact (an integer) when `x` is a
power of 2; `log2RatPrecRound q p mode` is `Float::log_base_2_rational_prec_round(q, p, mode)`; and
`log2P1PrecRound x p mode` is `x.log_base_2_1_plus_x_prec_round(p, mode)`, ≈ for the sign of a
zero result, as with `lnP1`.

`log10PrecRound x p mode` is `x.log_base_10_prec_round(p, mode)`; `log10RatPrecRound q p mode` is
`Float::log_base_10_rational_prec_round(q, p, mode)`; and `log10P1PrecRound x p mode` is
`x.log_base_10_1_plus_x_prec_round(p, mode)`, ≈ for the sign of a zero result, as with `lnP1`.
Azurite defines all three as `logBase` with the base 10.

`logBasePow2PrecRound x k p mode` is `x.log_base_power_of_2_prec_round(k, p, mode)`, the logarithm
to base $$2^k$$ for a nonzero integer `k` (Malachite's `pow`, an `i64`), which may be negative;
`logBasePow2RatPrecRound q k p mode` is `Float::log_base_power_of_2_rational_prec_round(q, k, p,
mode)`; and `logBasePow2P1PrecRound x k p mode` is `x.log_base_power_of_2_1_plus_x_prec_round(k, p,
mode)`, ≈ for the sign of a zero result, as with `lnP1`. When the argument (or `1 + x`) is $$2^j$$,
both give the rational $$j / k$$, rounded once. Azurite returns `NaN` for $$k = 0$$, where Malachite
panics.

`logBasePrecRound x b p mode` is `x.log_base_prec_round(b, p, mode)`, the logarithm to a `u64`
base `b`; `logBaseRatPrecRound q b p mode` is `Float::log_base_rational_prec_round(q, b, p, mode)`;
and `logBaseP1PrecRound x b p mode` is `x.log_base_1_plus_x_prec_round(b, p, mode)`, ≈ for the
sign of a zero result, as with `lnP1`. A power-of-2 base delegates to the `logBasePow2` functions,
and an argument that is a power of the base's primitive root gives a rational, rounded once.
Azurite returns `NaN` for the bases 0 and 1, where Malachite panics.

`logBaseFloatPrecRound x b p mode` is `x.log_base_float_base_prec_round(&b, p, mode)`, and
`logBaseFloat x b` is `x.log_base(b)`, at `x`'s precision; `logBaseFloatP1PrecRound x b p mode` is
`x.log_base_float_base_1_plus_x_prec_round(&b, p, mode)`; and `logBaseRatFloatPrecRound q b p
mode` is `Float::log_base_rational_float_base_prec_round(q, b, p, mode)`. When the argument and
the base are integer powers of a common rational, both give the rational quotient of the
exponents, rounded once, and the special values agree. All three are ≈ for the sign of a zero
result: Malachite gives $$\log_b 1$$ (and $$\log_b(1 + 0)$$) the sign of $$1 / \ln b$$, where
Azurite, with no signed zero, gives `0`.

`logBaseRatBasePrecRound x b p mode` is `x.log_base_rational_base_prec_round(&b, p, mode)`, and
`logBaseRatBase x b` is `x.log_base(b)`, at `x`'s precision; `logBaseRatBaseP1PrecRound x b p mode`
is `x.log_base_rational_base_1_plus_x_prec_round(&b, p, mode)`; and `logBaseRatRatPrecRound q b p
mode` is `Float::log_base_rational_rational_base_prec_round(q, b, p, mode)`. Malachite requires
$$b > 1$$ and panics otherwise, where Azurite is defined for every base: `NaN` for $$b < 0$$, the
`logBaseFloat` values for $$b \in \{0, 1\}$$, and the logarithm itself for $$0 < b < 1$$. Only
`logBaseRatBaseP1` is ≈, for the sign of a zero result, as with `lnP1`.

`lambertW0PrecRound x p mode` and `lambertWm1PrecRound x p mode` are the principal branch $$W_0$$
and the lower branch $$W_{-1}$$ of the Lambert $$W$$ function, the inverse of $$w e^w$$, and the
`Rat` forms take a rational argument. `lambertW0PrecRound x p mode` is
`x.lambert_w0_prec_round(p, mode)`, `lambertWm1PrecRound x p mode` is
`x.lambert_w_minus_1_prec_round(p, mode)`, and `lambertW0RatPrecRound q p mode` and
`lambertWm1RatPrecRound q p mode` are `Float::lambert_w0_rational_prec_round(q, p, mode)` and
`Float::lambert_w_minus_1_rational_prec_round(q, p, mode)`. Malachite's functions are ports of
Azurite's, so the two agree wherever a `Float` can hold the result; the only results it cannot
hold are those of $$W_0$$ of the smallest positive `Float`s, which underflow.
`lambertW0PrecRound` is ≈ because
`Float` has a signed zero: $$W_0(\pm 0.0) = \pm 0.0$$. The same functions exist for
`BigExponentFloat`, whose exponent, like `AzFloat`'s, is unbounded.

`treeT0PrecRound x p mode` and `treeTm1PrecRound x p mode` are the two real branches $$T_0$$ and
$$T_{-1}$$ of the tree function $$T(x) = -W(-x)$$, a solution of $$T e^{-T} = x$$, named as in the
Digital Library of Mathematical Functions, and the `Rat` forms take a rational argument.
`treeT0PrecRound x p mode` is `x.tree_t0_prec_round(p, mode)`, `treeTm1PrecRound x p mode` is
`x.tree_t_minus_1_prec_round(p, mode)`, and `treeT0RatPrecRound q p mode` and
`treeTm1RatPrecRound q p mode` are `Float::tree_t0_rational_prec_round(q, p, mode)` and
`Float::tree_t_minus_1_rational_prec_round(q, p, mode)`. On both sides they are the Lambert $$W$$
functions of the negated argument, rounded in the opposite direction and negated, so they agree
wherever a `Float` can hold the result; the only results it cannot hold are those of $$T_0$$ of
the negative `Float`s closest to zero, which underflow. `treeT0PrecRound` is ≈ because `Float` has
a signed zero: $$T_0(\pm 0.0) = \pm 0.0$$. The same functions exist for `BigExponentFloat`.

`wrightOmegaPrecRound x p mode` is `x.wright_omega_prec_round(p, mode)`, the Wright omega function
$$\omega(x) = W_0(e^x)$$, the positive solution $$y$$ of $$y + \ln y = x$$, and
`wrightOmegaRatPrecRound q p mode` is `Float::wright_omega_rational_prec_round(q, p, mode)`.
Malachite's functions are ports of Azurite's: a Newton candidate verified by one enclosure of its
residual, a single rounding test for a large argument, and the bracket of $$W_0$$ at $$e^x$$ as the
fallback. Malachite adds a path of its own for a very negative argument, where
$$E - E^2 < \omega(x) < E - E^2 + \tfrac{3}{2}E^3$$ with $$E = e^x$$ needs one exponential. The
results agree wherever a `Float` can hold them: a `Float` result underflows once
$$\omega(x) \approx e^x$$ drops below $$2^{-2^{30}}$$. For an argument at or below about
$$-2^{31}$$ Malachite returns that underflow result without computing anything, where Azurite
computes the exponential, which takes time and memory in proportion to the argument. The sign of a
zero argument makes no difference: $$\omega(\pm 0.0) = W_0(1)$$. The same functions exist for
`BigExponentFloat`, which never underflows and, like Azurite, computes the exponential however
negative the argument is.

## Elliptic integrals {#elliptic-integrals}

| | Azurite | Malachite |
| :---: | --- | --- |
| ≈ | `agmPrecRound (x y : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `agm` | [`agm_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.agm_prec_round), [`agm_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.agm_prec), [`agm_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.agm_round), [`Agm`](https://docs.rs/malachite-base/latest/malachite_base/num/elliptic/traits/trait.Agm.html), [`AgmAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/elliptic/traits/trait.AgmAssign.html) |
| ≈ | `agmRatPrecRound (q₁ q₂ : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `agmRat` | [`agm_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.agm_rational_prec_round), [`agm_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.agm_rational_prec) |
| ≈ | `agmFloatRatPrecRound (x : AzFloat) (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `agmFloatRat` | [`agm_float_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.agm_float_rational_prec_round), [`agm_float_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.agm_float_rational_prec), [`agm_float_rational_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.agm_float_rational_round), `Agm<Rational>` and `AgmAssign<Rational>` for `Float` |
| — | `AgmBounds`, `agmStep`, `agmStepLo`, `agmIterLoN`, `agmConverged`, `agmIter`, `expBits`, `agmIterFuel`, `agmApproxFrom`, `agmApprox`, `ratBounds`, `ratExpBits`, `agmRatApprox`, `agmFloatRatApprox`, `agmFuel` | |

`agmPrecRound x y p mode` is `x.agm_prec_round(y, p, mode)`, and `agm x y p` is
`x.agm_prec(y, p).0`; Malachite's `agm` rounds to the larger of the two precisions.
`agmRatPrecRound q₁ q₂ p mode` is `Float::agm_rational_prec_round(q₁, q₂, p, mode)`, and
`agmFloatRatPrecRound x q p mode` is `x.agm_float_rational_prec_round(q, p, mode)`, whose `Agm`
form rounds to `x`'s precision. All three are ≈ for one special case. Malachite follows MPFR:
after `NaN` and the infinities, a zero argument gives `0.0` even when the other argument is
negative, so $$\operatorname{AGM}(0, -1) = 0$$. Azurite checks for a negative argument first and
gives `NaN`.

## Trigonometric functions {#trigonometric-functions}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `cosPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cos` | [`cos_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cos_prec_round), [`cos_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cos_prec), [`cos_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cos_round), [`Cos`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.Cos.html), [`CosAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.CosAssign.html) |
| ✓ | `cosRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cosRat` | [`cos_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cos_rational_prec_round), [`cos_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cos_rational_prec) |
| ✓ | `cosWithPeriodPrecRound (x : AzFloat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cosWithPeriod` | [`cos_with_period_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cos_with_period_prec_round), [`cos_with_period_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cos_with_period_prec), [`cos_with_period_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cos_with_period_round), [`cos_with_period`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cos_with_period) |
| ✓ | `cosRatWithPeriodPrecRound (q : AzRat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cosRatWithPeriod` | [`cos_with_period_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cos_with_period_rational_prec_round), [`cos_with_period_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cos_with_period_rational_prec) |
| ✓ | `cosPiPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cosPi` | [`cos_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cos_pi_prec_round), [`cos_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cos_pi_prec), [`cos_pi_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cos_pi_round), [`cos_pi`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cos_pi) |
| ✓ | `cosPiRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cosPiRat` | [`cos_pi_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cos_pi_rational_prec_round), [`cos_pi_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cos_pi_rational_prec) |
| — | `cosWork`, `cosTerms`, `enclClamp0`, `enclAbs`, `cosDouble`, `cosSeriesLoop`, `cosSeries`, `cosTail`, `cosBracket`, `cosTiny?`, `cosApprox`, `cosRatBracket`, `cosRatApprox`, `periodFrac`, `periodFracRat`, `cosPeriodBracket`, `cosPeriodApprox` | |
| ≈ | `sinPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sin` | [`sin_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_prec_round), [`sin_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_prec), [`sin_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_round), [`Sin`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.Sin.html), [`SinAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.SinAssign.html) |
| ✓ | `sinRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sinRat` | [`sin_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_rational_prec_round), [`sin_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_rational_prec) |
| ≈ | `sinWithPeriodPrecRound (x : AzFloat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sinWithPeriod` | [`sin_with_period_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_with_period_prec_round), [`sin_with_period_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_with_period_prec), [`sin_with_period_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_with_period_round), [`sin_with_period`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_with_period) |
| ≈ | `sinRatWithPeriodPrecRound (q : AzRat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sinRatWithPeriod` | [`sin_with_period_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_with_period_rational_prec_round), [`sin_with_period_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_with_period_rational_prec) |
| ≈ | `sinPiPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sinPi` | [`sin_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_pi_prec_round), [`sin_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_pi_prec), [`sin_pi_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_pi_round), [`sin_pi`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_pi) |
| ≈ | `sinPiRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sinPiRat` | [`sin_pi_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_pi_rational_prec_round), [`sin_pi_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_pi_rational_prec) |
| — | `sinBracket`, `sinApprox`, `sinTiny?`, `sinRatBracket`, `sinRatApprox`, `sinTinyCore?`, `negAbsRat`, `sinTinyRat?`, `sinPeriodBracket`, `sinPeriodApprox`, `sinPeriodTinyBracket`, `sinPeriodTiny?`, `signedPow2` | |
| ≈ | `sinCosPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : (AzFloat × Ordering) × (AzFloat × Ordering)`, `sinCos` | [`sin_cos_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_cos_prec_round), [`sin_cos_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_cos_prec), [`sin_cos_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_cos_round), [`SinCos`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.SinCos.html), [`SinCosAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.SinCosAssign.html) |
| ✓ | `sinCosRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : (AzFloat × Ordering) × (AzFloat × Ordering)`, `sinCosRat` | [`sin_cos_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_cos_rational_prec_round), [`sin_cos_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_cos_rational_prec) |
| ≈ | `sinCosWithPeriodPrecRound (x : AzFloat) (u : UInt64) (p : Nat) (mode : RoundingMode) : (AzFloat × Ordering) × (AzFloat × Ordering)`, `sinCosWithPeriod` | [`sin_cos_with_period_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_cos_with_period_prec_round), [`sin_cos_with_period_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_cos_with_period_prec), [`sin_cos_with_period_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_cos_with_period_round), [`sin_cos_with_period`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_cos_with_period) |
| ≈ | `sinCosRatWithPeriodPrecRound (q : AzRat) (u : UInt64) (p : Nat) (mode : RoundingMode) : (AzFloat × Ordering) × (AzFloat × Ordering)`, `sinCosRatWithPeriod` | [`sin_cos_with_period_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_cos_with_period_rational_prec_round), [`sin_cos_with_period_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_cos_with_period_rational_prec) |
| ≈ | `sinCosPiPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : (AzFloat × Ordering) × (AzFloat × Ordering)`, `sinCosPi` | [`sin_cos_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_cos_pi_prec_round), [`sin_cos_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_cos_pi_prec), [`sin_cos_pi_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_cos_pi_round), [`sin_cos_pi`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_cos_pi) |
| ≈ | `sinCosPiRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : (AzFloat × Ordering) × (AzFloat × Ordering)`, `sinCosPiRat` | [`sin_cos_pi_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_cos_pi_rational_prec_round), [`sin_cos_pi_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sin_cos_pi_rational_prec) |
| — | `sinCosBlockBase`, `sinCosCut`, `sinCosDigit`, `sinCosTermsLoop`, `sinCosTerms`, `enclFraction`, `sinCosBlock`, `sinCosRotate`, `sinCosFixLoop`, `sinCosFix`, `enclDouble`, `enclMulNonnegSigned`, `sinCosTail`, `sinCosTailSin`, `sinCosTailCos`, `sinCosReduce`, `sinCosBracket`, `sinCosApprox`, `tinyExtra`, `ratTinyExtra`, `sinCosRatBracket`, `sinCosRatApprox`, `sinCosPeriodBracket`, `sinCosPeriodApprox` | |
| ≈ | `tanPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `tan` | [`tan_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tan_prec_round), [`tan_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tan_prec), [`tan_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tan_round), [`Tan`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.Tan.html), [`TanAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.TanAssign.html) |
| ✓ | `tanRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `tanRat` | [`tan_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tan_rational_prec_round), [`tan_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tan_rational_prec) |
| ≈ | `tanWithPeriodPrecRound (x : AzFloat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `tanWithPeriod` | [`tan_with_period_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tan_with_period_prec_round), [`tan_with_period_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tan_with_period_prec), [`tan_with_period_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tan_with_period_round), [`tan_with_period`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tan_with_period) |
| ≈ | `tanRatWithPeriodPrecRound (q : AzRat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `tanRatWithPeriod` | [`tan_with_period_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tan_with_period_rational_prec_round), [`tan_with_period_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tan_with_period_rational_prec) |
| ≈ | `tanPiPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `tanPi` | [`tan_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tan_pi_prec_round), [`tan_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tan_pi_prec), [`tan_pi_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tan_pi_round), [`tan_pi`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tan_pi) |
| ≈ | `tanPiRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `tanPiRat` | [`tan_pi_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tan_pi_rational_prec_round), [`tan_pi_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tan_pi_rational_prec) |
| — | `enclDivSignedPos`, `tanOfSinCos`, `negSwapApprox`, `tanBracket`, `tanApprox`, `tanTiny?`, `tanRatBracket`, `tanRatApprox`, `tanTinyCore?`, `tanTinyRat?`, `tanPeriodBracket`, `tanPeriodApprox`, `tanPeriodTinyBracket`, `tanPeriodTiny?`, `tanPeriodCore` | |
| ✓ | `secPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sec` | [`sec_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sec_prec_round), [`sec_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sec_prec), [`sec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sec_round), [`Sec`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.Sec.html), [`SecAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.SecAssign.html) |
| ✓ | `secRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `secRat` | [`sec_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sec_rational_prec_round), [`sec_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sec_rational_prec) |
| ✓ | `secWithPeriodPrecRound (x : AzFloat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `secWithPeriod` | [`sec_with_period_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sec_with_period_prec_round), [`sec_with_period_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sec_with_period_prec), [`sec_with_period_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sec_with_period_round), [`sec_with_period`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sec_with_period) |
| ✓ | `secRatWithPeriodPrecRound (q : AzRat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `secRatWithPeriod` | [`sec_with_period_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sec_with_period_rational_prec_round), [`sec_with_period_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sec_with_period_rational_prec) |
| ✓ | `secPiPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `secPi` | [`sec_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sec_pi_prec_round), [`sec_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sec_pi_prec), [`sec_pi_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sec_pi_round), [`sec_pi`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sec_pi) |
| ✓ | `secPiRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `secPiRat` | [`sec_pi_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sec_pi_rational_prec_round), [`sec_pi_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sec_pi_rational_prec) |
| — | `secBracket`, `secApprox`, `secTiny?`, `secPeriodBracket`, `secPeriodApprox`, `secRatBracket`, `secRatApprox`, `secPeriodCore` | |
| ≈ | `cscPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `csc` | [`csc_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csc_prec_round), [`csc_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csc_prec), [`csc_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csc_round), [`Csc`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.Csc.html), [`CscAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.CscAssign.html) |
| ✓ | `cscRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cscRat` | [`csc_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csc_rational_prec_round), [`csc_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csc_rational_prec) |
| ≈ | `cscWithPeriodPrecRound (x : AzFloat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cscWithPeriod` | [`csc_with_period_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csc_with_period_prec_round), [`csc_with_period_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csc_with_period_prec), [`csc_with_period_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csc_with_period_round), [`csc_with_period`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csc_with_period) |
| ✓ | `cscRatWithPeriodPrecRound (q : AzRat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cscRatWithPeriod` | [`csc_with_period_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csc_with_period_rational_prec_round), [`csc_with_period_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csc_with_period_rational_prec) |
| ≈ | `cscPiPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cscPi` | [`csc_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csc_pi_prec_round), [`csc_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csc_pi_prec), [`csc_pi_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csc_pi_round), [`csc_pi`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csc_pi) |
| ✓ | `cscPiRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cscPiRat` | [`csc_pi_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csc_pi_rational_prec_round), [`csc_pi_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csc_pi_rational_prec) |
| — | `cscBracket`, `cscApprox`, `cscRatBracket`, `cscRatApprox`, `cscPeriodBracket`, `cscPeriodApprox`, `tinyRecipOf?`, `cscTinyCore?`, `cscTiny?`, `cscTinyRat?`, `periodTinyArg`, `cscPeriodTiny?`, `cscPeriodCore` | |
| ≈ | `cotPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cot` | [`cot_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cot_prec_round), [`cot_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cot_prec), [`cot_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cot_round), [`Cot`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.Cot.html), [`CotAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.CotAssign.html) |
| ✓ | `cotRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cotRat` | [`cot_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cot_rational_prec_round), [`cot_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cot_rational_prec) |
| ≈ | `cotWithPeriodPrecRound (x : AzFloat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cotWithPeriod` | [`cot_with_period_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cot_with_period_prec_round), [`cot_with_period_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cot_with_period_prec), [`cot_with_period_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cot_with_period_round), [`cot_with_period`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cot_with_period) |
| ≈ | `cotRatWithPeriodPrecRound (q : AzRat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cotRatWithPeriod` | [`cot_with_period_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cot_with_period_rational_prec_round), [`cot_with_period_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cot_with_period_rational_prec) |
| ≈ | `cotPiPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cotPi` | [`cot_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cot_pi_prec_round), [`cot_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cot_pi_prec), [`cot_pi_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cot_pi_round), [`cot_pi`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cot_pi) |
| ≈ | `cotPiRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cotPiRat` | [`cot_pi_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cot_pi_rational_prec_round), [`cot_pi_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cot_pi_rational_prec) |
| — | `cotBracket`, `cotApprox`, `cotRatBracket`, `cotRatApprox`, `cotPeriodBracket`, `cotPeriodApprox`, `cotTinyCore?`, `cotTiny?`, `cotTinyRat?`, `cotPeriodTiny?`, `cotPeriodCore` | |
| ≈ | `atanPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `atan` | [`atan_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan_prec_round), [`atan_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan_prec), [`atan_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan_round), [`Atan`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.Atan.html), [`AtanAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.AtanAssign.html) |
| ✓ | `atanRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `atanRat` | [`atan_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan_rational_prec_round), [`atan_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan_rational_prec) |
| ≈ | `atanWithPeriodPrecRound (x : AzFloat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `atanWithPeriod` | [`atan_with_period_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan_with_period_prec_round), [`atan_with_period_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan_with_period_prec), [`atan_with_period_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan_with_period_round), [`atan_with_period`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan_with_period) |
| ≈ | `atanRatWithPeriodPrecRound (q : AzRat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `atanRatWithPeriod` | [`atan_with_period_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan_with_period_rational_prec_round), [`atan_with_period_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan_with_period_rational_prec) |
| ≈ | `atanPiPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `atanPi` | [`atan_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan_pi_prec_round), [`atan_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan_pi_prec), [`atan_pi_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan_pi_round), [`atan_pi`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan_pi) |
| ✓ | `atanPiRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `atanPiRat` | [`atan_pi_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan_pi_rational_prec_round), [`atan_pi_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan_pi_rational_prec) |
| — | `atanBase`, `atanStepAt`, `atanStep`, `atanSeedSum`, `atanSeedFix`, `atanSeed`, `atanSeeded?`, `atanLoop`, `atanWork`, `atanBracket`, `atanApprox`, `atanTiny?`, `atanWiden`, `atanRatBracket`, `atanRatApprox`, `atanTinyRat?`, `atanRatAbsEncl`, `atanPeriodScale`, `turnFraction` | |
| ≈ | `atan2PrecRound (y x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `atan2` | [`atan2_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan2_prec_round), [`atan2_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan2_prec), [`atan2_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan2_round), [`Atan2`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.Atan2.html), [`Atan2Assign`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.Atan2Assign.html) |
| ✓ | `atan2RatPrecRound (y x : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `atan2Rat` | [`atan2_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan2_rational_prec_round), [`atan2_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan2_rational_prec) |
| ≈ | `atan2WithPeriodPrecRound (y x : AzFloat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `atan2WithPeriod` | [`atan2_with_period_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan2_with_period_prec_round), [`atan2_with_period_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan2_with_period_prec), [`atan2_with_period_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan2_with_period_round) |
| ✓ | `atan2RatWithPeriodPrecRound (y x : AzRat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `atan2RatWithPeriod` | [`atan2_with_period_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan2_with_period_rational_prec_round), [`atan2_with_period_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan2_with_period_rational_prec) |
| ≈ | `atan2PiPrecRound (y x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `atan2Pi` | [`atan2_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan2_pi_prec_round), [`atan2_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan2_pi_prec), [`atan2_pi_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan2_pi_round) |
| ✓ | `atan2PiRatPrecRound (y x : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `atan2PiRat` | [`atan2_pi_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan2_pi_rational_prec_round), [`atan2_pi_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atan2_pi_rational_prec) |
| — | `piThreeQuartersPrecRound`, `signedRound`, `enclApprox`, `piShiftEncl`, `atanQuot`, `atanQuotRel`, `atan2Bracket`, `atan2Approx`, `piThreeQuartersApprox`, `atan2RatBracket`, `atan2RatApprox`, `turnFracN`, `turnShift`, `atan2PeriodBracket`, `atan2PeriodOrient`, `atan2PeriodApprox`, `atan2RatPeriodBracket`, `atan2RatPeriodOrient`, `atan2RatPeriodApprox` | |
| ≈ | `asinPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `asin` | [`asin_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asin_prec_round), [`asin_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asin_prec), [`asin_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asin_round), [`Asin`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.Asin.html), [`AsinAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.AsinAssign.html) |
| ✓ | `asinRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `asinRat` | [`asin_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asin_rational_prec_round), [`asin_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asin_rational_prec) |
| ≈ | `asinWithPeriodPrecRound (x : AzFloat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `asinWithPeriod` | [`asin_with_period_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asin_with_period_prec_round), [`asin_with_period_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asin_with_period_prec), [`asin_with_period_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asin_with_period_round), [`asin_with_period`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asin_with_period) |
| ≈ | `asinRatWithPeriodPrecRound (q : AzRat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `asinRatWithPeriod` | [`asin_with_period_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asin_with_period_rational_prec_round), [`asin_with_period_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asin_with_period_rational_prec) |
| ≈ | `asinPiPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `asinPi` | [`asin_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asin_pi_prec_round), [`asin_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asin_pi_prec), [`asin_pi_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asin_pi_round), [`asin_pi`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asin_pi) |
| ✓ | `asinPiRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `asinPiRat` | [`asin_pi_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asin_pi_rational_prec_round), [`asin_pi_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asin_pi_rational_prec) |
| — | `asinSqrt`, `asinArgOf`, `asinArg`, `asinBracket`, `asinApprox`, `asinTiny?`, `asinRatBracket`, `asinRatApprox`, `asinTinyRat?`, `asinAbsEncl`, `asinPeriodApprox`, `asinRatAbsEncl`, `asinRatPeriodApprox`, `turnTwelfth` | |
| ✓ | `acosPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acos` | [`acos_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acos_prec_round), [`acos_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acos_prec), [`acos_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acos_round), [`Acos`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.Acos.html), [`AcosAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.AcosAssign.html) |
| ✓ | `acosRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acosRat` | [`acos_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acos_rational_prec_round), [`acos_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acos_rational_prec) |
| ✓ | `acosWithPeriodPrecRound (x : AzFloat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acosWithPeriod` | [`acos_with_period_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acos_with_period_prec_round), [`acos_with_period_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acos_with_period_prec), [`acos_with_period_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acos_with_period_round), [`acos_with_period`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acos_with_period) |
| ✓ | `acosRatWithPeriodPrecRound (q : AzRat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acosRatWithPeriod` | [`acos_with_period_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acos_with_period_rational_prec_round), [`acos_with_period_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acos_with_period_rational_prec) |
| ✓ | `acosPiPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acosPi` | [`acos_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acos_pi_prec_round), [`acos_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acos_pi_prec), [`acos_pi_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acos_pi_round), [`acos_pi`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acos_pi) |
| ✓ | `acosPiRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acosPiRat` | [`acos_pi_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acos_pi_rational_prec_round), [`acos_pi_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acos_pi_rational_prec) |
| — | `diffQuot`, `diffQuotSqrt`, `acosQuot`, `acosArg`, `acosWork`, `acosBracket`, `acosApprox`, `acosRatBracket`, `acosRatApprox`, `acosPeriodOf`, `acosPeriodOrientOf`, `acosPeriodBracket`, `acosPeriodApprox`, `ratAbsLeHalf`, `acosRatPeriodBracket`, `acosRatPeriodApprox`, `turnRatio` | |
| ✓ | `asecPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `asec` | [`asec_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asec_prec_round), [`asec_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asec_prec), [`asec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asec_round), [`Asec`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.Asec.html), [`AsecAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.AsecAssign.html) |
| ✓ | `asecRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `asecRat` | [`asec_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asec_rational_prec_round), [`asec_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asec_rational_prec) |
| ✓ | `asecWithPeriodPrecRound (x : AzFloat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `asecWithPeriod` | [`asec_with_period_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asec_with_period_prec_round), [`asec_with_period_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asec_with_period_prec), [`asec_with_period_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asec_with_period_round), [`asec_with_period`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asec_with_period) |
| ✓ | `asecRatWithPeriodPrecRound (q : AzRat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `asecRatWithPeriod` | [`asec_with_period_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asec_with_period_rational_prec_round), [`asec_with_period_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asec_with_period_rational_prec) |
| ✓ | `asecPiPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `asecPi` | [`asec_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asec_pi_prec_round), [`asec_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asec_pi_prec), [`asec_pi_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asec_pi_round), [`asec_pi`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asec_pi) |
| ✓ | `asecPiRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `asecPiRat` | [`asec_pi_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asec_pi_rational_prec_round), [`asec_pi_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asec_pi_rational_prec) |
| — | `asecWork`, `asecBracket`, `asecApprox`, `asecComplement`, `asecPeriodBracket`, `asecPeriodApprox` | |
| ≈ | `acscPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acsc` | [`acsc_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsc_prec_round), [`acsc_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsc_prec), [`acsc_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsc_round), [`Acsc`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.Acsc.html), [`AcscAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.AcscAssign.html) |
| ✓ | `acscRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acscRat` | [`acsc_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsc_rational_prec_round), [`acsc_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsc_rational_prec) |
| ≈ | `acscWithPeriodPrecRound (x : AzFloat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acscWithPeriod` | [`acsc_with_period_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsc_with_period_prec_round), [`acsc_with_period_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsc_with_period_prec), [`acsc_with_period_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsc_with_period_round), [`acsc_with_period`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsc_with_period) |
| ≈ | `acscRatWithPeriodPrecRound (q : AzRat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acscRatWithPeriod` | [`acsc_with_period_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsc_with_period_rational_prec_round), [`acsc_with_period_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsc_with_period_rational_prec) |
| ≈ | `acscPiPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acscPi` | [`acsc_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsc_pi_prec_round), [`acsc_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsc_pi_prec), [`acsc_pi_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsc_pi_round), [`acsc_pi`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsc_pi) |
| ✓ | `acscPiRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acscPiRat` | [`acsc_pi_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsc_pi_rational_prec_round), [`acsc_pi_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsc_pi_rational_prec) |
| — | `acscArg`, `acscTinyEncl`, `acscBracket`, `acscApprox`, `acscPeriodApprox` | |
| ≈ | `acotPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acot` | [`acot_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acot_prec_round), [`acot_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acot_prec), [`acot_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acot_round), [`Acot`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.Acot.html), [`AcotAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/trigonometric/traits/trait.AcotAssign.html) |
| ✓ | `acotRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acotRat` | [`acot_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acot_rational_prec_round), [`acot_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acot_rational_prec) |
| ≈ | `acotWithPeriodPrecRound (x : AzFloat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acotWithPeriod` | [`acot_with_period_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acot_with_period_prec_round), [`acot_with_period_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acot_with_period_prec), [`acot_with_period_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acot_with_period_round), [`acot_with_period`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acot_with_period) |
| ≈ | `acotRatWithPeriodPrecRound (q : AzRat) (u : UInt64) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acotRatWithPeriod` | [`acot_with_period_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acot_with_period_rational_prec_round), [`acot_with_period_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acot_with_period_rational_prec) |
| ≈ | `acotPiPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acotPi` | [`acot_pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acot_pi_prec_round), [`acot_pi_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acot_pi_prec), [`acot_pi_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acot_pi_round), [`acot_pi`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acot_pi) |
| ✓ | `acotPiRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acotPiRat` | [`acot_pi_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acot_pi_rational_prec_round), [`acot_pi_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acot_pi_rational_prec) |
| — | `acotTinyEncl`, `acotBracket`, `acotApprox` | |

`cosPrecRound x p mode` is `x.cos_prec_round(p, mode)`, and `cosRatPrecRound q p mode` is
`Float::cos_rational_prec_round(q, p, mode)`. `cosWithPeriodPrecRound x u p mode` is
`x.cos_with_period_prec_round(u, p, mode)`, $$\cos 2\pi x / u$$, and
`cosRatWithPeriodPrecRound q u p mode` is `Float::cos_with_period_rational_prec_round(q, u, p, mode)`;
both give `NaN` for $$u = 0$$. `cosPiPrecRound x p mode` is `x.cos_pi_prec_round(p, mode)`,
$$\cos \pi x$$, and `cosPiRatPrecRound q p mode` is `Float::cos_pi_rational_prec_round(q, p, mode)`.
The forms without a rounding mode round to nearest, at the precision of `x` for a `Float`
argument. A zero result is $$+0$$ in Malachite, so the absence of a signed zero in Azurite makes no
difference.

The `sin` functions correspond in the same way, with `sin` in place of `cos`. All but `sinRat` are
≈ for the sign of a zero result: Malachite, following MPFR, gives an exact zero the sign of the
argument, so $$\sin(-0) = -0$$, and `x.sin_pi()` is $$-0$$ for a negative integer `x`, as is
`x.sin_with_period(u)` for a negative multiple of $$u/2$$. Azurite's zero has no sign. The
argument of `sinRat` is zero only when it is $$0$$, whose sine is $$+0$$ on both sides.

`sinCosPrecRound` gives the sine and cosine together, each with its own `Ordering`, as
`x.sin_cos_prec_round(p, mode)` does, and the other `sinCos` functions correspond to Malachite's
`sin_cos` functions in the same way. Each is ≈ or ✓ as the `sin` function of the same argument is,
since only the sine can be a zero of either sign.

The `tan` functions correspond in the same way, with `tan` in place of `sin`, and are ≈ or ✓ as
the `sin` functions are, for the same reason. At a pole, where $$x/u$$ is an odd multiple of
$$1/4$$, or $$x$$ is an odd multiple of $$1/2$$ for `tanPi`, both give the infinity with the sign
of the sine, as IEEE 754 does: $$\tan(\pi/2) = +\infty$$.

The `sec` functions correspond in the same way, with `sec` in place of `cos`. The secant is never
zero, so all six are ✓; at a pole, where the cosine is zero, both give $$+\infty$$.

The `csc` functions correspond in the same way, with `csc` in place of `sin`. At a pole, where the
sine is zero, both give the infinity with the sign of that zero, which for a nonzero argument is the
sign of the argument, so `cscPi` of $$-1$$ is $$-\infty$$. The forms of a `Float` are ≈ for
the sign of the zero argument: Malachite, following MPFR, gives $$\csc(-0) = -\infty$$, while
Azurite's single zero gives $$+\infty$$. A `Rational` zero has no sign, so the `Rat` forms are ✓.

The `cot` functions correspond in the same way, with `cot` in place of `tan`. Their poles are those
of the cosecant: the infinity with the sign of the sine's zero times the sign of the cosine, so
$$\cot(-0)$$ is $$-\infty$$ in Malachite and $$+\infty$$ in Azurite. Their zeros are those of the
cosine, which Malachite, following MPFR, signs, as in `Float::cot_pi_rational_prec(-1/2, p)`, which
is $$-0$$. So all but `cotRat`, which has no zero result and no pole at a nonzero argument, are ≈.

`atanPrecRound x p mode` is `x.atan_prec_round(p, mode)`, and the other `atan` functions correspond
in the same way: `atanWithPeriodPrecRound x u p mode` is $$u \arctan(x) / 2\pi$$, in $$u$$ths of a
turn, and `atanPiPrecRound x p mode` is $$\arctan(x) / \pi$$. Both give $$\pm\pi/2$$ (or $$\pm u/4$$,
or $$\pm 1/2$$) at $$\pm\infty$$. The forms of a `Float` are ≈ for the sign of a zero result:
Malachite, following MPFR, gives $$\arctan(-0) = -0$$, and Azurite's zero has no sign. So is
`atanRatWithPeriod`, whose result for $$u = 0$$ is a zero with the sign of $$q$$ in Malachite.

`atan2PrecRound y x p mode` is `y.atan2_prec_round(x, p, mode)`, the angle of the point
$$(x, y)$$ in $$(-\pi, \pi]$$, and the other `atan2` functions correspond in the same way, in
$$u$$ths of a turn or in half-turns; without a precision, Malachite rounds to the larger of the
arguments' precisions. Malachite, following MPFR and IEEE 754, distinguishes the zeros:
$$\operatorname{atan2}(\pm 0, x)$$ is $$\pm 0$$ for $$x$$ positive or $$+0$$ and $$\pm\pi$$ for
$$x$$ negative or $$-0$$. Azurite's single zero acts as $$+0$$ in that table, so the forms of a
`Float` are ≈; a `Rational` zero has no sign, and the `Rat` forms are ✓.

`asinPrecRound x p mode` is `x.asin_prec_round(p, mode)`, and the other `asin` functions correspond
in the same way, in $$u$$ths of a turn or in half-turns, as for the arctangent; outside
$$[-1, 1]$$ both give `NaN`. The marks are those of the `atan` functions, for the same reasons:
Malachite gives $$\arcsin(-0) = -0$$, and `asinRatWithPeriod` with $$u = 0$$ gives a zero with the
sign of $$q$$.

The `acos` functions correspond in the same way. The arccosine is never negative, and its only zero,
$$\arccos(1)$$, is $$+0$$ on both sides, so all six are ✓.

The `asec` functions correspond in the same way; inside $$(-1, 1)$$ both give `NaN`. Like the
arccosine, the arcsecant is never negative, and its only zero, $$\operatorname{arcsec}(1)$$, is
$$+0$$ on both sides, so all six are ✓.

The `acsc` functions correspond in the same way, with the marks of the `asin` functions: the
arccosecant is odd and vanishes only at infinity, where Malachite, following MPFR, gives
$$\operatorname{arccsc}(-\infty) = -0$$, and `acscRatWithPeriod` with $$u = 0$$ gives a zero with the
sign of $$q$$; Azurite's zero has no sign.

The `acot` functions correspond in the same way, for the odd arccotangent
$$\operatorname{arccot} x = \arctan(1/x)$$, with range $$(-\pi/2, \pi/2]$$, on both sides. It jumps
at zero, and Malachite, following the signed zeros, gives $$\operatorname{arccot}(\pm0) = \pm\pi/2$$
(a quarter turn, or $$\pm1/2$$, for the other forms), where Azurite's single zero gives the positive
side; it also gives $$\operatorname{arccot}(\pm\infty) = \pm0$$, and `acotRatWithPeriod` with
$$u = 0$$ gives a zero with the sign of $$q$$. The forms of a `Float` and `acotRatWithPeriod` are
≈, and `acotRat` and `acotPiRat`, whose zero argument has no sign, are ✓.

## Hyperbolic functions {#hyperbolic-functions}

| | Azurite | Malachite |
| :---: | --- | --- |
| ≈ | `sinhPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sinh` | [`sinh_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sinh_prec_round), [`sinh_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sinh_prec), [`sinh_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sinh_round), [`Sinh`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.Sinh.html), [`SinhAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.SinhAssign.html) |
| ✓ | `sinhRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sinhRat` | [`sinh_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sinh_rational_prec_round), [`sinh_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sinh_rational_prec) |
| — | `sinhOfExpM1`, `sinhBracket`, `sinhApprox`, `sinhRatBracket`, `sinhRatApprox` | |
| ✓ | `coshPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cosh` | [`cosh_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cosh_prec_round), [`cosh_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cosh_prec), [`cosh_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cosh_round), [`Cosh`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.Cosh.html), [`CoshAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.CoshAssign.html) |
| ✓ | `coshRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `coshRat` | [`cosh_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cosh_rational_prec_round), [`cosh_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.cosh_rational_prec) |
| — | `coshOfExp`, `coshBracket`, `coshApprox`, `coshTinyExp`, `coshRatBracket`, `coshRatApprox`, `coshRatTinyExp` | |
| ≈ | `sinhCoshPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : (AzFloat × Ordering) × (AzFloat × Ordering)`, `sinhCosh` | [`sinh_cosh_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sinh_cosh_prec_round), [`sinh_cosh_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sinh_cosh_prec), [`sinh_cosh_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sinh_cosh_round), [`SinhCosh`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.SinhCosh.html), [`SinhCoshAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.SinhCoshAssign.html) |
| ✓ | `sinhCoshRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : (AzFloat × Ordering) × (AzFloat × Ordering)`, `sinhCoshRat` | [`sinh_cosh_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sinh_cosh_rational_prec_round), [`sinh_cosh_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sinh_cosh_rational_prec) |
| — | `expOfExpM1`, `sinhCoshOfExpM1`, `sinhCoshApprox`, `sinhCoshRatApprox` | |
| ≈ | `tanhPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `tanh` | [`tanh_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tanh_prec_round), [`tanh_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tanh_prec), [`tanh_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tanh_round), [`Tanh`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.Tanh.html), [`TanhAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.TanhAssign.html) |
| ✓ | `tanhRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `tanhRat` | [`tanh_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tanh_rational_prec_round), [`tanh_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.tanh_rational_prec) |
| — | `tanhOfExpM1`, `tanhBracket`, `tanhApprox`, `tanhRatBracket`, `tanhRatApprox`, `tanhNearOne?`, `tanhFloatN`, `tanhRatN` | |
| ≈ | `cothPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `coth` | [`coth_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.coth_prec_round), [`coth_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.coth_prec), [`coth_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.coth_round), [`Coth`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.Coth.html), [`CothAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.CothAssign.html) |
| ✓ | `cothRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cothRat` | [`coth_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.coth_rational_prec_round), [`coth_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.coth_rational_prec) |
| — | `cothOfExpM1`, `cothBracket`, `cothApprox`, `cothRatBracket`, `cothRatApprox`, `cothNearOne?` | |
| ✓ | `sechPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sech` | [`sech_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sech_prec_round), [`sech_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sech_prec), [`sech_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sech_round), [`Sech`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.Sech.html), [`SechAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.SechAssign.html) |
| ✓ | `sechRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `sechRat` | [`sech_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sech_rational_prec_round), [`sech_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sech_rational_prec) |
| — | `sechOfCosh`, `sechBracket`, `sechApprox`, `sechRatBracket`, `sechRatApprox` | |
| ≈ | `cschPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `csch` | [`csch_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csch_prec_round), [`csch_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csch_prec), [`csch_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csch_round), [`Csch`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.Csch.html), [`CschAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.CschAssign.html) |
| ✓ | `cschRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `cschRat` | [`csch_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csch_rational_prec_round), [`csch_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.csch_rational_prec) |
| — | `cschOfSinh`, `cschBracket`, `cschApprox`, `cschRatBracket`, `cschRatApprox`, `cschTinyCore?`, `cschTiny?`, `cschTinyRat?` | |
| ≈ | `asinhPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `asinh` | [`asinh_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asinh_prec_round), [`asinh_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asinh_prec), [`asinh_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asinh_round), [`Asinh`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.Asinh.html), [`AsinhAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.AsinhAssign.html) |
| ✓ | `asinhRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `asinhRat` | [`asinh_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asinh_rational_prec_round), [`asinh_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asinh_rational_prec) |
| — | `asinhArg`, `asinhBracket`, `asinhApprox`, `asinhRatBracket`, `asinhRatApprox` | |
| ✓ | `acoshPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acosh` | [`acosh_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acosh_prec_round), [`acosh_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acosh_prec), [`acosh_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acosh_round), [`Acosh`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.Acosh.html), [`AcoshAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.AcoshAssign.html) |
| ✓ | `acoshRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acoshRat` | [`acosh_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acosh_rational_prec_round), [`acosh_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acosh_rational_prec) |
| — | `acoshArg`, `acoshBracket`, `acoshApprox`, `acoshRatBracket`, `acoshRatApprox`, `acoshExtra` | |
| ≈ | `atanhPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `atanh` | [`atanh_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atanh_prec_round), [`atanh_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atanh_prec), [`atanh_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atanh_round), [`Atanh`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.Atanh.html), [`AtanhAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.AtanhAssign.html) |
| ✓ | `atanhRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `atanhRat` | [`atanh_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atanh_rational_prec_round), [`atanh_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.atanh_rational_prec) |
| — | `atanhArg`, `atanhOfArg`, `atanhBracket`, `atanhApprox`, `atanhRatBracket`, `atanhRatApprox` | |
| ✓ | `asechPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `asech` | [`asech_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asech_prec_round), [`asech_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asech_prec), [`asech_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asech_round), [`Asech`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.Asech.html), [`AsechAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.AsechAssign.html) |
| ✓ | `asechRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `asechRat` | [`asech_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asech_rational_prec_round), [`asech_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.asech_rational_prec) |
| — | `asechArg`, `asechBracket`, `asechApprox` | |
| ≈ | `acschPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acsch` | [`acsch_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsch_prec_round), [`acsch_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsch_prec), [`acsch_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsch_round), [`Acsch`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.Acsch.html), [`AcschAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.AcschAssign.html) |
| ✓ | `acschRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acschRat` | [`acsch_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsch_rational_prec_round), [`acsch_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acsch_rational_prec) |
| — | `acschArg`, `acschBracket`, `acschApprox`, `acschExtra` | |
| ≈ | `acothPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acoth` | [`acoth_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acoth_prec_round), [`acoth_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acoth_prec), [`acoth_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acoth_round), [`Acoth`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.Acoth.html), [`AcothAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.AcothAssign.html) |
| ✓ | `acothRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `acothRat` | [`acoth_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acoth_rational_prec_round), [`acoth_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.acoth_rational_prec) |
| — | `acothBracket`, `acothApprox` | |
| ≈ | `gdPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `gd` | [`gudermannian_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.gudermannian_prec_round), [`gudermannian_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.gudermannian_prec), [`gudermannian_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.gudermannian_round), [`Gudermannian`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.Gudermannian.html), [`GudermannianAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.GudermannianAssign.html) |
| ✓ | `gdRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `gdRat` | [`gudermannian_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.gudermannian_rational_prec_round), [`gudermannian_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.gudermannian_rational_prec) |
| — | `atanAltClose`, `atanAltLoop`, `atanAlt`, `gdSeriesTerms`, `enclNarrow?`, `gdSmallOf`, `gdLargeOf`, `gdCoreOf`, `gdSmallSeries?`, `gdLargeSeries?`, `gdHuge`, `gdCore`, `gdBracket`, `gdApprox`, `gdRatBracket`, `gdRatApprox` | |
| ≈ | `gdInvPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `gdInv` | [`inverse_gudermannian_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.inverse_gudermannian_prec_round), [`inverse_gudermannian_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.inverse_gudermannian_prec), [`inverse_gudermannian_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.inverse_gudermannian_round), [`InverseGudermannian`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.InverseGudermannian.html), [`InverseGudermannianAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/hyperbolic/traits/trait.InverseGudermannianAssign.html) |
| ✓ | `gdInvRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `gdInvRat` | [`inverse_gudermannian_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.inverse_gudermannian_rational_prec_round), [`inverse_gudermannian_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.inverse_gudermannian_rational_prec) |
| — | `gdInvNum`, `gdInvOfSinCos`, `gdInvBracket`, `gdInvApprox`, `gdInvInDomain`, `gdInvRatBracket`, `gdInvRatApprox`, `gdInvRatInDomain` | |

`sinhPrecRound x p mode` is `x.sinh_prec_round(p, mode)`, and `sinhRatPrecRound q p mode` is
`Float::sinh_rational_prec_round(q, p, mode)`; the other functions correspond in the same way.
`sinhCoshPrecRound` gives the hyperbolic sine and cosine together, each with its own `Ordering`, as
`x.sinh_cosh_prec_round(p, mode)` does. The forms without a rounding mode round to nearest, at the
precision of `x` for a `Float` argument.

Every ≈ here comes from Azurite's single zero. Malachite, following MPFR, gives an exact zero
result of an odd function the sign of the argument, so $$\sinh(-0.0)$$, $$\tanh(-0.0)$$,
$$\operatorname{asinh}(-0.0)$$, and $$\operatorname{atanh}(-0.0)$$ are $$-0.0$$, as is the
hyperbolic sine in `sinh_cosh`. The functions with a pole at zero take the sign of the zero:
$$\coth(-0.0)$$, $$\operatorname{csch}(-0.0)$$, and $$\operatorname{acsch}(-0.0)$$ are
$$-\infty$$ in Malachite, where Azurite's are $$+\infty$$. And the odd functions that vanish at
infinity give $$-0.0$$ at $$-\infty$$: `csch`, `acsch`, and `acoth`. The `Rat` forms are all ✓:
a rational zero has no sign, and the two sides agree on it.

`gdPrecRound x p mode` is `x.gudermannian_prec_round(p, mode)`, the Gudermannian function
$$\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$$, and
`gdRatPrecRound q p mode` is `Float::gudermannian_rational_prec_round(q, p, mode)`. Malachite's
functions are ports of Azurite's, with the same paths: one rounding test for a tiny argument,
$$\pi/2$$ alone for a huge one, the alternating series of the arctangent when few terms suffice,
and $$2\arctan(\tanh(x/2))$$ otherwise. Malachite takes the exponential, the hyperbolic tangent,
the arctangent, and $$\pi$$ from its own correctly rounded functions instead of Azurite's
brackets, and chooses between the series and the general path by thresholds measured with them.
The results agree wherever a `Float` can hold them; the only ones it cannot hold are those of the
nonzero `Float`s closest to zero, which underflow. `gdPrecRound` is ≈ because `Float` has a signed
zero: $$\operatorname{gd}(\pm 0.0) = \pm 0.0$$. The same functions exist for `BigExponentFloat`.

`gdInvPrecRound x p mode` is `x.inverse_gudermannian_prec_round(p, mode)`, the inverse
Gudermannian function
$$\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1 + \sin x)/\cos x)$$ for
$$|x| < \pi/2$$, and `gdInvRatPrecRound q p mode` is
`Float::inverse_gudermannian_rational_prec_round(q, p, mode)`. Both sides give `NaN` outside the
domain, which includes $$\pm\infty$$, and decide $$|x| < \pi/2$$ exactly. Malachite's functions
are ports of Azurite's: one rounding test for a tiny argument, and otherwise a sine and cosine
computed together and one logarithm, taken from Malachite's own correctly rounded functions
instead of Azurite's brackets. A `Float` result never overflows or underflows, since
$$|\operatorname{gd}^{-1}(x)| \geq |x|$$; the `Rat` form underflows for a rational close enough
to zero. `gdInvPrecRound` is ≈ because `Float` has a signed zero:
$$\operatorname{gd}^{-1}(\pm 0.0) = \pm 0.0$$. The same functions exist for `BigExponentFloat`.

## The gamma function {#the-gamma-function}

| | Azurite | Malachite |
| :---: | --- | --- |
| ≈ | `gammaPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `gamma` | [`gamma_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.gamma_prec_round), [`gamma_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.gamma_prec), [`gamma_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.gamma_round), [`Gamma`](https://docs.rs/malachite-base/latest/malachite_base/num/gamma/traits/trait.Gamma.html), [`GammaAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/gamma/traits/trait.GammaAssign.html) |
| ✓ | `gammaRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `gammaRat` | [`gamma_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.gamma_rational_prec_round), [`gamma_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.gamma_rational_prec) |
| ≈ | `reciprocalGammaPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `reciprocalGamma` | [`reciprocal_gamma_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.reciprocal_gamma_prec_round), [`reciprocal_gamma_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.reciprocal_gamma_prec), [`reciprocal_gamma_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.reciprocal_gamma_round), [`ReciprocalGamma`](https://docs.rs/malachite-base/latest/malachite_base/num/gamma/traits/trait.ReciprocalGamma.html), [`ReciprocalGammaAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/gamma/traits/trait.ReciprocalGammaAssign.html) |
| ✓ | `reciprocalGammaRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `reciprocalGammaRat` | [`reciprocal_gamma_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.reciprocal_gamma_rational_prec_round), [`reciprocal_gamma_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.reciprocal_gamma_rational_prec) |
| ✓ | `lnGammaPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `lnGamma` | [`ln_gamma_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_gamma_prec_round), [`ln_gamma_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_gamma_prec), [`ln_gamma_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_gamma_round), [`LnGamma`](https://docs.rs/malachite-base/latest/malachite_base/num/gamma/traits/trait.LnGamma.html), [`LnGammaAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/gamma/traits/trait.LnGammaAssign.html) |
| ✓ | `lnGammaRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `lnGammaRat` | [`ln_gamma_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_gamma_rational_prec_round), [`ln_gamma_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_gamma_rational_prec) |
| ≈ | `lnAbsGammaPrecRound (x : AzFloat) (p : Nat) (mode : RoundingMode) : (AzFloat × Ordering) × Bool`, `lnAbsGamma` | [`ln_abs_gamma_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_abs_gamma_prec_round), [`ln_abs_gamma_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_abs_gamma_prec), [`ln_abs_gamma_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_abs_gamma_round), [`LnAbsGamma`](https://docs.rs/malachite-base/latest/malachite_base/num/gamma/traits/trait.LnAbsGamma.html), [`LnAbsGammaAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/gamma/traits/trait.LnAbsGammaAssign.html) |
| ✓ | `lnAbsGammaRatPrecRound (q : AzRat) (p : Nat) (mode : RoundingMode) : (AzFloat × Ordering) × Bool`, `lnAbsGammaRat` | [`ln_abs_gamma_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_abs_gamma_rational_prec_round), [`ln_abs_gamma_rational_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_abs_gamma_rational_prec) |
| ✗ | `gammaOneQuarterPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `gammaOneQuarter`, `gammaOneThirdPrecRound (p : Nat) (mode : RoundingMode) : AzFloat × Ordering`, `gammaOneThird` | |
| — | `gammaPrecRoundFuel`, `reciprocalGammaPrecRoundFuel`, `gammaRatPrecRoundFuel`, `reciprocalGammaRatPrecRoundFuel`, `gammaFuel`, `gammaRatCertified` | |
| — | `lnGammaPrecRoundFuel`, `lnGammaRatPrecRoundFuel`, `lnAbsGammaPrecRoundFuel`, `lnAbsGammaRatPrecRoundFuel`, `lnGammaFuel` | |
| — | `lnGammaCancelBits`, `lnGammaRatCancelBits`, `lnGammaApproxOf`, `lnGammaApprox`, `lnGammaRatBracket`, `lnGammaRatApprox`, `lnAbsGammaReflect`, `lnAbsGammaNegEncl`, `lnAbsGammaApproxOf`, `lnAbsGammaApprox`, `gammaSign`, `lnAbsGammaRatNegEncl`, `lnAbsGammaRatApprox`, `gammaRatSign` | |
| — | `lnGammaShiftExp`, `lnGammaShift`, `lnGammaTermsGo`, `lnGammaTerms`, `risingEncl`, `stirlingTermEncl`, `stirlingSumLoop`, `stirlingMuEncl`, `lnGammaStirlingEncl`, `Encl.half`, `lnGammaBracketEncl`, `lnGammaBracket`, `lnGammaRatEncl`, `gammaLnBits`, `expEnclPos?`, `gammaExpEncl`, `gammaNearInt`, `gammaFracEncl`, `sinOfEncl`, `sinLostBits`, `gammaSinOfFrac`, `gammaSinEncl`, `gammaReflect`, `gammaCore`, `gammaStrict`, `gammaApproxOf`, `gammaApprox`, `gammaTiny?`, `reciprocalGammaTiny?`, `gammaIntBound`, `gammaSmallInt`, `gammaSpecial`, `isQuarter`, `gammaRatExp`, `gammaRatSinEncl`, `gammaRatCore`, `gammaRatApprox`, `gammaRatSpecial`, `gammaRatSmallInt`, `gammaRatIsTiny`, `gammaRatTiny?`, `reciprocalGammaRatTiny?`, `gammaRatStart`, `isUnitFraction`, `gammaAP`, `gammaOneSixthEncl`, `gammaBaseEncl`, `gammaSpecialEncl`, `gammaSpecialApprox`, `gammaSpecialFast?` | |

`gammaPrecRound x p mode` is `x.gamma_prec_round(p, mode)`, the gamma function $$\Gamma(x)$$, and
`reciprocalGammaPrecRound x p mode` is `x.reciprocal_gamma_prec_round(p, mode)`, its reciprocal
$$1/\Gamma(x)$$, which is defined for every real $$x$$; `gammaRatPrecRound q p mode` and
`reciprocalGammaRatPrecRound q p mode` are `Float::gamma_rational_prec_round(q, p, mode)` and
`Float::reciprocal_gamma_rational_prec_round(q, p, mode)`. The forms without a rounding mode round
to nearest, at the precision of `x` for a `Float` argument. Both sides give `NaN` for $$\Gamma$$ at
the negative integers and at $$-\infty$$, and 0 for $$1/\Gamma$$ at the negative integers.

Malachite's functions are ports of Azurite's, with the same paths: the exact factorial for a
positive integer up to $$4p + 1500$$ (Azurite's bound is $$8p + 2048$$); one rounding test for a tiny argument, where $$\Gamma(x)$$ is
within 2 of $$1/x$$ and $$1/\Gamma(x)$$ lies between $$x$$ and $$x + x^2$$; and otherwise Ziv's loop
on the exponential of an enclosure of $$\ln\Gamma$$, which comes from Stirling's series (with
coefficients from the tangent numbers) after a shift of the argument, with the reflection formula
for a negative argument. Malachite takes the logarithm, the exponential, the sine, and $$\pi$$ from
its own correctly rounded functions instead of Azurite's brackets. It also evaluates Stirling's
formula differently. Where Azurite carries an enclosure through every operation, Malachite computes
each quantity once, rounding to nearest, and keeps a bound for its error, which gives the enclosure
at the end. Azurite takes the logarithm of the shifting product, to enclose $$\ln\Gamma$$, and
exponentiates the sum; Malachite exponentiates the rest of the formula and divides by the product,
which saves a logarithm. The threshold up to which the argument is shifted is $$3w/8$$ at working
precision $$w$$, and larger from about 1250 bits on, where Azurite's is the power of 2 in $$(w/4,
w/2]$$. And the tangent numbers are computed one at a time, as the series asks for them. The results
are the same.

There are two differences in how the loop is run. Azurite cannot prove that Ziv's loop terminates
for every argument, since that would take $$\Gamma(x)$$ not to be a rounding boundary, so its loop
has an iteration limit (the `Fuel` functions take it as an argument), and its theorems say
"correctly rounded, or `NaN`". Malachite's loop has no limit: it always returns the correctly
rounded value, and would run forever at a counterexample, which is not known or expected to exist.
And where $$\Gamma(x)$$ is proven irrational, at the multiples of $$1/4$$ and, for a rational
argument, the denominators 2, 3, 4, and 6, Azurite evaluates a closed form in $$\sqrt\pi$$,
$$\Gamma(1/4)$$, and $$\Gamma(1/3)$$, constants that it computes with the arithmetic-geometric mean
(`gammaOneQuarterPrecRound` and `gammaOneThirdPrecRound`). Malachite does not have these constants
yet, and uses the general loop there; the results are the same.

A `Float` result overflows for an argument above about $$4.5\times10^7$$ and underflows for a
negative one of that size that is not an integer; the reciprocal does the opposite. From $$\pm
2^{26}$$ on Malachite decides this without computing the value, which Azurite, with its unbounded
exponent, does compute. `gammaPrecRound` is ≈ because `Float` has a signed zero and Malachite,
following MPFR, gives $$\Gamma(\pm 0.0) = \pm\infty$$, where Azurite gives $$+\infty$$; and
`reciprocalGammaPrecRound` is ≈ because $$1/\Gamma(\pm 0.0) = \pm 0.0$$. The `Rat` forms are ✓: a
rational zero has no sign, and both sides give $$+\infty$$ and 0 there. The same functions exist for
`BigExponentFloat`, which has no negative zero and gives `NaN` for $$\Gamma(0)$$, as it does at the
other poles.

`lnGammaPrecRound x p mode` is `x.ln_gamma_prec_round(p, mode)`, the logarithm $$\ln\Gamma(x)$$ of
the gamma function for $$x > 0$$, and `lnAbsGammaPrecRound x p mode` is
`x.ln_abs_gamma_prec_round(p, mode)`, which gives $$\ln|\Gamma(x)|$$ for every $$x$$ together with
the sign of $$\Gamma(x)$$; `lnGammaRatPrecRound q p mode` and `lnAbsGammaRatPrecRound q p mode` are
`Float::ln_gamma_rational_prec_round(q, p, mode)` and
`Float::ln_abs_gamma_rational_prec_round(q, p, mode)`. Azurite returns `((r, o), sign)` and Malachite
`(r, sign, o)`; on both sides the sign is `true` for a positive $$\Gamma(x)$$, and where the gamma
function has no sign. Both sides give exactly 0 at 1 and 2, $$+\infty$$ at 0 and at $$+\infty$$, and,
for `lnGamma`, `NaN` at every negative argument and at $$-\infty$$; `lnAbsGamma` is $$+\infty$$ at the
negative integers and at $$\pm\infty$$.

These are ports too, with the same structure: Ziv's loop on the bracket of $$\ln\Gamma$$, started
with as many extra bits as are lost next to the zeros 1 and 2; and, for a negative argument, the
reflection formula $$\ln|\Gamma(x)| = \ln(\pi/|\sin \pi x|) - \ln\Gamma(1-x)$$, with the sine
taken at the distance to the nearest integer. The bracket is the one described above, computed with
single values and error bounds; here the constant and the shifting product do share a logarithm, as
in Azurite. As with the gamma function, Malachite's loop has no iteration limit. One path is
Malachite's own: for a positive integer up to $$4p + 1500$$, the gamma function's bound, the result
is the correctly rounded logarithm of the exact factorial, which is several times as fast as the
series there. With the
`std` feature Malachite also keeps the tangent numbers from one call to the next, per thread;
`Float::clear_constant_cache` frees them.

`lnAbsGammaPrecRound` is ≈ because `Float` has a signed zero: Malachite, following MPFR's
`mpfr_lgamma`, takes $$\Gamma(-0.0) = -\infty$$ and gives the sign `false` there, where Azurite's
zero gives `true`. A `Float` result overflows for an argument above about $$2^{2^{30}-30.5}$$. The
same functions exist for `BigExponentFloat`, where they never overflow.

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

`AzFloat` is the newest Azurite type, and the Malachite side of this page is still wider than the
Azurite side. Roots other than square and cube roots, `hypot`, `average`, the positive
difference, `compound`, `powr`, the IEEE remainder and the remainder with quotient bits, the
integer and fractional parts, the Gaussian conversions, exhaustive and random generation, and the
rest of the [MPFR page](/mapping/mpfr-floats/) have no `AzFloat` counterpart yet. They will be
added here as Azurite gains them.

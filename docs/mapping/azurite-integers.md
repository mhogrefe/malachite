---
layout: default
title: "Malachite for Azurite Users: Integers"
permalink: /mapping/azurite-integers/
theme: jekyll-theme-slate
---

# Malachite for Azurite Users: Integers

This page maps the operations of [Azurite](https://github.com/mhogrefe/azurite)'s `AzInt` type,
its signed multi-limb integer, onto their Malachite counterparts on
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), from
the `malachite-nz` crate. It is the companion of
[Malachite for Azurite Users: Naturals](/mapping/azurite-naturals/), which covers `AzNat` and
whose [Conventions](/mapping/azurite-naturals/#conventions) (total functions, the `toNat` bridge,
`Nat` counts, rounding modes) carry over unchanged; only what is new for signed integers is
repeated here. The page covers Azurite as of commit `3ab0b03` (2026-10-02). The
[mapping index](/mapping/) lists the whole family of pages.

As on the naturals page, the sections are organized by theme, since Azurite's operations are
definitions spread across the files of `Azurite/AzInt/`.

## Conventions {#conventions}

### The types

An `AzInt` is a sign and an `AzNat` magnitude, with a proof that a zero magnitude carries the
positive sign, so there is exactly one zero. An
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) is the
same construction: a sign and a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
magnitude, never a negative zero. Both are canonical, so equality is structural on both sides, and
both expose the pair: `z.sign` and `z.abs` in Azurite,
[`Integer::from_sign_and_abs`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_sign_and_abs)
and
[`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html)
in Malachite, with `true` meaning non-negative in both.

### Division

This is the one place the two libraries' *default* operations differ. Azurite's `/` and `%` on
`AzInt` are Euclidean, as Lean's are on `Int`: the remainder is always nonnegative, $$0 \leq r <
|b|$$. Malachite's `/` and `%` truncate toward zero, as Rust's do on the primitive integers, and
its Euclidean division is the named
[`DivEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivEuclidean.html)
family. So `AzInt.div` maps to `div_euclidean`, not to `/`, and the tables below say so row by
row. Azurite also provides floor division (`fdiv`), which is Malachite's
[`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html)
family; it has no truncating or ceiling division. Right shifts floor on both sides. A zero divisor
makes every Azurite division return `(0, a)`, Lean's convention; Malachite panics.

### Typeclasses and traits

`AzInt` is a verified Mathlib `CommRing` and `IsDomain`, with `Neg`, `Div`, `Mod`, `HShiftLeft`,
`HShiftRight`, `Ord`, `ToString`, and `OfNat` literals, and `NatCast`/`IntCast` routed through the
limb-level constructors. Malachite has no ring abstraction; the operators and the
[`malachite_base`](https://docs.rs/malachite-base/latest/malachite_base/) traits stand in, as on
the naturals page. Azurite's `ExactDiv` and `NormalizedGcd` typeclass instances correspond to
Malachite's
[`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html)
and
[`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html)
traits.

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
| ✓ | `instance : OfNat AzInt 0`, `instance : Zero AzInt` | [`Integer::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| ✓ | `instance : OfNat AzInt 1`, `instance : One AzInt` | [`Integer::ONE`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.One.html) |
| ✓ | `mkNorm (s : Bool) (a : AzNat) : AzInt` | [`from_sign_and_abs`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_sign_and_abs) |
| ✓ | `mkNonzero (s : Bool) (a : AzNat) (h : a ≠ 0) : AzInt` | [`from_sign_and_abs`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_sign_and_abs) |
| ✓ | `AzNat.toAzInt (n : AzNat) : AzInt` | [`From<Natural>`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_natural/index.html) |
| ✓ | `natAbs (z : AzInt) : AzNat`, `AzInt.abs : AzNat` | [`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html) |
| ✓ | `AzInt.sign : Bool` | [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html), [`PartialOrd<u32>`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/partial_cmp_primitive_int/index.html) |
| — | `AzInt.ofInt (i : Int) : AzInt`, `AzInt.toInt (z : AzInt) : Int` | |
| ✓ | `UInt64.toAzInt (u : UInt64) : AzInt`, `UInt32.toAzInt`, `UInt16.toAzInt`, `UInt8.toAzInt`, `USize.toAzInt` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_int/index.html) |
| ✓ | `Int64.toAzInt (i : Int64) : AzInt`, `Int32.toAzInt`, `Int16.toAzInt`, `Int8.toAzInt`, `ISize.toAzInt` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_int/index.html) |
| ✓ | `toUInt64 (z : AzInt) : UInt64`, `toUInt32`, `toUInt16`, `toUInt8`, `toUSize` | [`WrappingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.WrappingFrom.html) |
| ✓ | `toInt64 (z : AzInt) : Int64`, `toInt32`, `toInt16`, `toInt8`, `toISize` | [`WrappingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.WrappingFrom.html) |

**Sign and magnitude.** `mkNorm s a` builds the integer with sign `s` and magnitude `a`, forcing
the sign positive when `a` is zero, which is exactly `Integer::from_sign_and_abs(s, a)`;
`mkNonzero` is the same constructor with a proof that the normalization is not needed. The
magnitude comes back as `z.abs` or `natAbs`, which is `z.unsigned_abs()` (or
[`unsigned_abs_ref`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.unsigned_abs_ref)
to borrow it). `z.sign` is a `Bool`; Malachite's `sign()` is an `Ordering` against zero, and
`z >= 0u32` is the direct spelling of the `Bool`.

**`ofInt`, `toInt`.** The bridge to Lean's `Int`, the specification side of Azurite's proofs; it
has no counterpart, as `toNat` has none [on the naturals page](/mapping/azurite-naturals/#proofs).

**Machine integers.** The conversions from any fixed-width type are `Integer::from(x)`. Toward
them, `toUInt64` returns the value modulo $$2^{64}$$ (a negative integer's two's complement) and
`toInt64` reinterprets those bits, so all ten are `u64::wrapping_from(&z)`,
`i64::wrapping_from(&z)`, and so on; Malachite's
[`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_int_from_integer/index.html),
[`SaturatingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.SaturatingFrom.html),
and
[`OverflowingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.OverflowingFrom.html)
give the exact, clamped, and flagged alternatives.

## Comparison {#comparison}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `compare (a b : AzInt) : Ordering`, `instance : Ord AzInt`, `LE`, `LT` | [`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html), [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `deriving DecidableEq` | [`Eq`](https://doc.rust-lang.org/nightly/std/cmp/trait.Eq.html), [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ✓ | `instance : Max AzInt`, `instance : Min AzInt` | [`Ord::max`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html#method.max), [`Ord::min`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html#method.min) |
| ✓ | `AzInt.compareUInt64 (z : AzInt) (u : UInt64) : Ordering` | [`PartialOrd<u64>`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/partial_cmp_primitive_int/index.html) |
| ✓ | `AzInt.compareInt64 (z : AzInt) (i : Int64) : Ordering` | [`PartialOrd<i64>`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/partial_cmp_primitive_int/index.html) |
| ✓ | `AzInt.compareAzNat (z : AzInt) (a : AzNat) : Ordering` | [`PartialOrd<Natural>`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/partial_cmp_natural/index.html) |
| ✓ | `AzInt.beqUInt64 (z : AzInt) (u : UInt64) : Bool`, `AzInt.beqInt64` | [`PartialEq<u64>`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/partial_eq_primitive_int/index.html), [`PartialEq<i64>`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/partial_eq_primitive_int/index.html) |
| ✓ | `AzInt.beqAzNat (z : AzInt) (a : AzNat) : Bool` | [`PartialEq<Natural>`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/partial_eq_natural/index.html) |

**Mixed comparisons.** Azurite compares an `AzInt` against the two 64-bit machine types and
against `AzNat`; Malachite against every primitive integer and float and against `Natural`, in
both argument orders. Malachite also orders by magnitude
([`OrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.OrdAbs.html)
and its mixed-type variants), which Azurite spells as a comparison of `natAbs` values.

## Addition, subtraction, multiplication, and negation {#addition-subtraction-multiplication-and-negation}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `add (a b : AzInt) : AzInt`, `instance : Add AzInt` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html), [`AddAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.AddAssign.html) |
| ✓ | `addUInt64 (z : AzInt) (u : UInt64) : AzInt`, `addInt64 (z : AzInt) (i : Int64) : AzInt` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html) |
| ✓ | `sub (a b : AzInt) : AzInt`, `instance : Sub AzInt` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html), [`SubAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.SubAssign.html) |
| ✓ | `subUInt64 (z : AzInt) (u : UInt64) : AzInt`, `subInt64 (z : AzInt) (i : Int64) : AzInt` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html) |
| ✓ | `mul (a b : AzInt) : AzInt`, `instance : Mul AzInt` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.MulAssign.html) |
| ✓ | `mulUInt64 (z : AzInt) (u : UInt64) : AzInt`, `mulInt64 (z : AzInt) (i : Int64) : AzInt` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html) |
| ✓ | `neg (z : AzInt) : AzInt`, `instance : Neg AzInt` | [`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html), [`NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegAssign.html) |

**The machine-word variants.** As [on the GMP page](/mapping/gmp-integers/#conventions), Malachite
has no mixed `Integer`-and-word arithmetic; `z + Integer::from(i)` is the spelling, and the
conversion allocates nothing. Subtraction never truncates here, both types being signed.

## Division {#division}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `AzInt.edivMod (a b : AzInt) : AzInt × AzInt`, `AzInt.divMod` | [`DivModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivModEuclidean.html) |
| ✓ | `AzInt.ediv (a b : AzInt) : AzInt`, `AzInt.div`, `instance : Div AzInt` | [`DivEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivEuclidean.html) |
| ✓ | `AzInt.emod (a b : AzInt) : AzInt`, `AzInt.mod`, `instance : Mod AzInt` | [`ModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModEuclidean.html) |
| ✓ | `AzInt.fdivMod (a b : AzInt) : AzInt × AzInt` | [`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html) |
| ✓ | `AzInt.fdiv (a b : AzInt) : AzInt` | [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html), [`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html) |
| ✓ | `AzInt.fmod (a b : AzInt) : AzInt` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ✓ | `AzInt.divRound (a b : AzInt) (mode : RoundingMode) : AzInt × Ordering` | [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html) |
| ✓ | `instance : ExactDiv AzInt` | [`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html) |

**Euclidean division.** `edivMod a b` returns $$(q, r)$$ with $$a = qb + r$$ and $$0 \leq r <
|b|$$, and `div`/`mod` and the `/` and `%` operators are the same operation under Lean's names. In
Malachite these are `a.div_mod_euclidean(b)`, `a.div_euclidean(b)`, and `a.mod_euclidean(b)`;
Malachite's own `/` and `%` round the quotient toward zero instead
([`DivRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRem.html)
and
[`Rem`](https://doc.rust-lang.org/nightly/std/ops/trait.Rem.html)),
so `-7 / 2` is `-4` in Azurite and `-3` in Malachite. The two agree whenever `a` is nonnegative.

**Floor division.** `fdivMod` rounds the quotient toward $$-\infty$$, the remainder taking the
divisor's sign, which is `a.div_mod(b)` and `a.mod_op(b)`; `fdiv` alone is `a.div_round(b, Floor)`
or the first component of `div_mod`. Malachite's ceiling family
([`CeilingDivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingDivMod.html))
and truncating family have no Azurite names; `divRound` with `Ceiling` or `Down` reaches the
quotients.

**`divRound`.** Both return the quotient rounded in the given mode and its `Ordering` against the
exact quotient, with ties to even under `Nearest`; Azurite implements the negative case by
rounding the magnitudes with the reflected mode, which is the same rule Malachite applies. A zero
divisor is unspecified in Azurite and panics in Malachite.

**Exact division.** Azurite's `ExactDiv` instance is `div` under the assumption that it is exact;
`a.div_exact(b)` makes the same assumption.

## Shifts {#shifts}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `shiftLeft (z : AzInt) (sh : Nat) : AzInt`, `instance : HShiftLeft AzInt Nat AzInt` | [`Shl`](https://doc.rust-lang.org/nightly/std/ops/trait.Shl.html), [`ShlAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.ShlAssign.html) |
| ✓ | `shiftRight (z : AzInt) (sh : Nat) : AzInt`, `instance : HShiftRight AzInt Nat AzInt` | [`Shr`](https://doc.rust-lang.org/nightly/std/ops/trait.Shr.html), [`ShrAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.ShrAssign.html) |
| ✓ | `AzInt.shiftRightRound (z : AzInt) (mode : RoundingMode) (sh : Nat) : AzInt × Ordering` | [`ShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ShrRound.html) |

**Rounding of right shifts.** Both `z >>> sh` and `z >> sh` floor, so a negative value's
magnitude rounds away from zero: $$-7 \gg 1 = -4$$ on both sides. `shiftRightRound` and `shr_round`
take the mode explicitly and return the `Ordering`, with the argument order swapped. Shift amounts
are `Nat` in Azurite and any primitive integer in Malachite, a negative count reversing the
direction.

## Bits {#bits}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `AzInt.size (z : AzInt) : Nat` | [`SignificantBits`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.SignificantBits.html) |
| ✓ | `AzInt.trailingZeros (z : AzInt) : Option Nat` | [`trailing_zeros`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.trailing_zeros) |
| ✓ | `AzInt.pow2 (k : Nat) : AzInt` | [`PowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowerOf2.html) |
| ✓ | `AzInt.lowMask (k : Nat) : AzInt` | [`LowMask`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.LowMask.html) |
| ✓ | `AzInt.isPowerOfTwo (z : AzInt) : Bool` | [`IsPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.IsPowerOf2.html) |
| ✓ | `AzInt.isEven (z : AzInt) : Bool`, `AzInt.isOdd` | [`Parity`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Parity.html) |

**Magnitude-based bits.** `size` and `significant_bits` both count the bits of $$|z|$$, 0 for
zero; `trailingZeros` and `trailing_zeros` both count from the low end of $$|z|$$ and are `None`
for zero; `isPowerOfTwo` and `is_power_of_2` are false for zero and for every negative value.
Malachite's two's-complement bit operations have no counterpart on `AzInt`: single-bit access
([`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html)),
which reads a negative value as an infinite string of leading ones, the logical operators, bit
blocks, bit scans, population counts, and the conversions to and from bits. The `AzNat` bit
accessors (`testBit`, `setBit`, `clearBit`, `getBits`) apply to the magnitude `z.abs`, which in
Malachite is `z.unsigned_abs()` followed by the `Natural` operation.

## Powers and GCD {#powers-and-gcd}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `pow (z : AzInt) (n : ℕ) : AzInt`, the `CommRing` `npow` | [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html) |
| ✓ | `instance : NormalizedGcd AzInt` | [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html) |
| ≈ | `egcd (a b : AzNat) : AzNat × AzInt × AzInt` | [`ExtendedGcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ExtendedGcd.html) |

**`pow`.** Exact exponentiation, the sign following the exponent's parity; Malachite's `pow` takes
a `u64` exponent.

**GCD.** The `NormalizedGcd` instance is the nonnegative GCD of the magnitudes, which is what
`a.gcd(b)` returns for two `Integer`s. `egcd a b` returns $$(g, s, t)$$ with $$sa + tb = g$$ for
two `AzNat`s, which is `Natural::extended_gcd` with its `(Natural, Integer, Integer)` result; the
GCDs agree, but the Bézout coefficients need not, hence ≈. Azurite's come from the extended
binary algorithm and are not further normalized, while Malachite's follow GMP's rule: $$|s| \leq
b/g$$ and $$|t| \leq a/g$$, with $$(1, 0)$$ and $$(0, 1)$$ for the cases where one argument
divides the other and $$(0, 0, 0)$$ for two zeros. Code that needs *a* Bézout pair can use
either; code that compares the pairs needs the normalization.

## Strings {#strings}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `toString (z : AzInt) : String`, `instance : ToString AzInt` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |
| ≈ | `parse (s : String) : Option AzInt` | [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) |
| — | `instance : ParsableElement AzInt` | |

**Formatting and parsing.** Both print decimal with a leading `-` for a negative value and no
sign otherwise. `parse` reads an optional `-` and then the magnitude with `AzNat.parse`'s rules, so
a `0b`, `0o`, or `0x` prefix selects the base, and it rejects `"-0"`; `Integer::from_str` reads an
optional sign and decimal digits only, and accepts `"-0"` as zero. Malachite's other bases are
[`FromStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromStringBase.html)
and
[`ToStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToStringBase.html),
for which `AzInt` has no counterpart yet. `ParsableElement` is Azurite's typeclass for parsing a
coefficient inside a vector, matrix, or polynomial literal.

## Exhaustive generation {#exhaustive-generation}

The `AzInt` instances of Azurite's `ExhaustiveGenerator` typeclass produce the same sequences as
Malachite's functions, as the `AzNat` ones do
[on the naturals page](/mapping/azurite-naturals/#exhaustive-generation).

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `instance integersGen : ExhaustiveGenerator AzInt` | [`exhaustive_integers`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/exhaustive/fn.exhaustive_integers.html) |
| ✓ | `instance nonnegativeIntegersGen : ExhaustiveGenerator {z : AzInt // 0 ≤ z}` | [`exhaustive_natural_integers`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/exhaustive/fn.exhaustive_natural_integers.html) |
| ✓ | `instance positiveIntegersGen : ExhaustiveGenerator {z : AzInt // 0 < z}` | [`exhaustive_positive_integers`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/exhaustive/fn.exhaustive_positive_integers.html) |
| ✓ | `instance negativeIntegersGen : ExhaustiveGenerator {z : AzInt // z < 0}` | [`exhaustive_negative_integers`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/exhaustive/fn.exhaustive_negative_integers.html) |
| ✓ | `azIntIncreasingRangeGen (a b : AzInt) : ExhaustiveGenerator {x : AzInt // a ≤ x ∧ x < b}` | [`integer_increasing_range`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/exhaustive/fn.integer_increasing_range.html) |
| ✓ | `azIntIncreasingRangeInclusiveGen (a b : AzInt) : ExhaustiveGenerator {x : AzInt // a ≤ x ∧ x ≤ b}` | [`integer_increasing_inclusive_range`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/exhaustive/fn.integer_increasing_inclusive_range.html) |
| ✓ | `azIntIncreasingRangeToInfinityGen (a : AzInt) : ExhaustiveGenerator {x : AzInt // a ≤ x}` | [`integer_increasing_range_to_infinity`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/exhaustive/fn.integer_increasing_range_to_infinity.html) |
| ✓ | `azIntDecreasingRangeToNegativeInfinityGen (b : AzInt) : ExhaustiveGenerator {x : AzInt // x ≤ b}` | [`integer_decreasing_range_to_negative_infinity`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/exhaustive/fn.integer_decreasing_range_to_negative_infinity.html) |
| ✓ | `azIntRangeGen (a b : AzInt) : ExhaustiveGenerator {x : AzInt // a ≤ x ∧ x < b}` | [`exhaustive_integer_range`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/exhaustive/fn.exhaustive_integer_range.html) |
| ✓ | `azIntRangeInclusiveGen (a b : AzInt) : ExhaustiveGenerator {x : AzInt // a ≤ x ∧ x ≤ b}` | [`exhaustive_integer_inclusive_range`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/exhaustive/fn.exhaustive_integer_inclusive_range.html) |

**Orders.** The whole-type generator zig-zags outward from zero, $$0, 1, -1, 2, -2, \ldots$$, as
`exhaustive_integers` does, with the positive member of each pair first; the one-sided generators
count away from zero. The `Increasing` ranges ascend and the `Decreasing` one descends, while
`azIntRangeGen` and `azIntRangeInclusiveGen` run by increasing magnitude, positive first, which
is the order of `exhaustive_integer_range`. Azurite's random generators produce Lean `Int`s rather
than `AzInt`s, so they are not mapped here.

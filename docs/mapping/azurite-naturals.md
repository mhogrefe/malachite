---
layout: default
title: "Malachite for Azurite Users: Naturals"
permalink: /mapping/azurite-naturals/
theme: jekyll-theme-slate
---

# Malachite for Azurite Users: Naturals

This page maps the operations of [Azurite](https://github.com/mhogrefe/azurite)'s `AzNat` type,
its multi-limb natural number, onto their Malachite counterparts on
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), from
the `malachite-nz` crate. Azurite is a Lean 4 library of formally verified arithmetic: every
`AzNat` operation comes with a proof that it agrees with the corresponding operation on Lean's
`Nat`, and through Mathlib with the mathematical definition. It shares an author with Malachite,
which is why its generator machinery below is a close relative of Malachite's. The page covers
Azurite as of commit `3ab0b03` (2026-10-02). The [mapping index](/mapping/) lists the whole
family of pages.

Azurite has no manual whose organization a mapping can follow: its operations are definitions
spread across the files of `Azurite/AzNat/`, together with the typeclass instances that make
`AzNat` a Mathlib semiring. The sections below are therefore organized by theme. Definitions that
exist only to be benchmarked or tuned, and the limb-level helpers beneath the public operations,
are listed only where it is useful to say that they have no counterpart.

## Conventions {#conventions}

### The types

An `AzNat` is a structure holding an `Array UInt64` of limbs, least significant first, together
with a proof that the last limb is nonzero; zero is the empty array. A
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) stores
values below $$2^{64}$$ inline with no allocation and larger ones as a vector of limbs, also least
significant first with no high zero limb, as described
[on the GMP integers page](/mapping/gmp-integers/#allocation). Both representations are canonical,
so equality is structural on both sides and no normalization caveats appear below. `AzNat`'s limbs
are always 64 bits; Malachite's are 64 bits by default and 32 bits under the `32_bit_limbs`
feature, which changes the meaning of the limb-level rows but nothing else.

### Total functions

Lean functions are total, so where Malachite panics or returns an `Option`, `AzNat` returns a
value by convention: `divMod x 0 = (0, x)`, `sub` truncates to zero, `toUInt64` keeps the low
limb, `toStringBaseWith` returns `""` for a base outside `[2, 36]`. Other operations state a
precondition in their documentation (`jacobi` for an odd denominator, `invMod` for coprime
arguments) and leave the result unspecified outside it; Malachite panics there, or returns `None`.
The page treats these pairs as ✓ when the two agree wherever Azurite defines the result. The ≈
mark is reserved for operations that return different values on inputs both libraries accept, or
that one library accepts and the other rejects *with a documented different answer*; the notes
give the one-line adjustment in each case.

### Proofs

Azurite's specification is its theorems: `toNat (a + b) = toNat a + toNat b`, and so on for every
operation, where `toNat : AzNat → Nat` is the bridge to Lean's natural numbers and `ofNat` its
inverse. Malachite's specification is its documentation and tests. The bridge itself has no
counterpart: `Natural` is not implemented in terms of another natural-number type, and the nearest
thing to `toNat` is `Natural` itself. The equivalence lemmas in `Azurite/AzNat/Equiv/` are not
mapped.

### Typeclasses and traits

`AzNat` reaches much of its API through Lean and Mathlib typeclasses: `Add`, `Mul`, `Div`, `Mod`,
`HShiftLeft`, `Ord`, `Max`, `Min`, `ToString`, `OfNat` for literals, and a verified `CommSemiring`
instance whose `npow` is the sliding-window power and whose `natCast` is the limb-level `ofNat`.
Malachite reaches its API through Rust's operator traits and the traits of
[`malachite_base`](https://docs.rs/malachite-base/latest/malachite_base/), one per operation;
there is no semiring abstraction, and `n • a` is `Natural::from(n) * a`. Azurite also defines a
`Square` typeclass whose `AzNat` instance is the dedicated squaring routine, which corresponds to
Malachite's
[`Square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html)
trait exactly.

### Counts and exponents

Azurite takes bit indices, shift amounts, exponents, and digit counts as Lean's unbounded `Nat`.
Malachite takes them as `u64` (and shift amounts as any primitive integer, a negative count
reversing the direction). A count that does not fit in a `u64` describes a value that does not fit
in memory, so these rows are ✓.

### Rounding

Azurite's `RoundingMode` has `Floor`, `Ceiling`, `Down`, `Up`, and `Nearest`, with `Nearest`
breaking ties toward the even result, and its rounding operations return an `Ordering` recording
whether the rounded value is below, equal to, or above the exact one. Malachite's
[`RoundingMode`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html)
is the same five modes with the same tie rule, returning the same `Ordering`, plus `Exact`, which
panics if any rounding would be needed.

### Tuning parameters and forced algorithms

`AzNat` exposes its algorithm selection: `divModWith threshold`, `mulWithThresholds`, and the
`mulKaratsuba`, `mulToomCook3`, `squareSchoolbook`, `fftMul` family force one algorithm for
benchmarking and tuning. Malachite's thresholds are compile-time constants tuned per platform, and
its single-algorithm routines are internal, so these rows are —. Call the dispatching operation.

### Categories

Each definition falls into one of five categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| ⚙ | Malachite does not expose this algorithm or helper; the Malachite column or the notes say what to call instead. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## Construction and limbs {#construction-and-limbs}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `instance : OfNat AzNat 0`, `instance : Zero AzNat` | [`Natural::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| ✓ | `instance : OfNat AzNat 1`, `instance : One AzNat` | [`Natural::ONE`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.One.html) |
| — | `AzNat.ofNat (n : Nat) : AzNat` | |
| — | `AzNat.toNat (n : AzNat) : Nat` | |
| ✓ | `ofLimbs (a : Array UInt64) : AzNat` | [`from_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.from_limbs_asc) |
| ✓ | `AzNat.limbs : Array UInt64` | [`to_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_limbs_asc), [`limbs`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.limbs) |
| ✓ | `AzNat.limbs.size` | [`limb_count`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.limb_count) |
| ✓ | `AzNat.pow2 (k : Nat) : AzNat` | [`PowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowerOf2.html) |
| ✓ | `AzNat.lowMask (k : Nat) : AzNat` | [`LowMask`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.LowMask.html) |

**Literals.** `(5 : AzNat)` elaborates through `OfNat`; the Malachite spelling is
`Natural::from(5u32)`, or `Natural::const_from(5)` in a `const` context.

**`ofNat`, `toNat`.** The bridge to Lean's `Nat`, described under [Proofs](#proofs). Malachite's
conversions from and to primitive integers are the next section.

**Limbs.** Both libraries expose the limb array least significant first and with no high zero
limb. `ofLimbs` strips trailing zeros from its input as `from_limbs_asc` does, so both accept an
unnormalized array. `AzNat.limbs` is a field; Malachite offers the vector (`to_limbs_asc`) or an
iterator (`limbs`), and a descending order as well. With 32-bit limbs each 64-bit Azurite limb
corresponds to two Malachite limbs.

## Conversion {#conversion}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `UInt64.toAzNat (u : UInt64) : AzNat`, `UInt32.toAzNat`, `UInt16.toAzNat`, `UInt8.toAzNat`, `USize.toAzNat` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_int/index.html) |
| ✓ | `Int64.toAzNatClampNeg (i : Int64) : AzNat`, `Int32.toAzNatClampNeg`, `Int16.toAzNatClampNeg`, `Int8.toAzNatClampNeg`, `ISize.toAzNatClampNeg` | [`SaturatingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.SaturatingFrom.html) |
| ✓ | `toUInt64 (n : AzNat) : UInt64`, `toUInt32`, `toUInt16`, `toUInt8`, `toUSize` | [`WrappingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.WrappingFrom.html) |
| ✓ | `toInt64 (n : AzNat) : Int64`, `toInt32`, `toInt16`, `toInt8`, `toISize` | [`WrappingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.WrappingFrom.html) |

**From machine integers.** The unsigned conversions are
`Natural::from(u)` for every unsigned primitive. `toAzNatClampNeg` maps a negative signed input to
zero, which is `Natural::saturating_from(i)`; Malachite also offers `Natural::try_from(i)`, which
rejects a negative input instead.

**To machine integers.** `toUInt64` returns the low limb, that is, the value modulo $$2^{64}$$,
and the narrower conversions truncate further; `toInt64` reinterprets the low limb's bits. All of
these are `u64::wrapping_from(&n)`, `i64::wrapping_from(&n)`, and so on. Malachite's
[`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/primitive_int_from_natural/index.html),
[`SaturatingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.SaturatingFrom.html),
and
[`OverflowingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.OverflowingFrom.html)
give the exact, clamped, and flagged alternatives.

**Floats.** `AzNat` has no float conversion of its own; a natural reaches `f64` through `AzFloat`,
as `AzFloat.toFloat64 (AzFloat.ofAzNat n) mode`, which is
[`f64::rounding_from(&n, mode)`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/primitive_float_from_natural/index.html).
Azurite has no `Float32` conversion. See the
[floats page](azurite-floats.md#conversion) for `AzFloat`'s conversions.

## Comparison {#comparison}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `compare (a b : AzNat) : Ordering`, `instance : Ord AzNat`, `LE`, `LT` | [`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html), [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `deriving DecidableEq` | [`Eq`](https://doc.rust-lang.org/nightly/std/cmp/trait.Eq.html), [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ✓ | `instance : Max AzNat`, `instance : Min AzNat` | [`Ord::max`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html#method.max), [`Ord::min`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html#method.min) |
| ✓ | `AzNat.compareUInt64 (a : AzNat) (u : UInt64) : Ordering` | [`PartialOrd<u64>`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/partial_cmp_primitive_int/index.html) |
| ✓ | `AzNat.compareInt64 (a : AzNat) (i : Int64) : Ordering` | [`PartialOrd<i64>`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/partial_cmp_primitive_int/index.html) |
| ✓ | `AzNat.beqUInt64 (a : AzNat) (u : UInt64) : Bool` | [`PartialEq<u64>`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/partial_eq_primitive_int/index.html) |
| ✓ | `AzNat.beqInt64 (a : AzNat) (i : Int64) : Bool` | [`PartialEq<i64>`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/partial_eq_primitive_int/index.html) |
| ✓ | `normalizedCompare (x y : AzNat) : Ordering` | [`cmp_normalized`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.cmp_normalized) |

**Mixed comparisons.** Azurite compares against the two 64-bit machine types; Malachite against
every primitive integer and float, in both argument orders, with `a > 5u8` and `a == -3i64` both
written directly. A negative signed operand compares below every `AzNat` and every `Natural`, and
equals none.

**`normalizedCompare`.** Both compare the two values with their most significant bits aligned, as
if each were scaled by a power of 2 into $$[1, 2)$$: `normalizedCompare 5 6` and
`Natural::from(5u32).cmp_normalized(&Natural::from(6u32))` are both `Less`, since $$1.25 < 1.5$$.

## Addition, subtraction, and multiplication {#addition-subtraction-and-multiplication}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `add (a b : AzNat) : AzNat`, `instance : Add AzNat` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html), [`AddAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.AddAssign.html) |
| ✓ | `addUInt64 (a : AzNat) (b : UInt64) : AzNat` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html) |
| ✓ | `sub (a b : AzNat) : AzNat`, `instance : Sub AzNat` | [`SaturatingSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SaturatingSub.html) |
| ✓ | `subUInt64 (a : AzNat) (b : UInt64) : AzNat` | [`SaturatingSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SaturatingSub.html) |
| ✓ | `mul (a b : AzNat) : AzNat`, `instance : Mul AzNat` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.MulAssign.html) |
| ✓ | `mulUInt64 (a : AzNat) (b : UInt64) : AzNat` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html) |
| ✓ | `square (a : AzNat) : AzNat`, `instance : Square AzNat` | [`Square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html) |
| — | `mulSchoolbook`, `mulKaratsuba`, `mulToomCook3`, `mulToomCook4`, `fftMul`, `mulWithThresholds`, `MulThresholds` | |
| — | `squareSchoolbook`, `squareKaratsuba`, `squareToomCook3`, `squareToomCook4`, `fftSquare` | |

**Subtraction.** `AzNat.sub` is truncated subtraction, returning zero when `b > a`, as Lean's
`Nat` subtraction does. Malachite's `-` panics in that case; the truncating operation is
`a.saturating_sub(b)`, and
[`CheckedSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedSub.html)
returns `None` instead.

**The `UInt64` variants.** Malachite has no mixed `Natural`-and-`u64` arithmetic; as
[on the GMP page](/mapping/gmp-integers/#conventions), convert the word first,
`a + Natural::from(b)`. The conversion allocates nothing.

**Forced algorithms.** See [Tuning parameters and forced algorithms](#tuning-parameters-and-forced-algorithms).
`mul` and `square` dispatch across the same ladder Malachite's `*` and `square` use (schoolbook,
Karatsuba, Toom–Cook, FFT), with each library's own thresholds.

## Division {#division}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `divMod (U V : AzNat) : AzNat × AzNat` | [`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html) |
| ✓ | `div (U V : AzNat) : AzNat`, `instance : Div AzNat` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html) |
| ✓ | `mod (U V : AzNat) : AzNat`, `instance : Mod AzNat` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html), [`Rem`](https://doc.rust-lang.org/nightly/std/ops/trait.Rem.html) |
| ✓ | `divModUInt64 (U : AzNat) (d : UInt64) (hd : d ≠ 0) : AzNat × UInt64` | [`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html) |
| ✓ | `AzNat.divRound (x y : AzNat) (mode : RoundingMode) : AzNat × Ordering` | [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html) |
| ⚙ | `exactDivOdd (d dinv : UInt64) (n : AzNat) : AzNat` | [`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html) |
| ⚙ | `divBy6 (U : AzNat) : AzNat × UInt64` | [`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html) |
| ⚙ | `divMod10p19 (U : AzNat) : AzNat × UInt64` | [`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html) |
| — | `divModWith (threshold : Nat) (U V : AzNat)`, `divWith`, `modWith`, `divDispatchThreshold` | |

**Zero divisors.** `divMod U 0 = (0, U)` by Azurite's convention, and `divRound` with a zero
divisor is unspecified; Malachite panics on both. On every nonzero divisor the quotient and
remainder agree, and `divMod` is the floor division both libraries' `/` and `%` compute, there
being only one kind of division for naturals. `divModUInt64` carries a proof that the divisor is
nonzero; Malachite's counterpart is `U.div_mod(Natural::from(d))`, with the remainder converted
back by `u64::exact_from`.

**`divRound`.** Both return the quotient rounded in the given mode and the `Ordering` of that
quotient against the exact one, with the same tie rule, as described under
[Rounding](#rounding). Malachite's `Exact` mode has no Azurite counterpart.

**`exactDivOdd`.** A division by an odd limb `d` known to divide `n`, given `d`'s inverse modulo
$$2^{64}$$. Malachite's `n.div_exact(Natural::from(d))` makes the same divisibility assumption and
computes the inverse itself; Azurite's `inv3`, `inv5`, `inv9`, and `inv45` constants exist for the
Toom–Cook interpolation that Malachite performs in its internal exact-division routines.

**`divBy6`, `divMod10p19`.** Constant-folded specializations of single-limb division, used by the
Toom–Cook and base-10 conversion code. `U.div_mod(Natural::from(6u32))` and
`U.div_mod(Natural::from(10u64.pow(19)))` compute the same values; Malachite's own string
conversion reaches its base-$$10^{19}$$ digits internally.

## Shifts {#shifts}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `shiftLeft (a : AzNat) (sh : Nat) : AzNat`, `instance : HShiftLeft AzNat Nat AzNat` | [`Shl`](https://doc.rust-lang.org/nightly/std/ops/trait.Shl.html), [`ShlAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.ShlAssign.html) |
| ✓ | `shiftRight (a : AzNat) (sh : Nat) : AzNat`, `instance : HShiftRight AzNat Nat AzNat` | [`Shr`](https://doc.rust-lang.org/nightly/std/ops/trait.Shr.html), [`ShrAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.ShrAssign.html) |
| ✓ | `AzNat.shiftRightRound (n : AzNat) (mode : RoundingMode) (sh : Nat) : AzNat × Ordering` | [`ShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ShrRound.html) |

**Shift amounts.** Azurite's are `Nat`; Malachite's `<<` and `>>` accept any primitive integer, a
negative count shifting the other way, as described under
[Counts and exponents](#counts-and-exponents). `a >>> sh` floors, as `a >> sh` does;
`shiftRightRound` and `shr_round` apply the rounding mode and return the `Ordering`, with the
argument order swapped.

## Bits {#bits}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `AzNat.size (n : AzNat) : Nat` | [`SignificantBits`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.SignificantBits.html) |
| ✓ | `testBit (n : AzNat) (i : Nat) : Bool` | [`BitAccess::get_bit`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html) |
| ✓ | `setBit (n : AzNat) (i : Nat) : AzNat` | [`BitAccess::set_bit`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html) |
| ✓ | `clearBit (n : AzNat) (i : Nat) : AzNat` | [`BitAccess::clear_bit`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html) |
| ✓ | `AzNat.getBits (n : AzNat) (i j : Nat) : AzNat` | [`BitBlockAccess::get_bits`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitBlockAccess.html) |
| ⚙ | `AzNat.getBitsAsLimb (n : AzNat) (i j : Nat) (h : j - i ≤ 64) : UInt64` | [`BitBlockAccess::get_bits`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitBlockAccess.html) |
| ✓ | `AzNat.modPow2 (n : AzNat) (k : Nat) : AzNat` | [`ModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2.html) |
| ✓ | `AzNat.isMultipleOfPow2 (n : AzNat) (k : Nat) : Bool` | [`DivisibleByPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivisibleByPowerOf2.html) |
| ✓ | `AzNat.isPowerOfTwo (a : AzNat) : Bool` | [`IsPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.IsPowerOf2.html) |
| ✓ | `trailingZeros (n : AzNat) : Option Nat` | [`trailing_zeros`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.trailing_zeros) |
| ✓ | `AzNat.isEven (n : AzNat) : Bool`, `AzNat.isOdd` | [`Parity`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Parity.html) |

**Bit indexing.** Both index from the least significant bit at 0, and both let `setBit` grow the
number and `clearBit` or `testBit` run past its end. `getBits n i j` and `n.get_bits(i, j)` both
take the half-open range $$[i, j)$$. `getBitsAsLimb` is the same extraction with a proof that the
width fits in a limb; in Malachite, `u64::exact_from(&n.get_bits(i, j))`, or `wrapping_from` to
skip the check. `size` and `significant_bits` are both 0 for zero, and `trailingZeros` and
`trailing_zeros` are both `None` there. `isPowerOfTwo 0` and `Natural::ZERO.is_power_of_2()` are
both false.

**Bitwise operations.** `AzNat` has no counterpart yet for Malachite's logical operators
([`BitAnd`](https://doc.rust-lang.org/nightly/std/ops/trait.BitAnd.html),
[`BitOr`](https://doc.rust-lang.org/nightly/std/ops/trait.BitOr.html),
[`BitXor`](https://doc.rust-lang.org/nightly/std/ops/trait.BitXor.html), and `Not`, which turns a
`Natural` into an `Integer`), for `flip_bit` and `assign_bit` (spelled with `testBit`, `setBit`, and
`clearBit`), for writing a block of bits
([`BitBlockAccess::assign_bits`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitBlockAccess.html)),
or for the bit scans
([`BitScan`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitScan.html)),
the population and Hamming counts
([`CountOnes`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.CountOnes.html),
[`HammingDistance`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.HammingDistance.html)),
and the conversions to and from sequences of bits
([`BitIterable`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitIterable.html),
[`BitConvertible`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitConvertible.html)).

## Arithmetic modulo a power of 2 {#arithmetic-modulo-a-power-of-2}

| | Azurite | Malachite |
| :---: | --- | --- |
| ≈ | `addModPow2 (a b : AzNat) (k : Nat) : AzNat` | [`ModPowerOf2Add`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Add.html) |
| ≈ | `subModPow2 (a b : AzNat) (k : Nat) : AzNat` | [`ModPowerOf2Sub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Sub.html) |
| ≈ | `mulDispatchModPow2 (a b : AzNat) (k : Nat) : AzNat` | [`ModPowerOf2Mul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Mul.html) |
| ≈ | `squareDispatchModPow2 (a : AzNat) (k : Nat) : AzNat` | [`ModPowerOf2Square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Square.html) |
| — | `mulSchoolbookModPow2`, `mulKaratsubaModPow2`, `mulToomCook3ModPow2`, `squareSchoolbookModPow2`, `squareKaratsubaModPow2`, `squareToomCook3ModPow2` | |

**Reduced inputs.** The four Azurite operations accept any operands and reduce the result modulo
$$2^k$$, reading only the low limbs; Malachite's `mod_power_of_2_add` and its siblings require
their inputs to be already reduced and panic otherwise, so an unreduced operand must first pass
through `mod_power_of_2(k)`. On reduced inputs the results agree. The forced single-algorithm
variants are benchmark instruments, as under
[Tuning parameters and forced algorithms](#tuning-parameters-and-forced-algorithms).

## Powers and roots {#powers-and-roots}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `pow (a : AzNat) (n : ℕ) : AzNat`, the `CommSemiring` `npow` | [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html) |
| — | `powBinary (a : AzNat) (n : ℕ) : AzNat` | |
| ✓ | `sqrt (m : AzNat) : AzNat` | [`FloorSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorSqrt.html) |
| ✓ | `sqrtRem (m : AzNat) : AzNat × AzNat` | [`SqrtRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SqrtRem.html) |
| ✓ | `isSquare (n : AzNat) : Bool` | [`IsSquare`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.IsSquare.html) |
| ✓ | `rootInt (m : AzNat) (k : ℕ) : AzNat` | [`FloorRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorRoot.html) |
| ⚙ | `isPow (m : AzNat) (k : ℕ) : Bool` | [`CheckedRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedRoot.html) |

**`pow`, `powBinary`.** Both are exact exponentiation; `powBinary` is the right-to-left binary
method kept for benchmarking against the sliding-window `pow`. Malachite's `pow` takes a `u64`
exponent.

**Roots.** `rootInt m k` is $$\lfloor m^{1/k} \rfloor$$, which is `m.floor_root(k)`; a zero `k`
panics in Malachite and is unspecified in Azurite. `isPow m k` asks whether `m` is a perfect
`k`-th power for the given `k`; Malachite's `m.checked_root(k).is_some()` answers that. Malachite's
[`IsPower`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.IsPower.html)
asks a different question, whether *some* exponent greater than 1 works.

## GCD and modular arithmetic {#gcd-and-modular-arithmetic}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `gcd (a b : AzNat) : AzNat` | [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html) |
| ✓ | `coprime (a b : AzNat) : Bool` | [`CoprimeWith`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CoprimeWith.html) |
| ≈ | `invMod (a n : AzNat) : AzNat` | [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html) |
| ✓ | `jacobi (a n : AzNat) : ℤ` | [`JacobiSymbol`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.JacobiSymbol.html) |
| ≈ | `garner (ms ns : List AzNat) : AzNat` | [`multi_crt`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.multi_crt) |

**`gcd`, `coprime`.** Azurite's `gcd` is Stein's binary algorithm and Malachite's is the
subquadratic half-GCD, both with $$\gcd(0, b) = b$$; `coprime` is `a.coprime_with(b)`.

**`invMod`.** Azurite returns the least nonnegative inverse of `a` modulo `n` for coprime
arguments, with `a` of any size. Malachite's `a.mod_inverse(n)` requires `a` to be nonzero and
already reduced modulo `n`, panicking otherwise, and returns `None` when no inverse exists. Reduce
first: `(a % &n).mod_inverse(n)`.

**`jacobi`.** Defined for an odd `n` on both sides; Malachite panics on an even one and returns an
`i8` where Azurite returns an `ℤ`, both taking the values −1, 0, and 1.

**`garner`.** Garner's algorithm reconstructs the unique `n` below the product of the moduli from
residues `nᵢ` modulo pairwise coprime `mᵢ`; Malachite's `Natural::multi_crt(&moduli, &values)`
computes the same `n`, but returns `None` if the moduli are not pairwise coprime or if any residue
is at least its modulus, where Azurite leaves the result unspecified and accepts an unreduced
residue. Reduce the residues first to match. For two congruences,
[`Crt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Crt.html)
is the pairwise form.

## Primality and divisors {#primality-and-divisors}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✗ | `isPrime (n : AzNat) : Bool` | |
| ✗ | `millerRabin (n : AzNat) (rounds : ℕ) (seed : UInt64) : Bool` | |
| — | `isPrimeNaive (n : AzNat) : Bool`, `aprclOrNaive`, `millerRabinBase`, `millerRabinBases` | |
| — | `lucasLehmerTest (p : ℕ) : Bool` | |
| — | `nMinusOneTest (n : AzNat) (factors : List (AzNat × ℕ))`, `prattCertify`, `findPrattPrime` | |
| — | `lenstraDivisors (n r s rs : AzNat) : List AzNat` | |
| ✗ | `divisors (n : AzNat) : List AzNat` | |

**`isPrime`, `millerRabin`.** Azurite's `isPrime` is a Miller–Rabin filter followed by a fully
proven APR-CL certificate, with the theorem `isPrime n = true ↔ Nat.Prime n.toNat`; `millerRabin`
alone is the probabilistic filter, whose `false` verdicts are proven composite. Malachite does not
yet test the primality of a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), the gap
recorded [on the GMP page](/mapping/gmp-integers/#number-theoretic-functions) for
`mpz_probab_prime_p`; its
[`Primes`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.Primes.html)
iterator and the primitive-integer
[`IsPrime`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.IsPrime.html)
are the current extent.

**The certificate machinery.** `lucasLehmerTest`, the `n − 1` test, the Pratt certificates, and
Lenstra's divisors in a residue class are formalized textbook algorithms from Crandall–Pomerance,
the proofs of which are what `isPrime`'s theorem rests on; `isPrimeNaive` is the trial-division
reference. Malachite's planned primality test is a single predicate, and its naive reference
implementations live in its test utilities, so none of these will get a public counterpart.

**`divisors`.** The list of all divisors needs a factorization, which Malachite does not yet have
for a `Natural`; this is the same gap as `arith_divisors`
[on the FLINT arithmetic-functions page](/mapping/flint-arithmetic-functions/).

## Strings and digits {#strings-and-digits}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `toString (n : AzNat) : String`, `instance : ToString AzNat` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |
| ✓ | `toStringBase (b : UInt64) (n : AzNat) : String` | [`ToStringBase::to_string_base`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToStringBase.html) |
| ✓ | `toStringBaseWith (b : UInt64) (uppercase : Bool) (usePrefix : Bool) (n : AzNat) : String` | [`ToStringBase::to_string_base_upper`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToStringBase.html), [`Binary`](https://doc.rust-lang.org/nightly/std/fmt/trait.Binary.html), [`Octal`](https://doc.rust-lang.org/nightly/std/fmt/trait.Octal.html), [`LowerHex`](https://doc.rust-lang.org/nightly/std/fmt/trait.LowerHex.html), [`UpperHex`](https://doc.rust-lang.org/nightly/std/fmt/trait.UpperHex.html) |
| ≈ | `parse (s : String) : Option AzNat` | [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) |
| ≈ | `parseBase (b : UInt64) (s : String) : Option AzNat` | [`FromStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromStringBase.html) |
| ✓ | `AzNat.limbDigits (b : UInt64) (n : AzNat) : Array UInt64` | [`Digits::to_digits_asc`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.Digits.html) |
| ✓ | `AzNat.ofLimbDigits (b : UInt64) (digits : Array UInt64) : AzNat`, `AzNat.ofBase10Digits` | [`Digits::from_digits_asc`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.Digits.html) |
| ✓ | `AzNat.limbDigitsPow2 (k : Nat) (n : AzNat) : Array UInt64` | [`PowerOf2Digits::to_power_of_2_digits_asc`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.PowerOf2Digits.html) |
| ✓ | `AzNat.ofLimbDigitsPow2 (k : Nat) (digits : Array UInt64) : AzNat` | [`PowerOf2Digits::from_power_of_2_digits_asc`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.PowerOf2Digits.html) |
| — | `instance : ParsableElement AzNat` | |

**Formatting.** Both print decimal by default, and both render a general base with lowercase
letters past 9. `toStringBaseWith` adds uppercase letters, `n.to_string_base_upper(b)`, and a
`0b`, `0o`, or `0x` prefix for bases 2, 8, and 16 only, which is the `#` flag in `{:#b}`,
`{:#o}`, and `{:#x}`; `{:#X}` is the uppercase prefixed form. Azurite's bases run to 36 and
Malachite's to 62, the digits past 35 being distinguished by case as in GMP. A base outside the
range gives `""` in Azurite and panics in Malachite.

**Parsing.** `parse` chooses the base from a `0b`, `0o`, or `0x` prefix and otherwise reads
decimal; Malachite's `Natural::from_str` reads decimal only. `parseBase b s` strips any such
prefix before reading `s` in base `b`; `Natural::from_string_base(b, s)` does not, so `"0x1f"` in
base 16 parses in Azurite and is rejected in Malachite. Remove the prefix before the call to
match. Both accept letters in either case and reject an empty string, an invalid character, or a
digit at least the base; Malachite additionally accepts a single leading `+` (`"+12"` parses,
`"++12"` does not), which Azurite rejects. Both return the absent value rather than failing.

**Digits.** `limbDigits b` and `to_digits_asc(&b)` give the base-`b` digits least significant
first, with no high zero digit, for any base from 2 to $$2^{64} - 1$$, and the reverse
conversions rebuild the number from such an array; a digit at least the base is unspecified in
Azurite and `None` in Malachite. `ofBase10Digits` is `from_digits_asc(&10)` with the base's
constants folded in. `limbDigitsPow2 k` and `to_power_of_2_digits_asc(k)` are the same for the
base $$2^k$$, $$1 \leq k \leq 64$$. Malachite also provides the most-significant-first orders.

**`ParsableElement`.** Azurite's typeclass for parsing a coefficient inside a vector, matrix, or
polynomial literal; Malachite's polynomial parsing is its own `FromStr` implementations.

## Exhaustive generation {#exhaustive-generation}

Azurite's `ExhaustiveGenerator` typeclass is a Lean port of Malachite's `exhaustive_*` iterators:
a generator `gen : ℕ → Option T` that enumerates every value of `T` exactly once, with proofs.
The `AzNat` instances produce the same sequences as Malachite's functions.

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `instance naturalsGen : ExhaustiveGenerator AzNat` | [`exhaustive_naturals`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/exhaustive/fn.exhaustive_naturals.html) |
| ✓ | `instance positiveNaturalsGen : ExhaustiveGenerator {n : AzNat // 0 < n}` | [`exhaustive_positive_naturals`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/exhaustive/fn.exhaustive_positive_naturals.html) |
| ✓ | `azNatRangeGen (a b : AzNat) : ExhaustiveGenerator {x : AzNat // a ≤ x ∧ x < b}` | [`exhaustive_natural_range`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/exhaustive/fn.exhaustive_natural_range.html) |
| ✓ | `azNatRangeInclusiveGen (a b : AzNat) : ExhaustiveGenerator {x : AzNat // a ≤ x ∧ x ≤ b}` | [`exhaustive_natural_inclusive_range`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/exhaustive/fn.exhaustive_natural_inclusive_range.html) |
| ✓ | `azNatRangeToInfinityGen (a : AzNat) : ExhaustiveGenerator {x : AzNat // a ≤ x}` | [`exhaustive_natural_range_to_infinity`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/exhaustive/fn.exhaustive_natural_range_to_infinity.html) |

**Order.** All five enumerate in ascending order, $$0, 1, 2, \ldots$$ or from the range's lower
end, so the `k`-th element of each Azurite generator is the `k`-th item of the Malachite
iterator. An empty range gives `none` from the first index in Azurite and an empty iterator in
Malachite. Azurite's random generators produce Lean `Nat`s rather than `AzNat`s, so they are not
mapped here.

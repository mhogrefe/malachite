---
layout: default
title: "Malachite for Azurite Users: Integers Modulo a Power of 2"
permalink: /mapping/azurite-mod-power-of-2/
theme: jekyll-theme-slate
---

# Malachite for Azurite Users: Integers Modulo a Power of 2

This page maps the operations of [Azurite](https://github.com/mhogrefe/azurite)'s `AzZModPow2 k`
type, its ring of integers modulo $$2^k$$, onto their Malachite counterparts, which are the
`mod_power_of_2_*` operations on
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) from the
`malachite-nz` crate. It is a companion of
[Malachite for Azurite Users: Naturals](/mapping/azurite-naturals/), whose
[Conventions](/mapping/azurite-naturals/#conventions) carry over; that page's section
[Arithmetic modulo a power of 2](/mapping/azurite-naturals/#arithmetic-modulo-a-power-of-2) maps
the underlying `AzNat` functions, and this one maps the type built on them. The page covers
Azurite as of commit `b5da19d` (2026-10-04). The [mapping index](/mapping/) lists the whole
family of pages.

## Conventions {#conventions}

### The types

An `AzZModPow2 k` is a structure holding an `AzNat` residue `val` together with a proof that
`val` is less than $$2^k$$, so every value is canonical and equality is structural; the modulus
$$k$$ is a type index, fixed at compile time, and `AzZModPow2 0` is the trivial ring. Malachite
has no residue type. Its modular arithmetic is a family of traits on `Natural` that take the
modulus as a runtime argument,
[`ModPowerOf2Add`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Add.html)
and its siblings, every one of which requires its inputs to be already reduced and panics
otherwise. So Azurite's type invariant and Malachite's precondition are the same statement: an
`AzZModPow2 k` *is* a reduced `Natural` with the modulus remembered, and each row below maps a
method of the type onto the trait method applied to such a `Natural`, with `k` passed along as
`pow`. Mixing two moduli is a type error in Azurite and a programming error in Malachite that the
reducedness assertions catch when the operand of the larger modulus is unreduced for the smaller.

Both libraries reduce by masking the low $$k$$ bits rather than by division, which is the reason
each singles out powers of two from general moduli.

### Typeclasses and traits

`AzZModPow2 k` is a verified Mathlib `CommRing` with `Neg`, `Add`, `Sub`, `Mul`, Azurite's
`Square`, `OfNat` literals, and `NatCast`/`IntCast`, all routed through the limb-level
operations. Malachite's traits stand in for the ring structure, as on the other Azurite pages.

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
| ✓ | `ofAzNat (k : Nat) (n : AzNat) : AzZModPow2 k` | [`ModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2.html) |
| ✓ | `ofAzInt (k : Nat) (z : AzInt) : AzZModPow2 k` | [`ModPowerOf2`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/mod_power_of_2/index.html) for `Integer` |
| ✓ | `ofNat (k m : Nat) : AzZModPow2 k`, `instance : OfNat (AzZModPow2 k) m`, `instance : NatCast (AzZModPow2 k)` | [`Natural::from`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_int/index.html) then [`ModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2.html) |
| ✓ | `instance : IntCast (AzZModPow2 k)` | [`Integer::from`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_int/index.html) then [`ModPowerOf2`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/mod_power_of_2/index.html) |
| ✓ | `instance : Zero (AzZModPow2 k)`, `instance : One (AzZModPow2 k)` | [`Natural::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html), [`Natural::ONE`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.One.html) |
| — | `toAzNat (a : AzZModPow2 k) : AzNat`, `AzZModPow2.val : AzNat` | |
| ⚙ | `AzZModPow2.isLt : val.toNat < 2 ^ k` | [`ModPowerOf2IsReduced`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2IsReduced.html) |
| ✓ | `deriving DecidableEq` | [`Eq`](https://doc.rust-lang.org/nightly/std/cmp/trait.Eq.html), [`EqModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EqModPowerOf2.html) |

**Reduction.** `ofAzNat k n` masks `n` to its low $$k$$ bits, which is `n.mod_power_of_2(k)`; the
result is the `Natural` that stands for the residue from then on. `ofAzInt k z` reduces the
magnitude and negates when `z` is negative, giving the representative in $$[0, 2^k)$$ of a
negative integer, which is what `Integer`'s `mod_power_of_2` returns (its output type is
`Natural`); `Integer` also has `rem_power_of_2` and `ceiling_mod_power_of_2`, which keep the
sign and have no Azurite names. The literal and cast instances reduce a `Nat` or `Int` the same
way. `One` is `ofAzNat k 1`, so it is `0` when $$k = 0$$, as `Natural::ONE.mod_power_of_2(0)` is.

**The residue and its proof.** `val` (or `toAzNat`) is the `Natural` itself in Malachite, where
the residue was never wrapped; the proof `isLt` has no value counterpart, but the property it
states is `n.mod_power_of_2_is_reduced(k)`, which is what Malachite's modular operations assert
on their inputs. Two residues are equal when their `val`s are, so `==` on `Natural` compares two
reduced values, and `x.eq_mod_power_of_2(&y, k)` compares two unreduced ones, as
`ofAzNat k x = ofAzNat k y` does.

## Arithmetic {#arithmetic}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `add (a b : AzZModPow2 k) : AzZModPow2 k`, `instance : Add (AzZModPow2 k)` | [`ModPowerOf2Add`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Add.html), [`ModPowerOf2AddAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2AddAssign.html) |
| ✓ | `sub (a b : AzZModPow2 k) : AzZModPow2 k`, `instance : Sub (AzZModPow2 k)` | [`ModPowerOf2Sub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Sub.html), [`ModPowerOf2SubAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2SubAssign.html) |
| ✓ | `neg (a : AzZModPow2 k) : AzZModPow2 k`, `instance : Neg (AzZModPow2 k)` | [`ModPowerOf2Neg`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Neg.html), [`ModPowerOf2NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2NegAssign.html) |
| ✓ | `mul (a b : AzZModPow2 k) : AzZModPow2 k`, `instance : Mul (AzZModPow2 k)` | [`ModPowerOf2Mul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Mul.html), [`ModPowerOf2MulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2MulAssign.html) |
| ✓ | `instance instSquare : Square (AzZModPow2 k)` | [`ModPowerOf2Square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Square.html), [`ModPowerOf2SquareAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2SquareAssign.html) |
| ✓ | `pow (a : AzZModPow2 k) (n : ℕ) : AzZModPow2 k`, the `CommRing` `npow` | [`ModPowerOf2Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Pow.html), [`ModPowerOf2PowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2PowAssign.html) |
| ✓ | `isOdd (a : AzZModPow2 k) : Bool` | [`Parity`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Parity.html) |
| ✓ | `invOdd (a : AzZModPow2 k) (h : a.isOdd = true) : AzZModPow2 k` | [`ModPowerOf2Inverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Inverse.html) |
| — | `invOddAux (a : AzZModPow2 k) : Nat → AzZModPow2 k` | |
| — | `instance instCommRing : CommRing (AzZModPow2 k)` | |

**Ring operations.** `a + b` is `a.mod_power_of_2_add(b, k)`, and likewise `sub`, `mul`, and
`neg`; `-a` is the two's complement $$2^k - a$$ with $$-0 = 0$$ on both sides. The `Square`
instance is `mod_power_of_2_square`, which both libraries route to a dedicated low-half squaring
rather than a general product, and `pow` is `mod_power_of_2_pow`; Azurite's exponent is a `Nat`
and Malachite's a `Natural`, both unbounded. The `CommRing` instance bundles these with their
proofs and has no counterpart.

**Units and inverses.** A residue is a unit exactly when it is odd, so `isOdd` is `odd()` on the
representative. `invOdd` takes a proof of oddness and returns the inverse, by Newton lifting;
`mod_power_of_2_inverse` returns `Some(inverse)` for an odd input and `None` for an even one,
which is the `Option` form of the same function. `invOddAux` is the lifting iteration.

**Shifts.** Malachite also has
[`ModPowerOf2Shl`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Shl.html)
and
[`ModPowerOf2Shr`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Shr.html),
multiplication and floor division by a power of 2 modulo $$2^k$$, with a negative shift count
reversing the direction. `AzZModPow2` has no shift operations; the spellings are
`ofAzNat k (a.val <<< s)` and `ofAzNat k (a.val >>> s)`, the second of which needs no reduction.

## Strings {#strings}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `toString (a : AzZModPow2 k) : String`, `instance : ToString (AzZModPow2 k)` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |
| ≈ | `parse (s : String) : Option (AzZModPow2 k)` | [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) then [`ModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2.html) |
| — | `toChars`, `parseChars`, `instance : ParsableElement (AzZModPow2 k)` | |

**Formatting and parsing.** `toString` prints the representative in decimal, which is `Display`
on the `Natural`. `parse` reads a natural with `AzNat.parse`'s rules and reduces it, so `"19"` is
`3` in `AzZModPow2 4`; `Natural::from_str(s)` followed by `mod_power_of_2(k)` is the same
computation, with the differences in what the two parsers accept
[noted on the naturals page](/mapping/azurite-naturals/#strings-and-digits). `ParsableElement` is
Azurite's typeclass for parsing a coefficient inside a vector, matrix, or polynomial literal.

## The bridge to `ZMod` {#the-bridge-to-zmod}

| | Azurite | Malachite |
| :---: | --- | --- |
| — | `toZMod (a : AzZModPow2 k) : ZMod (2 ^ k)`, `ofZMod (z : ZMod (2 ^ k)) : AzZModPow2 k` | |
| — | `equivZMod : AzZModPow2 k ≃ ZMod (2 ^ k)`, `ringEquivZMod : AzZModPow2 k ≃+* ZMod (2 ^ k)`, `toZModRingHom : AzZModPow2 k →+* ZMod (2 ^ k)` | |
| — | `instance instNeZeroTwoPow : NeZero ((2 : ℕ) ^ k)` | |

**Proofs.** These connect the type to Mathlib's `ZMod (2 ^ k)`, the specification side of its
theorems, as `toNat` does for `AzNat`
[on the naturals page](/mapping/azurite-naturals/#proofs); they have no counterpart, Malachite's
specification being its documentation and tests. The equivalence lemmas in
`Azurite/AzZModPow2/Equiv/` are likewise not mapped.

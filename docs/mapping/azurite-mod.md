---
layout: default
title: "Malachite for Azurite Users: Integers Modulo a Natural"
permalink: /mapping/azurite-mod/
theme: jekyll-theme-slate
---

# Malachite for Azurite Users: Integers Modulo a Natural

This page maps the operations of [Azurite](https://github.com/mhogrefe/azurite)'s `AzZMod m` type,
its ring of integers modulo an arbitrary natural $$m$$, onto their Malachite counterparts, which
are the `mod_*` operations on
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) from the
`malachite-nz` crate. It is a companion of
[Malachite for Azurite Users: Naturals](/mapping/azurite-naturals/), whose
[Conventions](/mapping/azurite-naturals/#conventions) carry over, and of
[Malachite for Azurite Users: Integers Modulo a Power of 2](/mapping/azurite-mod-power-of-2/),
which maps the power-of-two specialization `AzZModPow2 k` and whose conventions this page shares;
only what differs for a general modulus is repeated here. The page covers Azurite as of commit
`b5da19d` (2026-10-04). The [mapping index](/mapping/) lists the whole family of pages.

## Conventions {#conventions}

### The types

An `AzZMod m` is a structure holding an `AzNat` residue `val` together with a proof that `val` is
less than `m`, so every value is canonical and equality is structural. The modulus is an `AzNat`
*value index*: it is part of the type, but as a limb-level value rather than a Lean `Nat`, so that
reduction (`AzNat.mod` against `m`) never routes a large modulus through `Nat` arithmetic. A
nonzero modulus is required wherever a residue is constructed, as the instance argument
`[NeZero m.toNat]`, mirroring Mathlib's `ZMod`. Malachite has no residue type; its modular
arithmetic is the family of traits
[`ModAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAdd.html),
[`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html),
and so on, which take the modulus as a runtime `Natural` argument `m`, require their inputs to be
reduced, and panic otherwise. As on the power-of-two page, the type invariant and the precondition
are the same statement, and each row below maps a method of the type onto the trait method applied
to a reduced `Natural` with `m` passed along. A zero modulus is a type with no values in Azurite
and a panic in Malachite.

### Typeclasses and traits

`AzZMod m` is a verified Mathlib `CommRing` with `Neg`, `Add`, `Sub`, `Mul`, Azurite's `Square`,
`OfNat` literals, `NatCast`/`IntCast`, a `LinearOrder` by residue value, and a `Fintype`
instance; for a prime modulus it is a `Field`. Malachite's traits stand in for the ring
structure, and `Natural`'s own `Ord` for the order.

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
| ✓ | `ofAzNat (m : AzNat) [NeZero m.toNat] (n : AzNat) : AzZMod m` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ⚙ | `ofAzInt (m : AzNat) [NeZero m.toNat] (z : AzInt) : AzZMod m` | [`Mod`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/mod_op/index.html) for `Integer`, then [`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html) |
| ✓ | `ofNat (m : AzNat) [NeZero m.toNat] (a : Nat) : AzZMod m`, `instance : OfNat (AzZMod m) a`, `instance : NatCast (AzZMod m)` | [`Natural::from`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_int/index.html) then [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ⚙ | `instance : IntCast (AzZMod m)` | [`Integer::from`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_int/index.html) then [`Mod`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/mod_op/index.html) and [`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html) |
| ✓ | `instance : Zero (AzZMod m)`, `instance : One (AzZMod m)` | [`Natural::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html), [`Natural::ONE`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.One.html) |
| — | `toAzNat (a : AzZMod m) : AzNat`, `AzZMod.val : AzNat` | |
| ⚙ | `AzZMod.isLt : val.toNat < m.toNat` | [`ModIsReduced`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModIsReduced.html) |
| — | `instance instNeZeroToNatOfNat (n : Nat) [NeZero n] : NeZero (AzNat.ofNat n).toNat` | |
| ✓ | `deriving DecidableEq` | [`Eq`](https://doc.rust-lang.org/nightly/std/cmp/trait.Eq.html), [`EqMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EqMod.html) |
| ✓ | `instance instLinearOrder : LinearOrder (AzZMod m)` | [`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html) |
| — | `ofAzNatRingHom : AzNat →+* AzZMod m`, `ofAzIntRingHom : AzInt →+* AzZMod m` | |
| — | `finEquiv (m : AzNat) : AzZMod m ≃ Fin m.toNat`, `instance instFintype : Fintype (AzZMod m)` | |

**Reduction.** `ofAzNat m n` is `n % m`, which is `n.mod_op(&m)` (or `n % &m`, the same operation
on naturals); the result is the `Natural` that stands for the residue from then on. `ofAzInt m z`
reduces the magnitude and negates it in the ring when `z` is negative, giving the representative
in $$[0, m)$$ of a negative integer. Malachite's `Integer` has `mod_op` with an `Integer` modulus,
whose result takes the modulus's sign, so for a positive modulus it is nonnegative and
`z.mod_op(Integer::from(&m)).unsigned_abs()` is the `Natural` residue, hence ⚙; the `IntCast`
instance is the same route from a machine integer.

**The residue, its proof, and its order.** `val` is the `Natural` itself in Malachite; the proof
`isLt` has no value counterpart, but the property it states is `n.mod_is_reduced(&m)`, which
Malachite's modular operations assert on their inputs. Two residues are equal when their `val`s
are, so `==` compares two reduced `Natural`s and `x.eq_mod(&y, &m)` two unreduced ones, as
`ofAzNat m x = ofAzNat m y` does. The `LinearOrder` compares residues by value, which is
`Natural`'s `Ord`; it is a sorting order for canonical forms, not a ring order, on both sides. The
ring homomorphisms package `ofAzNat` and `ofAzInt` for mapping polynomial coefficients, and the
`Fintype` instance states that there are `m` residues; neither has a Malachite counterpart.

## Arithmetic {#arithmetic}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `add (a b : AzZMod m) : AzZMod m`, `instance : Add (AzZMod m)` | [`ModAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAdd.html), [`ModAddAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAddAssign.html) |
| ✓ | `sub (a b : AzZMod m) : AzZMod m`, `instance : Sub (AzZMod m)` | [`ModSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSub.html), [`ModSubAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSubAssign.html) |
| ✓ | `neg (a : AzZMod m) : AzZMod m`, `instance : Neg (AzZMod m)` | [`ModNeg`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModNeg.html), [`ModNegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModNegAssign.html) |
| ✓ | `mul (a b : AzZMod m) : AzZMod m`, `instance : Mul (AzZMod m)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html), [`ModMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMulAssign.html) |
| ✓ | `instance instSquare : Square (AzZMod m)` | [`ModSquare`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSquare.html), [`ModSquareAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSquareAssign.html) |
| ✓ | `pow (a : AzZMod m) (n : ℕ) : AzZMod m`, the `CommRing` `npow` | [`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html), [`ModPowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowAssign.html) |
| ✓ | `powAzNat (a : AzZMod m) (n : AzNat) : AzZMod m` | [`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html) |
| ✓ | `inv (a : AzZMod m) (h : Nat.Coprime a.val.toNat m.toNat) : AzZMod m` | [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html) |
| ✓ | `tryInv (x : AzZMod m) : Option (AzZMod m)` | [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html) |
| ⚙ | `fieldInv [Fact (Nat.Prime p.toNat)] (a : AzZMod p) : AzZMod p` | [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html) |
| ≈ | `instance instField [Fact (Nat.Prime p.toNat)] : Field (AzZMod p)` | [`ModDiv`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModDiv.html), [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html) |
| — | `instance instCommRing : CommRing (AzZMod m)`, `instance instNeZeroToNatOfPrime` | |

**Ring operations.** `a + b` is `a.mod_add(b, &m)`, and likewise `sub`, `mul`, and `neg`, with
$$-0 = 0$$ on both sides; the `Square` instance is `mod_square`, and `pow` and `powAzNat` are
both `mod_pow`, whose exponent is a `Natural`, so `powAzNat` is the exact match and `pow` the
same function on a Lean `Nat`. Malachite additionally offers
[`ModMulPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMulPrecomputed.html),
[`ModSquarePrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSquarePrecomputed.html),
and
[`ModPowPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowPrecomputed.html),
which take precomputed data about the modulus (from `precompute_mod_mul_data` and
`precompute_mod_pow_data`) to speed up repeated operations with the same `m`; they return the same
values as the plain versions. Azurite reduces by `AzNat` division each time and has no
precomputed variants.

**Inverses and division.** A residue is a unit exactly when it is coprime to `m`. `inv` takes a
proof of coprimality; `tryInv` tests it and returns `none` otherwise, which is exactly
`mod_inverse`'s `Option<Natural>`. `fieldInv` is the total inverse of the field case, prime `p`,
with the convention $$0^{-1} = 0$$; `mod_inverse` returns `None` there (and panics on an input of
`0` when the modulus is composite), so the spelling is `n.mod_inverse(&p).unwrap_or(Natural::ZERO)`.
The `Field` instance's `a / b` is `a · b⁻¹` for a prime modulus;
[`ModDiv`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModDiv.html)
works for any modulus and returns `Some(q)` with $$qb \equiv a$$ whenever $$\gcd(b, m)$$ divides
$$a$$, choosing one quotient when `b` is not a unit, and `None` otherwise, hence ≈: the two agree
for a prime modulus and `b ≠ 0`, and differ at `b = 0`, where the field gives `0` and `mod_div`
gives `Some(0)` for `a = 0` and `None` for other `a`.

**Shifts and square roots.** Malachite also has
[`ModShl`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModShl.html)
and
[`ModShr`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModShr.html),
multiplication and floor division by a power of 2 modulo `m` with a negative count reversing the
direction, spelled `ofAzNat m (a.val <<< s)` and `ofAzNat m (a.val >>> s)` in Azurite, and
[`ModSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSqrt.html),
a modular square root, which `AzZMod` does not have.

## Strings {#strings}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `toString (a : AzZMod m) : String`, `instance : ToString (AzZMod m)` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |
| ≈ | `parse (s : String) : Option (AzZMod m)` | [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) then [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| — | `toChars`, `parseChars`, `instance : ParsableElement (AzZMod m)` | |

**Formatting and parsing.** `toString` prints the representative in decimal, which is `Display`
on the `Natural`. `parse` reads a natural with `AzNat.parse`'s rules and reduces it, so `"19"` is
`5` in `AzZMod 7`; `Natural::from_str(s)` followed by `mod_op(&m)` is the same computation, with
the differences in what the two parsers accept
[noted on the naturals page](/mapping/azurite-naturals/#strings-and-digits). `ParsableElement` is
Azurite's typeclass for parsing a coefficient inside a vector, matrix, or polynomial literal.

## Internals and the bridge to `ZMod` {#internals-and-the-bridge-to-zmod}

| | Azurite | Malachite |
| :---: | --- | --- |
| — | `QuadT (m : AzNat) (u a : AzZMod m)` and its operations, `NormOne`, `normOneCandidate` | |
| — | `toZMod`, `ofZMod`, `equivZMod`, `ringEquivZMod`, `toZModRingHom` | |

**The quadratic ring.** `AzZMod.QuadT` is the ring $$(\mathbb{Z}/m)[T]/(T^2 - uT - a)$$ used by
Azurite's APR-CL primality proof, with its norm, conjugate, and norm-one powering. Malachite's
primality testing exposes no such intermediate structure, so it is outside the mapping.

**Proofs.** The `ZMod m.toNat` bridge is the specification side of the type's theorems, as
`toNat` is for `AzNat` [on the naturals page](/mapping/azurite-naturals/#proofs); it has no
counterpart, Malachite's specification being its documentation and tests. The equivalence lemmas
in `Azurite/AzZMod/Equiv/` are likewise not mapped.

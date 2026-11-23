---
layout: default
title: "Malachite for Azurite Users: Vectors Modulo a Power of 2"
permalink: /mapping/azurite-vectors-mod-power-of-2/
theme: jekyll-theme-slate
---

# Malachite for Azurite Users: Vectors Modulo a Power of 2

This page maps the operations of [Azurite](https://github.com/mhogrefe/azurite)'s `AzVector (AzZModPow2 k) n`,
its vector of `n` integers modulo $$2^k$$, onto their Malachite counterparts, which are the
`mod_power_of_2_*` operations on two vector types:
[`UnsignedVector<T>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html), from the `malachite-base` crate, whose elements are primitive
unsigned integers and whose modulus therefore must be small enough that $$k$$ is at most the width of `T`, and
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html), from the `malachite-nz` crate, whose modulus may be any size. It is a
companion of [Malachite for Azurite Users: Integers Modulo a Power of 2](/mapping/azurite-mod-power-of-2/), which maps the elements
themselves and whose [Conventions](/mapping/azurite-mod-power-of-2/#conventions) carry over, and of
[Malachite for Azurite Users: Integer Vectors](/mapping/azurite-integer-vectors/), whose
[Conventions](/mapping/azurite-integer-vectors/#conventions) on dimensions apply here unchanged.
The page covers Azurite as of commit `a2b4586` (2026-11-08). The [mapping index](/mapping/) lists
the whole family of pages.

Every definition of `Azurite/AzVector/` that applies to these vectors is mapped below: 19
rows across 4 sections, of which 12 are ✓, 3 ≈, 1 ⚙, and 3 —.
Theorems are not mapped.

## Conventions {#conventions}

### The types

An `AzVector (AzZModPow2 k) n` is a vector of residues, each of which carries the modulus in its type and
is reduced by construction. Malachite has no residue type: a vector modulo $$2^k$$ is an
[`UnsignedVector<T>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html) or a [`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html) whose elements are already reduced, and
each operation takes the modulus as a runtime argument, `pow`, as the element operations do on the
[element page](/mapping/azurite-mod-power-of-2/#conventions). Every one of these operations requires its vectors to be
reduced and panics otherwise; [`ModPowerOf2IsReduced`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2IsReduced.html) checks a vector, and [`ModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2.html) reduces one. So Azurite's
type invariant and Malachite's precondition are again the same statement, and each row below maps a
method of `AzVector` onto the trait method applied to reduced vectors, with the modulus passed
along. The traits are implemented for both vector types; an [`UnsignedVector<T>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html) suits a
modulus that fits in a word, and a [`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html) one of any size.

As on the [integer vector page](/mapping/azurite-integer-vectors/#dimensions), the dimension is
part of the type in Azurite and a runtime value in Malachite, which panics on mismatched
dimensions and out-of-range indices.

### Fixed-width words

Lean's `UInt64`, `UInt32`, `UInt16`, and `UInt8` wrap around, so the generic `AzVector` operations
on vectors of them are arithmetic modulo $$2^{64}$$, $$2^{32}$$, and so on. Azurite gives these
vectors no string form and does not use them; their operations are the ones on this page with
$$k$$ equal to the width, which an `UnsignedVector<u64>` with `pow` set to 64 computes, with every
`u64` already reduced.

### Categories

Each definition falls into one of five categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| ⚙ | Malachite does not expose this algorithm or helper; the Malachite column or the notes say what to call instead. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## Construction and access {#construction-and-access}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `AzVector.ofList (l : List R) (h : l.length = n) : AzVector R n` | [`from_owned_elements`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.from_owned_elements), [`from_elements`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#method.from_elements) |
| ✓ | `AzVector.toList (v : AzVector R n) : List R` | [`into_elements`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.into_elements), [`to_elements`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#method.to_elements), [`elements_ref`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.elements_ref) |
| ✓ | `AzVector.get (v : AzVector R n) (i : Fin n) : R`, `instance : GetElem (AzVector R n) (Fin n) R _` | [`Index<usize>`](https://doc.rust-lang.org/nightly/core/ops/trait.Index.html) |
| ≈ | `AzVector.toFn (v : AzVector R n) : Fin n → R` | [`Index<usize>`](https://doc.rust-lang.org/nightly/core/ops/trait.Index.html) |
| — | `AzVector.ofFn (f : Fin n → R) : AzVector R n` | [`FromIterator`](https://doc.rust-lang.org/nightly/core/iter/trait.FromIterator.html) |
| — | `AzVector.map (f : R → S) (v : AzVector R n) : AzVector S n`, `AzVector.zip (f : R → S → T) (v : AzVector R n) (w : AzVector S n) : AzVector T n` | [`IntoIterator`](https://doc.rust-lang.org/nightly/core/iter/trait.IntoIterator.html), [`FromIterator`](https://doc.rust-lang.org/nightly/core/iter/trait.FromIterator.html) |
| ✓ | `instance : Zero (AzVector R n)` | [`zero`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.zero) |
| ≈ | `AzVector.stdBasis (i : Fin n) : AzVector R n` | [`standard_basis_vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.standard_basis_vector) |

**Reduced elements.** The constructors take the elements as they are, so a Malachite vector meant
to be read modulo $$2^k$$ must be built from reduced elements, or reduced afterwards with
[`ModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2.html), which is also the counterpart of mapping `AzZModPow2.ofAzNat k` over an `AzVector AzNat n`.
Indexing, `ofFn`, `map`, and `zip` work as on the
[integer vector page](/mapping/azurite-integer-vectors/#construction-and-access): `v.toFn i` is
`v[i]`, and the others are Rust's iterator adapters, collected into an `UnsignedVector<T>` or a
`NaturalVector`.

**Standard basis vectors.** `stdBasis` puts `1` at its index, and in the trivial ring, k = 0,
`1` is `0`, so every standard basis vector is zero. Malachite's `standard_basis_vector` always puts
the integer 1 there, which is not reduced in that ring, so for k = 0 the zero vector stands in
for it.

## Arithmetic {#arithmetic}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `instance : Add (AzVector R n)` | [`ModPowerOf2Add`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2Add.html) |
| ✓ | `instance : Sub (AzVector R n)` | [`ModPowerOf2Sub`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2Sub.html) |
| ✓ | `instance : Neg (AzVector R n)` | [`ModPowerOf2Neg`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2Neg.html) |
| ✓ | `instance : SMul α (AzVector R n)` | [`ModPowerOf2Mul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2Mul.html) |
| ✓ | `AzVector.dot (v w : AzVector R n) : R` | [`ModPowerOf2Dot`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2Dot.html) |
| ✓ | `AzVector.normSq (v : AzVector R n) : R` | [`ModPowerOf2AbsSquared`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2AbsSquared.html) |
| ✓ | `AzVector.cross (v w : AzVector R 3) : AzVector R 3` | [`ModPowerOf2Cross`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2Cross.html) |

**Operators.** Azurite's instances become the `mod_power_of_2_*` trait methods, each taking the modulus
after its operands, for example `v.mod_power_of_2_add(w, pow)` for `v + w`; each has by-value and
by-reference forms and an in-place `ModPowerOf2*Assign` form. The scalar in `mod_power_of_2_mul` is a single
reduced element, on the right: `v.mod_power_of_2_mul(c, pow)`. The dot product, the squared length,
and the cross product reduce their results modulo $$2^k$$, as Azurite's do through its
element arithmetic; the cross product follows the usual sign convention, $$v \times w = (v_1w_2 -
v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0)$$. Malachite also has
[`ModPowerOf2GeneralizedCross`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2GeneralizedCross.html), for $$n - 1$$ vectors of dimension $$n$$,
which Azurite does not.

## Strings {#strings}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `instance : ToString (AzVector R n)`, `AzVector.toChars (v : AzVector R n) : List Char` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) |
| ≈ | `AzVector.parseStr (s : String) : Option (AzVector R n)`, `AzVector.parseChars (cs : List Char) : Option (AzVector R n)` | [`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html) |
| ⚙ | `AzVector.parseElementsN : ℕ → List Char → Option (List R)` | [`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html) |

**Formatting and parsing.** Both libraries write a vector of residues as `(3, 0, 5)`, each
residue in decimal, and neither writes the modulus. As on the
[integer vector page](/mapping/azurite-integer-vectors/#strings), Azurite's parser checks the
number of elements against the dimension in the type, and it reduces each residue, as the
[element page](/mapping/azurite-mod-power-of-2/#strings) describes, so `(19, 2)` is `(3, 2)` in `AzVector (AzZModPow2 4) 2`. Malachite's parser reads any number of elements
of any size and does not reduce them; parsing and then applying [`ModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2.html) to the vector is the same
computation.

## Algebraic structure {#algebraic-structure}

| | Azurite | Malachite |
| :---: | --- | --- |
| — | `instance : AddCommMonoid (AzVector R n)`, `instance : AddCommGroup (AzVector R n)`, `instance : Module R (AzVector R n)`, `AzVector.toFnLM`, `AzVector.linearEquivPi`, `nsmulAz`, `zsmulAz` |  |

**Proof infrastructure.** `AzZModPow2 k` is a commutative ring, so `AzVector (AzZModPow2 k) n` is a module over it,
identified with `Fin n → AzZModPow2 k` so that Azurite's proofs can use Mathlib's linear algebra. Malachite
has no module abstraction; the traits above stand in for it.

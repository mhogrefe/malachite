---
layout: default
title: "Malachite for Azurite Users: Vectors Modulo a Natural"
permalink: /mapping/azurite-vectors-mod/
theme: jekyll-theme-slate
---

# Malachite for Azurite Users: Vectors Modulo a Natural

This page maps the operations of [Azurite](https://github.com/mhogrefe/azurite)'s `AzVector (AzZMod m) n`,
its vector of `n` integers modulo $$m$$, onto their Malachite counterparts, which are the
`mod_*` operations on two vector types:
[`UnsignedVector<T>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html), from the `malachite-base` crate, whose elements are primitive
unsigned integers and whose modulus therefore must be small enough that $$m$$ fits in `T`, and
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html), from the `malachite-nz` crate, whose modulus may be any size. It is a
companion of [Malachite for Azurite Users: Integers Modulo a Natural](/mapping/azurite-mod/), which maps the elements
themselves and whose [Conventions](/mapping/azurite-mod/#conventions) carry over, and of
[Malachite for Azurite Users: Integer Vectors](/mapping/azurite-integer-vectors/), whose
[Conventions](/mapping/azurite-integer-vectors/#conventions) on dimensions apply here unchanged.
The page covers Azurite as of commit `a2b4586` (2026-11-08). The [mapping index](/mapping/) lists
the whole family of pages.

Every definition of `Azurite/AzVector/` that applies to these vectors is mapped below: 20
rows across 4 sections, of which 12 are ✓, 4 ≈, 1 ⚙, and 3 —.
Theorems are not mapped.

## Conventions {#conventions}

### The types

An `AzVector (AzZMod m) n` is a vector of residues, each of which carries the modulus in its type and
is reduced by construction. Malachite has no residue type: a vector modulo $$m$$ is an
[`UnsignedVector<T>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html) or a [`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html) whose elements are already reduced, and
each operation takes the modulus as a runtime argument, `m`, as the element operations do on the
[element page](/mapping/azurite-mod/#conventions). Every one of these operations requires its vectors to be
reduced and panics otherwise; [`ModIsReduced`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModIsReduced.html) checks a vector, and [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.Mod.html) reduces one. So Azurite's
type invariant and Malachite's precondition are again the same statement, and each row below maps a
method of `AzVector` onto the trait method applied to reduced vectors, with the modulus passed
along. The traits are implemented for both vector types; an [`UnsignedVector<T>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html) suits a
modulus that fits in a word, and a [`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html) one of any size.

As on the [integer vector page](/mapping/azurite-integer-vectors/#dimensions), the dimension is
part of the type in Azurite and a runtime value in Malachite, which panics on mismatched
dimensions and out-of-range indices.

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
to be read modulo $$m$$ must be built from reduced elements, or reduced afterwards with
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.Mod.html), which is also the counterpart of mapping `AzZMod.ofAzNat m` over an `AzVector AzNat n`.
Indexing, `ofFn`, `map`, and `zip` work as on the
[integer vector page](/mapping/azurite-integer-vectors/#construction-and-access): `v.toFn i` is
`v[i]`, and the others are Rust's iterator adapters, collected into an `UnsignedVector<T>` or a
`NaturalVector`.

**Standard basis vectors.** `stdBasis` puts `1` at its index, and in the trivial ring, m = 1,
`1` is `0`, so every standard basis vector is zero. Malachite's `standard_basis_vector` always puts
the integer 1 there, which is not reduced in that ring, so for m = 1 the zero vector stands in
for it.

## Arithmetic {#arithmetic}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `instance : Add (AzVector R n)` | [`ModAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModAdd.html) |
| ✓ | `instance : Sub (AzVector R n)` | [`ModSub`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModSub.html) |
| ✓ | `instance : Neg (AzVector R n)` | [`ModNeg`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModNeg.html) |
| ✓ | `instance : SMul α (AzVector R n)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html) |
| ✓ | `AzVector.dot (v w : AzVector R n) : R` | [`ModDot`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModDot.html) |
| ✓ | `AzVector.normSq (v : AzVector R n) : R` | [`ModAbsSquared`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModAbsSquared.html) |
| ✓ | `AzVector.cross (v w : AzVector R 3) : AzVector R 3` | [`ModCross`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModCross.html) |
| ≈ | `AzVector.gramSchmidt (vs : List (AzVector R n)) : List (AzVector R n)` | [`ModGramSchmidt`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModGramSchmidt.html) |

**Operators.** Azurite's instances become the `mod_*` trait methods, each taking the modulus
after its operands, for example `v.mod_add(w, m)` for `v + w`; each has by-value and
by-reference forms and an in-place `Mod*Assign` form. The scalar in `mod_mul` is a single
reduced element, on the right: `v.mod_mul(c, m)`. The dot product, the squared length,
and the cross product reduce their results modulo $$m$$, as Azurite's do through its
element arithmetic; the cross product follows the usual sign convention, $$v \times w = (v_1w_2 -
v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0)$$. Malachite also has
[`ModGeneralizedCross`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModGeneralizedCross.html), for $$n - 1$$ vectors of dimension $$n$$,
which Azurite does not.

**Gram–Schmidt.** `AzVector.gramSchmidt` needs a field, so it applies when $$m$$ is prime. It
orthogonalizes a list of vectors, dividing by each squared length, and in a Lean field a division
by zero gives zero, so the projection onto an earlier vector whose squared length is zero modulo
$$m$$ is silently dropped. Malachite's
[`ModGramSchmidt`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModGramSchmidt.html),
on [`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
and [`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html),
works on the columns of a matrix, so the vectors are the columns rather than the list, and it
returns `None` if a squared length it needs to divide by is not invertible modulo $$m$$. Neither
normalizes the vectors.

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
[element page](/mapping/azurite-mod/#strings) describes, so `(19, 2)` is `(5, 2)` in `AzVector (AzZMod 7) 2`. Malachite's parser reads any number of elements
of any size and does not reduce them; parsing and then applying [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.Mod.html) to the vector is the same
computation.

## Algebraic structure {#algebraic-structure}

| | Azurite | Malachite |
| :---: | --- | --- |
| — | `instance : AddCommMonoid (AzVector R n)`, `instance : AddCommGroup (AzVector R n)`, `instance : Module R (AzVector R n)`, `AzVector.toFnLM`, `AzVector.linearEquivPi`, `nsmulAz`, `zsmulAz` |  |

**Proof infrastructure.** `AzZMod m` is a commutative ring, so `AzVector (AzZMod m) n` is a module over it,
identified with `Fin n → AzZMod m` so that Azurite's proofs can use Mathlib's linear algebra. Malachite
has no module abstraction; the traits above stand in for it.

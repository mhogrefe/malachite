---
layout: default
title: "Malachite for Azurite Users: Natural Vectors"
permalink: /mapping/azurite-natural-vectors/
theme: jekyll-theme-slate
---

# Malachite for Azurite Users: Natural Vectors

This page maps the operations of [Azurite](https://github.com/mhogrefe/azurite)'s `AzVector AzNat n`,
its vector of `n` multi-limb natural numbers, onto their Malachite counterparts on
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html),
from the `malachite-nz` crate. `AzVector` is generic in its element type; this page covers it with
`AzNat` elements. It is a companion of
[Malachite for Azurite Users: Naturals](/mapping/azurite-naturals/), which maps the elements
themselves and whose [Conventions](/mapping/azurite-naturals/#conventions) carry over, and of
[Malachite for Azurite Users: Integer Vectors](/mapping/azurite-integer-vectors/), whose
[Conventions](/mapping/azurite-integer-vectors/#conventions) on dimensions apply here unchanged.
The page covers Azurite as of commit `a2b4586` (2026-11-08). The [mapping index](/mapping/) lists
the whole family of pages.

Every definition of `Azurite/AzVector/` that applies to natural vectors is mapped below: 18 rows
across 4 sections, of which 11 are ✓, 2 ≈, 1 ⚙, and 4 —. Theorems are not mapped.

## Conventions {#conventions}

### The types

An `AzVector AzNat n` is a structure with one field, `data : Vector AzNat n`. A
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html)
is a single public field,
[`elements`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html#structfield.elements),
a `Vec` of
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s, and
most of its shared methods are on the
[`Vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html) trait. As
on the [integer vector page](/mapping/azurite-integer-vectors/#dimensions), the dimension is part
of the type in Azurite and a runtime value in Malachite, which panics on mismatched dimensions and
out-of-range indices.

### What does not apply

Some of `AzVector`'s generic definitions need structure that `AzNat` does not have, so they have
no natural-vector form and no rows here: the cross product `AzVector.cross` needs a ring,
negation needs `Neg`, and the `AddCommGroup` instance and `zsmulAz` need subtraction with
negatives. `AzVector.gramSchmidt` needs a field. Malachite's
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html)
likewise has no negation and no cross product.

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
| ✓ | `AzVector.stdBasis (i : Fin n) : AzVector R n` | [`standard_basis_vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.standard_basis_vector) |

**Functions and iterators.** These work as on the
[integer vector page](/mapping/azurite-integer-vectors/#construction-and-access): `v.toFn i` is
`v[i]`, and `ofFn`, `map`, and `zip` are Rust's iterator adapters collected into a
`NaturalVector`, for example `v.into_iter().map(f).collect::<NaturalVector>()`. The map to integer
vectors, `v.map AzNat.toAzInt`, is
[`From<NaturalVector>`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/conversion/from_natural_vector/index.html)
for
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html),
and the reverse, which fails on a negative element, is
[`TryFrom<IntegerVector>`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/conversion/natural_vector_from_integer_vector/index.html).

## Arithmetic {#arithmetic}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `instance : Add (AzVector R n)` | [`Add`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/arithmetic/add/index.html) |
| — | `instance : Sub (AzVector R n)` | [`SaturatingSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SaturatingSub.html) |
| ✓ | `instance : SMul α (AzVector R n)` | [`Mul<Natural>`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/arithmetic/scalar_mul/index.html) |
| ✓ | `AzVector.dot (v w : AzVector R n) : R` | [`Dot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Dot.html) |
| ✓ | `AzVector.normSq (v : AzVector R n) : R` | [`AbsSquared`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsSquared.html) |
| ✓ | `AzNat.dotSign (v w : AzVector AzNat n) : Ordering` | [`DotSign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DotSign.html) |

**Subtraction.** `AzVector`'s `Sub` instance subtracts entrywise with `AzNat`'s subtraction, which
truncates to zero as Lean's `Nat` subtraction does, so `(1, 5) - (3, 2)` is `(0, 3)`. Malachite's
`NaturalVector` deliberately has no subtraction; the entrywise truncated difference is
`v.into_iter().zip(w).map(|(x, y)| x.saturating_sub(y)).collect::<NaturalVector>()`, with
[`SaturatingSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SaturatingSub.html)
on the elements as on the [naturals page](/mapping/azurite-naturals/), and a difference that may
be negative is a subtraction of
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html)s.

**Operators.** Addition and scalar multiplication take vectors by value or by reference, and each
has an in-place form, `+=` and `*=`. The scalar is a `Natural`, on either side, `v * c` or `c * v`;
Azurite's `SMul` instance scales by any type that acts on the elements.

**Signs of dot products.** `AzNat.dotSign` and `DotSign` both return the
[`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html) of $$v \cdot w$$
against zero. Every term of a natural dot product is nonnegative, so the sign is `Greater` exactly
when some coordinate is nonzero in both vectors and `Equal` otherwise, and both libraries find it
in a single pass, with no multiplication. Malachite's oracle tests check `DotSign` against
`AzNat.dotSign`.

## Strings {#strings}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `instance : ToString (AzVector R n)`, `AzVector.toChars (v : AzVector R n) : List Char` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) |
| ≈ | `AzVector.parseStr (s : String) : Option (AzVector R n)`, `AzVector.parseChars (cs : List Char) : Option (AzVector R n)` | [`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html) |
| ⚙ | `AzVector.parseElementsN : ℕ → List Char → Option (List R)` | [`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html) |

**Formatting and parsing.** As on the [integer vector page](/mapping/azurite-integer-vectors/#strings),
both libraries write `(1, 2, 3)`, and the empty vector `()`. The elements are written and parsed
as on the [naturals page](/mapping/azurite-naturals/). Azurite's parser checks the number of
elements against the dimension in the type and returns `none` if they differ; Malachite's reads
any number of elements and returns an error for a malformed string.

## Algebraic structure {#algebraic-structure}

| | Azurite | Malachite |
| :---: | --- | --- |
| — | `instance : AddCommMonoid (AzVector R n)`, `instance : Module R (AzVector R n)`, `AzVector.toFnLM`, `AzVector.linearEquivPi`, `nsmulAz` | |

**Proof infrastructure.** These noncomputable instances and maps make `AzVector AzNat n` an
`AzNat`-module, since `AzNat` is a commutative semiring, and identify it with `Fin n → AzNat`, so
that Azurite's proofs can use Mathlib's linear algebra. Malachite has no module abstraction; the
operators and traits above stand in for it.

---
layout: default
title: "Malachite for Azurite Users: Integer Vectors"
permalink: /mapping/azurite-integer-vectors/
theme: jekyll-theme-slate
---

# Malachite for Azurite Users: Integer Vectors

This page maps the operations of [Azurite](https://github.com/mhogrefe/azurite)'s `AzVector AzInt n`,
its vector of `n` signed multi-limb integers, onto their Malachite counterparts on
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html),
from the `malachite-nz` crate. `AzVector` is generic in its element type; this page covers it with
`AzInt` elements, and other element types will have pages of their own. It is a companion of
[Malachite for Azurite Users: Integers](/mapping/azurite-integers/), which maps the elements
themselves and whose [Conventions](/mapping/azurite-integers/#conventions) carry over. The page
covers Azurite as of commit `a2b4586` (2026-11-08). The [mapping index](/mapping/) lists the whole
family of pages.

As on the other Azurite pages, the sections are organized by theme, since `AzVector`'s operations
are definitions spread across the files of `Azurite/AzVector/`.

Every definition of `Azurite/AzVector/` that applies to integer vectors is mapped below: 21 rows
across 4 sections, of which 14 are ✓, 2 ≈, 2 ⚙, and 3 —. Theorems are not mapped.

## Conventions {#conventions}

### The types

An `AzVector R n` is a structure with one field, `data : Vector R n`, Lean's array-backed vector of
length `n`. An
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html)
is likewise a single public field,
[`elements`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#structfield.elements),
a `Vec` of
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)s. Most of
the methods shared by Malachite's vector types are on the
[`Vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html) trait.

### Dimensions

In Azurite the dimension is part of the type: `AzVector AzInt 3` and `AzVector AzInt 4` are
different types, so adding vectors of different dimensions, or taking the cross product of vectors
that are not 3-dimensional, does not typecheck, and indices are `Fin n`, which cannot be out of
range. In Malachite the dimension is a runtime value,
[`dimension`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.dimension),
so these mistakes are caught when the function runs: Malachite panics on vectors of different
dimensions, on a cross product of vectors that are not 3-dimensional, and on an index that is out
of range. Constructors that take the dimension from the type in Azurite, such as the zero vector,
take it as an argument in Malachite.

### What is not mapped

`AzVector.gramSchmidt` requires a field, so it does not apply to integer vectors; it will be
mapped on the rational vector page. The product of a matrix and a vector, `AzMatrix.mulVec`, will
be mapped on a matrix page. The theorems that relate `AzVector` to Mathlib's `Fin n → R`, such as
`dot_eq_dotProduct` and `toFn_add`, are not mapped, since Malachite has no proofs.

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

**Functions and iterators.** `toFn` turns a vector into a function on `Fin n`, the form Mathlib's
linear algebra works with; in Malachite a vector is read by indexing, `v[i]`, so `v.toFn i`
becomes `v[i]` and the function is the closure `|i| &v[i]`. Building a vector from a function, and
mapping or zipping vectors, are Rust's iterator adapters: `ofFn f` is
`(0..n).map(f).collect::<IntegerVector>()`, `v.map f` is
`v.into_iter().map(f).collect::<IntegerVector>()`, and `v.zip f w` is
`v.into_iter().zip(w).map(|(x, y)| f(x, y)).collect::<IntegerVector>()`, which, unlike `zip`,
silently stops at the shorter of two vectors of different dimensions. A map from `AzNat` vectors,
`v.map AzNat.toAzInt`, is
[`From<NaturalVector>`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/conversion/from_natural_vector/index.html).

## Arithmetic {#arithmetic}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `instance : Add (AzVector R n)` | [`Add`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/add/index.html) |
| ✓ | `instance : Sub (AzVector R n)` | [`Sub`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/sub/index.html) |
| ✓ | `instance : Neg (AzVector R n)` | [`Neg`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/neg/index.html) |
| ✓ | `instance : SMul α (AzVector R n)` | [`Mul<Integer>`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/scalar_mul/index.html) |
| ✓ | `AzVector.dot (v w : AzVector R n) : R` | [`Dot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Dot.html) |
| ✓ | `AzVector.normSq (v : AzVector R n) : R` | [`AbsSquared`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsSquared.html) |
| ✓ | `AzVector.cross (v w : AzVector R 3) : AzVector R 3` | [`Cross`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Cross.html) |
| ✓ | `AzInt.dotSign (v w : AzVector AzInt n) : Ordering` | [`DotSign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DotSign.html) |
| ⚙ | `AzInt.extractMax`, `AzInt.sumProducts`, `AzInt.maxSize`, `AzInt.compareSumsGeneral`, `AzInt.compareSums`, `AzInt.splitTerms`, `AzInt.dotTerms` | [`DotSign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DotSign.html) |

**Operators.** The operators take vectors by value or by reference in Malachite, and each has an
in-place form, such as `+=` or
[`NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegAssign.html).
Azurite's `SMul` instance scales by any type that acts on the elements; for integer vectors the
scalar is usually an `AzInt`, which is Malachite's `Mul<Integer>`, available on either side,
`v * c` or `c * v`. The dot product and the cross product take the same inputs and give the same
results in both libraries, the cross product with the usual sign convention, $$v \times w =
(v_1w_2 - v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0)$$. Malachite also has
[`GeneralizedCross`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.GeneralizedCross.html),
for $$n - 1$$ vectors of dimension $$n$$, which Azurite does not.

**Signs of dot products.** `AzInt.dotSign` and `DotSign` both return the
[`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html) of $$v \cdot w$$
against zero, and both avoid computing the dot product where they can: the terms are split by
sign, and screens on bit lengths often settle the comparison without multiplying. Malachite's
oracle tests check `DotSign` against `AzInt.dotSign`. The helpers that `dotSign` is built from are
not exposed in Malachite; they are listed under ⚙ because `DotSign` is what to call instead.

## Strings {#strings}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `instance : ToString (AzVector R n)`, `AzVector.toChars (v : AzVector R n) : List Char` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) |
| ≈ | `AzVector.parseStr (s : String) : Option (AzVector R n)`, `AzVector.parseChars (cs : List Char) : Option (AzVector R n)` | [`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html) |
| ⚙ | `AzVector.parseElementsN : ℕ → List Char → Option (List R)` | [`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html) |

**Formatting and parsing.** Both libraries write a vector as its elements in parentheses,
separated by a comma and a space, `(1, -2, 3)`, and the empty vector as `()`. The elements are
written and parsed as on the [integers page](/mapping/azurite-integers/#strings). Azurite's parser
checks the number of elements against the dimension in the type and returns `none` if they
differ; Malachite's reads any number of elements and returns an error, rather than `None`, for a
malformed string. `parseElementsN` is the helper that reads the elements between the
parentheses.

## Algebraic structure {#algebraic-structure}

| | Azurite | Malachite |
| :---: | --- | --- |
| — | `instance : AddCommMonoid (AzVector R n)`, `instance : AddCommGroup (AzVector R n)`, `instance : Module R (AzVector R n)`, `AzVector.toFnLM`, `AzVector.linearEquivPi`, `nsmulAz`, `zsmulAz` | |

**Proof infrastructure.** These noncomputable instances and maps exist so that Azurite's proofs can
use Mathlib's linear algebra: they make `AzVector R n` an `R`-module and identify it with
`Fin n → R`. Malachite has no ring or module abstraction, as on the
[integers page](/mapping/azurite-integers/#typeclasses-and-traits); the operators and traits above
stand in for them.

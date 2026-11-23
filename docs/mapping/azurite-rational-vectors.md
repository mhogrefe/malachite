---
layout: default
title: "Malachite for Azurite Users: Rational Vectors"
permalink: /mapping/azurite-rational-vectors/
theme: jekyll-theme-slate
---

# Malachite for Azurite Users: Rational Vectors

This page maps the operations of [Azurite](https://github.com/mhogrefe/azurite)'s `AzVector AzRat n`,
its vector of `n` rationals, onto their Malachite counterparts on
[`RationalVector`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html), from the `malachite-q` crate. `AzVector` is generic in its element type;
this page covers it with `AzRat` elements. It is a companion of
[Malachite for Azurite Users: Rationals](/mapping/azurite-rationals/), which maps the elements
themselves and whose [Conventions](/mapping/azurite-rationals/#conventions) carry over, and of
[Malachite for Azurite Users: Integer Vectors](/mapping/azurite-integer-vectors/), whose
[Conventions](/mapping/azurite-integer-vectors/#conventions) on dimensions apply here unchanged.
The page covers Azurite as of commit `a2b4586` (2026-11-08). The [mapping index](/mapping/) lists
the whole family of pages.

Every definition of `Azurite/AzVector/` that applies to rational vectors is mapped below:
22 rows across 4 sections, of which 14 are ✓, 3 ≈, 2 ⚙, and 3 —. Theorems are not mapped.

## Conventions {#conventions}

### The types

An `AzVector AzRat n` is a structure with one field, `data : Vector AzRat n`. A
[`RationalVector`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html) is a single public field,
[`elements`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#structfield.elements), a `Vec` of
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)s, and
most of its shared methods are on the [`Vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html) trait. As on the
[integer vector page](/mapping/azurite-integer-vectors/#dimensions), the dimension is part of the
type in Azurite and a runtime value in Malachite, which panics on mismatched dimensions and
out-of-range indices.

`AzRat` is a field, so every one of `AzVector`'s generic definitions applies to rational vectors,
including the Gram–Schmidt process, which needs one.

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
`RationalVector`. The maps from integer and natural vectors, `v.map AzInt.toAzRat` and its
`AzNat` analog, are
[`From<IntegerVector>`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/from_integer_vector/index.html) and
[`From<NaturalVector>`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/from_natural_vector/index.html); the reverse, which fails
on a non-integer element, is
[`TryFrom<RationalVector>`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/integer_vector_from_rational_vector/index.html) for
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html).
Malachite can also split a vector into integer numerators over one common denominator,
[`to_numerators_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/to_numerators_and_denominator/index.html),
which Azurite does not.

## Arithmetic {#arithmetic}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `instance : Add (AzVector R n)` | [`Add`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/add/index.html) |
| ✓ | `instance : Sub (AzVector R n)` | [`Sub`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/sub/index.html) |
| ✓ | `instance : Neg (AzVector R n)` | [`Neg`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/neg/index.html) |
| ✓ | `instance : SMul α (AzVector R n)` | [`Mul<Rational>`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/scalar_mul/index.html) |
| ✓ | `AzVector.dot (v w : AzVector R n) : R` | [`Dot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Dot.html) |
| ✓ | `AzVector.normSq (v : AzVector R n) : R` | [`AbsSquared`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsSquared.html) |
| ✓ | `AzVector.cross (v w : AzVector R 3) : AzVector R 3` | [`Cross`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Cross.html) |
| ✓ | `AzRat.dotSign (v w : AzVector AzRat n) : Ordering` | [`DotSign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DotSign.html) |
| ⚙ | `Frac`, `Frac.sum`, `Term`, `Term.toFrac`, `Term.exp`, `maxExp`, `compareSumsGeneral`, `compareSums`, `splitTerms` | [`DotSign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DotSign.html) |
| ≈ | `AzVector.gramSchmidt (vs : List (AzVector R n)) : List (AzVector R n)` | [`GramSchmidt`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.GramSchmidt.html) |

**Operators.** The operators take vectors by value or by reference, and each has an in-place
form. The scalar is a `Rational`, on either side, `v * c` or `c * v`; Azurite's `SMul` instance
scales by any type that acts on the elements. Malachite also divides a vector by a scalar, `v / c`,
and has
[`GeneralizedCross`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.GeneralizedCross.html), for $$n - 1$$ vectors of dimension $$n$$, neither
of which Azurite has. The cross product follows the usual sign convention, $$v \times w = (v_1w_2 -
v_2w_1, v_2w_0 - v_0w_2, v_0w_1 - v_1w_0)$$.

**Signs of dot products.** `AzRat.dotSign` and `DotSign` both return the
[`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html) of $$v \cdot w$$
against zero, and both try to avoid forming the exact dot product: the terms are split by sign,
and screens on the bit lengths of the numerators and denominators often settle the comparison.
When they do not, Azurite sums each side as an unreduced fraction and compares the two by cross
multiplication. Malachite's oracle tests check `DotSign` against `AzRat.dotSign`. The helpers that
`dotSign` is built from, including the unreduced-fraction type `Frac`, are not exposed in
Malachite; they are listed under ⚙ because `DotSign` is what to call instead.

**Gram–Schmidt.** `AzVector.gramSchmidt` takes a list of vectors and returns the list of their
orthogonalizations, each vector minus its projections onto the earlier results; Malachite's
[`GramSchmidt`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.GramSchmidt.html), on
[`RationalMatrix`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html), does the same to the columns of a matrix, so the vectors are its columns
rather than a list. Neither normalizes the results, which would need square roots. The two treat
dependent vectors alike: a vector that depends on the earlier ones becomes zero, and projections
onto a zero vector are left out, explicitly in Malachite and, in Azurite, because Lean's division
by zero gives zero. So the results agree.

## Strings {#strings}

| | Azurite | Malachite |
| :---: | --- | --- |
| ✓ | `instance : ToString (AzVector R n)`, `AzVector.toChars (v : AzVector R n) : List Char` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) |
| ≈ | `AzVector.parseStr (s : String) : Option (AzVector R n)`, `AzVector.parseChars (cs : List Char) : Option (AzVector R n)` | [`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html) |
| ⚙ | `AzVector.parseElementsN : ℕ → List Char → Option (List R)` | [`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html) |

**Formatting and parsing.** As on the [integer vector page](/mapping/azurite-integer-vectors/#strings),
both libraries write `(1/2, -3, 0)`, and the empty vector `()`. The elements are written and
parsed as on the [rationals page](/mapping/azurite-rationals/#strings-and-scientific-notation).
Azurite's parser checks the number of elements against the dimension in the type and returns
`none` if they differ; Malachite's reads any number of elements and returns an error for a
malformed string.

## Algebraic structure {#algebraic-structure}

| | Azurite | Malachite |
| :---: | --- | --- |
| — | `instance : AddCommMonoid (AzVector R n)`, `instance : AddCommGroup (AzVector R n)`, `instance : Module R (AzVector R n)`, `AzVector.toFnLM`, `AzVector.linearEquivPi`, `nsmulAz`, `zsmulAz` |  |

**Proof infrastructure.** These noncomputable instances and maps make `AzVector AzRat n` a vector
space over `AzRat`, identified with `Fin n → AzRat`, so that Azurite's proofs can use Mathlib's
linear algebra; `Equiv/GramSchmidt.lean` uses them to prove that `gramSchmidt` returns pairwise
orthogonal vectors. Malachite has no module abstraction; the operators and traits above stand in
for it.

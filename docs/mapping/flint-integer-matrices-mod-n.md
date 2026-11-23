---
layout: default
title: "Malachite for FLINT Users: Integer Matrices mod n"
permalink: /mapping/flint-integer-matrices-mod-n/
theme: jekyll-theme-slate
---

# Malachite for FLINT Users: Integer Matrices mod n

This page maps the functions of FLINT's `fmpz_mod_mat.h`, matrices over the integers mod $$n$$,
onto their Malachite counterpart:
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html),
from the `malachite-nz` crate, with the `Mod*` traits. It follows the organization of the
[fmpz_mod_mat.h chapter](https://flintlib.org/doc/fmpz_mod_mat.html) of the FLINT manual, as of
FLINT 3.6.0. It is a companion to [Malachite for FLINT Users: Integer
Matrices](/mapping/flint-integer-matrices/), whose
[conventions](/mapping/flint-integer-matrices/#conventions) apply here as well, and to [Malachite
for FLINT Users: Integers mod n](/mapping/flint-integers-mod-n/); the [mapping index](/mapping/)
lists the whole family.

Every section of the chapter is mapped below: 57 rows across 23 sections, of which 33 are ✓,
18 ≈, 5 —, and 1 ✗. (The manual's "Basic manipulation" heading is misformatted, so it renders as
part of Memory management; this page gives it its own section.) Functions whose names begin with
an underscore are omitted, as on [the fmpq page](/mapping/flint-rationals/), and functions that
`fmpz_mod_mat.h` declares but the chapter does not document are mentioned in the notes of the
sections they belong to.

## Conventions {#conventions}

### Representation {#representation}

In FLINT 3.6.0, `fmpz_mod_mat_struct` is not a type of its own: `fmpz_mod_types.h` defines it as
`fmpz_mat_struct`, so an `fmpz_mod_mat_t` is an [integer
matrix](/mapping/flint-integer-matrices/#representation) whose entries are residues in $$[0, n)$$.
The modulus is not stored in the matrix; every function takes it in a separate `fmpz_mod_ctx_t`,
as the [`fmpz_mod`](/mapping/flint-integers-mod-n/#conventions) functions do. As the chapter's
introduction says, the entries may be written directly, and it is then the caller's
responsibility to keep them in $$[0, n)$$.

The Malachite counterpart is a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
whose elements are all less than $$n$$, which
[`ModIsReduced`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModIsReduced.html)
checks. The `Mod*` traits take the modulus as an ordinary
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
argument, in place of the context. FLINT trusts that its inputs are reduced; Malachite's modular
matrix functions check, and panic if any element of an operand is not less than $$n$$.

For a modulus that fits in a machine word, FLINT has a separate module, `nmod_mat`, whose
counterpart is
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html),
with the same `Mod*` traits and a primitive modulus.

### Categories

The categories are those of [the integer matrix
page](/mapping/flint-integer-matrices/#categories): ✓, ≈, ⚙, —, and ✗.

## [Types, macros and constants](https://flintlib.org/doc/fmpz_mod_mat.html#types-macros-and-constants) {#types-macros-and-constants}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `fmpz_mod_mat_struct` | [`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html) |
| — | `fmpz_mod_mat_t` | |

`fmpz_mod_mat_t` is the pass-by-reference device that `&` and `&mut` already are. Since
`fmpz_mod_mat_struct` is `fmpz_mat_struct`, the same matrix can be passed to `fmpz_mat` and
`fmpz_mod_mat` functions alike; in Malachite, a reduced `NaturalMatrix` converts to an
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html)
with [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html), and back with
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.Mod.html)
or [`TryFrom`](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html).

## [Element access](https://flintlib.org/doc/fmpz_mod_mat.html#element-access) {#element-access}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `fmpz * fmpz_mod_mat_entry(const fmpz_mod_mat_t mat, slong i, slong j)` | [`Index`](https://doc.rust-lang.org/nightly/core/ops/trait.Index.html), [`IndexMut`](https://doc.rust-lang.org/nightly/core/ops/trait.IndexMut.html), [`get`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html#method.get), [`get_mut`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html#method.get_mut) |
| ✓ | `void fmpz_mod_mat_set_entry(fmpz_mod_mat_t mat, slong i, slong j, const fmpz_t val, const fmpz_mod_ctx_t ctx)` | [`IndexMut`](https://doc.rust-lang.org/nightly/core/ops/trait.IndexMut.html) |

As for integer matrices (see [the integer matrix
page](/mapping/flint-integer-matrices/#basic-assignment-and-manipulation)), `fmpz_mod_mat_entry`
is indexing, `a[(i, j)]`, which panics on an index out of range, or
[`get`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html#method.get)
and
[`get_mut`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html#method.get_mut),
which return
[`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None)
instead.
`fmpz_mod_mat_set_entry` is the assignment `a[(i, j)] = val`. It does not reduce `val`: the
context argument is unused, and the caller must pass a value in $$[0, n)$$, as it must when
assigning to a `NaturalMatrix` element that will later be used with the `Mod*` traits, which
check. `fmpz_mod_mat.h` also declares an undocumented `fmpz_mod_mat_get_entry`, which copies an
entry out, and a `fmpz_mod_mat_row` that points to the start of a row; their counterparts are
`a[(i, j)].clone()` and
[`row_iter`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html#method.row_iter).

## [Memory management](https://flintlib.org/doc/fmpz_mod_mat.html#memory-management) {#memory-management}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_mat_init(fmpz_mod_mat_t mat, slong rows, slong cols, const fmpz_mod_ctx_t ctx)` | [`zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.zero) |
| ✓ | `void fmpz_mod_mat_init_set(fmpz_mod_mat_t mat, const fmpz_mod_mat_t src, const fmpz_mod_ctx_t ctx)` | [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) |
| — | `void fmpz_mod_mat_clear(fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx)` | |

All three call their `fmpz_mat` counterparts (see [the integer matrix
page](/mapping/flint-integer-matrices/#memory-management)) and ignore the context: `init` gives
the zero matrix, `NaturalMatrix::zero(rows, cols)`, `init_set` is
[`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html), and `clear` is
[`Drop`](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html). The manual says that
`init_set` copies the modulus along with the shape and the entries; since the matrix no longer
stores a modulus (see [Representation](#representation)), there is nothing to copy, and the
modulus stays with the context.

In the manual's source, the next section's heading, "Basic manipulation", is not formatted as a
heading, so the website shows its functions as part of this section. They are mapped below under
[Basic manipulation](#basic-manipulation), as the chapter intends.

## [Basic manipulation](https://flintlib.org/doc/fmpz_mod_mat.html#memory-management) {#basic-manipulation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `slong fmpz_mod_mat_nrows(const fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx)` | [`height`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.height) |
| ✓ | `slong fmpz_mod_mat_ncols(const fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx)` | [`width`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.width) |
| ≈ | `void fmpz_mod_mat_one(fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx)` | [`rectangular_identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.rectangular_identity), [`identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.identity) |
| ✓ | `void fmpz_mod_mat_zero(fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx)` | [`set_zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_zero) |
| ✓ | `void fmpz_mod_mat_swap(fmpz_mod_mat_t mat1, fmpz_mod_mat_t mat2, const fmpz_mod_ctx_t ctx)` | [`mem::swap`](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html) |
| — | `void fmpz_mod_mat_swap_entrywise(fmpz_mod_mat_t mat1, fmpz_mod_mat_t mat2, const fmpz_mod_ctx_t ctx)` | |
| ✓ | `int fmpz_mod_mat_is_empty(const fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx)` | [`is_empty`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_empty) |
| ✓ | `int fmpz_mod_mat_is_square(const fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx)` | [`is_square`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_square) |

The heading above links to the section the website shows these functions in, since the misformatted
heading gives them no section of their own (see [Memory management](#memory-management)). They
are the `fmpz_mat` functions (see [the integer matrix
page](/mapping/flint-integer-matrices/#basic-assignment-and-manipulation)), with the context
ignored, except in `fmpz_mod_mat_one`: it gives the truncated identity of any shape, and the zero
matrix when $$n = 1$$, since then $$1 \equiv 0$$. Its counterpart is
[`rectangular_identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.rectangular_identity) (or
[`identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.identity)
for a square matrix), which puts ones on the diagonal whatever the modulus, so for $$n = 1$$ use
[`zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.zero)
instead, hence ≈. `fmpz_mod_mat_swap_entrywise` matters only for windows, which Malachite does not
have.

The section also documents two underscore functions. `_fmpz_mod_mat_reduce`, which reduces every
entry modulo $$n$$, is
[`ModAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModAssign.html).
`_fmpz_mod_mat_set_mod` does not exist in FLINT 3.6.0: it is documented, but neither declared nor
defined, a remnant of the time when the matrix stored its modulus.

## [Random generation](https://flintlib.org/doc/fmpz_mod_mat.html#random-generation) {#random-generation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpz_mod_mat_randtest(fmpz_mod_mat_t mat, flint_rand_t state, const fmpz_mod_ctx_t ctx)` | [`random_natural_matrices_with_dimensions_from_iterator`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/random/fn.random_natural_matrices_with_dimensions_from_iterator.html), [`striped_random_natural_range`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.striped_random_natural_range.html) |
| ≈ | `void fmpz_mod_mat_randfull(fmpz_mod_mat_t mat, flint_rand_t state, const fmpz_mod_ctx_t ctx)` | [`random_natural_matrices_with_dimensions_from_iterator`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/random/fn.random_natural_matrices_with_dimensions_from_iterator.html), [`striped_random_natural_range`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.striped_random_natural_range.html) |
| ✓ | `void fmpz_mod_mat_rand(fmpz_mod_mat_t mat, flint_rand_t state, const fmpz_mod_ctx_t ctx)` | [`random_natural_matrices_with_dimensions_from_iterator`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/random/fn.random_natural_matrices_with_dimensions_from_iterator.html), [`random_naturals_less_than`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.random_naturals_less_than.html) |

Malachite's generators are infinite iterators of matrices of a given shape, which take their
elements from any iterator. For residues modulo $$n$$, pass an iterator of naturals less than
$$n$$. `fmpz_mod_mat_rand` draws each entry uniformly from $$[0, n)$$, which
[`random_naturals_less_than`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.random_naturals_less_than.html)
does too. `fmpz_mod_mat_randtest` fills the matrix with
[`fmpz_mat_randtest`](/mapping/flint-integer-matrices/#random-matrix-generation) at the bit size of
$$n$$, including its half-the-time sparse mode, and reduces the entries modulo $$n$$; its closest
counterpart is the striped generator
[`striped_random_natural_range`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.striped_random_natural_range.html)
on $$[0, n)$$, which favors values with long runs of equal bits, and so values near 0 and near
$$n$$, hence ≈.

Two of FLINT's implementations do not do what the manual says. `fmpz_mod_mat/randfull.c`, which is
documented to give entries "likely to be close to the modulus", draws each entry uniformly and
then replaces every nonzero entry by 1, so its entries are all 0 or 1; for the documented intent,
entries near the modulus to test overflow, use the striped generator, as for `randtest`. And both
`rand.c` and `randfull.c` take the number of columns from `fmpz_mod_mat_nrows`, so for a matrix that
is not square they leave columns unset or write past the end of the matrix. (The description of
`rand`, copied from `randfull`, also says "close to the modulus", though it is uniform.)
`fmpz_mod_mat.h` also declares four undocumented generators: `randrank` and `randops`, the modular
versions of the [integer matrix
generators](/mapping/flint-integer-matrices/#random-matrix-generation) of those names, and
`randtril` and `randtriu`, which give random lower and upper triangular matrices. Their
counterparts, iterators over `NaturalMatrix`s like those on [the `nmod_mat`
page](/mapping/flint-word-sized-integer-matrices-mod-n/#random-matrix-generation), are
[`random_natural_matrices_with_rank_mod`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/random/fn.random_natural_matrices_with_rank_mod.html),
[`random_natural_matrices_from_elementary_operations_mod`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/random/fn.random_natural_matrices_from_elementary_operations_mod.html),
[`random_natural_lower_triangular_matrices_mod`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/random/fn.random_natural_lower_triangular_matrices_mod.html),
and
[`random_natural_upper_triangular_matrices_mod`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/random/fn.random_natural_upper_triangular_matrices_mod.html);
[`random_natural_matrices_with_permuted_diagonal`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/random/fn.random_natural_matrices_with_permuted_diagonal.html)
places a given diagonal.

## [Windows and concatenation](https://flintlib.org/doc/fmpz_mod_mat.html#windows-and-concatenation) {#windows-and-concatenation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpz_mod_mat_window_init(fmpz_mod_mat_t window, const fmpz_mod_mat_t mat, slong r1, slong c1, slong r2, slong c2, const fmpz_mod_ctx_t ctx)` | [`window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.window), [`into_window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.into_window), [`set_window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_window), [`set_window_owned`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_window_owned) |
| — | `void fmpz_mod_mat_window_clear(fmpz_mod_mat_t window, const fmpz_mod_ctx_t ctx)` | |
| ✓ | `void fmpz_mod_mat_concat_horizontal(fmpz_mod_mat_t res, const fmpz_mod_mat_t mat1, const fmpz_mod_mat_t mat2, const fmpz_mod_ctx_t ctx)` | [`concat_horizontal`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.concat_horizontal), [`into_concat_horizontal`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.into_concat_horizontal) |
| ✓ | `void fmpz_mod_mat_concat_vertical(fmpz_mod_mat_t res, const fmpz_mod_mat_t mat1, const fmpz_mod_mat_t mat2, const fmpz_mod_ctx_t ctx)` | [`concat_vertical`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.concat_vertical), [`into_concat_vertical`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.into_concat_vertical) |

All four call their `fmpz_mat` counterparts and ignore the context, so the integer matrix page's
notes on [windows](/mapping/flint-integer-matrices/#window) and
[concatenation](/mapping/flint-integer-matrices/#concatenate) apply unchanged. Malachite has no
windows that share their parent's elements, because a matrix always owns its elements; the
[`window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.window)
and
[`set_window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_window)
families copy a block out of a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
and back, hence ≈. Copying keeps the elements reduced, so no modulus is involved, and the same
holds for concatenation, which panics if the shapes do not fit. The manual swaps the two
descriptions: it says that `concat_horizontal` stacks the matrices vertically and that
`concat_vertical` places them side by side, while the code calls `fmpz_mat_concat_horizontal` and
`fmpz_mat_concat_vertical` respectively, which do what their names say.

## [Input and output](https://flintlib.org/doc/fmpz_mod_mat.html#input-and-output) {#input-and-output}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpz_mod_mat_print_pretty(const fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) |

`fmpz_mod_mat_print_pretty` calls `fmpz_mat_print_pretty`, whose format, with each row in brackets
on its own line and the closing bracket on a line of its own, is described on [the integer matrix
page](/mapping/flint-integer-matrices/#input-and-output), as is Malachite's one-line
[`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) format, hence ≈.
Neither prints the modulus. `fmpz_mod_mat.h` also declares undocumented `fmpz_mod_mat_fprint`,
`fmpz_mod_mat_fprint_pretty`, and `fmpz_mod_mat_print`, which likewise call their `fmpz_mat`
counterparts.

## [Comparison](https://flintlib.org/doc/fmpz_mod_mat.html#comparison) {#comparison}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpz_mod_mat_equal(const fmpz_mod_mat_t mat1, const fmpz_mod_mat_t mat2, const fmpz_mod_ctx_t ctx)` | [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) |
| ✓ | `int fmpz_mod_mat_is_zero(const fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx)` | [`is_zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_zero) |

Both call their `fmpz_mat` counterparts (see [the integer matrix
page](/mapping/flint-integer-matrices/#comparison)). The manual's "the moduli are ignored" is a
remnant of the time when each matrix stored a modulus; now there is only the context, which
`equal` does not use. `fmpz_mod_mat.h` also declares an undocumented `fmpz_mod_mat_is_one`, which
is `fmpz_mat_is_one`, accepting the truncated identity of any shape, except that it returns 1 for
every matrix when $$n = 1$$, since then every matrix is the identity; the header marks its context
argument as unused, though the definition reads it. Its counterpart is
[`is_rectangular_identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_rectangular_identity),
with $$n = 1$$ handled separately.

## [Set and transpose](https://flintlib.org/doc/fmpz_mod_mat.html#set-and-transpose) {#set-and-transpose}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_mat_set(fmpz_mod_mat_t B, const fmpz_mod_mat_t A, const fmpz_mod_ctx_t ctx)` | [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) |
| ✓ | `void fmpz_mod_mat_transpose(fmpz_mod_mat_t B, const fmpz_mod_mat_t A, const fmpz_mod_ctx_t ctx)` | [`transpose`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.transpose), [`transpose_assign`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.transpose_assign) |

Both call their `fmpz_mat` counterparts and ignore the context (see [the integer matrix
page](/mapping/flint-integer-matrices/#basic-assignment-and-manipulation) and its
[Transpose](/mapping/flint-integer-matrices/#transpose) section). Transposition keeps the entries
reduced, so no modulus is involved; as there, FLINT can transpose in place only a square matrix,
while
[`transpose_assign`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.transpose_assign)
handles any shape.

## [Conversions](https://flintlib.org/doc/fmpz_mod_mat.html#conversions) {#conversions}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_mat_set_fmpz_mat(fmpz_mod_mat_t A, const fmpz_mat_t B, const fmpz_mod_ctx_t ctx)` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.Mod.html) |
| ✓ | `void fmpz_mod_mat_get_fmpz_mat(fmpz_mat_t A, const fmpz_mod_mat_t B, const fmpz_mod_ctx_t ctx)` | [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) |

`set_fmpz_mat` reduces every entry of an integer matrix into $$[0, n)$$, splitting the rows among
threads for large matrices. In Malachite this is
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.Mod.html)
of an
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html)
by a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
modulus, `a.mod_op(n)`, which returns a `NaturalMatrix`, single-threaded. `get_fmpz_mat` copies the
residues into an integer matrix unchanged, the lift into $$[0, n)$$; that is
[`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)`<NaturalMatrix>` for
`IntegerMatrix`, and the balanced lift into $$(-n/2, n/2]$$ is
[`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.BalancedMod.html).
`fmpz_mod_mat.h` also declares an undocumented `fmpz_mod_mat_set_nmod_mat`, which copies a matrix of
word-sized residues; its counterpart is
[`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)`<UnsignedMatrix<T>>` for
`NaturalMatrix`.

## [Addition and subtraction](https://flintlib.org/doc/fmpz_mod_mat.html#addition-and-subtraction) {#addition-and-subtraction}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_mat_add(fmpz_mod_mat_t C, const fmpz_mod_mat_t A, const fmpz_mod_mat_t B, const fmpz_mod_ctx_t ctx)` | [`ModAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModAdd.html), [`ModAddAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModAddAssign.html) |
| ✓ | `void fmpz_mod_mat_sub(fmpz_mod_mat_t C, const fmpz_mod_mat_t A, const fmpz_mod_mat_t B, const fmpz_mod_ctx_t ctx)` | [`ModSub`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModSub.html), [`ModSubAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModSubAssign.html) |
| ✓ | `void fmpz_mod_mat_neg(fmpz_mod_mat_t B, const fmpz_mod_mat_t A, const fmpz_mod_ctx_t ctx)` | [`ModNeg`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModNeg.html), [`ModNegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModNegAssign.html) |

Each function applies the vector operation `_fmpz_mod_vec_add`, `_fmpz_mod_vec_sub`, or
`_fmpz_mod_vec_neg` to every row (see [the integer vectors mod n
page](/mapping/flint-integer-vectors-mod-n/#arithmetic)). On
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html),
`a.mod_add(b, m)` takes the modulus as an argument, and the matrices and the modulus may each be
passed by value or by reference. The aliased calls `fmpz_mod_mat_add(A, A, B, ctx)` and
`fmpz_mod_mat_neg(A, A, ctx)` are the `*_assign` forms: `a.mod_add_assign(&b, &m)`. FLINT takes
the shape from `A` and checks neither the other shapes nor whether the entries are reduced;
Malachite panics if the shapes differ or if any element is not less than $$m$$. The power-of-2
forms,
[`ModPowerOf2Add`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2Add.html),
[`ModPowerOf2Sub`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2Sub.html),
and
[`ModPowerOf2Neg`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2Neg.html),
take the exponent $$k$$ of a modulus $$2^k$$ instead. The same traits are implemented for
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
with a word-sized modulus.

## [Scalar arithmetic](https://flintlib.org/doc/fmpz_mod_mat.html#scalar-arithmetic) {#scalar-arithmetic}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpz_mod_mat_scalar_mul_si(fmpz_mod_mat_t B, const fmpz_mod_mat_t A, slong c, const fmpz_mod_ctx_t ctx)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html), [`ModMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMulAssign.html) |
| ≈ | `void fmpz_mod_mat_scalar_mul_ui(fmpz_mod_mat_t B, const fmpz_mod_mat_t A, ulong c, const fmpz_mod_ctx_t ctx)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html), [`ModMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMulAssign.html) |
| ≈ | `void fmpz_mod_mat_scalar_mul_fmpz(fmpz_mod_mat_t B, const fmpz_mod_mat_t A, fmpz_t c, const fmpz_mod_ctx_t ctx)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html), [`ModMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMulAssign.html) |

All three accept any integer $$c$$. `scalar_mul_si` and `scalar_mul_ui` multiply by $$c$$ over the
integers with `fmpz_mat_scalar_mul_si` or `fmpz_mat_scalar_mul_ui` and then reduce every entry;
`scalar_mul_fmpz` first reduces $$c$$ into $$[0, n)$$, then does the same. In Malachite, scalar
multiplication modulo $$m$$ is
[`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html)
of a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
by a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
scalar, `a.mod_mul(&c, &m)`, or
[`ModMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMulAssign.html)
for the aliased call `fmpz_mod_mat_scalar_mul_fmpz(A, A, c, ctx)`. It multiplies the reduced
entries modulo $$m$$ directly, and it requires the scalar to be reduced too, hence ≈: it panics if
$$c$$ or any entry is not less than $$m$$, where FLINT checks neither. A scalar that may be out of
range is reduced first: `Natural::from(c) % &m` for a `ulong`, and
`Integer::from(c).mod_op(Integer::from(&m)).unsigned_abs()` for an `slong` or an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html).
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
implements the same traits with a word-sized scalar and modulus.

## [Matrix multiplication](https://flintlib.org/doc/fmpz_mod_mat.html#matrix-multiplication) {#matrix-multiplication}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_mat_mul(fmpz_mod_mat_t C, const fmpz_mod_mat_t A, const fmpz_mod_mat_t B, const fmpz_mod_ctx_t ctx)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html), [`ModMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMulAssign.html) |
| — | `void fmpz_mod_mat_mul_classical_threaded(fmpz_mod_mat_t C, const fmpz_mod_mat_t A, const fmpz_mod_mat_t B, const fmpz_mod_ctx_t ctx)` | |
| ✓ | `void fmpz_mod_mat_sqr(fmpz_mod_mat_t B, const fmpz_mod_mat_t A, const fmpz_mod_ctx_t ctx)` | [`ModSquare`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModSquare.html), [`ModSquareAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModSquareAssign.html) |
| ✓ | `void fmpz_mod_mat_pow_ui(fmpz_mod_mat_t B, const fmpz_mod_mat_t A, ulong e, const fmpz_mod_ctx_t ctx)` | [`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPow.html), [`ModPowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowAssign.html) |
| ≈ | `void fmpz_mod_mat_mul_fmpz_vec(fmpz * c, const fmpz_mod_mat_t A, const fmpz * b, slong blen, const fmpz_mod_ctx_t ctx)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html) |
| ≈ | `void fmpz_mod_mat_mul_fmpz_vec_ptr(fmpz * const * c, const fmpz_mod_mat_t A, const fmpz * const * b, slong blen, const fmpz_mod_ctx_t ctx)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html) |
| ≈ | `void fmpz_mod_mat_fmpz_vec_mul(fmpz * c, const fmpz * a, slong alen, const fmpz_mod_mat_t B, const fmpz_mod_ctx_t ctx)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html) |
| ≈ | `void fmpz_mod_mat_fmpz_vec_mul_ptr(fmpz * const * c, const fmpz * const * a, slong alen, const fmpz_mod_mat_t B, const fmpz_mod_ctx_t ctx)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html) |

[`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html)
of two
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
values and a modulus, with each operand by value or by reference, is `fmpz_mod_mat_mul`, and
[`ModMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMulAssign.html)
is the product in place, which FLINT allows by aliasing `C` with `A`. Like FLINT, it multiplies over the integers (see the
integer matrix page's [Matrix multiplication](/mapping/flint-integer-matrices/#matrix-multiplication))
and then reduces every entry; when the modulus fits in a word, it instead multiplies as
`nmod_mat_mul` does (see the word-sized page's [Matrix
multiplication](/mapping/flint-word-sized-integer-matrices-mod-n/#matrix-multiplication)). They
panic if the modulus is 0, if the width of `A` is not the height of `B`, or if any entry of
either is at least $$m$$. `fmpz_mod_mat_sqr` is
[`ModSquare`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModSquare.html)
of a square `NaturalMatrix` and a modulus, with the matrix by value or by reference, and
[`ModSquareAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModSquareAssign.html)
squares in place. Like FLINT, it squares over the integers as `fmpz_mat_sqr` does and then reduces
every entry; when the modulus fits in a word, it instead squares an `UnsignedMatrix`, which
multiplies the matrix by itself. They panic if the modulus is 0, if the matrix is not square, or if
any entry is at least $$m$$.
`fmpz_mod_mat_mul_classical_threaded` uses the schoolbook method instead, splitting the rows among
threads. `fmpz_mod_mat_pow_ui` calls the generic `gr_mat_pow_ui`. That function returns the
identity when the exponent is 0 or the matrix is $$0 \times 0$$, uses `sqr` for exponent 2, and
uses binary powering above that. If `A` is not square it aborts, rather than panicking as Malachite
does. The counterpart is
[`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPow.html)
of a square `NaturalMatrix` with a `u64` exponent and a modulus, with the matrix and the modulus by
value or by reference, and
[`ModPowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowAssign.html)
raises it to the power in place. It uses left-to-right binary powering, with each exact product
reduced, and stops once a power is the zero matrix; when the modulus fits in a word, it raises an
`UnsignedMatrix` to the power instead (see the word-sized page's [Matrix
Exponentiation](/mapping/flint-word-sized-integer-matrices-mod-n/#matrix-exponentiation)). It
panics if the modulus is 0, if the matrix is not square, or if any entry is at least $$m$$. `fmpz_mod_mat.h` also
declares an undocumented `fmpz_mod_mat_submul`, which sets $$D$$ to $$C - AB$$ by computing the
product into a temporary matrix. The manual documents `_fmpz_mod_mat_mul_classical_threaded_op`,
but no such function exists: the function in `mul_classical_threaded.c` is
`fmpz_mod_mat_mul_classical_threaded_op`, without the underscore, and it is `static`. (The
underscore functions are omitted.)

The matrix-vector products are
[`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html)
of a `NaturalMatrix`, a
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html),
and a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
modulus, with the matrix as the receiver for $$Ab$$ and the vector as the receiver for $$aB$$.
FLINT computes the product over the integers with the `fmpz_mat` function and reduces the result,
so the vector may hold any integers, and it truncates or zero-extends the vector to the length the
product needs. Malachite panics unless the vector's dimension is the matrix's width (for $$Ab$$) or
height (for $$aB$$), and unless every element of both is less than $$m$$, hence ≈. The `_ptr`
forms read and write through arrays of pointers to elements; in Malachite, collect the elements
into a `NaturalVector` first. The same products exist modulo a power of 2, through
[`ModPowerOf2Mul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2Mul.html),
and for
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
with an
[`UnsignedVector`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html)
and a word-sized modulus.

## [Trace](https://flintlib.org/doc/fmpz_mod_mat.html#trace) {#trace}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_mat_trace(fmpz_t trace, const fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx)` | [`ModTrace`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModTrace.html) |

`fmpz_mod_mat_trace` computes the trace over the integers with `fmpz_mat_trace` and reduces the
sum. It therefore has the same unchecked shape as `fmpz_mat_trace`: for a matrix with more rows
than columns, it reads past the last element (see the integer matrix page's
[Trace](/mapping/flint-integer-matrices/#trace)). The counterpart is
[`ModTrace`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModTrace.html)
on a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html),
`a.to_mod_trace(&m)` or `a.into_mod_trace(&m)`, with the modulus by value or by reference. It adds
the diagonal elements modulo $$m$$, panics if the matrix is not square, and, like every Malachite
modular function, requires every element to be reduced, which FLINT assumes but does not check.

## [Gaussian elimination](https://flintlib.org/doc/fmpz_mod_mat.html#gaussian-elimination) {#gaussian-elimination}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_mat_det(fmpz_t res, const fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx)` | [`ModDeterminant`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModDeterminant.html) |
| ≈ | `slong fmpz_mod_mat_rref(fmpz_mod_mat_t res, const fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx)` | [`ModReducedRowEchelonForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModReducedRowEchelonForm.html), [`ModReducedRowEchelonFormAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModReducedRowEchelonFormAssign.html) |

In Malachite, `fmpz_mod_mat_det` is
[`ModDeterminant`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModDeterminant.html)
on a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html),
with the matrix and the modulus by value or by reference. When the modulus fits in a limb, the
determinant is computed as for an `UnsignedMatrix` of limbs (see [the `nmod_mat`
page](/mapping/flint-word-sized-integer-matrices-mod-n/#determinant-and-rank)); otherwise the
choices are FLINT's, described below, and the result is the same either way, since the determinant
is unique. Malachite panics on a non-square matrix and on unreduced entries.

`fmpz_mod_mat_rref` is
[`ModReducedRowEchelonForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModReducedRowEchelonForm.html)
and
[`ModReducedRowEchelonFormAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModReducedRowEchelonFormAssign.html)
on a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html),
with the matrix and the modulus by value or by reference, hence ≈: it returns the form but not the
rank, which
[`ModRank`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModRank.html)
computes. When the modulus fits in a limb, the form is computed as for an `UnsignedMatrix` of limbs
(see [the `nmod_mat` page](/mapping/flint-word-sized-integer-matrices-mod-n/#reduced-row-echelon-form));
otherwise the steps are FLINT's, described below. Since the reduced row echelon form is unique, the
result is the same either way. With a composite modulus, Malachite panics at the first nonzero
pivot that is not invertible, where FLINT aborts, and a result that is returned is correct.

Both functions call the generic `gr_mat` code. `fmpz_mod_mat_det` uses
cofactor expansion up to dimension 4 and LU decomposition above that. If LU decomposition meets a
pivot that is not invertible, which can happen only when $$n$$ is composite, it falls back on the
division-free Berkowitz algorithm, so the determinant is correct for any modulus. A non-square
matrix throws an exception whose message wrongly names `fmpz_mod_mat_charpoly_berkowitz`.

`fmpz_mod_mat_rref` computes the reduced row echelon form from an LU decomposition and returns the
rank. As the manual says, it requires $$n$$ to be prime: with a composite modulus it aborts the
process when it meets a pivot that is not invertible. (A reduced form over $$\mathbb{Z}/n$$ for
any $$n$$ is the Howell form; see [the next section](#strong-echelon-form-and-howell-form).) The
manual does not mention one bug: when `mat` is zero, the function returns rank 0 without writing
`res`, so a `res` distinct from `mat` keeps its old entries. For example, with `mat` the
$$2 \times 2$$ zero matrix and `res` holding $$\mathrm{diag}(3, 5)$$ modulo 7, `res` is still
$$\mathrm{diag}(3, 5)$$ afterwards.

The undocumented `fmpz_mod_mat_rank` is
[`ModRank`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModRank.html)
on a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html),
which makes the same choices, through `ModLuDecomposition`'s algorithms, and panics where FLINT
aborts.

`fmpz_mod_mat.h` also declares three undocumented functions here. `fmpz_mod_mat_rank` returns
the rank from an LU decomposition, and it too aborts on a composite modulus when it meets a pivot
that is not invertible. `fmpz_mod_mat_nullspace` computes the reduced row echelon form of `A` and
writes a basis of the right nullspace, one vector per non-pivot column, into the first columns of
the $$w \times w$$ matrix `X`, and returns its dimension. In Malachite it is
[`ModNullspace`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModNullspace.html)
on a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html),
which returns a $$w \times (w - r)$$ matrix of the same basis vectors, for rank $$r$$, and panics
where FLINT aborts.
[`ModLeftNullspace`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModLeftNullspace.html),
the left nullspace, has no `fmpz_mod_mat` counterpart. `fmpz_mod_mat_reduce_row` is one step of incremental row
reduction: it reduces row `m` of `A` by the earlier pivot rows recorded in the arrays `P` and `L`,
and records a new pivot if it finds one.

## [Strong echelon form and Howell form](https://flintlib.org/doc/fmpz_mod_mat.html#strong-echelon-form-and-howell-form) {#strong-echelon-form-and-howell-form}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_mat_strong_echelon_form(fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx)` | [`ModStrongEchelonForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModStrongEchelonForm.html), [`ModStrongEchelonFormAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModStrongEchelonFormAssign.html) |
| ≈ | `slong fmpz_mod_mat_howell_form(fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx)` | [`ModHowellForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModHowellForm.html), [`ModHowellFormAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModHowellFormAssign.html) |

These are the reduced forms over $$\mathbb{Z}/n$$ when $$n$$ is composite, where
[`rref`](#gaussian-elimination) can fail. Both functions pass the context's modulus to
`fmpz_mat_strong_echelon_form_mod` or `fmpz_mat_howell_form_mod`, so everything on the integer
matrix page's [Strong echelon form and Howell
form](/mapping/flint-integer-matrices/#strong-echelon-form-and-howell-form) applies. In Malachite
they are
[`ModStrongEchelonForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModStrongEchelonForm.html)
and
[`ModHowellForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModHowellForm.html)
(and their `Assign` versions) on a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
with a modulus, with the matrix and the modulus each by value or by reference.
They panic if the modulus is 0 or if any element is not reduced. FLINT requires at least as many
rows as columns, and does not check it; Malachite gives a wider matrix zero rows until it is square,
which leaves its row span unchanged, so its forms are always $$\max(h, w) \times w$$. The Howell form of a
wide matrix can need more nonzero rows than the matrix has: modulo 4, the Howell form of
$$\begin{pmatrix} 2 & 1 \end{pmatrix}$$ is $$\begin{pmatrix} 2 & 1 \\ 0 & 2 \end{pmatrix}$$. The Howell
functions return the number of nonzero rows of the result, and Malachite's do not, hence ≈: it is
the number of leading nonzero rows of the result, since the zero rows come last.

## [Inverse](https://flintlib.org/doc/fmpz_mod_mat.html#inverse) {#inverse}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `int fmpz_mod_mat_inv(fmpz_mod_mat_t B, const fmpz_mod_mat_t A, const fmpz_mod_ctx_t ctx)` | [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModInverse.html) |

In Malachite this is
[`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModInverse.html)
on a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html),
with a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
modulus by value or by reference and the matrix by value or by reference, returning an
[`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html) where FLINT returns a
flag and leaves `B` undefined. It makes the same choices as `fmpz_mod_mat_inv`, hence the same
results, with three exceptions, hence ≈: it panics on a matrix that is not square; modulo 1, where
every matrix is the zero matrix and is its own inverse, it returns the zero matrix rather than
reporting failure; and it panics at a nonzero pivot that is not invertible, which can only happen
with a composite modulus, where FLINT returns 0 or throws. When the modulus fits in a limb, the
inverse is computed as for an
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
(see [the `nmod_mat` page's Inverse section](/mapping/flint-word-sized-integer-matrices-mod-n/#inverse)),
which gives the same results.

`fmpz_mod_mat_inv` handles dimensions 0 and 1 directly and otherwise solves $$AX = I$$ with
`fmpz_mod_mat_solve`, which uses LU decomposition (see [Solving](#solving)). Nothing checks that
`A` is square. The manual says the modulus is assumed to be prime. With a composite modulus the
function can fail in two ways. For dimension 1, a nonzero entry that is not invertible throws a
"Cannot invert" exception instead of returning 0. For larger dimensions, LU decomposition gives up
at the first pivot that is not invertible, even when $$A$$ is: modulo 6, the matrix
$$\begin{pmatrix} 2 & 3 \\ 3 & 2 \end{pmatrix}$$ has determinant 1 and is its own inverse, but
`fmpz_mod_mat_inv` returns 0. A matrix over $$\mathbb{Z}/n$$ is invertible exactly when its
determinant is, so a version that works for every $$n$$ could check that first, or work through
the [Howell form](#strong-echelon-form-and-howell-form).

## [LU decomposition](https://flintlib.org/doc/fmpz_mod_mat.html#lu-decomposition) {#lu-decomposition}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `slong fmpz_mod_mat_lu(slong * P, fmpz_mod_mat_t A, int rank_check, const fmpz_mod_ctx_t ctx)` | [`ModLuDecomposition`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModLuDecomposition.html) |

In Malachite this is
[`ModLuDecomposition`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModLuDecomposition.html)
on a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html),
with the matrix and the modulus by value or by reference, hence ≈: it returns the permutation,
$$L$$, and $$U$$ as values rather than packing them into the input, and has no public `rank_check`
(the rank check is used internally, by the determinant). When the modulus fits in a limb, the
decomposition is computed as for an `UnsignedMatrix` of limbs (see [the `nmod_mat`
page](/mapping/flint-word-sized-integer-matrices-mod-n/#lu-decomposition)); the pivots are chosen in
the same way, so the result is the same. Otherwise the algorithms are FLINT's, described below,
except that the classical algorithm delays the reductions of its updates, reducing each column (and
each pivot row) once just before it is used, and that the products in the recursive algorithm are
computed exactly and reduced once, as `fmpz_mod_mat_mul` computes them; the results are the same.
Malachite panics at a nonzero pivot that is not invertible, where FLINT aborts.

`fmpz_mod_mat_lu` calls the generic `gr_mat_lu`. It uses the classical algorithm when $$A$$ has
fewer than 8 rows or fewer than 8 columns, and the recursive (block) algorithm otherwise. The
decomposition overwrites `A`, packing $$L$$ below the diagonal and $$U$$ on and above it. `P`, an
array with one entry per row, records the row permutation: entry $$i$$ is the original index of the
row that ends up in position $$i$$.

In the classical algorithm, each pivot is the first nonzero entry at or below the current row in
the current column. Because the modulus is assumed to be prime, every such entry is taken to be
invertible. With a composite modulus, the first nonzero pivot that is not invertible aborts the
process, since the call is wrapped in `GR_MUST_SUCCEED`. With `rank_check` nonzero, the function
returns 0 as soon as a column has no nonzero pivot, so it is a cheap test for nonsingularity.

## [Triangular solving](https://flintlib.org/doc/fmpz_mod_mat.html#triangular-solving) {#triangular-solving}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_mat_solve_tril(fmpz_mod_mat_t X, const fmpz_mod_mat_t L, const fmpz_mod_mat_t B, int unit, const fmpz_mod_ctx_t ctx)` | [`ModSolveLowerTriangular`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSolveLowerTriangular.html) |
| ✓ | `void fmpz_mod_mat_solve_triu(fmpz_mod_mat_t X, const fmpz_mod_mat_t U, const fmpz_mod_mat_t B, int unit, const fmpz_mod_ctx_t ctx)` | [`ModSolveUpperTriangular`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSolveUpperTriangular.html) |

In Malachite these are
[`ModSolveLowerTriangular`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSolveLowerTriangular.html)
and
[`ModSolveUpperTriangular`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSolveUpperTriangular.html)
on a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html),
with each matrix and the modulus by value or by reference, and a `unit_diagonal` flag in place of
`unit`; the solution is returned. When the modulus fits in a limb, the solution is computed as for
an `UnsignedMatrix` of limbs (see [the `nmod_mat`
page](/mapping/flint-word-sized-integer-matrices-mod-n/#triangular-solving)), which gives the same
result. Otherwise the choices are FLINT's, described below: the classical algorithm, which Malachite
applies one row of the solution at a time rather than one column, or the recursive one, whose
products are computed exactly and reduced once, as `fmpz_mod_mat_mul` computes them. Malachite
panics on a diagonal entry that is not a unit, where FLINT aborts, and on incompatible shapes or
unreduced entries, which FLINT does not check. Both functions call the generic `gr_mat` code,
which substitutes forward or backward one row at a time (the classical algorithm) when `B` has
fewer than 10 rows or fewer than 10 columns, and splits the matrices into blocks (the recursive
algorithm) otherwise. Unless `unit` is set, the classical algorithm first inverts the diagonal
entries; if one of them is not invertible, it falls back on dividing by it.

Only the diagonal entries are ever inverted, so the solve works for any modulus whose diagonal
entries are units, even though the manual says the modulus is assumed to be prime. Modulo 6, for
example, the lower triangular matrix with rows $$(5, 0)$$ and $$(2, 1)$$ and the right-hand side
$$(1, 1)^T$$ give $$X = (5, 3)^T$$. When a diagonal entry is not a unit, the division fails, and
since the call is wrapped in `GR_MUST_SUCCEED`, the process aborts. This happens even modulo a
prime, as with a zero on the diagonal: the manual's "full rank" is a precondition, not something
reported back.

## [Solving](https://flintlib.org/doc/fmpz_mod_mat.html#solving) {#solving}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `int fmpz_mod_mat_solve(fmpz_mod_mat_t X, const fmpz_mod_mat_t A, const fmpz_mod_mat_t B, const fmpz_mod_ctx_t ctx)` | [`ModSolve`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSolve.html) |
| ≈ | `int fmpz_mod_mat_can_solve(fmpz_mod_mat_t X, const fmpz_mod_mat_t A, const fmpz_mod_mat_t B, const fmpz_mod_ctx_t ctx)` | [`ModSolve`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSolve.html) |

In Malachite both are
[`ModSolve`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSolve.html)
on a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html),
with a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
modulus and a right-hand side that is a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
or a
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html),
each by value or by reference. It returns an
[`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html) of a solution, where
FLINT returns a flag, and it handles any shape and rank, like `fmpz_mod_mat_can_solve`, whose
solutions it matches exactly. A square system is decomposed once, without the rank check; if the
matrix has full rank, the solution is computed as `fmpz_mod_mat_solve` computes it, and otherwise
the decomposition is reused for the general method. `fmpz_mod_mat_solve` is ≈ because it also
returns 0 for a singular square system that has solutions, where Malachite returns one.
`fmpz_mod_mat_can_solve` is ≈ because Malachite panics if the shapes do not match, and at a nonzero
pivot that is not invertible, which can only happen with a composite modulus, where FLINT returns
0. When the modulus fits in a limb, the solution is computed as for an
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
(see [the `nmod_mat` page's Nonsingular square solving
section](/mapping/flint-word-sized-integer-matrices-mod-n/#nonsingular-square-solving)), which gives
the same results.

`fmpz_mod_mat_solve` computes an LU decomposition of `A` with the rank check on and returns 0 if
the rank is short; otherwise it solves by forward and back substitution (see [LU
decomposition](#lu-decomposition) and [Triangular solving](#triangular-solving)). Nothing checks
that `A` is square. Unlike `fmpz_mod_mat_lu`, it does not abort on a composite modulus: when the
decomposition meets a nonzero pivot that is not invertible, it just returns 0, even if $$A$$ is
invertible (see [Inverse](#inverse) for an example).

`fmpz_mod_mat_can_solve` handles any shape and rank. It computes an LU decomposition of `A`,
solves for the pivot columns, checks the remaining rows for consistency, and sets the entries of
`X` for the non-pivot columns to zero. The manual says that when there is no solution, `X` is set
to zero. That is true only when the consistency check fails. When the shapes do not match, or when
a composite modulus makes the decomposition fail, the function returns 0 and leaves `X` unchanged.
In the second case a solution may well exist. For example, modulo 6, with $$A$$ the matrix with
rows $$(2, 3)$$ and $$(3, 2)$$ and $$B = I$$, the solution is $$X = A$$, but
`fmpz_mod_mat_can_solve` returns 0, and an `X` that held $$4I$$ still holds it.

## [Transforms](https://flintlib.org/doc/fmpz_mod_mat.html#transforms) {#transforms}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_mat_similarity(fmpz_mod_mat_t M, slong r, fmpz_t d, const fmpz_mod_ctx_t ctx)` | [`ModSimilarityTransformAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSimilarityTransformAssign.html), [`ModSimilarityTransform`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSimilarityTransform.html) |

`fmpz_mod_mat_similarity` replaces a square matrix $$M$$ with $$P^{-1}MP$$, doing the arithmetic
modulo $$n$$. It has the same loops as `fmpz_mat_similarity`, so the $$P$$ it uses differs from the
manual's in the same way (see the integer matrix page's
[Transforms](/mapping/flint-integer-matrices/#transforms)). In Malachite it is
[`ModSimilarityTransformAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSimilarityTransformAssign.html)
on a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html),
`a.mod_similarity_transform_assign(r, d, m)`, with `d` and `m`
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s, each by
value or by reference, or [`ModSimilarityTransform`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSimilarityTransform.html),
which returns the result; it computes exactly what the code does. The
[`ModPowerOf2SimilarityTransform`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModPowerOf2SimilarityTransform.html)
forms do the same modulo $$2^k$$. The manual says that the modulus is assumed to be prime, but the code only multiplies, adds, and
subtracts. $$P$$ is the identity plus entries off the diagonal in a single row, so $$P^{-1}$$ is
the identity minus those entries, and the transform is a similarity modulo any $$n$$. As the manual
says, $$d$$ must be reduced. Nothing in FLINT checks that, nor the squareness of $$M$$, nor the
range of $$r$$; Malachite panics in each of those cases, and if $$M$$ is not reduced.

## [Characteristic polynomial](https://flintlib.org/doc/fmpz_mod_mat.html#characteristic-polynomial) {#characteristic-polynomial}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_mat_charpoly(fmpz_mod_poly_t p, const fmpz_mod_mat_t M, const fmpz_mod_ctx_t ctx)` | [`ModCharacteristicPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModCharacteristicPolynomial.html) |

In Malachite this is
[`ModCharacteristicPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModCharacteristicPolynomial.html)
on a `NaturalMatrix`, with the matrix and the modulus each by value or by reference, returning a
[`NaturalPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html)
with reduced coefficients. It panics if the matrix is not square or not reduced. When $$n$$ fits in
one or two limbs, Malachite uses the word-sized algorithm (see the [`nmod_mat`
page](/mapping/flint-word-sized-integer-matrices-mod-n/#characteristic-polynomial)). Otherwise a
power-of-2 modulus goes to the 2-adic Hessenberg reduction described below, and other moduli use
Berkowitz's algorithm below dimension 12, 14, or 16, depending on the size of $$n$$, and
Danilevsky's algorithm from there; when Danilevsky's algorithm fails, Berkowitz's algorithm is used
below dimension 20 and a reduction to Hessenberg form with extended-GCD row operations from 20 on.
Modulo a power of 2 there is also
[`ModPowerOf2CharacteristicPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModPowerOf2CharacteristicPolynomial.html),
which reduces the matrix to Hessenberg form with pivots of smallest 2-adic valuation.
`fmpz_mod_mat_charpoly` throws an exception on a non-square matrix and
otherwise calls the generic `gr_mat` code. From dimension 16 on, that code tries Danilevsky's
algorithm, which divides by pivots. If a pivot is not invertible, which can happen only when $$n$$
is composite, it falls back on the division-free Berkowitz algorithm, which is also what it uses
below dimension 16. So, unlike most of this module, the function works for any modulus. Modulo 1,
the leading coefficient 1 is 0, so the result is the zero polynomial.

## [Minimal polynomial](https://flintlib.org/doc/fmpz_mod_mat.html#minimal-polynomial) {#minimal-polynomial}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_mod_mat_minpoly(fmpz_mod_poly_t p, const fmpz_mod_mat_t M, const fmpz_mod_ctx_t ctx)` | |

Malachite does not compute minimal polynomials yet (see the integer matrix page's [Minimal
polynomial](/mapping/flint-integer-matrices/#minimal-polynomial)). `fmpz_mod_mat_minpoly` throws
an exception on a non-square matrix. Otherwise it marks the ring as a field and calls the generic
`gr_mat_minpoly_field`, which uses the Krylov method. For each unit vector $$v$$ not already in
the span of the vectors found so far, it finds the first linear dependency among
$$v, Mv, M^2v, \ldots$$ by Gaussian elimination, and it returns the least common multiple of the
polynomials those dependencies give.

The manual says that the modulus is assumed to be prime, and here that matters more than
elsewhere. Over $$\mathbb{Z}/n$$ with $$n$$ composite, a monic polynomial of least degree that
vanishes at $$M$$ need not be unique, so the minimal polynomial is not always well defined. Modulo
4, for example, both $$x^2$$ and $$x^2 - 2x$$ vanish at $$\mathrm{diag}(0, 2)$$, and no monic
polynomial of degree 1 does. FLINT does not try: when the elimination meets a pivot that is not
invertible, it aborts the process, since the call is wrapped in `GR_MUST_SUCCEED`. This happens,
for example, for the diagonal matrix $$\mathrm{diag}(2, 3)$$ modulo 6.

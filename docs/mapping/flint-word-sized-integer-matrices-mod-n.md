---
layout: default
title: "Malachite for FLINT Users: Word-Sized Integer Matrices mod n"
permalink: /mapping/flint-word-sized-integer-matrices-mod-n/
theme: jekyll-theme-slate
---

# Malachite for FLINT Users: Word-Sized Integer Matrices mod n

This page maps the functions of FLINT's `nmod_mat.h`, matrices over the integers mod $$n$$ for a
modulus $$n$$ that fits in a machine word, onto their Malachite counterpart:
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html),
from the `malachite-base` crate, with the `Mod*` traits. It follows the organization of the
[nmod_mat.h chapter](https://flintlib.org/doc/nmod_mat.html) of the FLINT manual, as of FLINT
3.6.0. It is a companion to [Malachite for FLINT Users: Integer Matrices mod
n](/mapping/flint-integer-matrices-mod-n/), the multi-word version, to [Malachite for FLINT Users:
Integer Matrices](/mapping/flint-integer-matrices/), whose
[conventions](/mapping/flint-integer-matrices/#conventions) apply here as well, and to [Malachite
for FLINT Users: Word-Sized Modular Polynomials](/mapping/flint-word-sized-modular-polynomials/),
whose [conventions](/mapping/flint-word-sized-modular-polynomials/#conventions) on word sizes and
precomputed data apply too; the [mapping index](/mapping/) lists the whole family.

Every section of the chapter is mapped below: 91 rows across 25 sections, of which 35 are ✓,
13 ⚙, 30 ≈, 8 —, and 5 ✗. Functions whose names begin with an underscore are omitted, as on
[the fmpq page](/mapping/flint-rationals/), and functions that `nmod_mat.h` declares but the
chapter does not document are mentioned in the notes of the sections they belong to.

## Conventions {#conventions}

### Representation {#representation}

An `nmod_mat_struct` holds `entries`, a pointer to `ulong`s in row-major order, the dimensions `r`
and `c`, a `stride`, as for [integer matrices](/mapping/flint-integer-matrices/#representation),
and an `nmod_t` field, `mod`, which holds the modulus $$n$$ and precomputed data for reducing
modulo it. Unlike an `fmpz_mod_mat_t`, then, an `nmod_mat_t` carries its own modulus, and its
functions take no context argument. FLINT never checks that the moduli of the operands agree; as
the chapter's introduction says, all matrices passed to a function are assumed to have the same
one. The introduction also says that a separate array holds pointers to the start of each row, so
that rows can be permuted by swapping pointers. That is out of date: in FLINT 3.6.0 the struct has
no such array, and rows are found through the stride.

The Malachite counterpart is an
[`UnsignedMatrix<T>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
whose elements are all less than $$n$$, which
[`ModIsReduced`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModIsReduced.html)
checks. The matrix holds only its shape and elements; the modulus is a `T` passed to each `Mod*`
operation, so moduli cannot be mismatched, and any precomputed data is derived inside the
operation (see [precomputed
data](/mapping/flint-word-sized-modular-polynomials/#precomputed)). `T` may be any primitive
unsigned type, and this chapter is the `T = u64` case (see [word
sizes](/mapping/flint-word-sized-modular-polynomials/#word-sized)). FLINT trusts that its inputs
are reduced; Malachite's modular matrix functions check, and panic if any element of an operand is
not less than $$n$$.

The chapter allows any nonzero modulus. As in
[`nmod_poly`](/mapping/flint-word-sized-modular-polynomials/#degenerate), Malachite accepts a
modulus of 1 too, with only the zero matrix reduced modulo it, and a modulus of 0 fails the
reducedness check of every modular function. The chapter also says that
the modulus is assumed to be prime in functions that divide, solve, or do Gaussian elimination
(including rank and determinant), and may be composite in those that only use ring operations,
such as transposition and multiplication; none of these assumptions is checked.

### Categories

The categories are those of [the integer matrix
page](/mapping/flint-integer-matrices/#categories): ✓, ≈, ⚙, —, and ✗.

## [Types, macros and constants](https://flintlib.org/doc/nmod_mat.html#types-macros-and-constants) {#types-macros-and-constants}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `nmod_mat_struct` | [`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html) |
| — | `nmod_mat_t` | |

`nmod_mat_t` is the pass-by-reference device that `&` and `&mut` already are. The struct maps to
`UnsignedMatrix<u64>`, less its `mod` field (see [Representation](#representation)).

## [Memory management](https://flintlib.org/doc/nmod_mat.html#memory-management) {#memory-management}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void nmod_mat_init(nmod_mat_t mat, slong rows, slong cols, ulong n)` | [`zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.zero) |
| ✓ | `void nmod_mat_init_set(nmod_mat_t mat, const nmod_mat_t src)` | [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) |
| — | `void nmod_mat_clear(nmod_mat_t mat)` | |
| ✓ | `void nmod_mat_set(nmod_mat_t mat, const nmod_mat_t src)` | [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) |
| ✓ | `void nmod_mat_swap(nmod_mat_t mat1, nmod_mat_t mat2)` | [`mem::swap`](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html) |
| — | `void nmod_mat_swap_entrywise(nmod_mat_t mat1, nmod_mat_t mat2)` | |
| — | `nmod_t nmod_mat_mod(const nmod_mat_t mat)` | |
| — | `void nmod_mat_set_mod(nmod_mat_t mat, ulong n)` | |

`nmod_mat_init` gives the zero matrix, `UnsignedMatrix::<u64>::zero(rows, cols)`, and also stores
the modulus, hence ≈, as for
[`nmod_poly_init`](/mapping/flint-word-sized-modular-polynomials/#memory-management). It
computes the precomputed data with `nmod_mat_set_mod`, and nothing checks that $$n$$ is nonzero,
as the manual requires. `init_set` copies the shape, the modulus, and the entries, and is
[`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html); `clear` is
[`Drop`](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html). `nmod_mat_set` copies the
entries into a matrix that must already have the same shape, which nothing checks; it does not
copy the modulus, so `mat` keeps its own. In Malachite, `a.clone_from(&b)`, the
[`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) method, reuses `a`'s
allocation where it can and takes `b`'s shape.

`nmod_mat_swap` exchanges the structs, so each matrix takes the other's modulus with its entries,
and it is
[`mem::swap`](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html). `swap_entrywise` exchanges
the entries one by one and leaves the moduli in place; it matters only for windows, which
Malachite does not have (see [the integer matrix page](/mapping/flint-integer-matrices/#window)).

`nmod_mat_mod` returns the stored `nmod_t`, and `nmod_mat_set_mod` replaces it, recomputing the
precomputed data, without reducing the entries. Neither has a counterpart, since a Malachite
matrix stores no modulus: the caller already holds $$n$$, and using a matrix with a different
modulus means passing that modulus to the next operation. To reduce the entries modulo a smaller
modulus, use
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.Mod.html)
or
[`ModAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModAssign.html),
`a.mod_assign(m)`. The precomputed data that `nmod_t` holds comes from
[`precompute_mod_mul_data`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMulPrecomputed.html#tymethod.precompute_mod_mul_data)
when a caller needs it for single-residue operations.

`nmod_mat.h` also declares an undocumented `nmod_mat_one` among these functions. It sets the
diagonal entries to 1 and the others to 0, for any shape, so a non-square matrix gets the truncated
identity; modulo 1 it gives the zero matrix. Its counterpart is
[`rectangular_identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.rectangular_identity) (or
[`identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.identity)
for a square matrix), which puts ones on the diagonal whatever the modulus; for $$n = 1$$ use
[`zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.zero)
instead.

## [Basic properties and manipulation](https://flintlib.org/doc/nmod_mat.html#basic-properties-and-manipulation) {#basic-properties-and-manipulation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `nmod_mat_entry(mat, i, j)` | [`Index`](https://doc.rust-lang.org/nightly/core/ops/trait.Index.html), [`IndexMut`](https://doc.rust-lang.org/nightly/core/ops/trait.IndexMut.html), [`get`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html#method.get), [`get_mut`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html#method.get_mut) |
| ✓ | `ulong nmod_mat_get_entry(const nmod_mat_t mat, slong i, slong j)` | [`Index`](https://doc.rust-lang.org/nightly/core/ops/trait.Index.html) |
| ✓ | `ulong * nmod_mat_entry_ptr(const nmod_mat_t mat, slong i, slong j)` | [`IndexMut`](https://doc.rust-lang.org/nightly/core/ops/trait.IndexMut.html), [`get_mut`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html#method.get_mut) |
| ✓ | `void nmod_mat_set_entry(nmod_mat_t mat, slong i, slong j, ulong x)` | [`IndexMut`](https://doc.rust-lang.org/nightly/core/ops/trait.IndexMut.html) |
| ✓ | `slong nmod_mat_nrows(const nmod_mat_t mat)` | [`height`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.height) |
| ✓ | `slong nmod_mat_ncols(const nmod_mat_t mat)` | [`width`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.width) |
| ✓ | `void nmod_mat_zero(nmod_mat_t mat)` | [`set_zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_zero) |
| ✓ | `int nmod_mat_is_zero(const nmod_mat_t mat)` | [`is_zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_zero) |

As for integer matrices (see [the integer matrix
page](/mapping/flint-integer-matrices/#basic-assignment-and-manipulation)), the `nmod_mat_entry`
macro is indexing, `a[(i, j)]`. It panics on an index out of range, where FLINT performs no bounds
checking.
[`get`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html#method.get)
and
[`get_mut`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html#method.get_mut)
return [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None)
instead. Since the elements are `Copy`, `get_entry` is just `a[(i, j)]`, and `set_entry` is
`a[(i, j)] = x`. `set_entry` does not reduce `x`; the caller must pass a value in $$[0, n)$$, as it
must when assigning to an `UnsignedMatrix` element that will later be used with the `Mod*` traits,
which check. `entry_ptr` gives a pointer for writing in place, which is `&mut a[(i, j)]` or
`get_mut`.

`nmod_mat_zero` and `nmod_mat_is_zero` are
[`set_zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_zero)
and
[`is_zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_zero),
and a matrix with no entries is zero in both libraries.

`nmod_mat.h` also declares four undocumented functions here. `nmod_mat_row_ptr` points to the
start of a row; its counterpart is
[`row_iter`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html#method.row_iter),
or
[`row_iter_mut`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html#method.row_iter_mut)
for writing. `nmod_mat_is_square` is
[`is_square`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_square),
and `nmod_mat_is_empty` is
[`is_empty`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_empty). `nmod_mat_is_one` accepts the truncated
identity of any shape, and its counterpart is
[`is_rectangular_identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_rectangular_identity).

`nmod_mat_is_one` has a bug for $$n = 1$$. It is meant to return 1 for every matrix then, since
$$1 \equiv 0$$ and every matrix is the identity, but the test in `nmod_mat/is_one.c` is
`mat->mod.n == 0` rather than `== 1`. Modulo 1, then, the matrix that `nmod_mat_one` produces, which
is all zeros, fails `nmod_mat_is_one`. `fmpz_mod_mat_is_one` gets this case right (see [the
`fmpz_mod_mat` page](/mapping/flint-integer-matrices-mod-n/#comparison)).

## [Window](https://flintlib.org/doc/nmod_mat.html#window) {#window}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void nmod_mat_window_init(nmod_mat_t window, const nmod_mat_t mat, slong r1, slong c1, slong r2, slong c2)` | [`window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.window), [`into_window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.into_window), [`set_window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_window), [`set_window_owned`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_window_owned) |
| — | `void nmod_mat_window_clear(nmod_mat_t window)` | |

Both are inline functions in `nmod_mat.h` that work as their `fmpz_mat` counterparts do, so the
integer matrix page's [notes on windows](/mapping/flint-integer-matrices/#window) apply unchanged.
Malachite has no windows that share their parent's elements, because a matrix always owns its
elements, so the counterparts copy the block, hence ≈: on an
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html),
[`window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.window)
and
[`into_window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.into_window)
copy a block out, and
[`set_window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_window)
and
[`set_window_owned`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_window_owned)
write one back. The FLINT window also copies the parent's `mod` field, so it carries the same
modulus; a Malachite copy carries none, and the caller passes the same modulus to the operations on
it. Even the bounds
assertion is the same, including its typo: the column check tests `c2 >= 0` where the row check
tests the lower bound, so a negative `c1` is not caught even in a debug build.

## [Concatenate](https://flintlib.org/doc/nmod_mat.html#concatenate) {#concatenate}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void nmod_mat_concat_vertical(nmod_mat_t res, const nmod_mat_t mat1, const nmod_mat_t mat2)` | [`concat_vertical`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.concat_vertical), [`into_concat_vertical`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.into_concat_vertical) |
| ✓ | `void nmod_mat_concat_horizontal(nmod_mat_t res, const nmod_mat_t mat1, const nmod_mat_t mat2)` | [`concat_horizontal`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.concat_horizontal), [`into_concat_horizontal`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.into_concat_horizontal) |

As on the integer matrix page (see
[Concatenate](/mapping/flint-integer-matrices/#concatenate)), the methods return a new matrix and
panic if the shapes do not fit. Concatenation copies elements without changing them, so reduced
inputs give a reduced result and no modulus is needed. Unlike in `fmpz_mod_mat`, the manual's
descriptions match the code here. `nmod_mat/concat.c` copies the entries as raw words, ignoring
all three moduli, and, like `fmpz_mat/concat.c`, checks none of the shapes the manual gives.

## [Printing](https://flintlib.org/doc/nmod_mat.html#printing) {#printing}

As on [the fmpz page](/mapping/flint-integers/#input-and-output), Malachite has no `FILE *`
functions: formatting works with any Rust writer.

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void nmod_mat_print_pretty(const nmod_mat_t mat)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) |
| ≈ | `int nmod_mat_fprint_pretty(FILE * file, const nmod_mat_t mat)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) |
| ≈ | `int nmod_mat_print(const nmod_mat_t mat)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) |
| ≈ | `int nmod_mat_fprint(FILE * f, const nmod_mat_t mat)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) |

All four write the same format, which differs from both `fmpz_mat`'s (see [the integer matrix
page](/mapping/flint-integer-matrices/#input-and-output)) and Malachite's, hence ≈. It starts with
a header line giving the shape and the modulus, such as `<2 x 3 integer matrix mod 2903>`, and
then has one line per row, in brackets, with each entry right-aligned to the number of decimal
digits of the modulus and the entries separated by spaces. A matrix with no entries prints only
the header. Malachite's
[`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) writes the manual's
example as `[0, 0, 2607; 622, 0, 0]`, on one line, without the modulus, which the matrix does not
store; a matrix with no elements is written with its shape, as in `[]_{2,0}`.

Unlike the `fmpz_mat` and `fmpz_mod_mat` chapters, this one has no reading function. Malachite's
[`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html) parses the
[`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) format back, and with
the `serde` feature an
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
is serialized as its height, its width, and its elements in row-major order. `print_pretty`
returns nothing, discarding the status that `fprint_pretty` returns.

## [Random matrix generation](https://flintlib.org/doc/nmod_mat.html#random-matrix-generation) {#random-matrix-generation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void nmod_mat_randtest(nmod_mat_t mat, flint_rand_t state)` | [`random_unsigned_matrices_with_dimensions_from_iterator`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/random/fn.random_unsigned_matrices_with_dimensions_from_iterator.html), [`striped_random_unsigned_range`](https://docs.rs/malachite-base/latest/malachite_base/num/random/striped/fn.striped_random_unsigned_range.html) |
| ≈ | `void nmod_mat_randfull(nmod_mat_t mat, flint_rand_t state)` | [`random_unsigned_matrices_with_dimensions_from_iterator`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/random/fn.random_unsigned_matrices_with_dimensions_from_iterator.html), [`striped_random_unsigned_range`](https://docs.rs/malachite-base/latest/malachite_base/num/random/striped/fn.striped_random_unsigned_range.html) |
| ✓ | `void nmod_mat_rand(nmod_mat_t mat, flint_rand_t state)` | [`random_unsigned_matrices_with_dimensions_from_iterator`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/random/fn.random_unsigned_matrices_with_dimensions_from_iterator.html), [`random_unsigneds_less_than`](https://docs.rs/malachite-base/latest/malachite_base/num/random/fn.random_unsigneds_less_than.html) |
| ✓ | `int nmod_mat_randpermdiag(nmod_mat_t mat, flint_rand_t state, nn_srcptr diag, slong n)` | [`random_unsigned_matrices_with_permuted_diagonal`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/random/fn.random_unsigned_matrices_with_permuted_diagonal.html) |
| ≈ | `void nmod_mat_randrank(nmod_mat_t mat, flint_rand_t state, slong rank)` | [`random_unsigned_matrices_with_rank_mod`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/random/fn.random_unsigned_matrices_with_rank_mod.html) |
| ✓ | `void nmod_mat_randops(nmod_mat_t mat, flint_rand_t state, slong count)` | [`random_unsigned_matrices_from_elementary_operations_mod`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/random/fn.random_unsigned_matrices_from_elementary_operations_mod.html) |
| ≈ | `void nmod_mat_randtril(nmod_mat_t mat, flint_rand_t state, int unit)` | [`random_unsigned_lower_triangular_matrices_mod`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/random/fn.random_unsigned_lower_triangular_matrices_mod.html) |
| ≈ | `void nmod_mat_randtriu(nmod_mat_t mat, flint_rand_t state, int unit)` | [`random_unsigned_upper_triangular_matrices_mod`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/random/fn.random_unsigned_upper_triangular_matrices_mod.html) |

Malachite's generators are infinite iterators of matrices of a given shape, which take their
elements from any iterator. For residues modulo $$n$$, pass an iterator of unsigned integers less
than $$n$$. `nmod_mat_rand` fills each row with
[`_nmod_vec_rand`](/mapping/flint-word-sized-integer-vectors-mod-n/#random-functions), drawing
every entry uniformly from $$[0, n)$$, as
[`random_unsigneds_less_than`](https://docs.rs/malachite-base/latest/malachite_base/num/random/fn.random_unsigneds_less_than.html)`(seed, n)`
does. `nmod_mat_randtest` fills each row with `_nmod_vec_randtest`, so the vector page's note
applies: the entries come from `n_randtest`, which favors special and sparse values, and each row
is sparse half the time. The closest Malachite element iterator is the striped generator
[`striped_random_unsigned_range`](https://docs.rs/malachite-base/latest/malachite_base/num/random/striped/fn.striped_random_unsigned_range.html)
on $$[0, n)$$, which favors values with long runs of equal bits, and so values near 0 and near
$$n$$. Its distribution differs, and it has no sparse mode, hence ≈.

`nmod_mat_randfull` is documented to give entries "likely to be close to the modulus", to test
overflow, but it does not. `nmod_mat/randomisation.c` sets each entry to
`FLINT_MAX(1, n_randint(state, n))`, which looks like a uniform draw with 0 replaced by 1. Since
`FLINT_MAX` is a macro that evaluates its arguments twice, the entry is actually 1 when the first
draw is 0 and a second, independent uniform draw otherwise. The entries are therefore close to
uniform on $$[0, n)$$, with 1 slightly favored and 0 still possible: in one test with
$$n = 1000$$, 3 of 2000 entries were 0. Modulo 1 every entry is 1, which is not reduced. For the
documented intent, use the striped generator, as for `randtest`.

`randpermdiag`, `randrank`, and `randops` are the modular versions of the [integer matrix
generators](/mapping/flint-integer-matrices/#random-matrix-generation) of those names, and the
notes there apply. `randrank` draws its nonzero entries uniformly from $$[1, n)$$, not from
$$[0, n)$$ as the manual says, so that the rank is right; modulo 1 it uses zeros, giving the zero
matrix whatever the rank requested. `randtril` and `randtriu` give random lower and upper
triangular matrices of any shape. When `unit` is 0, a diagonal entry that comes out 0 is replaced
by 1, so the diagonal is nonzero but not uniform, with 1 twice as likely as any other value.
Modulo 1 the diagonal is all 1s, which again are not reduced.

Malachite's counterparts are iterators over
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)s
with the shape, the parameters, and the modulus as arguments:
[`random_unsigned_matrices_with_permuted_diagonal`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/random/fn.random_unsigned_matrices_with_permuted_diagonal.html),
which yields the parity with each matrix,
[`random_unsigned_matrices_with_rank_mod`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/random/fn.random_unsigned_matrices_with_rank_mod.html),
[`random_unsigned_matrices_from_elementary_operations_mod`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/random/fn.random_unsigned_matrices_from_elementary_operations_mod.html),
which applies `randops` to every matrix of another iterator, and
[`random_unsigned_lower_triangular_matrices_mod`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/random/fn.random_unsigned_lower_triangular_matrices_mod.html)
and
[`random_unsigned_upper_triangular_matrices_mod`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/random/fn.random_unsigned_upper_triangular_matrices_mod.html).
They produce only reduced entries. The rank and triangular generators reject the modulus 1 when
the matrix has a diagonal, where FLINT writes 1s or zeros, and the triangular ones draw a random
diagonal entry uniformly from $$[1, n)$$; those differences make them ≈. All of them panic on an
impossible shape or rank.

## [Comparison](https://flintlib.org/doc/nmod_mat.html#comparison) {#comparison}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int nmod_mat_equal(const nmod_mat_t mat1, const nmod_mat_t mat2)` | [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) |
| ✓ | `int nmod_mat_is_zero_row(const nmod_mat_t mat, slong i)` | [`is_zero_row`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_zero_row) |

Like `nmod_mat_equal`,
[`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) compares the shapes
as well as the elements (see the integer matrix page's
[Comparison](/mapping/flint-integer-matrices/#comparison)). Here the manual's "the moduli are
ignored" is accurate, since each `nmod_mat_t` does store one: two matrices with equal entries but
different moduli are equal. A Malachite matrix stores no modulus, so the question does not arise.
`nmod_mat/equal.c` has a redundant loop. For each row, it compares the whole row once for every
column, so equal $$r \times c$$ matrices take $$rc^2$$ comparisons rather than $$rc$$. The result is
still correct.

[`is_zero_row`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_zero_row)
panics if the row index is out of range; `nmod_mat_is_zero_row` does not check it.

## [Transposition and permutations](https://flintlib.org/doc/nmod_mat.html#transposition-and-permutations) {#transposition-and-permutations}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void nmod_mat_transpose(nmod_mat_t B, const nmod_mat_t A)` | [`transpose`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.transpose), [`transpose_assign`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.transpose_assign) |
| ≈ | `void nmod_mat_swap_rows(nmod_mat_t mat, slong * perm, slong r, slong s)` | [`swap_rows`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.swap_rows) |
| ≈ | `void nmod_mat_swap_cols(nmod_mat_t mat, slong * perm, slong r, slong s)` | [`swap_columns`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.swap_columns) |
| ≈ | `void nmod_mat_invert_rows(nmod_mat_t mat, slong * perm)` | [`reverse_rows`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.reverse_rows) |
| ≈ | `void nmod_mat_invert_cols(nmod_mat_t mat, slong * perm)` | [`reverse_columns`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.reverse_columns) |
| ✗ | `void nmod_mat_permute_rows(nmod_mat_t mat, const slong * perm_act, slong * perm_store)` | |

`nmod_mat_transpose` throws unless `B` has the transposed shape of `A`, so it can transpose in
place only a square matrix. As on the integer matrix page (see
[Transpose](/mapping/flint-integer-matrices/#transpose)),
[`transpose`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.transpose)
returns a new matrix, and
[`transpose_assign`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.transpose_assign)
transposes a matrix of any shape in place. Transposition keeps the entries reduced and does not
involve the modulus.

The row and column operations are inline functions in `nmod_mat.h`. `nmod_mat_swap_rows` and
`nmod_mat_swap_cols` are
[`swap_rows`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.swap_rows)
and
[`swap_columns`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.swap_columns),
≈ because they take no permutation array, and so are `nmod_mat_invert_rows` and
`nmod_mat_invert_cols`, which are
[`reverse_rows`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.reverse_rows)
and
[`reverse_columns`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.reverse_columns).
The integer matrix page's [notes on
them](/mapping/flint-integer-matrices/#basic-assignment-and-manipulation) apply, including how
to keep a permutation with the slice methods
[`swap`](https://doc.rust-lang.org/nightly/core/primitive.slice.html#method.swap) and
[`reverse`](https://doc.rust-lang.org/nightly/core/primitive.slice.html#method.reverse). The
manual repeats the integer chapter's slip here: it says that `invert_rows` swaps rows $$i$$ and
$$r - i$$ and that `invert_cols` swaps columns $$i$$ and $$c - i$$, but the code swaps rows $$i$$
and $$r - 1 - i$$ and columns $$i$$ and $$c - 1 - i$$, so each function reverses the order. All
four do nothing to a matrix with no entries, and then leave `perm` unchanged too, even for a
matrix with rows but no columns, whose rows could still be permuted.

`nmod_mat_permute_rows` makes row $$i$$ of the result the old row $$\mathit{perm\_act}[i]$$, and
applies the same rearrangement to `perm_store`, if given, so that
$$\mathit{perm\_store}[i]$$ becomes the old $$\mathit{perm\_store}[\mathit{perm\_act}[i]]$$. It
copies the matrix into a temporary buffer to do so. In Malachite, collect
`perm_act.iter().flat_map(|&p| a.row_iter(p)).copied()` and pass the result to
[`from_owned_elements`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.from_owned_elements)
with the same height and width.

## [Addition and subtraction](https://flintlib.org/doc/nmod_mat.html#addition-and-subtraction) {#addition-and-subtraction}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void nmod_mat_add(nmod_mat_t C, const nmod_mat_t A, const nmod_mat_t B)` | [`ModAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModAdd.html), [`ModAddAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModAddAssign.html) |
| ✓ | `void nmod_mat_sub(nmod_mat_t C, const nmod_mat_t A, const nmod_mat_t B)` | [`ModSub`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModSub.html), [`ModSubAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModSubAssign.html) |
| ✓ | `void nmod_mat_neg(nmod_mat_t B, const nmod_mat_t A)` | [`ModNeg`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModNeg.html), [`ModNegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModNegAssign.html) |

Each function applies the vector operation `_nmod_vec_add`, `_nmod_vec_sub`, or `_nmod_vec_neg` to
every row (see [the word-sized vectors mod n
page](/mapping/flint-word-sized-integer-vectors-mod-n/)). On
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html),
`a.mod_add(b, m)` takes the modulus as an argument, and the matrices may be passed by value or by
reference. The aliased calls `nmod_mat_add(A, A, B)` and `nmod_mat_neg(A, A)` are the `*_assign`
forms: `a.mod_add_assign(&b, m)`. FLINT checks neither the shapes nor whether the entries are
reduced, and takes the shape and the modulus from one matrix: from the output `C` for `add` and
`sub`, and from the input `A` for `neg`. Malachite panics if the shapes differ or if any element is
not less than $$m$$. The power-of-2 forms,
[`ModPowerOf2Add`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2Add.html),
[`ModPowerOf2Sub`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2Sub.html),
and
[`ModPowerOf2Neg`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2Neg.html),
take the exponent $$k$$ of a modulus $$2^k$$ instead. The same traits are implemented for
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
with a multi-word modulus (see [the `fmpz_mod_mat`
page](/mapping/flint-integer-matrices-mod-n/#addition-and-subtraction)).

## [Matrix-scalar arithmetic](https://flintlib.org/doc/nmod_mat.html#matrix-scalar-arithmetic) {#matrix-scalar-arithmetic}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void nmod_mat_scalar_mul(nmod_mat_t B, const nmod_mat_t A, ulong c)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html), [`ModMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMulAssign.html) |
| ≈ | `void nmod_mat_scalar_addmul_ui(nmod_mat_t C, const nmod_mat_t A, const nmod_mat_t B, const ulong c)` | [`ModAddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModAddMul.html), [`ModAddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModAddMulAssign.html) |
| ≈ | `void nmod_mat_scalar_mul_fmpz(nmod_mat_t B, const nmod_mat_t A, const fmpz_t c)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html), [`ModMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMulAssign.html) |

`nmod_mat_scalar_mul` multiplies each entry by a reduced scalar $$c$$, which is
[`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html)
of an
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
by a `T` modulo a `T`, `a.mod_mul(c, m)`, or
[`ModMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMulAssign.html)
for the aliased call. FLINT handles $$c$$ equal to 0, 1, and $$n - 1$$ directly, and for larger
matrices precomputes Shoup's quotient for $$c$$ and reuses it for every entry. Malachite likewise
computes the data for multiplying modulo $$m$$ once (see [precomputed
data](/mapping/flint-word-sized-modular-polynomials/#precomputed)). Both libraries require $$c$$ to
be reduced; Malachite checks, and panics if $$c$$ or any element is not less than $$m$$.

`nmod_mat_scalar_mul_fmpz` takes a multi-word integer scalar of either sign, reduces it modulo
$$n$$, and calls `nmod_mat_scalar_mul`, hence ≈: in Malachite, reduce the scalar first. For an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) `c`,
that is `u64::exact_from(&c.mod_op(Integer::from(m)))`, which needs `malachite-nz`. The same choice
was made for the [`fmpz_mod_mat` scalar
functions](/mapping/flint-integer-matrices-mod-n/#scalar-arithmetic).

`nmod_mat_scalar_addmul_ui` sets $$C$$ to $$A + cB$$, which is
[`ModAddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModAddMul.html)
of two `UnsignedMatrix`s and a `T` modulo a `T`, `a.mod_add_mul(b, c, m)`, or
[`ModAddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModAddMulAssign.html)
when $$C$$ is $$A$$. As for vectors, and as for the plain matrix
[`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html),
the multiplier is a scalar. The manual says that $$c$$ is assumed to be reduced, but the function
reduces it first, so any `ulong` works; Malachite requires $$c$$ to be reduced, and panics if it or
any element is not less than $$m$$, hence ≈. Both libraries handle $$c$$ equal to 0, 1, and
$$n - 1$$ as `set`, `add`, and `sub`; FLINT otherwise uses Shoup's method for larger matrices, and
Malachite computes the data for multiplying modulo $$m$$ once and reuses it for every element.
Malachite also has
[`ModSubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModSubMul.html),
for $$A - cB$$, and the `ModPowerOf2` forms of both, which FLINT does not.

## [Matrix multiplication](https://flintlib.org/doc/nmod_mat.html#matrix-multiplication) {#matrix-multiplication}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void nmod_mat_mul(nmod_mat_t C, const nmod_mat_t A, const nmod_mat_t B)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html), [`ModMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMulAssign.html) |
| ⚙ | `void nmod_mat_mul_classical(nmod_mat_t C, const nmod_mat_t A, const nmod_mat_t B)` | internal |
| — | `void nmod_mat_mul_classical_threaded(nmod_mat_t C, const nmod_mat_t A, const nmod_mat_t B)` | |
| ⚙ | `void nmod_mat_mul_strassen(nmod_mat_t C, const nmod_mat_t A, const nmod_mat_t B)` | internal |
| — | `int nmod_mat_mul_blas(nmod_mat_t C, const nmod_mat_t A, const nmod_mat_t B)` | |
| ✗ | `void nmod_mat_addmul(nmod_mat_t D, const nmod_mat_t C, const nmod_mat_t A, const nmod_mat_t B)` | |
| ✗ | `void nmod_mat_submul(nmod_mat_t D, const nmod_mat_t C, const nmod_mat_t A, const nmod_mat_t B)` | |
| ≈ | `void nmod_mat_mul_nmod_vec(ulong * c, const nmod_mat_t A, const ulong * b, slong blen)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html) |
| ≈ | `void nmod_mat_mul_nmod_vec_ptr(ulong * const * c, const nmod_mat_t A, const ulong * const * b, slong blen)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html) |
| ≈ | `void nmod_mat_nmod_vec_mul(ulong * c, const ulong * a, slong alen, const nmod_mat_t B)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html) |
| ≈ | `void nmod_mat_nmod_vec_mul_ptr(ulong * const * c, const ulong * const * a, slong alen, const nmod_mat_t B)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html) |

`nmod_mat_mul` is
[`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html)
of two
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)es
and a `T` modulus, with each matrix by value or by reference, and
[`ModMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMulAssign.html)
for the product in place, which FLINT allows by aliasing `C` with `A`. They panic if the modulus is
0, if the width of the first matrix is not the height of the second, which FLINT checks only in
debug builds, or if any element of either is at least $$m$$. The classical and Strassen
variants are internal choices behind `ModMul`. `nmod_mat_mul` uses BLAS, when FLINT is built with
it, for matrices whose smallest dimension is above a cutoff of at least 100. Otherwise it uses the
threaded classical method whenever more than one thread is available, the classical method when
the smallest dimension is below 400 (for a modulus below 2048 on 64-bit machines) or 200, and
Strassen–Winograd multiplication above that. Malachite, which is single-threaded and written in
pure Rust, makes the same choice as FLINT does with one thread and without BLAS, hence the — rows
for `nmod_mat_mul_classical_threaded` and `nmod_mat_mul_blas`. The classical method, like FLINT's,
packs several elements into each word when every dot product fits in one word and every dimension
is above 10, transposes the second matrix when all three dimensions are large, for memory locality, and
otherwise reads the second matrix's columns in place. Strassen–Winograd multiplication copies the
blocks it multiplies, where FLINT references them through windows, since Malachite's matrices own
their elements. If `C` is `A` or `B`, FLINT multiplies into a temporary matrix first;
`ModMulAssign` computes the product separately and then copies it into the first matrix's storage.
FLINT has no `nmod_mat_sqr`; Malachite's
[`ModSquare`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModSquare.html)
of a square `UnsignedMatrix` and a `T` modulus, and
[`ModSquareAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModSquareAssign.html),
compute the product of the matrix with itself, as `nmod_mat_mul(B, A, A)` does, since squaring a
matrix of words is no cheaper than multiplying two.
FLINT has no separate product modulo a power of 2; Malachite's is
[`ModPowerOf2Mul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2Mul.html)
of two `UnsignedMatrix`es and a `u64` exponent, which multiplies in wrapping arithmetic, with
Strassen's algorithm for large matrices, and clears the high bits once; for
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)es,
it reduces the exact product when $$2^k$$ does not fit in a word. (The underscore functions, including the documented `_nmod_mat_mul_classical_op`,
`_nmod_mat_mul_classical_threaded_op`, and `_nmod_mat_mul_classical_threaded_pool_op`, are
omitted.)

`nmod_mat_addmul` and `nmod_mat_submul` set $$D$$ to $$C + AB$$ and $$C - AB$$. The manual's
description of `submul` says $$C + AB$$, copied from `addmul`, but the code subtracts. For small
matrices on one thread, both fold the addition into the classical product; otherwise they compute
the product into a temporary matrix. Malachite does not have them yet (see [Matrix-scalar arithmetic](#matrix-scalar-arithmetic) on the name `ModAddMul`).

The matrix-vector products are
[`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html)
of an `UnsignedMatrix`, an
[`UnsignedVector`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html),
and a `T` modulus, with the matrix as the receiver for $$Ab$$ and the vector as the receiver for
$$aB$$. FLINT computes each entry with the dot product `_nmod_vec_dot`, choosing its method once
for the whole product (see [the word-sized vectors mod n
page](/mapping/flint-word-sized-integer-vectors-mod-n/)); Malachite likewise picks one reduction
strategy for the whole product. FLINT truncates or zero-extends the vector to the length the
product needs, while Malachite panics unless the vector's dimension is the matrix's width (for
$$Ab$$) or height (for $$aB$$), and unless every element of both is less than $$m$$, hence ≈. The
`_ptr` forms read and write through arrays of pointers to elements; in Malachite, collect the
elements into an `UnsignedVector` first. The same products exist modulo a power of 2, through
[`ModPowerOf2Mul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2Mul.html),
and for
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
with a multi-word modulus.

## [Matrix Exponentiation](https://flintlib.org/doc/nmod_mat.html#matrix-exponentiation) {#matrix-exponentiation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void nmod_mat_pow(nmod_mat_t dest, const nmod_mat_t mat, ulong pow)` | [`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPow.html), [`ModPowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowAssign.html) |

The counterpart is
[`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPow.html)
of a square
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
with a `u64` exponent and a `T` modulus, with the matrix by value or by reference, and
[`ModPowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowAssign.html)
raises it to the power in place, which FLINT allows by aliasing `dest` with `mat`. They panic if
the modulus is 0, if the matrix is not square, or if any element is at least $$m$$. `nmod_mat_pow` copies `mat` first if it is aliased with
`dest`, and then calls `_nmod_mat_pow` (omitted, like the other underscore functions). That
function sets `dest` to the identity for exponent 0, so modulo 1 to the zero matrix, and uses one
or two products for exponents 1 to 3. Above that it uses right-to-left binary powering, where
`fmpz_mat_pow` uses left-to-right (see the integer matrix page's [Matrix
multiplication](/mapping/flint-integer-matrices/#matrix-multiplication)). Nothing checks that the
matrix is square. For a $$0 \times 0$$ matrix, the function returns without touching `dest`.
Malachite powers left to right, like `fmpz_mat_pow`, and stops once a power is the zero matrix,
since every further power is zero too.

## [Trace](https://flintlib.org/doc/nmod_mat.html#trace) {#trace}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `ulong nmod_mat_trace(const nmod_mat_t mat)` | [`ModTrace`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModTrace.html) |

The counterpart is
[`ModTrace`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModTrace.html)
on an
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html),
`a.to_mod_trace(m)` or `a.into_mod_trace(m)`, which panics if the matrix is not square or any
element is not reduced. `nmod_mat_trace` adds the
diagonal entries modulo $$n$$ one at a time, starting from entry $$(0, 0)$$, and returns 0 for a
matrix with no rows. As in `fmpz_mat_trace` (see the integer matrix page's
[Trace](/mapping/flint-integer-matrices/#trace)), nothing checks that the matrix is square: it
reads positions $$(i, i)$$ for every row $$i$$, so for a matrix with more rows than columns it reads
past the end of a row, and past the last entry. A matrix with rows but no columns has no entry
array at all (`nmod_mat_init` leaves it `NULL`), so the first read dereferences a null pointer.

## [Determinant and rank](https://flintlib.org/doc/nmod_mat.html#determinant-and-rank) {#determinant-and-rank}

| | FLINT | Malachite |
| :---: | --- | --- |
| ⚙ | `ulong nmod_mat_det_howell(const nmod_mat_t A)` | internal |
| ✓ | `ulong nmod_mat_det(const nmod_mat_t A)` | [`ModDeterminant`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModDeterminant.html) |
| ✓ | `slong nmod_mat_rank(const nmod_mat_t A)` | [`ModRank`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModRank.html) |

In Malachite these are
[`ModDeterminant`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModDeterminant.html)
and
[`ModRank`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModRank.html)
on an
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html),
with a `T` modulus and the matrix by value or by reference. The determinant makes the same choices
as `nmod_mat_det`, described below, with `nmod_mat_det_howell`'s algorithm as an internal one. The
primality test that chooses between LU decomposition and that algorithm only covers moduli of up to
64 bits, so for `u128` matrices with larger moduli the Howell algorithm, which works for any
modulus, is always used. Malachite starts its unit cofactors from those of `extended_gcd` rather
than `n_xgcd`; since the determinant is unique, the result is the same. Both panic on a non-square
matrix, where FLINT throws, and on an element that is not reduced, which FLINT does not check.

Both determinant functions throw an exception on a non-square matrix, and both work for any
modulus. `nmod_mat_det` uses closed formulas up to dimension 4 and the division-free Berkowitz
algorithm up to dimension 8. Above that, it uses LU decomposition when the modulus is prime, and
the algorithm of `nmod_mat_det_howell` otherwise. That algorithm reduces the matrix to upper
triangular form with row swaps and Euclidean row operations. When a pivot does not divide an
entry below it, the pivot row is replaced by a combination whose leading entry is their gcd, with
a unit cofactor. The function records the cofactors and divides by their product at the end, so
no non-unit is ever inverted. Modulo 6, for example, both functions give 1 for the matrix with
rows $$(2, 3)$$ and $$(3, 2)$$. For a $$0 \times 0$$ matrix, both return 1, or 0 when $$n = 1$$.
(The underscore functions `_nmod_mat_det` and `_nmod_mat_det_howell`, which overwrite their input,
are omitted.)

`nmod_mat_rank` returns the rank from an LU decomposition, and 0 for a matrix with no entries. As
the manual says, the modulus must be prime. With a composite modulus, the decomposition throws an
"Impossible inverse" exception at the first nonzero pivot that is not invertible, as for the
matrix with rows $$(2, 3)$$ and $$(3, 2)$$ modulo 6. Over $$\mathbb{Z}/n$$ with $$n$$ composite,
rank has no single standard meaning; the [Howell form](#strong-echelon-form-and-howell-form) is
the usual substitute.

## [Inverse](https://flintlib.org/doc/nmod_mat.html#inverse) {#inverse}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `int nmod_mat_inv(nmod_mat_t B, const nmod_mat_t A)` | [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModInverse.html) |

In Malachite this is
[`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModInverse.html)
on an
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html),
with a `T` modulus and the matrix by value or by reference, returning an
[`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html) where FLINT returns a
flag and leaves `B` undefined. It makes the same choices as `nmod_mat_inv`, hence the same results,
with three exceptions, hence ≈: it panics on a matrix that is not square; modulo 1, where every
matrix is the zero matrix and is its own inverse, it returns the zero matrix rather than reporting
failure; and a nonzero $$1 \times 1$$ entry that is not invertible gives `None`, rather than a
panic. A larger matrix with a nonzero pivot that is not invertible makes it panic, as FLINT throws.

`nmod_mat_inv` handles dimensions 0 and 1 directly, and otherwise solves $$AX = I$$ with
`nmod_mat_solve` (see [Nonsingular square solving](#nonsingular-square-solving)). Nothing checks
the shapes. The manual says that the modulus must be prime, and a composite modulus can abort the
process. A nonzero $$1 \times 1$$ entry that is not invertible throws "Impossible inverse", and so
does LU decomposition at the first nonzero pivot that is not invertible, even when the matrix is
invertible: modulo 6, the matrix with rows $$(2, 3)$$ and $$(3, 2)$$ has determinant 1 and is its
own inverse, but `nmod_mat_inv` throws. The `fmpz_mod_mat` version returns 0 for the same matrix
instead (see [its Inverse section](/mapping/flint-integer-matrices-mod-n/#inverse)). Modulo 1,
where every matrix is invertible, `nmod_mat_inv` returns 0 for any matrix of dimension at least 1,
since the only entry value is 0.

## [Triangular solving](https://flintlib.org/doc/nmod_mat.html#triangular-solving) {#triangular-solving}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void nmod_mat_solve_tril(nmod_mat_t X, const nmod_mat_t L, const nmod_mat_t B, int unit)` | [`ModSolveLowerTriangular`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSolveLowerTriangular.html) |
| ⚙ | `void nmod_mat_solve_tril_classical(nmod_mat_t X, const nmod_mat_t L, const nmod_mat_t B, int unit)` | internal |
| ⚙ | `void nmod_mat_solve_tril_recursive(nmod_mat_t X, const nmod_mat_t L, const nmod_mat_t B, int unit)` | internal |
| ✓ | `void nmod_mat_solve_triu(nmod_mat_t X, const nmod_mat_t U, const nmod_mat_t B, int unit)` | [`ModSolveUpperTriangular`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSolveUpperTriangular.html) |
| ⚙ | `void nmod_mat_solve_triu_classical(nmod_mat_t X, const nmod_mat_t U, const nmod_mat_t B, int unit)` | internal |
| ⚙ | `void nmod_mat_solve_triu_recursive(nmod_mat_t X, const nmod_mat_t U, const nmod_mat_t B, int unit)` | internal |

In Malachite these are
[`ModSolveLowerTriangular`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSolveLowerTriangular.html)
and
[`ModSolveUpperTriangular`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSolveUpperTriangular.html)
on an
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html),
with a `T` modulus, each matrix by value or by reference, and a `unit_diagonal` flag in place of
`unit`; the solution is returned, so there is no aliasing of `X` and `B`. The classical and
recursive variants are internal choices behind them, made as FLINT makes them: the classical method
when `B` has fewer than 64 rows or fewer than 64 columns, and the recursive one otherwise. The
recursive method splits the matrices in half and reduces the problem to two smaller solves and one
`nmod_mat_submul`; Malachite reads the off-diagonal block in place, as FLINT does through a window.
The classical method inverts the diagonal entries once, unless `unit` is set. FLINT then solves one
column of `B` at a time, computing each entry of the solution as a dot product, which reads the
solution at a stride; Malachite computes one row of the solution at a time instead, accumulating
the sums for a whole row before reducing them, which gives the same result.

Only the diagonal entries are inverted, so, as for
[`fmpz_mod_mat`](/mapping/flint-integer-matrices-mod-n/#triangular-solving), the solve works for
any modulus whose diagonal entries are units. Unlike most of the chapter's functions, these do not
claim to need a prime modulus. A diagonal entry that is not invertible throws an "Impossible
inverse" exception, as the manual says; Malachite panics. FLINT does not check that the dimensions
are compatible or that the entries are reduced, and Malachite panics if they are not. Only the
entries in the triangle are read, in both libraries, and the diagonal is not read when `unit` is
set. The manual describes `nmod_mat_solve_triu_classical` as using forward substitution, copied
from the `tril` version, but it uses back substitution, from the last row up.

## [Nonsingular square solving](https://flintlib.org/doc/nmod_mat.html#nonsingular-square-solving) {#nonsingular-square-solving}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `int nmod_mat_solve(nmod_mat_t X, const nmod_mat_t A, const nmod_mat_t B)` | [`ModSolve`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSolve.html) |
| ≈ | `int nmod_mat_can_solve_inner(slong * rank, slong * perm, slong * pivots, nmod_mat_t X, const nmod_mat_t A, const nmod_mat_t B)` | [`ModSolve`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSolve.html) |
| ✓ | `int nmod_mat_can_solve(nmod_mat_t X, const nmod_mat_t A, const nmod_mat_t B)` | [`ModSolve`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSolve.html) |
| ≈ | `int nmod_mat_solve_vec(nn_ptr x, const nmod_mat_t A, nn_srcptr b)` | [`ModSolve`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSolve.html) |

In Malachite all four are
[`ModSolve`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSolve.html)
on an
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html),
with a `T` modulus and a right-hand side that is an
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
or an
[`UnsignedVector`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html),
each by value or by reference. It returns an
[`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html) of a solution, where
FLINT returns a flag, and it handles any shape and rank, like `nmod_mat_can_solve`, whose solutions
it matches exactly. `nmod_mat_solve` and `nmod_mat_solve_vec` are ≈ because they also return 0 for a
singular square system that has solutions, where Malachite returns one. A square system is
decomposed once, without the rank check; if the matrix has full rank, the solution is computed as
`nmod_mat_solve` computes it, and otherwise the decomposition is reused for the general method. The
extra outputs of `can_solve_inner` are not returned, hence ≈: the rank is given by
[`ModRank`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModRank.html),
and the permutation and the pivot columns by
[`ModLuDecomposition`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModLuDecomposition.html).
Malachite panics if the shapes do not match or an entry is not reduced, and, as FLINT throws, at a
nonzero pivot that is not invertible.

`nmod_mat_solve` computes an LU decomposition of `A` with the rank check on and returns 0 if the
rank is short. Otherwise it permutes the rows of `B` and solves by forward and back substitution
(see [LU decomposition](#lu-decomposition) and [Triangular solving](#triangular-solving)). Nothing
checks that `A` is square. `nmod_mat_solve_vec` wraps the vectors `b` and `x` as one-column
matrices and calls `nmod_mat_solve`. Like the `fmpz_mod_mat` versions (see [their
Solving](/mapping/flint-integer-matrices-mod-n/#solving) section), all of these require a prime
modulus. Here, though, a composite modulus does not make them return 0: the LU decomposition
throws an "Impossible inverse" exception at the first nonzero pivot that is not invertible, as for
[`nmod_mat_rank`](#determinant-and-rank).

`nmod_mat_can_solve` is `nmod_mat_can_solve_inner` without the extra outputs. It handles any shape
and rank. It computes an LU decomposition of `A`, solves for the pivot columns, checks the
remaining rows for consistency, and sets the entries of `X` for the non-pivot columns to zero. The
manual says that when there is no solution, `X` is set to zero. That is true when the consistency
check fails, but when the shapes do not match, the function returns 0 and leaves `X` unchanged, as
the `fmpz_mod_mat` version does.

## [LU decomposition](https://flintlib.org/doc/nmod_mat.html#lu-decomposition) {#lu-decomposition}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `slong nmod_mat_lu(slong * P, nmod_mat_t A, int rank_check)` | [`ModLuDecomposition`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModLuDecomposition.html) |
| ⚙ | `slong nmod_mat_lu_classical(slong * P, nmod_mat_t A, int rank_check)` | internal |
| ⚙ | `slong nmod_mat_lu_classical_delayed(slong * P, nmod_mat_t A, int rank_check)` | internal |
| ⚙ | `slong nmod_mat_lu_recursive(slong * P, nmod_mat_t A, int rank_check)` | internal |
| ⚙ | `slong nmod_mat_lu_with_pivots(slong * P, slong * pivots_nonpivots, nmod_mat_t A)` | internal |

In Malachite this is
[`ModLuDecomposition`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModLuDecomposition.html)
on an
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html),
with a `T` modulus and the matrix by value or by reference, hence ≈: it returns the permutation,
$$L$$, and $$U$$ as values, with $$L$$ of height $$h$$ and width equal to the rank and $$U$$ of
height equal to the rank, rather than packing them into the input as FLINT does (see [the
`fmpz_mod_mat` page](/mapping/flint-integer-matrices-mod-n/#lu-decomposition) for the layout and the
meaning of `P` and `rank_check`). There is no public `rank_check`; the rank check is used
internally, by the determinant. The algorithm variants are internal choices behind it, made as FLINT
makes them and with FLINT's pivots, so the results are the same. Malachite's delayed reduction
accumulates in words of the width of `T`; for 8-bit words, three words cannot hold more than 128
unreduced products, so larger matrices modulo large 8-bit moduli use the classical algorithm
instead. With a composite modulus, Malachite panics at the first nonzero pivot that is not
invertible, where FLINT throws; a result that is returned is correct, since every row operation used
a unit.

`nmod_mat_lu` chooses by the smaller dimension $$n$$ and the bit length of the modulus. It uses the
classical algorithm when $$n \le 3$$, or when $$n \le 7$$ and the modulus has more than 28 bits.
Otherwise it uses the classical algorithm with delayed reduction, which accumulates unreduced
products and reduces them less often, when $$n$$ is below a cutoff read from a table indexed by
the modulus's bit length, and the recursive block algorithm above it. The source says that the
tables, one for builds with BLAS or without AVX2 and one for the rest, were tuned on AMD Zen 3.
The cutoffs range from 56 to 1464 and do not vary monotonically with the bit length.
`nmod_mat_lu_with_pivots` also writes the pivot columns in order, followed by the non-pivot
columns. The pivots are inverted with `nmod_inv`, so with a composite modulus, the first nonzero
pivot that is not invertible throws an "Impossible inverse" exception.

## [Reduced row echelon form](https://flintlib.org/doc/nmod_mat.html#reduced-row-echelon-form) {#reduced-row-echelon-form}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `slong nmod_mat_rref(nmod_mat_t A)` | [`ModReducedRowEchelonForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModReducedRowEchelonForm.html), [`ModReducedRowEchelonFormAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModReducedRowEchelonFormAssign.html) |
| ✗ | `slong nmod_mat_reduce_row(nmod_mat_t A, slong * P, slong * L, slong n)` | |

In Malachite, `nmod_mat_rref` is
[`ModReducedRowEchelonForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModReducedRowEchelonForm.html)
and
[`ModReducedRowEchelonFormAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModReducedRowEchelonFormAssign.html)
on an
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html),
with a `T` modulus and the matrix by value or by reference, hence ≈: it returns the form but not the
rank, which
[`ModRank`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModRank.html)
computes. The steps are FLINT's, with `nmod_mat_lu_with_pivots` as an internal function, so the
results are the same; since the reduced row echelon form is unique, they would be the same anyway.
With a composite modulus, Malachite panics at the first nonzero pivot that is not invertible, where
FLINT throws, and a result that is returned is correct. Malachite does not have
`nmod_mat_reduce_row` yet.

`nmod_mat_rref` returns 0 for a matrix with no entries, and handles a
single row directly, scaling it by the inverse of its first nonzero entry. Otherwise it computes
an LU decomposition with the pivot columns recorded (see [LU decomposition](#lu-decomposition)),
clears $$L$$, solves a triangular system to reduce the non-pivot columns against the pivot block,
and writes the identity into the pivot columns. As in the rest of the chapter, the modulus must be
prime: with a composite modulus, the first nonzero pivot that is not invertible throws an
"Impossible inverse" exception. The [Howell form](#strong-echelon-form-and-howell-form) is the
reduced form that works for any modulus.

`nmod_mat_reduce_row` is one step of incremental row reduction, which
[`nmod_mat_minpoly`](#minimal-polynomial) uses: it reduces row `n` of `A` by the earlier pivot
rows recorded in `P`, and if a nonzero entry remains, scales the row to make it a new pivot,
records it, and returns its column, or returns $$-1$$ if the row reduces to zero. It accumulates
the products without reduction, in as many words per entry (one to three) as $$n + 1$$ times the
square of the modulus needs, and reduces each entry only when it reaches that entry's column.
(The definition names the row parameter `m`.)

`nmod_mat.h` also declares `nmod_mat_rref_classical` and `nmod_mat_rref_storjohann`, and their
underscore forms, but FLINT 3.6.0 defines none of them, so calling one fails at link time.

## [Nullspace](https://flintlib.org/doc/nmod_mat.html#nullspace) {#nullspace}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `slong nmod_mat_nullspace(nmod_mat_t X, const nmod_mat_t A)` | [`ModNullspace`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModNullspace.html) |
| ≈ | `slong nmod_mat_left_nullspace(nmod_mat_t X, const nmod_mat_t A)` | [`ModLeftNullspace`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModLeftNullspace.html) |

In Malachite these are
[`ModNullspace`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModNullspace.html)
and
[`ModLeftNullspace`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModLeftNullspace.html)
on an
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html),
with a `T` modulus and the matrix by value or by reference, hence ≈: as on the integer matrix page
(see [Nullspace](/mapping/flint-integer-matrices/#nullspace)), the result is a matrix with exactly
as many columns (or, for the left nullspace, rows) as the nullity, so the nullity is its width (or
height) and the caller does not size the output. The basis vectors are FLINT's, except that modulo
1, where every element is 0, they are 0 rather than columns of the identity.

`nmod_mat_nullspace` computes the reduced row echelon form of a copy of $$A$$ (see [Reduced row
echelon form](#reduced-row-echelon-form)), so it needs a prime modulus, and reads the basis off it.
For each non-pivot column $$j$$, the basis vector has a 1 in position $$j$$ and the negated
entries of column $$j$$ in the pivot positions. The vectors fill the first columns of `X`, and the
rest of `X` is set to zero. The caller must give `X` $$n$$ rows and at least as many columns as
the nullity, which nothing checks. `nmod_mat_left_nullspace` transposes $$A$$, computes the
nullspace of the transpose into an $$m \times m$$ temporary, and copies the basis into the first
rows of `X` as row vectors. Modulo 1, every matrix has rank 0, and `nmod_mat_nullspace` writes the
identity, whose entries 1 are not reduced.

## [Transforms](https://flintlib.org/doc/nmod_mat.html#transforms) {#transforms}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void nmod_mat_similarity(nmod_mat_t M, slong r, ulong d)` | [`ModSimilarityTransformAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSimilarityTransformAssign.html), [`ModSimilarityTransform`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSimilarityTransform.html) |

`nmod_mat_similarity` replaces a square matrix $$M$$ with $$P^{-1}MP$$, doing the arithmetic modulo
$$n$$. It has the same loops as `fmpz_mat_similarity`, so the $$P$$ it uses differs from the
manual's in the same way, by skipping column $$r - 1$$ (see the integer matrix page's
[Transforms](/mapping/flint-integer-matrices/#transforms)). In Malachite it is
[`ModSimilarityTransformAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSimilarityTransformAssign.html)
on an
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html),
`a.mod_similarity_transform_assign(r, d, m)`, with `d` and `m` of the element type, or
[`ModSimilarityTransform`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModSimilarityTransform.html),
which returns the result; it computes exactly what the code does, and the
[`ModPowerOf2SimilarityTransform`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModPowerOf2SimilarityTransform.html)
forms do the same modulo $$2^k$$. As for
the [`fmpz_mod_mat` version](/mapping/flint-integer-matrices-mod-n/#transforms), the code only
multiplies, adds, and subtracts, and $$P$$ is the identity plus entries off the diagonal in one
row, so the transform is a similarity modulo any $$n$$; unlike the `fmpz_mod_mat` manual, this one
does not claim that the modulus must be prime. $$d$$ must be reduced, as the manual says. Nothing in
FLINT checks that, nor the squareness of $$M$$, nor the range of $$r$$; Malachite panics in each of
those cases, and if $$M$$ is not reduced.

## [Characteristic polynomial](https://flintlib.org/doc/nmod_mat.html#characteristic-polynomial) {#characteristic-polynomial}

| | FLINT | Malachite |
| :---: | --- | --- |
| ⚙ | `void nmod_mat_charpoly_berkowitz(nmod_poly_t p, const nmod_mat_t M)` | internal |
| ⚙ | `int nmod_mat_charpoly_danilevsky(nmod_poly_t p, const nmod_mat_t M)` | internal |
| ✓ | `void nmod_mat_charpoly(nmod_poly_t p, const nmod_mat_t M)` | [`ModCharacteristicPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModCharacteristicPolynomial.html) |

In Malachite this is
[`ModCharacteristicPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModCharacteristicPolynomial.html)
on an `UnsignedMatrix`, by value or by reference, returning an
[`UnsignedPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html)
with reduced coefficients; the Berkowitz and Danilevsky algorithms are internal choices behind it.
It panics if the matrix is not square or not reduced. Malachite chooses differently from
`nmod_mat_charpoly`, from benchmarks: Danilevsky's algorithm is used from a dimension between 6 and
31 that depends on the width of the element type and the size of the modulus, rather than above
dimension 8; a power-of-2 modulus goes to the 2-adic Hessenberg reduction described below; and when
Danilevsky's algorithm fails, Berkowitz's algorithm is used below dimension 32, and from 32 on the
matrix is reduced to Hessenberg form instead: each element below a non-invertible pivot is cleared
by a row operation of determinant 1 built from the extended GCD of the two elements, followed by
the inverse column operation, so the reduction works for any modulus. Modulo 1 the result is the
zero polynomial. Modulo a power of 2 there is also
[`ModPowerOf2CharacteristicPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModPowerOf2CharacteristicPolynomial.html),
which FLINT has no separate function for; it reduces the matrix to Hessenberg form, choosing pivots
of smallest 2-adic valuation, which divide the other elements of their columns. These FLINT
functions are declared in `nmod_poly.h`, not `nmod_mat.h`.

`nmod_mat_charpoly` chooses as follows. For dimension at least 32 and a prime modulus below 256,
it copies the matrix into the generic `gr_mat` code with byte-sized entries. Otherwise, above
dimension 8, it tries Danilevsky's algorithm, which divides by pivots and returns 0 if a pivot is
not invertible, which can happen only when the modulus is composite. In that case, and up to
dimension 8, it uses the division-free Berkowitz algorithm, so the function works for any modulus.

Two details of the implementation differ from the manual. First, `nmod_mat_charpoly` itself does
not check that the matrix is square, and the byte-sized path takes the dimension from the number
of rows: for a $$32 \times 40$$ matrix modulo 7, it raises no exception and returns a polynomial of
degree 32. Only the paths through `charpoly_berkowitz` and `charpoly_danilevsky` throw. Second,
modulo 1, the result has length $$n + 1$$ and all its coefficients are 0, so it is not normalized,
although an `nmod_poly_t`'s leading coefficient is meant to be nonzero. The [`fmpz_mod_mat`
version](/mapping/flint-integer-matrices-mod-n/#characteristic-polynomial) normalizes it to the
zero polynomial.

## [Minimal polynomial](https://flintlib.org/doc/nmod_mat.html#minimal-polynomial) {#minimal-polynomial}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void nmod_mat_minpoly(nmod_poly_t p, const nmod_mat_t M)` | |

Malachite does not compute minimal polynomials yet (see the integer matrix page's [Minimal
polynomial](/mapping/flint-integer-matrices/#minimal-polynomial)); the result modulo $$m$$ will
be an
[`UnsignedPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html).
`nmod_mat_minpoly` is declared in `nmod_poly.h` and calls `nmod_mat_minpoly_with_gens`, an
undocumented function that can also mark, in an array, the unit vectors it starts from. It uses the
Krylov method, as the `fmpz_mod_mat` version does (see [its Minimal
polynomial](/mapping/flint-integer-matrices-mod-n/#minimal-polynomial)): for each unit vector not
yet in the span, it finds the first dependency among $$v, Mv, M^2v, \ldots$$ by incremental row
reduction with [`nmod_mat_reduce_row`](#reduced-row-echelon-form), and returns the least common
multiple of the polynomials found. A non-square matrix throws an exception, whose message names
`nmod_mat_charpoly` instead.

The manual mentions no condition on the modulus, but the pivots are inverted, so a composite
modulus throws an "Impossible inverse" exception, for example for $$\mathrm{diag}(2, 3)$$ modulo 6.
For such moduli the minimal polynomial is not always well defined (see the `fmpz_mod_mat` page).
The code also takes the modulus from the output polynomial `p` rather than from `M`, so a
mismatch is not detected: for $$\mathrm{diag}(2, 3)$$ modulo 7 with `p` initialized modulo 5, the
result is $$x^2 + 1$$, the minimal polynomial modulo 5. Modulo 1, a $$1 \times 1$$ matrix gives a
polynomial of length 2 whose coefficients are both 0, which is not normalized.

## [Strong echelon form and Howell form](https://flintlib.org/doc/nmod_mat.html#strong-echelon-form-and-howell-form) {#strong-echelon-form-and-howell-form}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void nmod_mat_strong_echelon_form(nmod_mat_t A)` | [`ModStrongEchelonForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModStrongEchelonForm.html), [`ModStrongEchelonFormAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModStrongEchelonFormAssign.html) |
| ≈ | `slong nmod_mat_howell_form(nmod_mat_t A)` | [`ModHowellForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModHowellForm.html), [`ModHowellFormAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModHowellFormAssign.html) |

These are the reduced forms over $$\mathbb{Z}/n$$ that work when $$n$$ is composite, where
[`rref`](#reduced-row-echelon-form) and [`rank`](#determinant-and-rank) can throw. In Malachite they
are
[`ModStrongEchelonForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModStrongEchelonForm.html)
and
[`ModHowellForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModHowellForm.html)
(and their `Assign` versions) on an
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
with a `T` modulus, with the matrix by value or by reference. The `nmod_mat` versions are word-sized
implementations of the same algorithm as `fmpz_mat_strong_echelon_form_mod` and
`fmpz_mat_howell_form_mod`, so the integer matrix page's
[notes](/mapping/flint-integer-matrices/#strong-echelon-form-and-howell-form) apply. Pivots are
never inverted unless they are units; the elimination uses gcds with a unit cofactor instead, as
[`nmod_mat_det_howell`](#determinant-and-rank) does. FLINT takes the cofactors from `n_xgcd`, which
returns nonnegative ones, and Malachite from `extended_gcd`, which returns signed ones; since both
forms are unique, the results are the same. As the manual says, the result is an upper right normal
form, as in Storjohann and Mulders' definition of the Howell form, although Fieker and Hofmann,
whose algorithm is used, define the strong echelon form as a lower left one. Malachite's functions
panic if the modulus is 0 or if any element is not reduced. FLINT requires at least as many rows as
columns, and does not check it; Malachite gives a wider matrix zero rows until it is square, which
leaves its row span unchanged, so its forms are always $$\max(h, w) \times w$$. The Howell form of a
wide matrix can need more nonzero rows than the matrix has: modulo 4, the Howell form of
$$\begin{pmatrix} 2 & 1 \end{pmatrix}$$ is $$\begin{pmatrix} 2 & 1 \\ 0 & 2 \end{pmatrix}$$. The
Howell functions return the number of nonzero rows of the result, and Malachite's do not, hence ≈:
it is the number of leading nonzero rows of the result, since the zero rows come last.

The manual says that the Howell form is computed by "ordering the rows" of the strong echelon form;
`nmod_mat_howell_form` only moves the zero rows to the bottom, keeping the nonzero rows in order.

# FLINT: `fmpq_mat_can_solve` reports a solution when A has no columns and B is nonzero

**Status:** pending, not yet reported upstream.
**Affects:** FLINT 3.7.0-dev, commit `feb6bcc66f4b770fc337719bb2348449044081d1` (2026-11-23).
**Not affected:** FLINT 3.6.0.
**Found by:** the Malachite oracle (`oracle-test`, modes `fmpq_mat_can_solve` and
`fmpq_mat_solve_or_can_solve`), 2026-11-26.

## Summary

If `A` has at least 16 rows and no columns, and `B` is nonzero, there is no solution, since
`A X = 0` for every `X`. But `fmpq_mat_can_solve(X, A, B)` returns 1.

FLINT's other `fmpq_mat_can_solve_*` functions return 0 correctly on the same input, and so does
`fmpq_mat_can_solve` itself up to 15 rows.

## Reproduction

```c
#include <stdio.h>
#include <flint/fmpq.h>
#include <flint/fmpq_mat.h>

int main(void)
{
    for (slong r = 15; r <= 17; r++)
    {
        fmpq_mat_t A, B, X;
        fmpq_mat_init(A, r, 0);
        fmpq_mat_init(B, r, 1);
        fmpq_mat_init(X, 0, 1);
        fmpq_set_si(fmpq_mat_entry(B, 0, 0), -3, 7);
        flint_printf("rows = %wd: can_solve = %d, can_solve_fraction_free = %d, "
                     "can_solve_multi_mod = %d, can_solve_dixon = %d\n", r,
                     fmpq_mat_can_solve(X, A, B), fmpq_mat_can_solve_fraction_free(X, A, B),
                     fmpq_mat_can_solve_multi_mod(X, A, B), fmpq_mat_can_solve_dixon(X, A, B));
        fmpq_mat_clear(A);
        fmpq_mat_clear(B);
        fmpq_mat_clear(X);
    }
    return 0;
}
```

Output with 3.7.0-dev:

```
rows = 15: can_solve = 0, can_solve_fraction_free = 0, can_solve_multi_mod = 0, can_solve_dixon = 0
rows = 16: can_solve = 1, can_solve_fraction_free = 0, can_solve_multi_mod = 0, can_solve_dixon = 0
rows = 17: can_solve = 1, can_solve_fraction_free = 0, can_solve_multi_mod = 0, can_solve_dixon = 0
```

FLINT 3.6.0 prints 0 for every call.

## Cause

In 3.7, `fmpq_mat_can_solve` (`src/fmpq_mat/can_solve.c`) handles more than 15 rows itself. It
converts the rational system to an integer one with `fmpq_mat_get_fmpz_mat_rowwise_2(Anum, Bnum,
NULL, A, B)` and passes the result to `_fmpz_mat_can_solve_auto_den`. In 3.6 that branch called
`fmpq_mat_can_solve_multi_mod`.

`_fmpq_mat_get_fmpz_mat_rowwise` (`src/fmpq_mat/get_fmpz_mat_rowwise.c`) starts with

```c
if (fmpq_mat_is_empty(mat[0]))
    return;
```

so when `A` has no columns it returns before it writes any of the later matrices. `Bnum` keeps
the zeros from `fmpz_mat_init`. `_fmpz_mat_can_solve_generic` then takes its `n == 0` branch,
`return fmpz_mat_is_zero(B)`, and sees a zero `B`, so it returns 1.

The other callers of `fmpq_mat_get_fmpz_mat_rowwise_2` aren't affected: the `can_solve_*`
variants handle `A->c == 0` before they convert, and the `solve_*` functions take square
matrices, which are empty only when `B` has no rows either.

## Possible fixes

- In `fmpq_mat_can_solve`, handle an empty `A` before converting, as
  `fmpq_mat_can_solve_multi_mod` does: if `A->c == 0`, zero `X` and return
  `fmpq_mat_is_zero(B)`. (If `A->r == 0`, every `X` is a solution, so the zero matrix will do.)
- Or make `_fmpq_mat_get_fmpz_mat_rowwise` return early only when every matrix is empty, so the
  other matrices are still converted when `mat[0]` has no columns.

The second fix is the more general one, since the comment in the loop ("the other matrices may
have no columns") shows the helper already expects matrices of different widths.

The FLINT test `t-can_solve` doesn't catch this, presumably because it never pairs a 0-column
`A` with more than 15 rows and a nonzero `B`.

## Malachite side

Malachite's `can_solve` returns `None` (no solution), which is correct. The oracle accepts that
answer for these inputs, with a comment pointing to this report.

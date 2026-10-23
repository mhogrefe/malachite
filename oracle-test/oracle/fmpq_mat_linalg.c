/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the RationalMatrix solving demos against FLINT's fmpq_mat
    functions.

    The demos print `op(A, B) = R`, where `R` is `None` or `Some(X)`, and the inverse demos print
    `inverse(A) = R`. The modes recompute the results with `fmpq_mat_solve_fraction_free`,
    `fmpq_mat_solve_dixon`, `fmpq_mat_solve_multi_mod`, `fmpq_mat_solve`,
    `fmpq_mat_can_solve_fraction_free`, `fmpq_mat_can_solve_multi_mod`, `fmpq_mat_can_solve_dixon`,
    `fmpq_mat_can_solve`, and `fmpq_mat_inv`; the `fmpq_mat_solve_or_can_solve` mode checks
    Malachite's `Solve`, which is `fmpq_mat_solve` when the matrix is square and nonsingular and
    `fmpq_mat_can_solve` otherwise.

    Matrices are read by fmpq_mat_init_set_str_malachite in util.c. Each mode treats any other
    nonempty line as an error, and an input with no lines at all.
*/

#include <stdio.h>
#include <string.h>

#include <flint/fmpq_mat.h>

#include "oracle.h"

static long checked;
static const char * current_name;
static const char * current_op;
static int current_kind;

enum { KIND_SOLVE_FRACTION_FREE, KIND_SOLVE_DIXON, KIND_SOLVE_MULTI_MOD, KIND_SOLVE,
       KIND_CAN_SOLVE_FRACTION_FREE, KIND_CAN_SOLVE_MULTI_MOD, KIND_CAN_SOLVE_DIXON, KIND_CAN_SOLVE,
       KIND_SOLVE_OR_CAN_SOLVE, KIND_INV };

/* The length of the matrix string at the start of `s`: `[]_{h,w}` or `[...]`. */
static size_t
matrix_length(const char * s)
{
    const char * end = strncmp(s, "[]_{", 4) == 0 ? strchr(s, '}') : strchr(s, ']');
    return end == NULL ? 0 : (size_t) (end - s + 1);
}

static int
error(const char * message, int line_number)
{
    flint_printf("error in %s test, line %d: %s\n", current_name, line_number, message);
    return 1;
}

static int
compute(fmpq_mat_t x, const fmpq_mat_t a, const fmpq_mat_t b)
{
    switch (current_kind)
    {
        case KIND_SOLVE_FRACTION_FREE:
            return fmpq_mat_solve_fraction_free(x, a, b);
        case KIND_SOLVE_DIXON:
            return fmpq_mat_solve_dixon(x, a, b);
        case KIND_SOLVE_MULTI_MOD:
            return fmpq_mat_solve_multi_mod(x, a, b);
        case KIND_SOLVE:
            return fmpq_mat_solve(x, a, b);
        case KIND_CAN_SOLVE_FRACTION_FREE:
            return fmpq_mat_can_solve_fraction_free(x, a, b);
        case KIND_CAN_SOLVE_MULTI_MOD:
            return fmpq_mat_can_solve_multi_mod(x, a, b);
        case KIND_CAN_SOLVE_DIXON:
            return fmpq_mat_can_solve_dixon(x, a, b);
        case KIND_CAN_SOLVE:
            return fmpq_mat_can_solve(x, a, b);
        case KIND_SOLVE_OR_CAN_SOLVE:
            if (fmpq_mat_is_square(a) && fmpq_mat_solve(x, a, b))
            {
                return 1;
            }
            fmpq_mat_zero(x);
            return fmpq_mat_can_solve(x, a, b);
        default:
            return fmpq_mat_inv(x, a);
    }
}

static int
check_line(char * line, int line_number)
{
    line[strcspn(line, "\r\n")] = '\0';
    if (line[0] == '\0')
    {
        return 0;
    }
    size_t op_len = strlen(current_op);
    char * eq;
    if (strncmp(line, current_op, op_len) != 0 || line[op_len] != '('
        || (eq = strstr(line, ") = ")) == NULL)
    {
        return error("unrecognized line", line_number);
    }
    *eq = '\0';
    char * args = line + op_len + 1;
    char * rest = eq + 4;
    checked++;
    size_t a_len = matrix_length(args);
    int single = current_kind == KIND_INV;
    if (a_len == 0 || (single ? args[a_len] != '\0' : strncmp(args + a_len, ", ", 2) != 0))
    {
        return error("unrecognized line", line_number);
    }
    args[a_len] = '\0';
    fmpq_mat_t a, b, x;
    if (!fmpq_mat_init_set_str_malachite(a, args))
    {
        fmpq_mat_clear(a);
        return error("unreadable input", line_number);
    }
    if (single)
    {
        fmpq_mat_init(b, 0, 0);
    }
    else if (!fmpq_mat_init_set_str_malachite(b, args + a_len + 2))
    {
        fmpq_mat_clear(a);
        fmpq_mat_clear(b);
        return error("unreadable input", line_number);
    }
    fmpq_mat_init(x, fmpq_mat_ncols(a), single ? fmpq_mat_nrows(a) : fmpq_mat_ncols(b));
    int success = compute(x, a, b);
    int result = 0;
    if (strcmp(rest, "None") == 0)
    {
        if (success)
        {
            result = error("Malachite gives None, but FLINT succeeds", line_number);
        }
    }
    else
    {
        size_t len = strlen(rest);
        if (len < 6 || strncmp(rest, "Some(", 5) != 0 || rest[len - 1] != ')')
        {
            result = error("unrecognized line", line_number);
        }
        else if (!success)
        {
            result = error("Malachite succeeds, but FLINT does not", line_number);
        }
        else
        {
            rest[len - 1] = '\0';
            fmpq_mat_t expected;
            if (!fmpq_mat_init_set_str_malachite(expected, rest + 5))
            {
                result = error("unreadable result", line_number);
            }
            else if (!fmpq_mat_equal(x, expected))
            {
                result = error("the results differ", line_number);
            }
            fmpq_mat_clear(expected);
        }
    }
    fmpq_mat_clear(a);
    fmpq_mat_clear(b);
    fmpq_mat_clear(x);
    return result;
}

static int
run(const char * arg, const char * name, const char * op, int kind)
{
    checked = 0;
    current_name = name;
    current_op = op;
    current_kind = kind;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_fmpq_mat_solve_fraction_free(const char * arg)
{
    return run(arg, "fmpq_mat_solve_fraction_free", "solve_fraction_free",
               KIND_SOLVE_FRACTION_FREE);
}

int
run_fmpq_mat_solve_dixon(const char * arg)
{
    return run(arg, "fmpq_mat_solve_dixon", "solve_dixon", KIND_SOLVE_DIXON);
}

int
run_fmpq_mat_solve_multi_mod(const char * arg)
{
    return run(arg, "fmpq_mat_solve_multi_mod", "solve_multi_mod", KIND_SOLVE_MULTI_MOD);
}

int
run_fmpq_mat_solve(const char * arg)
{
    return run(arg, "fmpq_mat_solve", "solve_nonsingular", KIND_SOLVE);
}

int
run_fmpq_mat_can_solve_fraction_free(const char * arg)
{
    return run(arg, "fmpq_mat_can_solve_fraction_free", "can_solve_fraction_free",
               KIND_CAN_SOLVE_FRACTION_FREE);
}

int
run_fmpq_mat_can_solve_multi_mod(const char * arg)
{
    return run(arg, "fmpq_mat_can_solve_multi_mod", "can_solve_multi_mod",
               KIND_CAN_SOLVE_MULTI_MOD);
}

int
run_fmpq_mat_can_solve_dixon(const char * arg)
{
    return run(arg, "fmpq_mat_can_solve_dixon", "can_solve_dixon", KIND_CAN_SOLVE_DIXON);
}

int
run_fmpq_mat_can_solve(const char * arg)
{
    return run(arg, "fmpq_mat_can_solve", "can_solve", KIND_CAN_SOLVE);
}

int
run_fmpq_mat_solve_or_can_solve(const char * arg)
{
    return run(arg, "fmpq_mat_solve_or_can_solve", "solve", KIND_SOLVE_OR_CAN_SOLVE);
}

int
run_fmpq_mat_inv(const char * arg)
{
    return run(arg, "fmpq_mat_inv", "inverse", KIND_INV);
}

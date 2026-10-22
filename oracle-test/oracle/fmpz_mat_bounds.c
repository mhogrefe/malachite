/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the IntegerMatrix bound demos against FLINT's fmpz_mat functions.

    The demos print `A.determinant_bound() = n`, `A.submatrix_determinant_bound() = n`, and
    `A.solve_bounds(B) = (n, d)`; the modes recompute the bounds with `fmpz_mat_det_bound`,
    `fmpz_mat_det_bound_submatrix`, and `fmpz_mat_solve_bound`.

    Matrices are read by fmpz_mat_init_set_str_malachite in util.c. Each mode treats any other
    nonempty line as an error, and an input with no lines at all.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <flint/fmpz.h>
#include <flint/fmpz_mat.h>

#include "oracle.h"

static long checked;
static const char * current_name;
static const char * current_op;
static int current_kind;

enum { KIND_DET_BOUND, KIND_DET_BOUND_SUBMATRIX, KIND_SOLVE_BOUND };

static int
unrecognized(int line_number)
{
    flint_printf("error in %s test, line %d: unrecognized line\n", current_name, line_number);
    return 1;
}

static int
compare(const fmpz_t actual, const char * expected_str, int line_number)
{
    fmpz_t expected;
    fmpz_init(expected);
    int result = 0;
    if (fmpz_set_str(expected, expected_str, 10) != 0)
    {
        flint_printf("error in %s test, line %d: unreadable result\n", current_name, line_number);
        result = 1;
    }
    else if (!fmpz_equal(actual, expected))
    {
        flint_printf("error in %s test, line %d: Malachite gives ", current_name, line_number);
        fmpz_print(expected);
        flint_printf(", but FLINT gives ");
        fmpz_print(actual);
        flint_printf("\n");
        result = 1;
    }
    fmpz_clear(expected);
    return result;
}

static int
check_line(char * line, int line_number)
{
    char needle[64];
    line[strcspn(line, "\r\n")] = '\0';
    snprintf(needle, sizeof(needle), ".%s(", current_op);
    char * at = strstr(line, needle);
    if (at == NULL)
    {
        return line[0] == '\0' ? 0 : unrecognized(line_number);
    }
    *at = '\0';
    char * a_str = line;
    char * arg = at + strlen(needle);
    char * eq = strstr(arg, ") = ");
    if (eq == NULL)
    {
        return unrecognized(line_number);
    }
    *eq = '\0';
    char * rest = eq + 4;
    checked++;
    fmpz_mat_t a;
    if (!fmpz_mat_init_set_str_malachite(a, a_str))
    {
        flint_printf("error in %s test, line %d: unreadable input\n", current_name, line_number);
        return 1;
    }
    int result = 0;
    fmpz_t n, d;
    fmpz_init(n);
    fmpz_init(d);
    if (current_kind == KIND_SOLVE_BOUND)
    {
        fmpz_mat_t b;
        size_t len = strlen(rest);
        char * comma = strstr(rest, ", ");
        if (!fmpz_mat_init_set_str_malachite(b, arg))
        {
            flint_printf("error in %s test, line %d: unreadable input\n", current_name,
                         line_number);
            result = 1;
        }
        else
        {
            if (len < 2 || rest[0] != '(' || rest[len - 1] != ')' || comma == NULL)
            {
                result = unrecognized(line_number);
            }
            else
            {
                rest[len - 1] = '\0';
                *comma = '\0';
                fmpz_mat_solve_bound(n, d, a, b);
                result = compare(n, rest + 1, line_number) || compare(d, comma + 2, line_number);
            }
            fmpz_mat_clear(b);
        }
    }
    else
    {
        if (arg[0] != '\0')
        {
            result = unrecognized(line_number);
        }
        else
        {
            if (current_kind == KIND_DET_BOUND)
            {
                fmpz_mat_det_bound(n, a);
            }
            else
            {
                fmpz_mat_det_bound_submatrix(n, a);
            }
            result = compare(n, rest, line_number);
        }
    }
    fmpz_clear(n);
    fmpz_clear(d);
    fmpz_mat_clear(a);
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
run_fmpz_mat_det_bound(const char * arg)
{
    return run(arg, "fmpz_mat_det_bound", "determinant_bound", KIND_DET_BOUND);
}

int
run_fmpz_mat_det_bound_submatrix(const char * arg)
{
    return run(arg, "fmpz_mat_det_bound_submatrix", "submatrix_determinant_bound",
               KIND_DET_BOUND_SUBMATRIX);
}

int
run_fmpz_mat_solve_bound(const char * arg)
{
    return run(arg, "fmpz_mat_solve_bound", "solve_bounds", KIND_SOLVE_BOUND);
}

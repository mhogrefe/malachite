/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `<name>(h, w) = M` lines, where M is a matrix as Malachite prints it, against
    arith_stirling_matrix_1u, arith_stirling_matrix_1, and arith_stirling_matrix_2, which fill an
    h-by-w matrix.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/arith.h>
#include <flint/fmpz_mat.h>

#include "oracle.h"

typedef void (*matrix_fn)(fmpz_mat_t);

static int
check_matrix_line(char * line, int line_number, const char * name, matrix_fn fill)
{
    char prefix[64];
    flint_sprintf(prefix, "%s(", name);
    line[strcspn(line, "\n")] = '\0';
    char * comma;
    char * close;
    if (strncmp(line, prefix, strlen(prefix)) != 0)
    {
        flint_printf("line %d: unreadable line\n", line_number);
        return 1;
    }
    ulong h = strtoul(line + strlen(prefix), &comma, 10);
    if (strncmp(comma, ", ", 2) != 0)
    {
        flint_printf("line %d: unreadable arguments\n", line_number);
        return 1;
    }
    ulong w = strtoul(comma + 2, &close, 10);
    if (strncmp(close, ") = ", 4) != 0)
    {
        flint_printf("line %d: unreadable line\n", line_number);
        return 1;
    }
    fmpz_mat_t actual, expected;
    if (!fmpz_mat_init_set_str_malachite(actual, close + 4))
    {
        flint_printf("line %d: unreadable matrix\n", line_number);
        return 1;
    }
    fmpz_mat_init(expected, h, w);
    fill(expected);
    int result = 0;
    if (fmpz_mat_nrows(actual) != (slong) h || fmpz_mat_ncols(actual) != (slong) w
        || !fmpz_mat_equal(expected, actual))
    {
        flint_printf("line %d: %s(%wu, %wu) mismatch\n", line_number, name, h, w);
        result = 1;
    }
    fmpz_mat_clear(actual);
    fmpz_mat_clear(expected);
    return result;
}

static int
check_1u_line(char * line, int line_number)
{
    return check_matrix_line(line, line_number, "unsigned_stirling_matrix_of_the_first_kind",
                             arith_stirling_matrix_1u);
}

static int
check_1_line(char * line, int line_number)
{
    return check_matrix_line(line, line_number, "stirling_matrix_of_the_first_kind",
                             arith_stirling_matrix_1);
}

static int
check_2_line(char * line, int line_number)
{
    return check_matrix_line(line, line_number, "stirling_matrix_of_the_second_kind",
                             arith_stirling_matrix_2);
}

int
run_arith_stirling_matrix_1u(const char * arg)
{
    return for_each_line(arg, check_1u_line);
}

int
run_arith_stirling_matrix_1(const char * arg)
{
    return for_each_line(arg, check_1_line);
}

int
run_arith_stirling_matrix_2(const char * arg)
{
    return for_each_line(arg, check_2_line);
}

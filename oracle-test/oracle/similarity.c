/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
    Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Oracle for Malachite's similarity transform of an `IntegerMatrix`. The demo prints
    `A.similarity_transform(r, d) = B`; the `fmpz_mat_similarity` mode reads A, r, and d, applies
    `fmpz_mat_similarity` to a copy of A, and requires the result to equal B. Malachite panics
    rather than printing a line when A is not square or r is not a row index, so FLINT, which checks
    neither, only sees valid inputs.

    Matrices are read by fmpz_mat_init_set_str_malachite in util.c. Any other nonempty line is an
    error, and so is an input with no lines at all.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <flint/fmpz.h>
#include <flint/fmpz_mat.h>

#include "oracle.h"

static long checked;

static int
error(const char * message, int line_number)
{
    flint_printf("error in fmpz_mat_similarity test, line %d: %s\n", line_number, message);
    return 1;
}

static int
check_line(char * line, int line_number)
{
    line[strcspn(line, "\r\n")] = '\0';
    if (line[0] == '\0')
    {
        return 0;
    }
    char * pieces[3];
    char * rest;
    if (!split_method_call(line, ".similarity_transform(", pieces, 3, &rest)
        || strncmp(rest, " = ", 3) != 0)
    {
        return error("unrecognized line", line_number);
    }
    rest += 3;
    checked++;
    char * end;
    long long r = strtoll(pieces[1], &end, 10);
    fmpz_t d;
    fmpz_init(d);
    fmpz_mat_t a, expected;
    if (end == pieces[1] || *end != '\0' || fmpz_set_str(d, pieces[2], 10) != 0)
    {
        fmpz_clear(d);
        return error("unreadable r or d", line_number);
    }
    if (!fmpz_mat_init_set_str_malachite(a, pieces[0]))
    {
        fmpz_clear(d);
        return error("unreadable input", line_number);
    }
    if (!fmpz_mat_init_set_str_malachite(expected, rest))
    {
        fmpz_clear(d);
        fmpz_mat_clear(a);
        return error("unreadable result", line_number);
    }
    int result = 0;
    if (fmpz_mat_nrows(a) != fmpz_mat_ncols(a) || r < 0 || r >= fmpz_mat_nrows(a))
    {
        result = error("the input is not a square matrix with a valid row index", line_number);
    }
    else
    {
        fmpz_mat_similarity(a, r, d);
        if (!fmpz_mat_equal(a, expected))
        {
            result = error("the transforms differ", line_number);
        }
    }
    fmpz_clear(d);
    fmpz_mat_clear(a);
    fmpz_mat_clear(expected);
    return result;
}

int
run_fmpz_mat_similarity(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("fmpz_mat_similarity", checked);
}

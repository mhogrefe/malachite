/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
    Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Oracles for Malachite's characteristic polynomials of `IntegerMatrix`es. The demo prints
    `A.characteristic_polynomial() = p`; the `fmpz_mat_charpoly` mode recomputes p with
    `fmpz_mat_charpoly`. The bound demo prints `A.characteristic_polynomial_bound() = b`; the
    `fmpz_mat_charpoly_bound` mode requires b to equal `fmpz_mat_charpoly_bound` exactly, since
    Malachite rounds its floating-point magnitudes as FLINT does. Malachite panics rather than
    printing a line for a non-square matrix.

    Matrices are read by fmpz_mat_init_set_str_malachite and polynomials by
    fmpz_poly_set_str_malachite in util.c. Any other nonempty line is an error, and so is an input
    with no lines at all.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <flint/fmpz.h>
#include <flint/fmpz_mat.h>
#include <flint/fmpz_poly.h>

#include "oracle.h"

static long checked;
static const char * current_name;
static int bound_mode;

static int
error(const char * message, int line_number)
{
    flint_printf("error in %s test, line %d: %s\n", current_name, line_number, message);
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
    const char * method = bound_mode ? ".characteristic_polynomial_bound() = "
                                     : ".characteristic_polynomial() = ";
    char * at = strstr(line, method);
    if (at == NULL)
    {
        return error("unrecognized line", line_number);
    }
    *at = '\0';
    char * rest = at + strlen(method);
    checked++;
    fmpz_mat_t a;
    if (!fmpz_mat_init_set_str_malachite(a, line))
    {
        return error("unreadable input", line_number);
    }
    int result = 0;
    if (fmpz_mat_nrows(a) != fmpz_mat_ncols(a))
    {
        result = error("the input is not square", line_number);
    }
    else if (bound_mode)
    {
        fmpz_t expected, actual;
        fmpz_init(expected);
        fmpz_init(actual);
        if (fmpz_set_str(expected, rest, 10) != 0)
        {
            result = error("unreadable result", line_number);
        }
        else
        {
            fmpz_mat_charpoly_bound(actual, a);
            if (!fmpz_equal(actual, expected))
            {
                result = error("the bounds differ", line_number);
            }
        }
        fmpz_clear(expected);
        fmpz_clear(actual);
    }
    else
    {
        fmpz_poly_t expected, actual;
        fmpz_poly_init(expected);
        fmpz_poly_init(actual);
        if (!fmpz_poly_set_str_malachite(expected, rest))
        {
            result = error("unreadable result", line_number);
        }
        else
        {
            fmpz_mat_charpoly(actual, a);
            if (!fmpz_poly_equal(actual, expected))
            {
                result = error("the characteristic polynomials differ", line_number);
            }
        }
        fmpz_poly_clear(expected);
        fmpz_poly_clear(actual);
    }
    fmpz_mat_clear(a);
    return result;
}

static int
run(const char * arg, const char * name, int bound)
{
    checked = 0;
    current_name = name;
    bound_mode = bound;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_fmpz_mat_charpoly(const char * arg)
{
    return run(arg, "fmpz_mat_charpoly", 0);
}

int
run_fmpz_mat_charpoly_bound(const char * arg)
{
    return run(arg, "fmpz_mat_charpoly_bound", 1);
}

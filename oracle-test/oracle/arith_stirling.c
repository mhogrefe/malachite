/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `touchard_polynomial(n) = p` and `touchard_polynomial_truncated(n, len) = p` lines
    against arith_stirling_number_2_vec, whose row of S(n, k) for k < len is the coefficients of
    the truncated Touchard polynomial.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/arith.h>
#include <flint/fmpz.h>
#include <flint/fmpz_poly.h>
#include <flint/fmpz_vec.h>

#include "oracle.h"

static int
check_touchard_line(char * line, int line_number)
{
    const char * full = "touchard_polynomial(";
    const char * truncated = "touchard_polynomial_truncated(";
    line[strcspn(line, "\n")] = '\0';
    char * needle = strstr(line, ") = ");
    ulong n, len;
    if (needle == NULL)
    {
        flint_printf("line %d: unreadable line\n", line_number);
        return 1;
    }
    *needle = '\0';
    const char * p_str = needle + strlen(") = ");
    if (strncmp(line, truncated, strlen(truncated)) == 0)
    {
        char * comma;
        n = strtoul(line + strlen(truncated), &comma, 10);
        if (strncmp(comma, ", ", 2) != 0)
        {
            flint_printf("line %d: unreadable arguments\n", line_number);
            return 1;
        }
        len = strtoul(comma + 2, NULL, 10);
    }
    else if (strncmp(line, full, strlen(full)) == 0)
    {
        n = strtoul(line + strlen(full), NULL, 10);
        len = n + 1;
    }
    else
    {
        flint_printf("line %d: unreadable line\n", line_number);
        return 1;
    }
    fmpz_poly_t actual, expected;
    fmpz_poly_init(actual);
    fmpz_poly_init(expected);
    int result = 1;
    if (fmpz_poly_set_str_malachite(actual, p_str))
    {
        fmpz * row = _fmpz_vec_init(len);
        arith_stirling_number_2_vec(row, n, len);
        fmpz_poly_fit_length(expected, len);
        _fmpz_vec_set(expected->coeffs, row, len);
        _fmpz_poly_set_length(expected, len);
        _fmpz_poly_normalise(expected);
        _fmpz_vec_clear(row, len);
        if (fmpz_poly_equal(expected, actual))
            result = 0;
        else
            flint_printf("line %d: touchard_polynomial_truncated(%wu, %wu) mismatch\n",
                         line_number, n, len);
    }
    else
    {
        flint_printf("line %d: unreadable polynomial\n", line_number);
    }
    fmpz_poly_clear(actual);
    fmpz_poly_clear(expected);
    return result;
}

int
run_arith_stirling_number_2_vec(const char * arg)
{
    return for_each_line(arg, check_touchard_line);
}

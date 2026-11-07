/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `touchard_polynomial(n) = p` and `touchard_polynomial_truncated(n, len) = p` lines
    against arith_stirling_number_2_vec, whose row of S(n, k) for k < len is the coefficients of
    the truncated Touchard polynomial; `rising_factorial_polynomial(n) = p` and
    `rising_factorial_polynomial_truncated(n, len) = p` lines against arith_stirling_number_1u_vec,
    whose row is the coefficients of the truncated rising factorial polynomial; `S(n, k) = s`
    lines against arith_stirling_number_2; and `[n, k] = s` lines against arith_stirling_number_1u.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/arith.h>
#include <flint/fmpz.h>
#include <flint/fmpz_poly.h>
#include <flint/fmpz_vec.h>

#include "oracle.h"

typedef void (*row_fn)(fmpz *, ulong, slong);

/* `name` is the Malachite function, without "_truncated" */
static int
check_row_line(char * line, int line_number, const char * name, row_fn row_function)
{
    char full[64];
    char truncated[80];
    flint_sprintf(full, "%s(", name);
    flint_sprintf(truncated, "%s_truncated(", name);
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
        row_function(row, n, len);
        fmpz_poly_fit_length(expected, len);
        _fmpz_vec_set(expected->coeffs, row, len);
        _fmpz_poly_set_length(expected, len);
        _fmpz_poly_normalise(expected);
        _fmpz_vec_clear(row, len);
        if (fmpz_poly_equal(expected, actual))
            result = 0;
        else
            flint_printf("line %d: %s_truncated(%wu, %wu) mismatch\n", line_number, name, n,
                         len);
    }
    else
    {
        flint_printf("line %d: unreadable polynomial\n", line_number);
    }
    fmpz_poly_clear(actual);
    fmpz_poly_clear(expected);
    return result;
}

static int
check_touchard_line(char * line, int line_number)
{
    return check_row_line(line, line_number, "touchard_polynomial", arith_stirling_number_2_vec);
}

int
run_arith_stirling_number_2_vec(const char * arg)
{
    return for_each_line(arg, check_touchard_line);
}

static int
check_rising_factorial_line(char * line, int line_number)
{
    return check_row_line(line, line_number, "rising_factorial_polynomial",
                          arith_stirling_number_1u_vec);
}

int
run_arith_stirling_number_1u_vec(const char * arg)
{
    return for_each_line(arg, check_rising_factorial_line);
}

typedef void (*value_fn)(fmpz_t, ulong, ulong);

/* Lines are `<open>n, k<close> = s`, such as `S(n, k) = s` or `[n, k] = s`. */
static int
check_value_line(char * line, int line_number, const char * open, const char * close_str,
                 value_fn value_function)
{
    line[strcspn(line, "\n")] = '\0';
    char * comma;
    char * close;
    char suffix[16];
    flint_sprintf(suffix, "%s = ", close_str);
    if (strncmp(line, open, strlen(open)) != 0)
    {
        flint_printf("line %d: unreadable line\n", line_number);
        return 1;
    }
    ulong n = strtoul(line + strlen(open), &comma, 10);
    if (strncmp(comma, ", ", 2) != 0)
    {
        flint_printf("line %d: unreadable arguments\n", line_number);
        return 1;
    }
    ulong k = strtoul(comma + 2, &close, 10);
    if (strncmp(close, suffix, strlen(suffix)) != 0)
    {
        flint_printf("line %d: unreadable line\n", line_number);
        return 1;
    }
    fmpz_t actual, expected;
    fmpz_init(actual);
    fmpz_init(expected);
    int result = 1;
    if (fmpz_set_str(actual, close + strlen(suffix), 10) == 0)
    {
        value_function(expected, n, k);
        if (fmpz_equal(expected, actual))
            result = 0;
        else
            flint_printf("line %d: %s%wu, %wu%s mismatch\n", line_number, open, n, k, close_str);
    }
    else
    {
        flint_printf("line %d: unreadable value\n", line_number);
    }
    fmpz_clear(actual);
    fmpz_clear(expected);
    return result;
}

static int
check_stirling_2_line(char * line, int line_number)
{
    return check_value_line(line, line_number, "S(", ")", arith_stirling_number_2);
}

static int
check_stirling_1u_line(char * line, int line_number)
{
    return check_value_line(line, line_number, "[", "]", arith_stirling_number_1u);
}

int
run_arith_stirling_number_1u(const char * arg)
{
    return for_each_line(arg, check_stirling_1u_line);
}

int
run_arith_stirling_number_2(const char * arg)
{
    return for_each_line(arg, check_stirling_2_line);
}

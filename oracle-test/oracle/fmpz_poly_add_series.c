/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the IntegerPolynomial add_truncated and sub_truncated demos against
    fmpz_poly_add_series and fmpz_poly_sub_series. The shapes are `(P).method(Q, n) = R`,
    `(P).method(&(Q), n) = R`, `(&(P)).method(Q, n) = R`, `(&(P)).method(&(Q), n) = R`,
    `p := P; p.method_assign(Q, n); p = R`, and `p := P; p.method_assign(&(Q), n); p = R`, with
    `method` either `add_truncated` or `sub_truncated`. Each mode treats any other nonempty line
    as an error, and an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/fmpz_poly.h>

#include "oracle.h"

static long checked;
static const char * current_name;
static const char * current_method;
static const char * current_assign_method;
static void (* current_function)(fmpz_poly_t, const fmpz_poly_t, const fmpz_poly_t, slong);

static int
check_line(char * line, int line_number)
{
    char * p_str;
    char * arg;
    char * r_str;
    if (!split_polynomial_scalar_line(line, current_method, current_assign_method, NULL, &p_str,
                                      &arg, &r_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in %s test, line %d: unrecognized line\n", current_name,
                     line_number);
        return 1;
    }
    /* The argument is `Q, n` or `&(Q), n`. Polynomials are displayed without spaces, so the last
       ", " separates the two. */
    char * comma = strrchr(arg, ',');
    if (comma == NULL || comma[1] != ' ')
    {
        flint_printf("error in %s test, line %d: unrecognized argument\n", current_name,
                     line_number);
        return 1;
    }
    *comma = '\0';
    char * n_str = comma + 2;
    char * q_str = arg;
    if (strncmp(q_str, "&(", 2) == 0)
    {
        size_t len = strlen(q_str);
        if (q_str[len - 1] != ')')
        {
            flint_printf("error in %s test, line %d: unrecognized argument\n", current_name,
                         line_number);
            return 1;
        }
        q_str[len - 1] = '\0';
        q_str += 2;
    }
    char * end;
    unsigned long long n = strtoull(n_str, &end, 10);
    checked++;
    int result = 0;
    fmpz_poly_t p, q, expected, r;
    fmpz_poly_init(p);
    fmpz_poly_init(q);
    fmpz_poly_init(expected);
    fmpz_poly_init(r);
    if (*end != '\0' || n > WORD_MAX || !fmpz_poly_set_str_malachite(p, p_str)
        || !fmpz_poly_set_str_malachite(q, q_str)
        || !fmpz_poly_set_str_malachite(expected, r_str))
    {
        flint_printf("error in %s test, line %d: unreadable input\n", current_name, line_number);
        result = 1;
    }
    else
    {
        current_function(r, p, q, (slong) n);
        if (!fmpz_poly_equal(r, expected))
        {
            flint_printf("error in %s test, line %d. FLINT: ", current_name, line_number);
            fmpz_poly_print_pretty(r, "x");
            flint_printf("\n");
            result = 1;
        }
    }
    fmpz_poly_clear(p);
    fmpz_poly_clear(q);
    fmpz_poly_clear(expected);
    fmpz_poly_clear(r);
    return result;
}

static int
run(const char * arg, const char * name, const char * method, const char * assign_method,
    void (* function)(fmpz_poly_t, const fmpz_poly_t, const fmpz_poly_t, slong))
{
    checked = 0;
    current_name = name;
    current_method = method;
    current_assign_method = assign_method;
    current_function = function;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_fmpz_poly_add_series(const char * arg)
{
    return run(arg, "fmpz_poly_add_series", "add_truncated", "add_truncated_assign",
               fmpz_poly_add_series);
}

int
run_fmpz_poly_sub_series(const char * arg)
{
    return run(arg, "fmpz_poly_sub_series", "sub_truncated", "sub_truncated_assign",
               fmpz_poly_sub_series);
}

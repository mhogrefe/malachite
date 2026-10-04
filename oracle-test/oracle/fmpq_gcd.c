/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `x.gcd(y) = g` lines against fmpq_gcd, and `x.extended_gcd(y) = (g, u, v)` lines
    against fmpq_gcd_cofactors. Malachite's extended GCD returns Bézout cofactors for FLINT's
    quotient cofactors, so the check is a bridge: the GCDs must be equal, and u and v must
    satisfy u * abar + v * bbar == 1 with FLINT's quotients abar and bbar.
*/

#include <string.h>

#include <flint/fmpq.h>

#include "oracle.h"

static int
parse_rational_prefix(char ** line, const char * needle, fmpq_t x, fmpq_t y)
{
    char * start = strstr(*line, needle);
    if (start == NULL)
    {
        return 0;
    }
    *start = '\0';
    char * y_str = start + strlen(needle);
    char * close = strstr(y_str, ") = ");
    if (close == NULL)
    {
        return 0;
    }
    *close = '\0';
    if (fmpq_set_str(x, *line, 10) != 0 || fmpq_set_str(y, y_str, 10) != 0)
    {
        return 0;
    }
    *line = close + strlen(") = ");
    return 1;
}

static int
check_gcd_line(char * line, int line_number)
{
    fmpq_t x, y, g, expected;
    fmpq_init(x);
    fmpq_init(y);
    fmpq_init(g);
    fmpq_init(expected);
    int result = 0;
    char * rest = line;
    if (parse_rational_prefix(&rest, ".gcd(", x, y))
    {
        if (fmpq_set_str(expected, rest, 10) != 0)
        {
            flint_printf("error in fmpq_gcd test, line %d: unreadable result\n", line_number);
            result = 1;
        }
        else
        {
            fmpq_gcd(g, x, y);
            if (!fmpq_equal(g, expected))
            {
                flint_printf("error in fmpq_gcd test, line %d. FLINT: ", line_number);
                fmpq_print(g);
                flint_printf("\n");
                result = 1;
            }
        }
    }
    fmpq_clear(x);
    fmpq_clear(y);
    fmpq_clear(g);
    fmpq_clear(expected);
    return result;
}

static int
check_gcd_cofactors_line(char * line, int line_number)
{
    fmpq_t x, y, g, expected_g;
    fmpz_t abar, bbar, u, v, t;
    fmpq_init(x);
    fmpq_init(y);
    fmpq_init(g);
    fmpq_init(expected_g);
    fmpz_init(abar);
    fmpz_init(bbar);
    fmpz_init(u);
    fmpz_init(v);
    fmpz_init(t);
    int result = 0;
    char * rest = line;
    if (parse_rational_prefix(&rest, ".extended_gcd(", x, y))
    {
        /* rest is "(g, u, v)" */
        rest++;
        char * sep = strstr(rest, ", ");
        *sep = '\0';
        int ok = fmpq_set_str(expected_g, rest, 10) == 0;
        rest = sep + 2;
        sep = strstr(rest, ", ");
        *sep = '\0';
        fmpz_set_str(u, rest, 10);
        rest = sep + 2;
        sep = strchr(rest, ')');
        *sep = '\0';
        fmpz_set_str(v, rest, 10);
        if (!ok)
        {
            flint_printf("error in fmpq_gcd_cofactors test, line %d: unreadable result\n",
                         line_number);
            result = 1;
        }
        else
        {
            fmpq_gcd_cofactors(g, abar, bbar, x, y);
            fmpz_mul(t, u, abar);
            fmpz_addmul(t, v, bbar);
            /* both zero: the identity degenerates and all outputs are zero */
            int both_zero = fmpq_is_zero(x) && fmpq_is_zero(y);
            if (!fmpq_equal(g, expected_g)
                || (!both_zero && !fmpz_is_one(t))
                || (both_zero && !(fmpz_is_zero(u) && fmpz_is_zero(v))))
            {
                flint_printf("error in fmpq_gcd_cofactors test, line %d. FLINT: g=",
                             line_number);
                fmpq_print(g);
                flint_printf(" abar=");
                fmpz_print(abar);
                flint_printf(" bbar=");
                fmpz_print(bbar);
                flint_printf("\n");
                result = 1;
            }
        }
    }
    fmpq_clear(x);
    fmpq_clear(y);
    fmpq_clear(g);
    fmpq_clear(expected_g);
    fmpz_clear(abar);
    fmpz_clear(bbar);
    fmpz_clear(u);
    fmpz_clear(v);
    fmpz_clear(t);
    return result;
}

int
run_fmpq_gcd(const char * arg)
{
    return for_each_line(arg, check_gcd_line);
}

int
run_fmpq_gcd_cofactors(const char * arg)
{
    return for_each_line(arg, check_gcd_cofactors_line);
}

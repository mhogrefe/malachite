/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `r1.crt(m1, r2, m2) = Some(x)|None` lines against fmpz_CRT with sign = 0, and
    `r1.balanced_crt(m1, r2, m2) = Some(x)|None` lines against fmpz_CRT with sign = 1. FLINT
    aborts on noncoprime moduli instead of reporting failure, so the oracle checks the GCD first
    and requires `None` exactly when the moduli share a factor. Both the Natural and the
    primitive-integer demos print the unsigned shape, so that mode serves both.
*/

#include "oracle.h"

static int
check_fmpz_CRT_line(char * line, int line_number, const char * method, int sign)
{
    char * pieces[4];
    char * rest;
    if (!split_method_call(line, method, pieces, 4, &rest))
    {
        return 0;
    }
    int result = 0;
    fmpz_t r1, m1, r2, m2, g, out, expected;
    fmpz_init(r1);
    fmpz_init(m1);
    fmpz_init(r2);
    fmpz_init(m2);
    fmpz_init(g);
    fmpz_init(out);
    fmpz_init(expected);
    fmpz_set_str(r1, pieces[0], 10);
    fmpz_set_str(m1, pieces[1], 10);
    fmpz_set_str(r2, pieces[2], 10);
    fmpz_set_str(m2, pieces[3], 10);
    int has_expected = parse_option_fmpz(rest, expected);
    fmpz_gcd(g, m1, m2);
    int coprime = fmpz_is_one(g);
    if (coprime != has_expected)
    {
        flint_printf("error in fmpz_CRT test, line %d: FLINT coprime=%d\n", line_number, coprime);
        result = 1;
    }
    else if (coprime)
    {
        fmpz_CRT(out, r1, m1, r2, m2, sign);
        if (!fmpz_equal(out, expected))
        {
            flint_printf("error in fmpz_CRT test, line %d. FLINT: out=", line_number);
            fmpz_print(out);
            flint_printf("\n");
            result = 1;
        }
    }
    fmpz_clear(r1);
    fmpz_clear(m1);
    fmpz_clear(r2);
    fmpz_clear(m2);
    fmpz_clear(g);
    fmpz_clear(out);
    fmpz_clear(expected);
    return result;
}

static int
check_unsigned_line(char * line, int line_number)
{
    return check_fmpz_CRT_line(line, line_number, ".crt(", 0);
}

static int
check_balanced_line(char * line, int line_number)
{
    return check_fmpz_CRT_line(line, line_number, ".balanced_crt(", 1);
}

int
run_fmpz_CRT(const char * arg)
{
    return for_each_line(arg, check_unsigned_line);
}

int
run_fmpz_CRT_balanced(const char * arg)
{
    return for_each_line(arg, check_balanced_line);
}

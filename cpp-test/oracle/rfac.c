/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `x.rising_factorial(n) = v` lines against fmpz_rfac_ui; when the base also fits in a
    ulong, fmpz_rfac_uiui is checked against the same expectation, so one mode covers both
    functions. Both the Natural and the Integer demos print this shape.
*/

#include <stdlib.h>

#include "oracle.h"

static int
check_rfac_line(char * line, int line_number)
{
    char * pieces[2];
    char * rest;
    if (!split_method_call(line, ".rising_factorial(", pieces, 2, &rest))
    {
        return 0;
    }
    int result = 0;
    fmpz_t x, out, expected;
    fmpz_init(x);
    fmpz_init(out);
    fmpz_init(expected);
    fmpz_set_str(x, pieces[0], 10);
    ulong n = strtoull(pieces[1], NULL, 10);
    /* skip " = " */
    fmpz_set_str(expected, rest + 3, 10);
    fmpz_rfac_ui(out, x, n);
    if (!fmpz_equal(out, expected))
    {
        flint_printf("error in fmpz_rfac_ui test, line %d. FLINT: out=", line_number);
        fmpz_print(out);
        flint_printf("\n");
        result = 1;
    }
    if (fmpz_sgn(x) >= 0 && fmpz_abs_fits_ui(x))
    {
        fmpz_rfac_uiui(out, fmpz_get_ui(x), n);
        if (!fmpz_equal(out, expected))
        {
            flint_printf("error in fmpz_rfac_uiui test, line %d. FLINT: out=", line_number);
            fmpz_print(out);
            flint_printf("\n");
            result = 1;
        }
    }
    fmpz_clear(x);
    fmpz_clear(out);
    fmpz_clear(expected);
    return result;
}

int
run_fmpz_rfac(const char * arg)
{
    return for_each_line(arg, check_rfac_line);
}

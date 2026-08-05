/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `b.mod_div(c, m) = Some(q)|None` lines against fmpz_mod_divides. Both the Natural and
    the primitive-integer demos print this shape, so one mode serves both.
*/

#include <string.h>

#include <flint/fmpz_mod.h>

#include "oracle.h"

static int
check_fmpz_mod_divides_line(char * line, int line_number)
{
    char * pieces[3];
    char * rest;
    if (!split_method_call(line, ".mod_div(", pieces, 3, &rest))
    {
        return 0;
    }
    int result = 0;
    fmpz_t b, c, n, a, expected;
    fmpz_init(b);
    fmpz_init(c);
    fmpz_init(n);
    fmpz_init(a);
    fmpz_init(expected);
    fmpz_set_str(b, pieces[0], 10);
    fmpz_set_str(c, pieces[1], 10);
    fmpz_set_str(n, pieces[2], 10);
    int has_expected = parse_option_fmpz(rest, expected);
    fmpz_mod_ctx_t ctx;
    fmpz_mod_ctx_init(ctx, n);
    int ok = fmpz_mod_divides(a, b, c, ctx);
    fmpz_mod_ctx_clear(ctx);
    if (ok != has_expected || (ok && !fmpz_equal(a, expected)))
    {
        flint_printf("error in fmpz_mod_divides test, line %d. FLINT: ok=%d a=", line_number, ok);
        fmpz_print(a);
        flint_printf("\n");
        result = 1;
    }
    fmpz_clear(b);
    fmpz_clear(c);
    fmpz_clear(n);
    fmpz_clear(a);
    fmpz_clear(expected);
    return result;
}

int
run_fmpz_mod_divides(const char * arg)
{
    return for_each_line(arg, check_fmpz_mod_divides_line);
}

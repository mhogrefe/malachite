/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `extended_gcd_partial(r2, r1, l) = (co2, co1, r2_out, r1_out)` lines against
    fmpz_xgcd_partial. The outputs are a mid-sequence state of a Lehmer run and are not
    canonical, so bit-for-bit agreement here pins the port to FLINT's exact stopping states and
    cofactor signs.
*/

#include <string.h>

#include "oracle.h"

static int
parse_fmpz_arg(char ** pp, fmpz_t out)
{
    char * p = *pp;
    char * end = strpbrk(p, ",)");
    if (end == NULL)
    {
        return 0;
    }
    char saved = *end;
    *end = '\0';
    fmpz_set_str(out, p, 10);
    *end = saved;
    *pp = (saved == ',') ? end + 2 : end + 1;
    return 1;
}

static int
check_xgcd_partial_line(char * line, int line_number)
{
    char * start = strstr(line, "extended_gcd_partial(");
    if (start == NULL)
    {
        return 0;
    }
    char * p = start + strlen("extended_gcd_partial(");
    int result = 0;
    fmpz_t r2, r1, l, co2, co1, e_co2, e_co1, e_r2, e_r1;
    fmpz_init(r2);
    fmpz_init(r1);
    fmpz_init(l);
    fmpz_init(co2);
    fmpz_init(co1);
    fmpz_init(e_co2);
    fmpz_init(e_co1);
    fmpz_init(e_r2);
    fmpz_init(e_r1);
    int ok = parse_fmpz_arg(&p, r2) && parse_fmpz_arg(&p, r1) && parse_fmpz_arg(&p, l);
    /* skip " = (" */
    p += 4;
    ok = ok && parse_fmpz_arg(&p, e_co2) && parse_fmpz_arg(&p, e_co1)
        && parse_fmpz_arg(&p, e_r2) && parse_fmpz_arg(&p, e_r1);
    if (!ok)
    {
        flint_printf("error in fmpz_xgcd_partial test, line %d: malformed line\n", line_number);
        result = 1;
    }
    else
    {
        fmpz_xgcd_partial(co2, co1, r2, r1, l);
        if (!fmpz_equal(co2, e_co2) || !fmpz_equal(co1, e_co1) || !fmpz_equal(r2, e_r2)
            || !fmpz_equal(r1, e_r1))
        {
            flint_printf("error in fmpz_xgcd_partial test, line %d. FLINT: (", line_number);
            fmpz_print(co2);
            flint_printf(", ");
            fmpz_print(co1);
            flint_printf(", ");
            fmpz_print(r2);
            flint_printf(", ");
            fmpz_print(r1);
            flint_printf(")\n");
            result = 1;
        }
    }
    fmpz_clear(r2);
    fmpz_clear(r1);
    fmpz_clear(l);
    fmpz_clear(co2);
    fmpz_clear(co1);
    fmpz_clear(e_co2);
    fmpz_clear(e_co1);
    fmpz_clear(e_r2);
    fmpz_clear(e_r1);
    return result;
}

int
run_fmpz_xgcd_partial(const char * arg)
{
    return for_each_line(arg, check_xgcd_partial_line);
}

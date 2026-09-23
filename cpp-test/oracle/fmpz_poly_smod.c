/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `(p).balanced_mod(m) = r`, `(&(p)).balanced_mod(m) = r`, and
    `p := p; p.balanced_mod_assign(m); p = r` lines, where `p` and `r` are integer polynomials and
    `m` a nonzero integer, against fmpz_poly_scalar_smod_fmpz. Any other nonempty line is an error,
    as is an input with no lines at all.

    FLINT documents that function for `p > 0` only, and for a negative modulus its result is not
    a balanced remainder at all: it reduces into `[0, |p|)` and then subtracts `p` from every
    coefficient, because `floor(p/2)` is negative. Malachite's balanced remainder depends only on
    `|m|`, so the oracle passes `|m|` to FLINT, which keeps every line in FLINT's contract without
    skipping any.
*/

#include <string.h>

#include <flint/fmpz_poly.h>

#include "oracle.h"

static long checked;

static int
check_balanced_mod_line(char * line, int line_number)
{
    char * receiver;
    char * modulus;
    char * expected_str;
    if (!split_polynomial_scalar_line(line, "balanced_mod", "balanced_mod_assign", NULL,
                                      &receiver, &modulus,
                                      &expected_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in fmpz_poly_scalar_smod_fmpz test, line %d: unrecognized line\n",
                     line_number);
        return 1;
    }
    checked++;
    int result = 0;
    fmpz_poly_t p, expected, r;
    fmpz_t m;
    fmpz_poly_init(p);
    fmpz_poly_init(expected);
    fmpz_poly_init(r);
    fmpz_init(m);
    if (!fmpz_poly_set_str_malachite(p, receiver)
        || !fmpz_poly_set_str_malachite(expected, expected_str)
        || fmpz_set_str(m, modulus, 10) != 0 || fmpz_is_zero(m))
    {
        flint_printf("error in fmpz_poly_scalar_smod_fmpz test, line %d: unreadable input\n",
                     line_number);
        result = 1;
    }
    else
    {
        fmpz_abs(m, m);
        fmpz_poly_scalar_smod_fmpz(r, p, m);
        if (!fmpz_poly_equal(r, expected))
        {
            flint_printf("error in fmpz_poly_scalar_smod_fmpz test, line %d. FLINT: ",
                         line_number);
            fmpz_poly_print_pretty(r, "x");
            flint_printf("\n");
            result = 1;
        }
    }
    fmpz_poly_clear(p);
    fmpz_poly_clear(expected);
    fmpz_poly_clear(r);
    fmpz_clear(m);
    return result;
}

int
run_fmpz_poly_scalar_smod_fmpz(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_balanced_mod_line);
    return result != 0 ? result : require_some_lines("fmpz_poly_scalar_smod_fmpz", checked);
}

/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `(P).taylor_shift(c) = R`, `(&(P)).taylor_shift(c) = R`, and
    `p := P; p.taylor_shift_assign(c); p = R` lines, where `P` and `R` are integer polynomials
    and `c` an integer, against fmpz_poly_taylor_shift, and also against each of its algorithm
    variants, fmpz_poly_taylor_shift_horner, fmpz_poly_taylor_shift_divconquer, and
    fmpz_poly_taylor_shift_multi_mod, all of which must agree. Any other nonempty line is an
    error, as is an input with no lines at all.
*/

#include <flint/fmpz_poly.h>

#include "oracle.h"

static long checked;

static int
check_line(char * line, int line_number)
{
    char * receiver;
    char * shift;
    char * expected_str;
    if (!split_polynomial_scalar_line(line, "taylor_shift", "taylor_shift_assign", NULL,
                                      &receiver, &shift, &expected_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in fmpz_poly_taylor_shift test, line %d: unrecognized line\n",
                     line_number);
        return 1;
    }
    checked++;
    int result = 0;
    fmpz_poly_t p, expected, r;
    fmpz_t c;
    fmpz_poly_init(p);
    fmpz_poly_init(expected);
    fmpz_poly_init(r);
    fmpz_init(c);
    if (!fmpz_poly_set_str_malachite(p, receiver)
        || !fmpz_poly_set_str_malachite(expected, expected_str) || fmpz_set_str(c, shift, 10) != 0)
    {
        flint_printf("error in fmpz_poly_taylor_shift test, line %d: unreadable input\n",
                     line_number);
        result = 1;
    }
    else
    {
        const char * names[4] = {"fmpz_poly_taylor_shift", "fmpz_poly_taylor_shift_horner",
                                 "fmpz_poly_taylor_shift_divconquer",
                                 "fmpz_poly_taylor_shift_multi_mod"};
        for (int k = 0; k < 4 && result == 0; k++)
        {
            switch (k)
            {
                case 0: fmpz_poly_taylor_shift(r, p, c); break;
                case 1: fmpz_poly_taylor_shift_horner(r, p, c); break;
                case 2: fmpz_poly_taylor_shift_divconquer(r, p, c); break;
                default: fmpz_poly_taylor_shift_multi_mod(r, p, c); break;
            }
            if (!fmpz_poly_equal(r, expected))
            {
                flint_printf("error in fmpz_poly_taylor_shift test, line %d, by %s. FLINT: ",
                             line_number, names[k]);
                fmpz_poly_print(r);
                flint_printf("\n");
                result = 1;
            }
        }
    }
    fmpz_poly_clear(p);
    fmpz_poly_clear(expected);
    fmpz_poly_clear(r);
    fmpz_clear(c);
    return result;
}

int
run_fmpz_poly_taylor_shift(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("fmpz_poly_taylor_shift", checked);
}

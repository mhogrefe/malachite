/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `(P).taylor_shift(c) = R`, `(&(P)).taylor_shift(c) = R`, and
    `p := P; p.taylor_shift_assign(c); p = R` lines, where `P` and `R` are rational polynomials
    and `c` a rational, against fmpq_poly_compose with the inner polynomial x + c: FLINT has no
    fmpq_poly_taylor_shift, and composing with x + c computes the same polynomial by another
    route. Any other nonempty line is an error, as is an input with no lines at all.
*/

#include <flint/fmpq.h>
#include <flint/fmpq_poly.h>

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
        flint_printf("error in fmpq_poly_taylor_shift test, line %d: unrecognized line\n",
                     line_number);
        return 1;
    }
    checked++;
    int result = 0;
    fmpq_poly_t p, expected, inner, r;
    fmpq_t c;
    fmpq_poly_init(p);
    fmpq_poly_init(expected);
    fmpq_poly_init(inner);
    fmpq_poly_init(r);
    fmpq_init(c);
    if (!fmpq_poly_set_str_malachite(p, receiver)
        || !fmpq_poly_set_str_malachite(expected, expected_str) || fmpq_set_str(c, shift, 10) != 0)
    {
        flint_printf("error in fmpq_poly_taylor_shift test, line %d: unreadable input\n",
                     line_number);
        result = 1;
    }
    else
    {
        fmpq_canonicalise(c);
        fmpq_poly_set_coeff_fmpq(inner, 0, c);
        fmpq_poly_set_coeff_ui(inner, 1, 1);
        fmpq_poly_compose(r, p, inner);
        if (!fmpq_poly_equal(r, expected))
        {
            flint_printf("error in fmpq_poly_taylor_shift test, line %d. FLINT: ", line_number);
            fmpq_poly_print(r);
            flint_printf("\n");
            result = 1;
        }
    }
    fmpq_poly_clear(p);
    fmpq_poly_clear(expected);
    fmpq_poly_clear(inner);
    fmpq_poly_clear(r);
    fmpq_clear(c);
    return result;
}

int
run_fmpq_poly_taylor_shift(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("fmpq_poly_taylor_shift", checked);
}

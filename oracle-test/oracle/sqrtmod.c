/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `x.mod_sqrt(m) = Some(r)|None` lines against fmpz_sqrtmod and n_sqrtmod, and provides
    the sqrtmod memory-stress diagnostic.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/ulong_extras.h>

#include "oracle.h"

static int
check_fmpz_sqrtmod_line(char * line, int line_number)
{
    char * pieces[2];
    char * rest;
    if (!split_method_call(line, ".mod_sqrt(", pieces, 2, &rest))
    {
        return 0;
    }
    int result = 0;
    fmpz_t a, p, b, expected;
    fmpz_init(a);
    fmpz_init(p);
    fmpz_init(b);
    fmpz_init(expected);
    fmpz_set_str(a, pieces[0], 10);
    fmpz_set_str(p, pieces[1], 10);
    int has_expected = parse_option_fmpz(rest, expected);
    /* Documented divergence: even moduli in (50, 600) hit n_jacobi_unsigned with an even
       modulus, whose behavior is undefined; Malachite runs the exhaustive search instead. */
    int skip = fmpz_is_even(p) && fmpz_cmp_ui(p, 50) > 0 && fmpz_cmp_ui(p, 600) < 0
        && fmpz_cmp_ui(a, 1) > 0;
    if (!skip)
    {
        int ok = fmpz_sqrtmod(b, a, p);
        if (ok != has_expected || (ok && !fmpz_equal(b, expected)))
        {
            flint_printf("error in fmpz_sqrtmod test, line %d. FLINT: ok=%d b=", line_number, ok);
            fmpz_print(b);
            flint_printf("\n");
            result = 1;
        }
    }
    fmpz_clear(a);
    fmpz_clear(p);
    fmpz_clear(b);
    fmpz_clear(expected);
    return result;
}

int
run_fmpz_sqrtmod(const char * arg)
{
    return for_each_line(arg, check_fmpz_sqrtmod_line);
}

static int
check_n_sqrtmod_line(char * line, int line_number)
{
    char * pieces[2];
    char * rest;
    if (!split_method_call(line, ".mod_sqrt(", pieces, 2, &rest))
    {
        return 0;
    }
    char * end;
    ulong a = strtoul(pieces[0], &end, 10);
    ulong p = strtoul(pieces[1], &end, 10);
    ulong expected = 0;
    int has_expected = parse_option_ulong(rest, &expected);
    /* Documented divergences: even moduli in (50, 600) hit n_jacobi_unsigned with an even
       modulus, whose behavior is undefined; and for p = 2^64 - 1 and 2^64 - 3 the exponents
       (p + 1) / 4 and (p + 3) / 8 wrap, while Malachite computes them exactly, as fmpz_sqrtmod
       does. */
    if ((p % 2 == 0 && p > 50 && p < 600 && a > 1) || p == UWORD_MAX || p == UWORD_MAX - 2)
    {
        return 0;
    }
    ulong flint_output = n_sqrtmod(a, p);
    /* n_sqrtmod returns 0 both for failure and for the root of 0 */
    int flint_ok = flint_output != 0 || a == 0;
    if (flint_ok != has_expected || (has_expected && flint_output != expected))
    {
        flint_printf(
            "error in n_sqrtmod test, line %d, on input (%wu, %wu). FLINT: %wu\n",
            line_number, a, p, flint_output);
        return 1;
    }
    return 0;
}

int
run_n_sqrtmod(const char * arg)
{
    return for_each_line(arg, check_n_sqrtmod_line);
}

/* A memory-stress diagnostic, not a diff: repeated fmpz_sqrtmod calls with promotions. This
   caught heap corruption in a FLINT 3.3.0-dev snapshot's fmpz allocator. */
int
run_sqrtmod_stress(const char * arg)
{
    fmpz_t a, p, b;
    fmpz_init(a);
    fmpz_init(p);
    fmpz_init(b);
    fmpz_set_str(p, "26959946667150639794667015087019630673557916260026308143510066298881", 10);
    for (int i = 0; i < atoi(arg); i++)
    {
        fmpz_set_ui(a, 15241578750190521UL + i);
        fmpz_sqrtmod(b, a, p);
    }
    fmpz_clear(a);
    fmpz_clear(p);
    fmpz_clear(b);
    flint_printf("stress done\n");
    return 0;
}

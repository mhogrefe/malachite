/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `(P).mod_power_of_2_taylor_shift(c, k) = R`, `(&(P)).mod_power_of_2_taylor_shift(c, k) =
    R`, and `p := P; p.mod_power_of_2_taylor_shift_assign(c, k); p = R` lines, where `P` and `R`
    are natural polynomials and `c` a natural, all reduced modulo 2^k, against
    fmpz_mod_poly_compose with modulus 2^k and inner polynomial x + c, and, when 2^k fits in an
    nmod_t, against nmod_poly_taylor_shift_horner as well. nmod_poly_taylor_shift itself is not
    used: for long polynomials it divides by factorials, which assumes a prime modulus. Any other
    nonempty line is an error, as is an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_poly.h>
#include <flint/fmpz_poly.h>
#include <flint/nmod_poly.h>

#include "oracle.h"

static long checked;
static long nmod_lines;

static int
check_line(char * line, int line_number)
{
    char * receiver;
    char * args;
    char * expected_str;
    if (!split_polynomial_scalar_line(line, "mod_power_of_2_taylor_shift",
                                      "mod_power_of_2_taylor_shift_assign", NULL, &receiver, &args,
                                      &expected_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in mod_power_of_2_taylor_shift test, line %d: unrecognized line\n",
                     line_number);
        return 1;
    }
    checked++;
    char * pow_str = strstr(args, ", ");
    if (pow_str == NULL)
    {
        flint_printf("error in mod_power_of_2_taylor_shift test, line %d: unreadable arguments\n",
                     line_number);
        return 1;
    }
    *pow_str = '\0';
    pow_str += 2;
    int result = 0;
    fmpz_poly_t p, expected, r;
    fmpz_t c, m;
    fmpz_poly_init(p);
    fmpz_poly_init(expected);
    fmpz_poly_init(r);
    fmpz_init(c);
    fmpz_init(m);
    ulong pow = strtoul(pow_str, NULL, 10);
    if (!fmpz_poly_set_str_malachite(p, receiver)
        || !fmpz_poly_set_str_malachite(expected, expected_str) || fmpz_set_str(c, args, 10) != 0)
    {
        flint_printf("error in mod_power_of_2_taylor_shift test, line %d: unreadable input\n",
                     line_number);
        result = 1;
        goto cleanup;
    }
    fmpz_one(m);
    fmpz_mul_2exp(m, m, pow);
    {
        /* fmpz_mod_poly_compose with x + c, modulo 2^k. */
        fmpz_mod_ctx_t ctx;
        fmpz_mod_ctx_init(ctx, m);
        fmpz_mod_poly_t a, inner, b;
        fmpz_mod_poly_init(a, ctx);
        fmpz_mod_poly_init(inner, ctx);
        fmpz_mod_poly_init(b, ctx);
        fmpz_mod_poly_set_fmpz_poly(a, p, ctx);
        fmpz_mod_poly_set_coeff_fmpz(inner, 0, c, ctx);
        fmpz_mod_poly_set_coeff_ui(inner, 1, 1, ctx);
        fmpz_mod_poly_compose(b, a, inner, ctx);
        fmpz_mod_poly_get_fmpz_poly(r, b, ctx);
        if (!fmpz_poly_equal(r, expected))
        {
            flint_printf("error in mod_power_of_2_taylor_shift test, line %d, by "
                         "fmpz_mod_poly_compose. FLINT: ",
                         line_number);
            fmpz_poly_print(r);
            flint_printf("\n");
            result = 1;
        }
        fmpz_mod_poly_clear(a, ctx);
        fmpz_mod_poly_clear(inner, ctx);
        fmpz_mod_poly_clear(b, ctx);
        fmpz_mod_ctx_clear(ctx);
    }
    if (result == 0 && pow >= 1 && pow < FLINT_BITS)
    {
        nmod_lines++;
        nmod_poly_t a;
        nmod_poly_init(a, UWORD(1) << pow);
        fmpz_poly_get_nmod_poly(a, p);
        nmod_poly_taylor_shift_horner(a, a, fmpz_get_ui(c));
        fmpz_poly_set_nmod_poly_unsigned(r, a);
        if (!fmpz_poly_equal(r, expected))
        {
            flint_printf("error in mod_power_of_2_taylor_shift test, line %d, by "
                         "nmod_poly_taylor_shift_horner. FLINT: ",
                         line_number);
            fmpz_poly_print(r);
            flint_printf("\n");
            result = 1;
        }
        nmod_poly_clear(a);
    }
cleanup:
    fmpz_poly_clear(p);
    fmpz_poly_clear(expected);
    fmpz_poly_clear(r);
    fmpz_clear(c);
    fmpz_clear(m);
    return result;
}

int
run_mod_power_of_2_taylor_shift(const char * arg)
{
    checked = 0;
    nmod_lines = 0;
    int result = for_each_line(arg, check_line);
    if (result == 0)
    {
        flint_printf("mod_power_of_2_taylor_shift: %ld lines by fmpz_mod_poly_compose, %ld of them "
                     "also by nmod_poly_taylor_shift_horner\n",
                     checked, nmod_lines);
    }
    return result != 0 ? result : require_some_lines("mod_power_of_2_taylor_shift", checked);
}

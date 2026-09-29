/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the IntegerPolynomial bit_pack demos, `(P).bit_pack(b) = n` and
    `(&(P)).bit_pack(b) = n`, against fmpz_poly_bit_pack. Malachite's bit_pack always returns
    p(2^b), which is what fmpz_poly_bit_pack computes when b is positive and every coefficient's
    absolute value is less than 2^b; FLINT gives 0 when b is 0 and truncates wider coefficients.
    Those inputs are not skipped: each is diffed against fmpz_poly_evaluate_fmpz at 2^b instead,
    and the counts of both kinds are reported. The mode treats any other nonempty line as an error,
    and an input with no lines at all.
*/

#include <stdlib.h>

#include <flint/fmpz_poly.h>

#include "oracle.h"

static long checked;
static long packed;
static long evaluated;

/* Whether FLINT's packing is exact for `p` with fields of `bits` bits. */
static int
fields_fit(const fmpz_poly_t p, flint_bitcnt_t bits)
{
    if (bits == 0)
    {
        return 0;
    }
    for (slong i = 0; i < fmpz_poly_length(p); i++)
    {
        if (fmpz_bits(p->coeffs + i) > bits)
        {
            return 0;
        }
    }
    return 1;
}

static int
check_line(char * line, int line_number)
{
    char * p_str;
    char * bits_str;
    char * n_str;
    if (!split_polynomial_scalar_line(line, "bit_pack", NULL, NULL, &p_str, &bits_str, &n_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in fmpz_poly_bit_pack test, line %d: unrecognized line\n",
                     line_number);
        return 1;
    }
    checked++;
    int result = 0;
    char * end;
    unsigned long long bits = strtoull(bits_str, &end, 10);
    fmpz_poly_t p;
    fmpz_t expected, n, x;
    fmpz_poly_init(p);
    fmpz_init(expected);
    fmpz_init(n);
    fmpz_init(x);
    if (*end != '\0' || bits > WORD_MAX || !fmpz_poly_set_str_malachite(p, p_str)
        || fmpz_set_str(expected, n_str, 10) != 0)
    {
        flint_printf("error in fmpz_poly_bit_pack test, line %d: unreadable input\n",
                     line_number);
        result = 1;
    }
    else
    {
        if (fields_fit(p, (flint_bitcnt_t) bits))
        {
            packed++;
            fmpz_poly_bit_pack(n, p, (flint_bitcnt_t) bits);
        }
        else
        {
            evaluated++;
            fmpz_one_2exp(x, (ulong) bits);
            fmpz_poly_evaluate_fmpz(n, p, x);
        }
        if (!fmpz_equal(n, expected))
        {
            flint_printf("error in fmpz_poly_bit_pack test, line %d. FLINT: ", line_number);
            fmpz_print(n);
            flint_printf("\n");
            result = 1;
        }
    }
    fmpz_poly_clear(p);
    fmpz_clear(expected);
    fmpz_clear(n);
    fmpz_clear(x);
    return result;
}

int
run_fmpz_poly_bit_pack(const char * arg)
{
    checked = 0;
    packed = 0;
    evaluated = 0;
    int result = for_each_line(arg, check_line);
    if (result == 0)
    {
        flint_printf("fmpz_poly_bit_pack: %ld lines packed, %ld evaluated at 2^b\n", packed,
                     evaluated);
    }
    return result != 0 ? result : require_some_lines("fmpz_poly_bit_pack", checked);
}

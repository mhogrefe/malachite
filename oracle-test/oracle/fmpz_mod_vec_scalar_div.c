/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `V.mod_div(c, m) = R` and `(&V).mod_div(c, m) = R` lines, where `V` is a vector of
    naturals and `c` a natural, all reduced modulo the positive `m` (each optionally written with
    a leading `&`), and `R` is `Some(W)` or `None`, against _fmpz_mod_vec_scalar_div_fmpz_mod. Any
    other nonempty line is an error, as is an input with no lines at all.

    _fmpz_mod_vec_scalar_div_fmpz_mod needs `c` to be invertible modulo `m`: it inverts `c` once
    and throws otherwise. Malachite instead divides each element as Natural::mod_div does, which
    matches fmpz_mod_divides. So that every line is checked rather than skipped:
    - modulo 1, every element is 0, which is its own quotient, so `W` must equal `V`;
    - when gcd(c, m) = 1, the line goes to _fmpz_mod_vec_scalar_div_fmpz_mod;
    - otherwise each element goes to fmpz_mod_divides, and `R` must be `None` exactly when some
      element has no quotient.
    `V.mod_power_of_2_div(c, k) = R` lines, by value or by reference, are checked the same way
    with `m` = 2^k, except that for an even `c` Malachite returns the smallest quotient, which
    fmpz_mod_divides need not: there the oracle uses fmpz_mod_divides only for whether each
    quotient exists, and checks each returned `w` directly, that `wc` is congruent to the element
    and that `w` < 2^(k - s), where `s` is the number of trailing zeros of `c`; a quotient in that
    range is unique, so it is the smallest.
    The counts of lines checked each way are printed, so a demo that never reaches a path shows.
*/

#include <stdlib.h>
#include <string.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_vec.h>
#include <flint/fmpz_vec.h>

#include "oracle.h"

static long checked;
static long modulus_one_lines;
static long vector_lines;
static long elementwise_lines;
static long none_lines;
static long power_of_2_lines;
static long smallest_lines;

static int
check_line(char * line, int line_number)
{
    char * receiver;
    char * args;
    char * expected_str;
    int power_of_2 = 0;
    if (split_vector_scalar_line(line, "mod_power_of_2_div", NULL, &receiver, &args,
                                 &expected_str))
    {
        power_of_2 = 1;
    }
    else if (!split_vector_scalar_line(line, "mod_div", NULL, &receiver, &args, &expected_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in _fmpz_mod_vec_scalar_div_fmpz_mod test, line %d: unrecognized "
                     "line\n",
                     line_number);
        return 1;
    }
    checked++;
    char * divisor = args;
    char * modulus = strstr(args, ", ");
    if (modulus == NULL)
    {
        flint_printf("error in _fmpz_mod_vec_scalar_div_fmpz_mod test, line %d: unreadable "
                     "arguments\n",
                     line_number);
        return 1;
    }
    *modulus = '\0';
    modulus += 2;
    if (divisor[0] == '&')
    {
        divisor++;
    }
    if (modulus[0] == '&')
    {
        modulus++;
    }
    int expect_some;
    char * w_str = NULL;
    size_t n_chars = strlen(expected_str);
    if (strcmp(expected_str, "None") == 0)
    {
        expect_some = 0;
    }
    else if (strncmp(expected_str, "Some(", 5) == 0 && n_chars > 6
             && expected_str[n_chars - 1] == ')')
    {
        expect_some = 1;
        expected_str[n_chars - 1] = '\0';
        w_str = expected_str + 5;
    }
    else
    {
        flint_printf("error in _fmpz_mod_vec_scalar_div_fmpz_mod test, line %d: unreadable "
                     "result\n",
                     line_number);
        return 1;
    }
    int result = 0;
    slong len, w_len = 0;
    fmpz * v = fmpz_vec_set_str_malachite(&len, receiver);
    fmpz * w = expect_some ? fmpz_vec_set_str_malachite(&w_len, w_str) : NULL;
    fmpz * r = NULL;
    fmpz_t c, m, g;
    fmpz_init(c);
    fmpz_init(m);
    fmpz_init(g);
    ulong pow = 0;
    int unreadable_modulus;
    if (power_of_2)
    {
        char * end;
        pow = strtoul(modulus, &end, 10);
        unreadable_modulus = end == modulus || *end != '\0';
        fmpz_one(m);
        fmpz_mul_2exp(m, m, pow);
        power_of_2_lines++;
    }
    else
    {
        unreadable_modulus = fmpz_set_str(m, modulus, 10) != 0;
    }
    if (v == NULL || (expect_some && w == NULL) || fmpz_set_str(c, divisor, 10) != 0
        || unreadable_modulus || fmpz_sgn(m) <= 0 || fmpz_sgn(c) < 0 || fmpz_cmp(c, m) >= 0)
    {
        flint_printf("error in _fmpz_mod_vec_scalar_div_fmpz_mod test, line %d: unreadable or "
                     "unreduced input\n",
                     line_number);
        result = 1;
        goto cleanup;
    }
    for (slong i = 0; i < len; i++)
    {
        if (fmpz_sgn(v + i) < 0 || fmpz_cmp(v + i, m) >= 0)
        {
            flint_printf("error in _fmpz_mod_vec_scalar_div_fmpz_mod test, line %d: unreduced "
                         "element\n",
                         line_number);
            result = 1;
            goto cleanup;
        }
    }
    r = _fmpz_vec_init(len);
    int has_quotients = 1;
    if (fmpz_is_one(m))
    {
        modulus_one_lines++;
        _fmpz_vec_set(r, v, len);
    }
    else
    {
        fmpz_mod_ctx_t ctx;
        fmpz_mod_ctx_init(ctx, m);
        fmpz_gcd(g, c, m);
        if (fmpz_is_one(g))
        {
            vector_lines++;
            _fmpz_mod_vec_scalar_div_fmpz_mod(r, v, len, c, ctx);
        }
        else
        {
            elementwise_lines++;
            for (slong i = 0; i < len && has_quotients; i++)
            {
                has_quotients = fmpz_mod_divides(r + i, v + i, c, ctx);
            }
            if (power_of_2 && has_quotients && expect_some && w_len == len)
            {
                /* Check Malachite's smallest quotients directly, and use them as FLINT's result
                   when they pass. */
                smallest_lines++;
                fmpz_t bound, t;
                fmpz_init(bound);
                fmpz_init(t);
                /* c is even and nonzero here, or c = 0 with every element 0. */
                ulong s = fmpz_is_zero(c) ? pow : fmpz_val2(c);
                fmpz_one(bound);
                fmpz_mul_2exp(bound, bound, pow - s);
                int ok = 1;
                for (slong i = 0; i < len && ok; i++)
                {
                    fmpz_mul(t, w + i, c);
                    fmpz_sub(t, t, v + i);
                    fmpz_mod(t, t, m);
                    ok = fmpz_sgn(w + i) >= 0 && fmpz_cmp(w + i, bound) < 0 && fmpz_is_zero(t);
                }
                if (ok)
                {
                    _fmpz_vec_set(r, w, len);
                }
                fmpz_clear(bound);
                fmpz_clear(t);
            }
        }
        fmpz_mod_ctx_clear(ctx);
    }
    if (has_quotients != expect_some)
    {
        flint_printf("error in _fmpz_mod_vec_scalar_div_fmpz_mod test, line %d: Malachite says "
                     "%s, but FLINT says %s\n",
                     line_number, expect_some ? "Some" : "None",
                     has_quotients ? "Some" : "None");
        result = 1;
    }
    else if (!expect_some)
    {
        none_lines++;
    }
    else if (w_len != len || !_fmpz_vec_equal(r, w, len))
    {
        flint_printf("error in _fmpz_mod_vec_scalar_div_fmpz_mod test, line %d. FLINT: ",
                     line_number);
        fmpz_vec_print_malachite(r, len);
        flint_printf("\n");
        result = 1;
    }
cleanup:
    if (v != NULL)
    {
        _fmpz_vec_clear(v, len);
    }
    if (w != NULL)
    {
        _fmpz_vec_clear(w, w_len);
    }
    if (r != NULL)
    {
        _fmpz_vec_clear(r, len);
    }
    fmpz_clear(c);
    fmpz_clear(m);
    fmpz_clear(g);
    return result;
}

int
run__fmpz_mod_vec_scalar_div_fmpz_mod(const char * arg)
{
    checked = 0;
    modulus_one_lines = 0;
    vector_lines = 0;
    elementwise_lines = 0;
    none_lines = 0;
    power_of_2_lines = 0;
    smallest_lines = 0;
    int result = for_each_line(arg, check_line);
    if (result == 0)
    {
        flint_printf("_fmpz_mod_vec_scalar_div_fmpz_mod: %ld lines by "
                     "_fmpz_mod_vec_scalar_div_fmpz_mod, %ld by fmpz_mod_divides, %ld modulo 1, "
                     "%ld of them None; %ld modulo a power of 2, %ld of them checked for the "
                     "smallest quotient\n",
                     vector_lines, elementwise_lines, modulus_one_lines, none_lines,
                     power_of_2_lines, smallest_lines);
    }
    return result != 0 ? result
                       : require_some_lines("_fmpz_mod_vec_scalar_div_fmpz_mod", checked);
}

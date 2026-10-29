/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the UnsignedPolynomial modular power series inversion demos against
    fmpz_mod_poly_inv_series and, when the modulus is at least 2 and fits in a word, against
    nmod_poly_inv_series as well. Mode mod_inverse_series reads
    `(P).mod_inverse_truncated(n, m) = R`, and mode mod_power_of_2_inverse_series reads
    `(P).mod_power_of_2_inverse_truncated(n, k) = R`, with the modulus 2^k; in both, `(P)` may be
    written `(&(P))`. R is a polynomial or `None`. FLINT aborts when the constant coefficient is not
    invertible, so a `None` is checked against fmpz_gcd instead, and the inverse modulo x^0 or
    modulo 1 must be 0. Any other nonempty line is an error, as is an input with no lines at all.
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
static long none_lines;
static int power_of_2;
static const char * mode_name;

/* Cuts `line` at `needle`, returning the text after it, or NULL if the needle is absent. */
static char *
cut(char * line, const char * needle)
{
    char * at = strstr(line, needle);
    if (at == NULL)
    {
        return NULL;
    }
    *at = '\0';
    return at + strlen(needle);
}

/* Splits a line into the polynomial, the length, the modulus or exponent, and the result, in
   place. */
static int
split_line(char * line, char ** p, char ** n, char ** m, char ** r)
{
    line[strcspn(line, "\r\n")] = '\0';
    char * args = cut(line, power_of_2 ? ").mod_power_of_2_inverse_truncated("
                                       : ").mod_inverse_truncated(");
    if (args == NULL || line[0] != '(')
    {
        return 0;
    }
    *p = line + 1;
    if ((*p)[0] == '&')
    {
        size_t len = strlen(*p);
        if (len < 3 || (*p)[1] != '(' || (*p)[len - 1] != ')')
        {
            return 0;
        }
        (*p)[len - 1] = '\0';
        *p += 2;
    }
    *r = cut(args, ") = ");
    if (*r == NULL)
    {
        return 0;
    }
    *n = args;
    *m = cut(args, ", ");
    return *m != NULL;
}

static int
check_line(char * line, int line_number)
{
    char * p_str;
    char * n_str;
    char * m_str;
    char * r_str;
    if (!split_line(line, &p_str, &n_str, &m_str, &r_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in %s test, line %d: unrecognized line\n", mode_name, line_number);
        return 1;
    }
    checked++;
    int result = 0;
    int is_none = strcmp(r_str, "None") == 0;
    slong n = strtol(n_str, NULL, 10);
    fmpz_poly_t p, expected, r;
    fmpz_t m, g;
    fmpz_poly_init(p);
    fmpz_poly_init(expected);
    fmpz_poly_init(r);
    fmpz_init(m);
    fmpz_init(g);
    if (!fmpz_poly_set_str_malachite(p, p_str)
        || (!is_none && !fmpz_poly_set_str_malachite(expected, r_str)))
    {
        flint_printf("error in %s test, line %d: unreadable input\n", mode_name, line_number);
        result = 1;
        goto cleanup;
    }
    if (power_of_2)
    {
        fmpz_one(m);
        fmpz_mul_2exp(m, m, strtoul(m_str, NULL, 10));
    }
    else if (fmpz_set_str(m, m_str, 10) != 0 || fmpz_sgn(m) <= 0)
    {
        flint_printf("error in %s test, line %d: unreadable input\n", mode_name, line_number);
        result = 1;
        goto cleanup;
    }
    if (n == 0 || fmpz_is_one(m))
    {
        if (is_none || !fmpz_poly_is_zero(expected))
        {
            flint_printf("error in %s test, line %d: the inverse modulo x^0 or 1 must be 0\n",
                         mode_name, line_number);
            result = 1;
        }
        goto cleanup;
    }
    /* The zero polynomial has no inverse: its constant coefficient's gcd with m is m. */
    if (fmpz_poly_is_zero(p))
    {
        fmpz_set(g, m);
    }
    else
    {
        fmpz_gcd(g, p->coeffs, m);
    }
    if (is_none)
    {
        none_lines++;
        if (fmpz_is_one(g))
        {
            flint_printf("error in %s test, line %d: None, but the constant coefficient is "
                         "invertible\n",
                         mode_name, line_number);
            result = 1;
        }
        goto cleanup;
    }
    if (!fmpz_is_one(g))
    {
        flint_printf("error in %s test, line %d: an inverse, but the constant coefficient is not "
                     "invertible\n",
                     mode_name, line_number);
        result = 1;
        goto cleanup;
    }
    {
        fmpz_mod_ctx_t ctx;
        fmpz_mod_ctx_init(ctx, m);
        fmpz_mod_poly_t a, c;
        fmpz_mod_poly_init(a, ctx);
        fmpz_mod_poly_init(c, ctx);
        fmpz_mod_poly_set_fmpz_poly(a, p, ctx);
        fmpz_mod_poly_inv_series(c, a, n, ctx);
        fmpz_mod_poly_get_fmpz_poly(r, c, ctx);
        if (!fmpz_poly_equal(r, expected))
        {
            flint_printf("error in %s test, line %d, by fmpz_mod_poly_inv_series. FLINT: ",
                         mode_name, line_number);
            fmpz_poly_print(r);
            flint_printf("\n");
            result = 1;
        }
        fmpz_mod_poly_clear(a, ctx);
        fmpz_mod_poly_clear(c, ctx);
        fmpz_mod_ctx_clear(ctx);
    }
    if (result == 0 && fmpz_abs_fits_ui(m))
    {
        nmod_lines++;
        nmod_poly_t a, c;
        nmod_poly_init(a, fmpz_get_ui(m));
        nmod_poly_init(c, fmpz_get_ui(m));
        fmpz_poly_get_nmod_poly(a, p);
        nmod_poly_inv_series(c, a, n);
        fmpz_poly_set_nmod_poly_unsigned(r, c);
        if (!fmpz_poly_equal(r, expected))
        {
            flint_printf("error in %s test, line %d, by nmod_poly_inv_series. FLINT: ", mode_name,
                         line_number);
            fmpz_poly_print(r);
            flint_printf("\n");
            result = 1;
        }
        nmod_poly_clear(a);
        nmod_poly_clear(c);
    }
cleanup:
    fmpz_poly_clear(p);
    fmpz_poly_clear(expected);
    fmpz_poly_clear(r);
    fmpz_clear(m);
    fmpz_clear(g);
    return result;
}

static int
run(const char * arg, int pow2, const char * name)
{
    checked = 0;
    nmod_lines = 0;
    none_lines = 0;
    power_of_2 = pow2;
    mode_name = name;
    int result = for_each_line(arg, check_line);
    if (result == 0)
    {
        flint_printf("%s: %ld lines, %ld of them None; the rest by fmpz_mod_poly_inv_series, "
                     "%ld of them also by nmod_poly_inv_series\n",
                     name, checked, none_lines, nmod_lines);
    }
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_mod_inverse_series(const char * arg)
{
    return run(arg, 0, "mod_inverse_series");
}

int
run_mod_power_of_2_inverse_series(const char * arg)
{
    return run(arg, 1, "mod_power_of_2_inverse_series");
}

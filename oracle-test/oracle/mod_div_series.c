/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the UnsignedPolynomial and NaturalPolynomial modular power series
    division demos against fmpz_mod_poly_div_series and, when the modulus is at least 2 and fits in
    a word, against nmod_poly_div_series as well. Mode mod_div_series reads the methods
    `mod_div_truncated`, `checked_mod_div_truncated`, and `mod_div_truncated_assign`, with the
    modulus m, and mode mod_power_of_2_div_series the `mod_power_of_2_` versions, with the modulus
    2^k. The shapes recognized are `(A).METHOD(B, n, m) = R` and
    `p := A; p.METHOD(B, n, m); p = R`, where any of `A`, `B`, and `m` may be written `&(...)`, and R
    is a polynomial or `None`. FLINT aborts when the divisor's constant coefficient is not
    invertible, so a `None` is checked against fmpz_gcd instead, and the quotient modulo x^0 or
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

/* Strips `&(` and the matching `)` from an argument written `&(X)`, in place. */
static char *
strip_ref(char * argument)
{
    size_t len = strlen(argument);
    if (len >= 3 && argument[0] == '&' && argument[1] == '(' && argument[len - 1] == ')')
    {
        argument[len - 1] = '\0';
        return argument + 2;
    }
    return argument;
}

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

/* The method names of the mode, without the suffix `_assign`. */
static int
known_method(const char * name)
{
    if (power_of_2)
    {
        return strcmp(name, "mod_power_of_2_div_truncated") == 0
               || strcmp(name, "checked_mod_power_of_2_div_truncated") == 0
               || strcmp(name, "mod_power_of_2_div_truncated_assign") == 0;
    }
    return strcmp(name, "mod_div_truncated") == 0 || strcmp(name, "checked_mod_div_truncated") == 0
           || strcmp(name, "mod_div_truncated_assign") == 0;
}

/* Splits a line into the dividend, the divisor, the length, the modulus or exponent, and the
   result, in place. Polynomials are displayed without spaces, commas, or parentheses, so the
   separators are unambiguous. */
static int
split_line(char * line, char ** a, char ** b, char ** n, char ** m, char ** r)
{
    line[strcspn(line, "\r\n")] = '\0';
    char * method;
    char * args;
    if (strncmp(line, "p := ", 5) == 0)
    {
        *a = line + 5;
        method = cut(*a, "; p.");
        if (method == NULL)
        {
            return 0;
        }
        args = cut(method, "(");
        if (args == NULL)
        {
            return 0;
        }
        *r = cut(args, "); p = ");
    }
    else
    {
        if (line[0] != '(')
        {
            return 0;
        }
        method = cut(line, ").");
        if (method == NULL)
        {
            return 0;
        }
        *a = strip_ref(line + 1);
        args = cut(method, "(");
        if (args == NULL)
        {
            return 0;
        }
        *r = cut(args, ") = ");
    }
    if (*r == NULL || !known_method(method))
    {
        return 0;
    }
    *b = args;
    *n = cut(*b, ", ");
    if (*n == NULL)
    {
        return 0;
    }
    *m = cut(*n, ", ");
    if (*m == NULL)
    {
        return 0;
    }
    *b = strip_ref(*b);
    *m = strip_ref(*m);
    return 1;
}

static int
check_line(char * line, int line_number)
{
    char * a_str;
    char * b_str;
    char * n_str;
    char * m_str;
    char * r_str;
    if (!split_line(line, &a_str, &b_str, &n_str, &m_str, &r_str))
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
    fmpz_poly_t a, b, expected, r;
    fmpz_t m, g;
    fmpz_poly_init(a);
    fmpz_poly_init(b);
    fmpz_poly_init(expected);
    fmpz_poly_init(r);
    fmpz_init(m);
    fmpz_init(g);
    if (!fmpz_poly_set_str_malachite(a, a_str) || !fmpz_poly_set_str_malachite(b, b_str)
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
            flint_printf("error in %s test, line %d: the quotient modulo x^0 or 1 must be 0\n",
                         mode_name, line_number);
            result = 1;
        }
        goto cleanup;
    }
    /* The zero divisor is not invertible: its constant coefficient's gcd with m is m. */
    if (fmpz_poly_is_zero(b))
    {
        fmpz_set(g, m);
    }
    else
    {
        fmpz_gcd(g, b->coeffs, m);
    }
    if (is_none)
    {
        none_lines++;
        if (fmpz_is_one(g))
        {
            flint_printf("error in %s test, line %d: None, but the divisor's constant coefficient "
                         "is invertible\n",
                         mode_name, line_number);
            result = 1;
        }
        goto cleanup;
    }
    if (!fmpz_is_one(g))
    {
        flint_printf("error in %s test, line %d: a quotient, but the divisor's constant "
                     "coefficient is not invertible\n",
                     mode_name, line_number);
        result = 1;
        goto cleanup;
    }
    {
        fmpz_mod_ctx_t ctx;
        fmpz_mod_ctx_init(ctx, m);
        fmpz_mod_poly_t x, y, z;
        fmpz_mod_poly_init(x, ctx);
        fmpz_mod_poly_init(y, ctx);
        fmpz_mod_poly_init(z, ctx);
        fmpz_mod_poly_set_fmpz_poly(x, a, ctx);
        fmpz_mod_poly_set_fmpz_poly(y, b, ctx);
        fmpz_mod_poly_div_series(z, x, y, n, ctx);
        fmpz_mod_poly_get_fmpz_poly(r, z, ctx);
        if (!fmpz_poly_equal(r, expected))
        {
            flint_printf("error in %s test, line %d, by fmpz_mod_poly_div_series. FLINT: ",
                         mode_name, line_number);
            fmpz_poly_print(r);
            flint_printf("\n");
            result = 1;
        }
        fmpz_mod_poly_clear(x, ctx);
        fmpz_mod_poly_clear(y, ctx);
        fmpz_mod_poly_clear(z, ctx);
        fmpz_mod_ctx_clear(ctx);
    }
    if (result == 0 && fmpz_abs_fits_ui(m))
    {
        nmod_lines++;
        nmod_poly_t x, y, z;
        nmod_poly_init(x, fmpz_get_ui(m));
        nmod_poly_init(y, fmpz_get_ui(m));
        nmod_poly_init(z, fmpz_get_ui(m));
        fmpz_poly_get_nmod_poly(x, a);
        fmpz_poly_get_nmod_poly(y, b);
        nmod_poly_div_series(z, x, y, n);
        fmpz_poly_set_nmod_poly_unsigned(r, z);
        if (!fmpz_poly_equal(r, expected))
        {
            flint_printf("error in %s test, line %d, by nmod_poly_div_series. FLINT: ", mode_name,
                         line_number);
            fmpz_poly_print(r);
            flint_printf("\n");
            result = 1;
        }
        nmod_poly_clear(x);
        nmod_poly_clear(y);
        nmod_poly_clear(z);
    }
cleanup:
    fmpz_poly_clear(a);
    fmpz_poly_clear(b);
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
        flint_printf("%s: %ld lines, %ld of them None; the rest by fmpz_mod_poly_div_series, %ld "
                     "of them also by nmod_poly_div_series\n",
                     name, checked, none_lines, nmod_lines);
    }
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_mod_div_series(const char * arg)
{
    return run(arg, 0, "mod_div_series");
}

int
run_mod_power_of_2_div_series(const char * arg)
{
    return run(arg, 1, "mod_power_of_2_div_series");
}

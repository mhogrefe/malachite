/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the UnsignedPolynomial and NaturalPolynomial demos of multiplication
    modulo a polynomial against fmpz_mod_poly_mulmod and, when the modulus is at least 2 and fits in
    a word, nmod_poly_mulmod; lines of the `_precomputed` methods are also checked against
    fmpz_mod_poly_mulmod_preinv and nmod_poly_mulmod_preinv. Mode mod_mul_mod_polynomial reads the
    methods `mod_mul_mod_polynomial`, `mod_mul_mod_polynomial_assign`, and their `_precomputed`
    versions, with the coefficient modulus m; mode mod_power_of_2_mul_mod_polynomial reads the
    `mod_power_of_2_` versions, with the modulus 2^k. The shapes recognized are
    `(A).METHOD(B, F, m) = R` and `a := A; a.METHOD(B, F, m); a = R`, where `A`, `B`, `F`, and `m`
    may be written `&(...)`, and the `_precomputed` methods take a last argument `&data`. Modulo 1,
    every result must be 0. Any other nonempty line is an error, as is an input with no lines at
    all.
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
static long preinv_lines;
static int power_of_2;
static int precomputed;
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
    if (len >= 1 && argument[0] == '&')
    {
        return argument + 1;
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

/* Whether `name` is one of the mode's methods; sets `precomputed` for the `_precomputed` ones. */
static int
method_ok(const char * name)
{
    const char * base = power_of_2 ? "mod_power_of_2_mul_mod_polynomial" : "mod_mul_mod_polynomial";
    size_t base_len = strlen(base);
    if (strncmp(name, base, base_len) != 0)
    {
        return 0;
    }
    name += base_len;
    precomputed = 0;
    if (strcmp(name, "") == 0 || strcmp(name, "_assign") == 0)
    {
        return 1;
    }
    if (strcmp(name, "_precomputed") == 0 || strcmp(name, "_precomputed_assign") == 0)
    {
        precomputed = 1;
        return 1;
    }
    return 0;
}

/* Splits a line into the factors, the polynomial modulus, the coefficient modulus or exponent, and
   the expected result, in place. Polynomials are displayed without spaces, commas, or parentheses,
   so the separators are unambiguous. Returns 0 if the line is not recognized. */
static int
split_line(char * line, char ** a, char ** b, char ** f, char ** m, char ** r)
{
    line[strcspn(line, "\r\n")] = '\0';
    char * method;
    char * args;
    if (strncmp(line, "a := ", 5) == 0)
    {
        *a = line + 5;
        method = cut(*a, "; a.");
        if (method == NULL)
            return 0;
        args = cut(method, "(");
        if (args == NULL || !method_ok(method))
            return 0;
        *r = cut(args, "); a = ");
        if (*r == NULL)
            return 0;
    }
    else
    {
        if (line[0] != '(')
            return 0;
        method = cut(line, ").");
        if (method == NULL)
            return 0;
        *a = strip_ref(line + 1);
        args = cut(method, "(");
        if (args == NULL || !method_ok(method))
            return 0;
        *r = cut(args, ") = ");
        if (*r == NULL)
            return 0;
    }
    *b = args;
    *f = cut(*b, ", ");
    if (*f == NULL)
        return 0;
    *m = cut(*f, ", ");
    if (*m == NULL)
        return 0;
    if (precomputed)
    {
        char * data = cut(*m, ", ");
        if (data == NULL || strcmp(data, "&data") != 0)
            return 0;
    }
    *b = strip_ref(*b);
    *f = strip_ref(*f);
    *m = strip_ref(*m);
    return 1;
}

/* Compares a FLINT result with the expected polynomial, printing an error that names `function` if
   they differ. */
static int
compare(const fmpz_poly_t actual, const fmpz_poly_t expected, const char * function,
        int line_number)
{
    if (fmpz_poly_equal(actual, expected))
    {
        return 0;
    }
    flint_printf("error in %s test, line %d, by %s. FLINT: ", mode_name, line_number, function);
    fmpz_poly_print(actual);
    flint_printf("\n");
    return 1;
}

static int
check_line(char * line, int line_number)
{
    char * a_str;
    char * b_str;
    char * f_str;
    char * m_str;
    char * r_str;
    if (!split_line(line, &a_str, &b_str, &f_str, &m_str, &r_str))
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
    fmpz_poly_t a, b, f, r_expected, r;
    fmpz_t m;
    fmpz_poly_init(a);
    fmpz_poly_init(b);
    fmpz_poly_init(f);
    fmpz_poly_init(r_expected);
    fmpz_poly_init(r);
    fmpz_init(m);
    if (!fmpz_poly_set_str_malachite(a, a_str) || !fmpz_poly_set_str_malachite(b, b_str)
        || !fmpz_poly_set_str_malachite(f, f_str)
        || !fmpz_poly_set_str_malachite(r_expected, r_str))
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
    if (fmpz_is_one(m))
    {
        if (!fmpz_poly_is_zero(r_expected))
        {
            flint_printf("error in %s test, line %d: every result modulo 1 must be 0\n",
                         mode_name, line_number);
            result = 1;
        }
        goto cleanup;
    }
    if (fmpz_poly_length(f) == 0 || fmpz_poly_length(a) >= fmpz_poly_length(f)
        || fmpz_poly_length(b) >= fmpz_poly_length(f))
    {
        flint_printf("error in %s test, line %d: a result for an invalid input\n", mode_name,
                     line_number);
        result = 1;
        goto cleanup;
    }
    {
        fmpz_mod_ctx_t ctx;
        fmpz_mod_ctx_init(ctx, m);
        fmpz_mod_poly_t x, y, z, w;
        fmpz_mod_poly_init(x, ctx);
        fmpz_mod_poly_init(y, ctx);
        fmpz_mod_poly_init(z, ctx);
        fmpz_mod_poly_init(w, ctx);
        fmpz_mod_poly_set_fmpz_poly(x, a, ctx);
        fmpz_mod_poly_set_fmpz_poly(y, b, ctx);
        fmpz_mod_poly_set_fmpz_poly(z, f, ctx);
        fmpz_mod_poly_mulmod(w, x, y, z, ctx);
        fmpz_mod_poly_get_fmpz_poly(r, w, ctx);
        result |= compare(r, r_expected, "fmpz_mod_poly_mulmod", line_number);
        if (result == 0 && precomputed)
        {
            fmpz_mod_poly_t z_inverse;
            fmpz_mod_poly_init(z_inverse, ctx);
            fmpz_mod_poly_reverse(z_inverse, z, fmpz_mod_poly_length(z, ctx), ctx);
            fmpz_mod_poly_inv_series(z_inverse, z_inverse, fmpz_mod_poly_length(z, ctx), ctx);
            fmpz_mod_poly_mulmod_preinv(w, x, y, z, z_inverse, ctx);
            fmpz_mod_poly_get_fmpz_poly(r, w, ctx);
            result |= compare(r, r_expected, "fmpz_mod_poly_mulmod_preinv", line_number);
            fmpz_mod_poly_clear(z_inverse, ctx);
            preinv_lines++;
        }
        fmpz_mod_poly_clear(x, ctx);
        fmpz_mod_poly_clear(y, ctx);
        fmpz_mod_poly_clear(z, ctx);
        fmpz_mod_poly_clear(w, ctx);
        fmpz_mod_ctx_clear(ctx);
    }
    if (result == 0 && fmpz_abs_fits_ui(m))
    {
        nmod_lines++;
        ulong n = fmpz_get_ui(m);
        nmod_poly_t x, y, z, w;
        nmod_poly_init(x, n);
        nmod_poly_init(y, n);
        nmod_poly_init(z, n);
        nmod_poly_init(w, n);
        fmpz_poly_get_nmod_poly(x, a);
        fmpz_poly_get_nmod_poly(y, b);
        fmpz_poly_get_nmod_poly(z, f);
        nmod_poly_mulmod(w, x, y, z);
        fmpz_poly_set_nmod_poly_unsigned(r, w);
        result |= compare(r, r_expected, "nmod_poly_mulmod", line_number);
        if (result == 0 && precomputed)
        {
            nmod_poly_t z_inverse;
            nmod_poly_init(z_inverse, n);
            nmod_poly_reverse(z_inverse, z, nmod_poly_length(z));
            nmod_poly_inv_series(z_inverse, z_inverse, nmod_poly_length(z));
            nmod_poly_mulmod_preinv(w, x, y, z, z_inverse);
            fmpz_poly_set_nmod_poly_unsigned(r, w);
            result |= compare(r, r_expected, "nmod_poly_mulmod_preinv", line_number);
            nmod_poly_clear(z_inverse);
        }
        nmod_poly_clear(x);
        nmod_poly_clear(y);
        nmod_poly_clear(z);
        nmod_poly_clear(w);
    }
cleanup:
    fmpz_poly_clear(a);
    fmpz_poly_clear(b);
    fmpz_poly_clear(f);
    fmpz_poly_clear(r_expected);
    fmpz_poly_clear(r);
    fmpz_clear(m);
    return result;
}

static int
run(const char * arg, int pow2, const char * name)
{
    checked = 0;
    nmod_lines = 0;
    preinv_lines = 0;
    power_of_2 = pow2;
    mode_name = name;
    int result = for_each_line(arg, check_line);
    if (result == 0)
    {
        flint_printf("%s: %ld lines; by fmpz_mod_poly, %ld of them also by a precomputed inverse, "
                     "and %ld also by nmod_poly\n",
                     name, checked, preinv_lines, nmod_lines);
    }
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_mod_mul_mod_polynomial(const char * arg)
{
    return run(arg, 0, "mod_mul_mod_polynomial");
}

int
run_mod_power_of_2_mul_mod_polynomial(const char * arg)
{
    return run(arg, 1, "mod_power_of_2_mul_mod_polynomial");
}

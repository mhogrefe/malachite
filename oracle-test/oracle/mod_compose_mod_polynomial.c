/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the UnsignedPolynomial and NaturalPolynomial demos of composition
    modulo a polynomial against fmpz_mod_poly_compose_mod and fmpz_mod_poly_compose_mod_horner,
    and, when the outer polynomial is shorter than the modulus, fmpz_mod_poly_compose_mod_brent_kung;
    when the modulus is at least 2 and fits in a word, the nmod_poly versions too. Lines of the
    `_precomputed` methods are also checked against the `compose_mod_brent_kung_preinv` functions,
    and lines of the `_precomputed_powers` methods against `precompute_matrix` followed by
    `compose_mod_brent_kung_precomp_preinv`, when the outer polynomial is shorter than the modulus.
    Mode mod_compose_mod_polynomial reads the methods `mod_compose_mod_polynomial`, its `_assign`
    version, and their `_precomputed` and `_precomputed_powers` versions, with the coefficient
    modulus m; mode mod_power_of_2_compose_mod_polynomial reads the `mod_power_of_2_` versions, with
    the modulus 2^k. The shapes recognized are `(F).METHOD(G, H, m) = R`,
    `f := F; f.METHOD(G, H, m); f = R`, and, for the powers methods, `(F).METHOD(H, m, &data) = R,
    where data is from g = G`, where the polynomials may be written `&(...)` and the precomputed
    methods take a last argument `&data`. Modulo 1, every result must be 0. Any other nonempty line
    is an error, as is an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>
#include <flint/fmpz_mat.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_poly.h>
#include <flint/fmpz_poly.h>
#include <flint/nmod_mat.h>
#include <flint/nmod_poly.h>
#include <flint/ulong_extras.h>

#include "oracle.h"

static long checked;
static long nmod_lines;
static long brent_kung_lines;
static int power_of_2;
/* 0: plain, 1: precomputed inverse, 2: precomputed powers */
static int kind;
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

/* Whether `name` is one of the mode's methods; sets `kind`. */
static int
method_ok(const char * name)
{
    const char * base =
        power_of_2 ? "mod_power_of_2_compose_mod_polynomial" : "mod_compose_mod_polynomial";
    size_t base_len = strlen(base);
    if (strncmp(name, base, base_len) != 0)
    {
        return 0;
    }
    name += base_len;
    if (strcmp(name, "") == 0 || strcmp(name, "_assign") == 0)
    {
        kind = 0;
        return 1;
    }
    if (strcmp(name, "_precomputed") == 0 || strcmp(name, "_precomputed_assign") == 0)
    {
        kind = 1;
        return 1;
    }
    if (strcmp(name, "_precomputed_powers") == 0
        || strcmp(name, "_precomputed_powers_assign") == 0)
    {
        kind = 2;
        return 1;
    }
    return 0;
}

/* Splits a line into the outer and inner polynomials, the polynomial modulus, the coefficient
   modulus or exponent, and the expected result, in place. Polynomials are displayed without spaces,
   commas, or parentheses, so the separators are unambiguous. Returns 0 if the line is not
   recognized. */
static int
split_line(char * line, char ** f, char ** g, char ** h, char ** m, char ** r)
{
    line[strcspn(line, "\r\n")] = '\0';
    char * method;
    char * args;
    if (strncmp(line, "f := ", 5) == 0)
    {
        *f = line + 5;
        method = cut(*f, "; f.");
        if (method == NULL)
            return 0;
        args = cut(method, "(");
        if (args == NULL || !method_ok(method) || kind == 2)
            return 0;
        *r = cut(args, "); f = ");
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
        *f = strip_ref(line + 1);
        args = cut(method, "(");
        if (args == NULL || !method_ok(method))
            return 0;
        *r = cut(args, ") = ");
        if (*r == NULL)
            return 0;
        if (kind == 2)
        {
            *g = cut(*r, ", where data is from g = ");
            if (*g == NULL)
                return 0;
        }
    }
    if (kind == 2)
    {
        *h = args;
    }
    else
    {
        *g = args;
        *h = cut(*g, ", ");
        if (*h == NULL)
            return 0;
        *g = strip_ref(*g);
    }
    *m = cut(*h, ", ");
    if (*m == NULL)
        return 0;
    if (kind != 0)
    {
        char * data = cut(*m, ", ");
        if (data == NULL || strcmp(data, "&data") != 0)
            return 0;
    }
    *h = strip_ref(*h);
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
check_fmpz_mod(const fmpz_poly_t a, const fmpz_poly_t b, const fmpz_poly_t c, const fmpz_t m,
               const fmpz_poly_t expected, int line_number)
{
    int result = 0;
    fmpz_poly_t r;
    fmpz_poly_init(r);
    fmpz_mod_ctx_t ctx;
    fmpz_mod_ctx_init(ctx, m);
    fmpz_mod_poly_t f, g, h, w, hinv;
    fmpz_mod_poly_init(f, ctx);
    fmpz_mod_poly_init(g, ctx);
    fmpz_mod_poly_init(h, ctx);
    fmpz_mod_poly_init(w, ctx);
    fmpz_mod_poly_init(hinv, ctx);
    fmpz_mod_poly_set_fmpz_poly(f, a, ctx);
    fmpz_mod_poly_set_fmpz_poly(g, b, ctx);
    fmpz_mod_poly_set_fmpz_poly(h, c, ctx);
    fmpz_mod_poly_compose_mod(w, f, g, h, ctx);
    fmpz_mod_poly_get_fmpz_poly(r, w, ctx);
    result |= compare(r, expected, "fmpz_mod_poly_compose_mod", line_number);
    fmpz_mod_poly_compose_mod_horner(w, f, g, h, ctx);
    fmpz_mod_poly_get_fmpz_poly(r, w, ctx);
    result |= compare(r, expected, "fmpz_mod_poly_compose_mod_horner", line_number);
    slong lenh = fmpz_mod_poly_length(h, ctx);
    if (fmpz_mod_poly_length(f, ctx) < lenh)
    {
        brent_kung_lines++;
        fmpz_mod_poly_compose_mod_brent_kung(w, f, g, h, ctx);
        fmpz_mod_poly_get_fmpz_poly(r, w, ctx);
        result |= compare(r, expected, "fmpz_mod_poly_compose_mod_brent_kung", line_number);
        fmpz_mod_poly_reverse(hinv, h, lenh, ctx);
        fmpz_mod_poly_inv_series(hinv, hinv, lenh, ctx);
        if (kind == 1)
        {
            fmpz_mod_poly_compose_mod_brent_kung_preinv(w, f, g, h, hinv, ctx);
            fmpz_mod_poly_get_fmpz_poly(r, w, ctx);
            result |= compare(r, expected, "fmpz_mod_poly_compose_mod_brent_kung_preinv",
                              line_number);
        }
        if (kind == 2 && lenh > 1)
        {
            fmpz_mat_t A;
            fmpz_mat_init(A, n_sqrt(lenh - 1) + 1, lenh - 1);
            fmpz_mod_poly_precompute_matrix(A, g, h, hinv, ctx);
            fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv(w, f, A, h, hinv, ctx);
            fmpz_mod_poly_get_fmpz_poly(r, w, ctx);
            result |= compare(r, expected, "fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv",
                              line_number);
            fmpz_mat_clear(A);
        }
    }
    fmpz_mod_poly_clear(f, ctx);
    fmpz_mod_poly_clear(g, ctx);
    fmpz_mod_poly_clear(h, ctx);
    fmpz_mod_poly_clear(w, ctx);
    fmpz_mod_poly_clear(hinv, ctx);
    fmpz_mod_ctx_clear(ctx);
    fmpz_poly_clear(r);
    return result;
}

static int
check_nmod(const fmpz_poly_t a, const fmpz_poly_t b, const fmpz_poly_t c, ulong n,
           const fmpz_poly_t expected, int line_number)
{
    int result = 0;
    fmpz_poly_t r;
    fmpz_poly_init(r);
    nmod_poly_t f, g, h, w, hinv;
    nmod_poly_init(f, n);
    nmod_poly_init(g, n);
    nmod_poly_init(h, n);
    nmod_poly_init(w, n);
    nmod_poly_init(hinv, n);
    fmpz_poly_get_nmod_poly(f, a);
    fmpz_poly_get_nmod_poly(g, b);
    fmpz_poly_get_nmod_poly(h, c);
    nmod_poly_compose_mod(w, f, g, h);
    fmpz_poly_set_nmod_poly_unsigned(r, w);
    result |= compare(r, expected, "nmod_poly_compose_mod", line_number);
    nmod_poly_compose_mod_horner(w, f, g, h);
    fmpz_poly_set_nmod_poly_unsigned(r, w);
    result |= compare(r, expected, "nmod_poly_compose_mod_horner", line_number);
    slong lenh = nmod_poly_length(h);
    if (nmod_poly_length(f) < lenh)
    {
        nmod_poly_compose_mod_brent_kung(w, f, g, h);
        fmpz_poly_set_nmod_poly_unsigned(r, w);
        result |= compare(r, expected, "nmod_poly_compose_mod_brent_kung", line_number);
        nmod_poly_reverse(hinv, h, lenh);
        nmod_poly_inv_series(hinv, hinv, lenh);
        if (kind == 1)
        {
            nmod_poly_compose_mod_brent_kung_preinv(w, f, g, h, hinv);
            fmpz_poly_set_nmod_poly_unsigned(r, w);
            result |= compare(r, expected, "nmod_poly_compose_mod_brent_kung_preinv",
                              line_number);
        }
        if (kind == 2 && lenh > 1)
        {
            nmod_mat_t A;
            nmod_mat_init(A, n_sqrt(lenh - 1) + 1, lenh - 1, n);
            nmod_poly_precompute_matrix(A, g, h, hinv);
            nmod_poly_compose_mod_brent_kung_precomp_preinv(w, f, A, h, hinv);
            fmpz_poly_set_nmod_poly_unsigned(r, w);
            result |= compare(r, expected, "nmod_poly_compose_mod_brent_kung_precomp_preinv",
                              line_number);
            nmod_mat_clear(A);
        }
    }
    nmod_poly_clear(f);
    nmod_poly_clear(g);
    nmod_poly_clear(h);
    nmod_poly_clear(w);
    nmod_poly_clear(hinv);
    fmpz_poly_clear(r);
    return result;
}

static int
check_line(char * line, int line_number)
{
    char * f_str;
    char * g_str;
    char * h_str;
    char * m_str;
    char * r_str;
    if (!split_line(line, &f_str, &g_str, &h_str, &m_str, &r_str))
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
    fmpz_poly_t f, g, h, r_expected;
    fmpz_t m;
    fmpz_poly_init(f);
    fmpz_poly_init(g);
    fmpz_poly_init(h);
    fmpz_poly_init(r_expected);
    fmpz_init(m);
    if (!fmpz_poly_set_str_malachite(f, f_str) || !fmpz_poly_set_str_malachite(g, g_str)
        || !fmpz_poly_set_str_malachite(h, h_str)
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
    if (fmpz_poly_length(h) == 0 || fmpz_poly_length(g) >= fmpz_poly_length(h))
    {
        flint_printf("error in %s test, line %d: a result for an invalid input\n", mode_name,
                     line_number);
        result = 1;
        goto cleanup;
    }
    result |= check_fmpz_mod(f, g, h, m, r_expected, line_number);
    if (result == 0 && fmpz_abs_fits_ui(m))
    {
        nmod_lines++;
        result |= check_nmod(f, g, h, fmpz_get_ui(m), r_expected, line_number);
    }
cleanup:
    fmpz_poly_clear(f);
    fmpz_poly_clear(g);
    fmpz_poly_clear(h);
    fmpz_poly_clear(r_expected);
    fmpz_clear(m);
    return result;
}

static int
run(const char * arg, int pow2, const char * name)
{
    checked = 0;
    nmod_lines = 0;
    brent_kung_lines = 0;
    power_of_2 = pow2;
    mode_name = name;
    int result = for_each_line(arg, check_line);
    if (result == 0)
    {
        flint_printf("%s: %ld lines; by fmpz_mod_poly, %ld of them also by Brent-Kung, and %ld also "
                     "by nmod_poly\n",
                     name, checked, brent_kung_lines, nmod_lines);
    }
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_mod_compose_mod_polynomial(const char * arg)
{
    return run(arg, 0, "mod_compose_mod_polynomial");
}

int
run_mod_power_of_2_compose_mod_polynomial(const char * arg)
{
    return run(arg, 1, "mod_power_of_2_compose_mod_polynomial");
}

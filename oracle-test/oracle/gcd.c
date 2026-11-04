/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the polynomial GCD, extended GCD, and LCM demos against FLINT.

    Mode mod_gcd reads the UnsignedPolynomial and NaturalPolynomial methods `mod_gcd`,
    `mod_gcd_assign`, `checked_mod_gcd`, `mod_lcm`, `mod_lcm_assign`, `checked_mod_lcm`,
    `mod_extended_gcd`, and `checked_mod_extended_gcd`, with the modulus m; mode
    mod_power_of_2_gcd reads `checked_mod_power_of_2_gcd`, `checked_mod_power_of_2_lcm`, and
    `checked_mod_power_of_2_extended_gcd`, with the modulus 2^k. The shapes recognized are
    `(A).METHOD(B, m) = R` and `p := A; p.METHOD(B, m); p = R`, where `A`, `B`, and `m` may be
    written `&(...)` or `&...`, and R is a polynomial, `None`, or `(G, S, T)`. Every line is checked
    against fmpz_mod_poly_gcd_euclidean_f and fmpz_mod_poly_xgcd_euclidean_f, which, like
    Malachite, divide only by leading coefficients that are invertible modulo m, and report a factor
    of m when one is not; the line must be `None` exactly when FLINT finds such a factor, and
    otherwise agree with FLINT. FLINT has no LCM modulo m, so the LCM is checked against the product
    of A and B / G, made monic, where G is FLINT's GCD; it is `None` when the GCD is, or when the
    product of the leading coefficients of A and B is not invertible. Lines from the panicking
    methods, whose demos use prime moduli, are also checked against fmpz_mod_poly_gcd and
    fmpz_mod_poly_xgcd and, when m fits in a word, nmod_poly_gcd and nmod_poly_xgcd, which assume
    the modulus prime. Modulo 1 every result is 0.

    Mode polynomial_gcd reads the IntegerPolynomial methods `gcd`, `gcd_assign`, `lcm`, and
    `lcm_assign` and checks them against fmpz_poly_gcd and fmpz_poly_lcm; mode
    rational_polynomial_gcd reads the RationalPolynomial methods `gcd`, `gcd_assign`, `lcm`,
    `lcm_assign`, and `extended_gcd` and checks them against fmpq_poly_gcd, fmpq_poly_lcm, and
    fmpq_poly_xgcd.

    Any other nonempty line is an error, as is an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>
#include <flint/fmpq_poly.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_poly.h>
#include <flint/fmpz_poly.h>
#include <flint/nmod_poly.h>

#include "oracle.h"

static long checked;
static long none_lines;
static long prime_lines;
static long nmod_lines;
static int power_of_2;
static int rational;
static const char * mode_name;

enum op { OP_GCD, OP_LCM, OP_XGCD };

/* Strips `&(` and the matching `)`, or a bare `&`, from an argument, in place. */
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

/* Which operation a method name is, or -1 if it is not one of the mode's. Sets `*checked_form`
   when the method is a checked one. */
static int
method_op(const char * name, int modular, int * checked_form)
{
    *checked_form = strncmp(name, "checked_", 8) == 0;
    if (*checked_form)
    {
        if (!modular)
            return -1;
        name += 8;
    }
    if (modular)
    {
        const char * prefix = power_of_2 ? "mod_power_of_2_" : "mod_";
        size_t prefix_len = strlen(prefix);
        if (strncmp(name, prefix, prefix_len) != 0)
            return -1;
        name += prefix_len;
        if (power_of_2 && !*checked_form)
            return -1;
    }
    if (strcmp(name, "gcd") == 0 || (!*checked_form && strcmp(name, "gcd_assign") == 0))
        return OP_GCD;
    if (strcmp(name, "lcm") == 0 || (!*checked_form && strcmp(name, "lcm_assign") == 0))
        return OP_LCM;
    if (strcmp(name, "extended_gcd") == 0 && (modular || rational))
        return OP_XGCD;
    return -1;
}

/* Splits a line into the two polynomials, the modulus or exponent (if `modular`), and the result,
   in place. Returns the operation, or -1 if the line is not recognized. */
static int
split_line(char * line, int modular, char ** a, char ** b, char ** m, char ** r,
           int * checked_form)
{
    line[strcspn(line, "\r\n")] = '\0';
    char * method;
    char * args;
    if (strncmp(line, "p := ", 5) == 0)
    {
        *a = line + 5;
        method = cut(*a, "; p.");
        if (method == NULL)
            return -1;
        args = cut(method, "(");
        if (args == NULL)
            return -1;
        *r = cut(args, "); p = ");
    }
    else
    {
        if (line[0] != '(')
            return -1;
        method = cut(line, ").");
        if (method == NULL)
            return -1;
        *a = strip_ref(line + 1);
        args = cut(method, "(");
        if (args == NULL)
            return -1;
        *r = cut(args, ") = ");
    }
    int op = method_op(method, modular, checked_form);
    if (*r == NULL || op < 0)
        return -1;
    *b = args;
    if (modular)
    {
        *m = cut(*b, ", ");
        if (*m == NULL)
            return -1;
        *m = strip_ref(*m);
    }
    *b = strip_ref(*b);
    return op;
}

/* Splits `(G, S, T)` into its three parts, in place. */
static int
split_triple(char * r, char ** g, char ** s, char ** t)
{
    size_t len = strlen(r);
    if (len < 2 || r[0] != '(' || r[len - 1] != ')')
        return 0;
    r[len - 1] = '\0';
    *g = r + 1;
    *s = cut(*g, ", ");
    if (*s == NULL)
        return 0;
    *t = cut(*s, ", ");
    return *t != NULL;
}

/* ------------------------------------------------------------------------------------- modular */

/* FLINT's result for a line modulo m: whether it exists and, if so, the polynomials. The GCD or LCM
   goes in `g`. */
static int
flint_mod_result(int op, fmpz_poly_t g, fmpz_poly_t s, fmpz_poly_t t, const fmpz_poly_t a,
                 const fmpz_poly_t b, const fmpz_t m)
{
    fmpz_mod_ctx_t ctx;
    fmpz_mod_ctx_init(ctx, m);
    fmpz_mod_poly_t x, y, xg, xs, xt;
    fmpz_mod_poly_init(x, ctx);
    fmpz_mod_poly_init(y, ctx);
    fmpz_mod_poly_init(xg, ctx);
    fmpz_mod_poly_init(xs, ctx);
    fmpz_mod_poly_init(xt, ctx);
    fmpz_mod_poly_set_fmpz_poly(x, a, ctx);
    fmpz_mod_poly_set_fmpz_poly(y, b, ctx);
    fmpz_t f;
    fmpz_init(f);
    int ok;
    if (op == OP_XGCD)
    {
        fmpz_mod_poly_xgcd_euclidean_f(f, xg, xs, xt, x, y, ctx);
        ok = fmpz_is_one(f);
    }
    else if (op == OP_GCD)
    {
        fmpz_mod_poly_gcd_euclidean_f(f, xg, x, y, ctx);
        ok = fmpz_is_one(f);
    }
    else if (fmpz_mod_poly_is_zero(x, ctx) || fmpz_mod_poly_is_zero(y, ctx))
    {
        fmpz_mod_poly_zero(xg, ctx);
        ok = 1;
    }
    else
    {
        fmpz_t c, inverse;
        fmpz_init(c);
        fmpz_init(inverse);
        fmpz_mod_mul(c, fmpz_mod_poly_lead(x, ctx), fmpz_mod_poly_lead(y, ctx), ctx);
        fmpz_gcdinv(f, inverse, c, m);
        ok = fmpz_is_one(f);
        if (ok)
        {
            fmpz_mod_poly_gcd_euclidean_f(f, xg, x, y, ctx);
            ok = fmpz_is_one(f);
        }
        if (ok)
        {
            /* G is monic, so dividing by it needs no inverse. */
            fmpz_mod_poly_div(xs, y, xg, ctx);
            fmpz_mod_poly_mul(xg, x, xs, ctx);
            fmpz_mod_poly_scalar_mul_fmpz(xg, xg, inverse, ctx);
        }
        fmpz_clear(c);
        fmpz_clear(inverse);
    }
    if (ok)
    {
        fmpz_mod_poly_get_fmpz_poly(g, xg, ctx);
        fmpz_mod_poly_get_fmpz_poly(s, xs, ctx);
        fmpz_mod_poly_get_fmpz_poly(t, xt, ctx);
    }
    fmpz_clear(f);
    fmpz_mod_poly_clear(x, ctx);
    fmpz_mod_poly_clear(y, ctx);
    fmpz_mod_poly_clear(xg, ctx);
    fmpz_mod_poly_clear(xs, ctx);
    fmpz_mod_poly_clear(xt, ctx);
    fmpz_mod_ctx_clear(ctx);
    return ok;
}

/* The same, by FLINT's functions for a prime modulus. */
static void
flint_prime_result(int op, fmpz_poly_t g, fmpz_poly_t s, fmpz_poly_t t, const fmpz_poly_t a,
                   const fmpz_poly_t b, const fmpz_t m)
{
    fmpz_mod_ctx_t ctx;
    fmpz_mod_ctx_init(ctx, m);
    fmpz_mod_poly_t x, y, xg, xs, xt;
    fmpz_mod_poly_init(x, ctx);
    fmpz_mod_poly_init(y, ctx);
    fmpz_mod_poly_init(xg, ctx);
    fmpz_mod_poly_init(xs, ctx);
    fmpz_mod_poly_init(xt, ctx);
    fmpz_mod_poly_set_fmpz_poly(x, a, ctx);
    fmpz_mod_poly_set_fmpz_poly(y, b, ctx);
    if (op == OP_XGCD)
    {
        fmpz_mod_poly_xgcd(xg, xs, xt, x, y, ctx);
    }
    else
    {
        fmpz_mod_poly_gcd(xg, x, y, ctx);
        if (op == OP_LCM)
        {
            if (fmpz_mod_poly_is_zero(x, ctx) || fmpz_mod_poly_is_zero(y, ctx))
            {
                fmpz_mod_poly_zero(xg, ctx);
            }
            else
            {
                fmpz_mod_poly_div(xs, y, xg, ctx);
                fmpz_mod_poly_mul(xg, x, xs, ctx);
                fmpz_mod_poly_make_monic(xg, xg, ctx);
            }
        }
    }
    fmpz_mod_poly_get_fmpz_poly(g, xg, ctx);
    fmpz_mod_poly_get_fmpz_poly(s, xs, ctx);
    fmpz_mod_poly_get_fmpz_poly(t, xt, ctx);
    fmpz_mod_poly_clear(x, ctx);
    fmpz_mod_poly_clear(y, ctx);
    fmpz_mod_poly_clear(xg, ctx);
    fmpz_mod_poly_clear(xs, ctx);
    fmpz_mod_poly_clear(xt, ctx);
    fmpz_mod_ctx_clear(ctx);
}

/* The same, by FLINT's word-sized functions for a prime modulus. */
static void
flint_nmod_result(int op, fmpz_poly_t g, fmpz_poly_t s, fmpz_poly_t t, const fmpz_poly_t a,
                  const fmpz_poly_t b, ulong n)
{
    nmod_poly_t x, y, xg, xs, xt;
    nmod_poly_init(x, n);
    nmod_poly_init(y, n);
    nmod_poly_init(xg, n);
    nmod_poly_init(xs, n);
    nmod_poly_init(xt, n);
    fmpz_poly_get_nmod_poly(x, a);
    fmpz_poly_get_nmod_poly(y, b);
    if (op == OP_XGCD)
    {
        nmod_poly_xgcd(xg, xs, xt, x, y);
    }
    else
    {
        nmod_poly_gcd(xg, x, y);
        if (op == OP_LCM)
        {
            if (nmod_poly_is_zero(x) || nmod_poly_is_zero(y))
            {
                nmod_poly_zero(xg);
            }
            else
            {
                nmod_poly_div(xs, y, xg);
                nmod_poly_mul(xg, x, xs);
                nmod_poly_make_monic(xg, xg);
            }
        }
    }
    fmpz_poly_set_nmod_poly_unsigned(g, xg);
    fmpz_poly_set_nmod_poly_unsigned(s, xs);
    fmpz_poly_set_nmod_poly_unsigned(t, xt);
    nmod_poly_clear(x);
    nmod_poly_clear(y);
    nmod_poly_clear(xg);
    nmod_poly_clear(xs);
    nmod_poly_clear(xt);
}

/* Whether FLINT's polynomials match the expected ones; `s` and `t` count only for an extended
   GCD. */
static int
matches(int op, const fmpz_poly_t g, const fmpz_poly_t s, const fmpz_poly_t t,
        const fmpz_poly_t eg, const fmpz_poly_t es, const fmpz_poly_t et)
{
    return fmpz_poly_equal(g, eg)
        && (op != OP_XGCD || (fmpz_poly_equal(s, es) && fmpz_poly_equal(t, et)));
}

static void
print_flint(const char * function, int line_number, int ok, const fmpz_poly_t g,
            const fmpz_poly_t s, const fmpz_poly_t t)
{
    flint_printf("error in %s test, line %d, by %s. FLINT: ", mode_name, line_number, function);
    if (!ok)
    {
        flint_printf("None\n");
        return;
    }
    fmpz_poly_print(g);
    flint_printf(", ");
    fmpz_poly_print(s);
    flint_printf(", ");
    fmpz_poly_print(t);
    flint_printf("\n");
}

static int
check_mod_line(char * line, int line_number)
{
    char * a_str;
    char * b_str;
    char * m_str;
    char * r_str;
    int checked_form;
    int op = split_line(line, 1, &a_str, &b_str, &m_str, &r_str, &checked_form);
    if (op < 0)
    {
        if (line[0] == '\0')
            return 0;
        flint_printf("error in %s test, line %d: unrecognized line\n", mode_name, line_number);
        return 1;
    }
    checked++;
    int result = 0;
    int claims_some = !(checked_form && strcmp(r_str, "None") == 0);
    fmpz_poly_t a, b, eg, es, et, g, s, t;
    fmpz_t m;
    fmpz_poly_init(a);
    fmpz_poly_init(b);
    fmpz_poly_init(eg);
    fmpz_poly_init(es);
    fmpz_poly_init(et);
    fmpz_poly_init(g);
    fmpz_poly_init(s);
    fmpz_poly_init(t);
    fmpz_init(m);
    int readable = fmpz_poly_set_str_malachite(a, a_str) && fmpz_poly_set_str_malachite(b, b_str);
    if (readable && claims_some)
    {
        if (op == OP_XGCD)
        {
            char * g_str;
            char * s_str;
            char * t_str;
            readable = split_triple(r_str, &g_str, &s_str, &t_str)
                && fmpz_poly_set_str_malachite(eg, g_str)
                && fmpz_poly_set_str_malachite(es, s_str)
                && fmpz_poly_set_str_malachite(et, t_str);
        }
        else
        {
            readable = fmpz_poly_set_str_malachite(eg, r_str);
        }
    }
    if (readable)
    {
        if (power_of_2)
        {
            fmpz_one(m);
            fmpz_mul_2exp(m, m, strtoul(m_str, NULL, 10));
        }
        else
        {
            readable = fmpz_set_str(m, m_str, 10) == 0 && fmpz_sgn(m) > 0;
        }
    }
    if (!readable)
    {
        flint_printf("error in %s test, line %d: unreadable input\n", mode_name, line_number);
        result = 1;
        goto cleanup;
    }
    if (fmpz_is_one(m))
    {
        if (!claims_some || !fmpz_poly_is_zero(eg) || !fmpz_poly_is_zero(es)
            || !fmpz_poly_is_zero(et))
        {
            flint_printf("error in %s test, line %d: modulo 1 every result is 0\n", mode_name,
                         line_number);
            result = 1;
        }
        goto cleanup;
    }
    int ok = flint_mod_result(op, g, s, t, a, b, m);
    if (ok != claims_some || (ok && !matches(op, g, s, t, eg, es, et)))
    {
        print_flint(op == OP_XGCD ? "fmpz_mod_poly_xgcd_euclidean_f"
                                  : "fmpz_mod_poly_gcd_euclidean_f",
                    line_number, ok, g, s, t);
        result = 1;
        goto cleanup;
    }
    if (!claims_some)
    {
        none_lines++;
    }
    if (!checked_form)
    {
        prime_lines++;
        flint_prime_result(op, g, s, t, a, b, m);
        if (!matches(op, g, s, t, eg, es, et))
        {
            print_flint(op == OP_XGCD ? "fmpz_mod_poly_xgcd" : "fmpz_mod_poly_gcd", line_number,
                        1, g, s, t);
            result = 1;
            goto cleanup;
        }
        if (fmpz_abs_fits_ui(m))
        {
            nmod_lines++;
            flint_nmod_result(op, g, s, t, a, b, fmpz_get_ui(m));
            if (!matches(op, g, s, t, eg, es, et))
            {
                print_flint(op == OP_XGCD ? "nmod_poly_xgcd" : "nmod_poly_gcd", line_number, 1,
                            g, s, t);
                result = 1;
            }
        }
    }
cleanup:
    fmpz_poly_clear(a);
    fmpz_poly_clear(b);
    fmpz_poly_clear(eg);
    fmpz_poly_clear(es);
    fmpz_poly_clear(et);
    fmpz_poly_clear(g);
    fmpz_poly_clear(s);
    fmpz_poly_clear(t);
    fmpz_clear(m);
    return result;
}

static int
run_mod(const char * arg, int pow2, const char * name)
{
    checked = 0;
    none_lines = 0;
    prime_lines = 0;
    nmod_lines = 0;
    power_of_2 = pow2;
    mode_name = name;
    int result = for_each_line(arg, check_mod_line);
    if (result == 0)
    {
        flint_printf("%s: %ld lines, %ld of them None; all by fmpz_mod_poly's Euclidean "
                     "functions, %ld also by fmpz_mod_poly's prime-modulus functions, %ld also by "
                     "nmod_poly\n",
                     name, checked, none_lines, prime_lines, nmod_lines);
    }
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_mod_gcd(const char * arg)
{
    return run_mod(arg, 0, "mod_gcd");
}

int
run_mod_power_of_2_gcd(const char * arg)
{
    return run_mod(arg, 1, "mod_power_of_2_gcd");
}

/* ------------------------------------------------------------------------- integers and rationals */

static int
check_integer_line(char * line, int line_number)
{
    char * a_str;
    char * b_str;
    char * r_str;
    int checked_form;
    int op = split_line(line, 0, &a_str, &b_str, NULL, &r_str, &checked_form);
    if (op < 0)
    {
        if (line[0] == '\0')
            return 0;
        flint_printf("error in %s test, line %d: unrecognized line\n", mode_name, line_number);
        return 1;
    }
    checked++;
    int result = 0;
    fmpz_poly_t a, b, expected, r;
    fmpz_poly_init(a);
    fmpz_poly_init(b);
    fmpz_poly_init(expected);
    fmpz_poly_init(r);
    if (!fmpz_poly_set_str_malachite(a, a_str) || !fmpz_poly_set_str_malachite(b, b_str)
        || !fmpz_poly_set_str_malachite(expected, r_str))
    {
        flint_printf("error in %s test, line %d: unreadable input\n", mode_name, line_number);
        result = 1;
    }
    else
    {
        if (op == OP_GCD)
            fmpz_poly_gcd(r, a, b);
        else
            fmpz_poly_lcm(r, a, b);
        if (!fmpz_poly_equal(r, expected))
        {
            flint_printf("error in %s test, line %d, by %s. FLINT: ", mode_name, line_number,
                         op == OP_GCD ? "fmpz_poly_gcd" : "fmpz_poly_lcm");
            fmpz_poly_print(r);
            flint_printf("\n");
            result = 1;
        }
    }
    fmpz_poly_clear(a);
    fmpz_poly_clear(b);
    fmpz_poly_clear(expected);
    fmpz_poly_clear(r);
    return result;
}

static int
check_rational_line(char * line, int line_number)
{
    char * a_str;
    char * b_str;
    char * r_str;
    int checked_form;
    int op = split_line(line, 0, &a_str, &b_str, NULL, &r_str, &checked_form);
    if (op < 0)
    {
        if (line[0] == '\0')
            return 0;
        flint_printf("error in %s test, line %d: unrecognized line\n", mode_name, line_number);
        return 1;
    }
    checked++;
    int result = 0;
    fmpq_poly_t a, b, eg, es, et, g, s, t;
    fmpq_poly_init(a);
    fmpq_poly_init(b);
    fmpq_poly_init(eg);
    fmpq_poly_init(es);
    fmpq_poly_init(et);
    fmpq_poly_init(g);
    fmpq_poly_init(s);
    fmpq_poly_init(t);
    int readable = fmpq_poly_set_str_malachite(a, a_str) && fmpq_poly_set_str_malachite(b, b_str);
    if (readable)
    {
        if (op == OP_XGCD)
        {
            char * g_str;
            char * s_str;
            char * t_str;
            readable = split_triple(r_str, &g_str, &s_str, &t_str)
                && fmpq_poly_set_str_malachite(eg, g_str)
                && fmpq_poly_set_str_malachite(es, s_str)
                && fmpq_poly_set_str_malachite(et, t_str);
        }
        else
        {
            readable = fmpq_poly_set_str_malachite(eg, r_str);
        }
    }
    if (!readable)
    {
        flint_printf("error in %s test, line %d: unreadable input\n", mode_name, line_number);
        result = 1;
        goto cleanup;
    }
    if (op == OP_GCD)
        fmpq_poly_gcd(g, a, b);
    else if (op == OP_LCM)
        fmpq_poly_lcm(g, a, b);
    else
        fmpq_poly_xgcd(g, s, t, a, b);
    if (!fmpq_poly_equal(g, eg)
        || (op == OP_XGCD && !(fmpq_poly_equal(s, es) && fmpq_poly_equal(t, et))))
    {
        flint_printf("error in %s test, line %d, by %s. FLINT: ", mode_name, line_number,
                     op == OP_GCD ? "fmpq_poly_gcd"
                                  : (op == OP_LCM ? "fmpq_poly_lcm" : "fmpq_poly_xgcd"));
        fmpq_poly_print(g);
        if (op == OP_XGCD)
        {
            flint_printf(", ");
            fmpq_poly_print(s);
            flint_printf(", ");
            fmpq_poly_print(t);
        }
        flint_printf("\n");
        result = 1;
    }
cleanup:
    fmpq_poly_clear(a);
    fmpq_poly_clear(b);
    fmpq_poly_clear(eg);
    fmpq_poly_clear(es);
    fmpq_poly_clear(et);
    fmpq_poly_clear(g);
    fmpq_poly_clear(s);
    fmpq_poly_clear(t);
    return result;
}

int
run_polynomial_gcd(const char * arg)
{
    checked = 0;
    rational = 0;
    mode_name = "polynomial_gcd";
    int result = for_each_line(arg, check_integer_line);
    if (result == 0)
    {
        flint_printf("polynomial_gcd: %ld lines\n", checked);
    }
    return result != 0 ? result : require_some_lines(mode_name, checked);
}

int
run_rational_polynomial_gcd(const char * arg)
{
    checked = 0;
    rational = 1;
    mode_name = "rational_polynomial_gcd";
    int result = for_each_line(arg, check_rational_line);
    if (result == 0)
    {
        flint_printf("rational_polynomial_gcd: %ld lines\n", checked);
    }
    return result != 0 ? result : require_some_lines(mode_name, checked);
}

/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the UnsignedPolynomial and NaturalPolynomial modular power removal
    and root division demos against FLINT. Mode mod_remove_power reads the methods
    `mod_remove_power` and `mod_remove_power_assign`, checked against fmpz_mod_poly_remove and,
    when the modulus fits in a word, nmod_poly_remove; mode mod_div_root reads `mod_div_root` and
    `mod_div_root_assign`, checked against fmpz_mod_poly_divrem by x - c and
    fmpz_mod_poly_evaluate_fmpz and, when the modulus fits in a word, nmod_poly_div_root. The
    mod_power_of_2_ modes read the `mod_power_of_2_` versions, with the modulus 2^k. The shapes
    recognized are `(A).METHOD(B, m) = (Q, e)` and `p := A; p.METHOD(B, m) = e; p = Q`, where `A`,
    `B`, and `m` may be written `&(...)`; for div_root, B is the number c and e the remainder.
    Modulo 1 every result must be 0. Any other nonempty line is an error, as is an input with no
    lines at all.
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
static int power_of_2;
static int div_root;
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

/* Whether a method name is one of the mode's. */
static int
known_method(const char * name)
{
    const char * prefix = power_of_2 ? "mod_power_of_2_" : "mod_";
    size_t prefix_len = strlen(prefix);
    const char * op = div_root ? "div_root" : "remove_power";
    size_t op_len = strlen(op);
    if (strncmp(name, prefix, prefix_len) != 0)
        return 0;
    name += prefix_len;
    if (strncmp(name, op, op_len) != 0)
        return 0;
    name += op_len;
    return *name == '\0' || strcmp(name, "_assign") == 0;
}

/* Splits a line into the polynomial, the second argument, the modulus or exponent, the expected
   polynomial result, and the expected number, in place. */
static int
split_line(char * line, char ** a, char ** b, char ** m, char ** q, char ** e)
{
    line[strcspn(line, "\r\n")] = '\0';
    char * method;
    char * args;
    if (strncmp(line, "p := ", 5) == 0)
    {
        *a = line + 5;
        method = cut(*a, "; p.");
        if (method == NULL)
            return 0;
        args = cut(method, "(");
        if (args == NULL)
            return 0;
        *e = cut(args, ") = ");
        if (*e == NULL)
            return 0;
        *q = cut(*e, "; p = ");
        if (*q == NULL)
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
        if (args == NULL)
            return 0;
        char * result = cut(args, ") = ");
        if (result == NULL)
            return 0;
        size_t len = strlen(result);
        if (len < 2 || result[0] != '(' || result[len - 1] != ')')
            return 0;
        result[len - 1] = '\0';
        *q = result + 1;
        *e = cut(*q, ", ");
        if (*e == NULL)
            return 0;
    }
    if (!known_method(method))
        return 0;
    *b = args;
    *m = cut(*b, ", ");
    if (*m == NULL)
        return 0;
    *b = strip_ref(*b);
    *m = strip_ref(*m);
    return 1;
}

static int
fail(const char * what, int line_number)
{
    flint_printf("error in %s test, line %d: %s\n", mode_name, line_number, what);
    return 1;
}

static int
check_line(char * line, int line_number)
{
    char * a_str;
    char * b_str;
    char * m_str;
    char * q_str;
    char * e_str;
    if (!split_line(line, &a_str, &b_str, &m_str, &q_str, &e_str))
    {
        if (line[0] == '\0')
            return 0;
        return fail("unrecognized line", line_number);
    }
    checked++;
    int result = 0;
    fmpz_poly_t a, b, q_expected, q;
    fmpz_t m, c, e_expected, e;
    fmpz_poly_init(a);
    fmpz_poly_init(b);
    fmpz_poly_init(q_expected);
    fmpz_poly_init(q);
    fmpz_init(m);
    fmpz_init(c);
    fmpz_init(e_expected);
    fmpz_init(e);
    if (!fmpz_poly_set_str_malachite(a, a_str) || !fmpz_poly_set_str_malachite(q_expected, q_str)
        || fmpz_set_str(e_expected, e_str, 10) != 0
        || (div_root ? fmpz_set_str(c, b_str, 10) != 0 : !fmpz_poly_set_str_malachite(b, b_str)))
    {
        result = fail("unreadable input", line_number);
        goto cleanup;
    }
    if (power_of_2)
    {
        fmpz_one(m);
        fmpz_mul_2exp(m, m, strtoul(m_str, NULL, 10));
    }
    else if (fmpz_set_str(m, m_str, 10) != 0 || fmpz_sgn(m) <= 0)
    {
        result = fail("unreadable input", line_number);
        goto cleanup;
    }
    if (fmpz_is_one(m))
    {
        if (!fmpz_poly_is_zero(q_expected) || !fmpz_is_zero(e_expected))
            result = fail("every result modulo 1 must be 0", line_number);
        goto cleanup;
    }
    {
        fmpz_mod_ctx_t ctx;
        fmpz_mod_ctx_init(ctx, m);
        fmpz_mod_poly_t x, y, z, r;
        fmpz_mod_poly_init(x, ctx);
        fmpz_mod_poly_init(y, ctx);
        fmpz_mod_poly_init(z, ctx);
        fmpz_mod_poly_init(r, ctx);
        fmpz_mod_poly_set_fmpz_poly(x, a, ctx);
        if (div_root)
        {
            /* y = x - c */
            fmpz_t neg;
            fmpz_init(neg);
            fmpz_mod_neg(neg, c, ctx);
            fmpz_mod_poly_set_coeff_fmpz(y, 0, neg, ctx);
            fmpz_mod_poly_set_coeff_ui(y, 1, 1, ctx);
            fmpz_clear(neg);
            fmpz_mod_poly_divrem(z, r, x, y, ctx);
            fmpz_mod_poly_get_coeff_fmpz(e, r, 0, ctx);
            fmpz_t value;
            fmpz_init(value);
            fmpz_mod_poly_evaluate_fmpz(value, x, c, ctx);
            if (!fmpz_equal(value, e))
                result = fail("fmpz_mod_poly_evaluate_fmpz disagrees with the remainder",
                              line_number);
            fmpz_clear(value);
        }
        else
        {
            fmpz_mod_poly_set_fmpz_poly(y, b, ctx);
            fmpz_mod_poly_set(z, x, ctx);
            fmpz_set_ui(e, fmpz_mod_poly_remove(z, y, ctx));
        }
        fmpz_mod_poly_get_fmpz_poly(q, z, ctx);
        if (!fmpz_poly_equal(q, q_expected) || !fmpz_equal(e, e_expected))
        {
            flint_printf("error in %s test, line %d, by fmpz_mod_poly. FLINT: ", mode_name,
                         line_number);
            fmpz_poly_print(q);
            flint_printf(", ");
            fmpz_print(e);
            flint_printf("\n");
            result = 1;
        }
        fmpz_mod_poly_clear(x, ctx);
        fmpz_mod_poly_clear(y, ctx);
        fmpz_mod_poly_clear(z, ctx);
        fmpz_mod_poly_clear(r, ctx);
        fmpz_mod_ctx_clear(ctx);
    }
    if (result == 0 && fmpz_abs_fits_ui(m))
    {
        nmod_lines++;
        ulong n = fmpz_get_ui(m);
        nmod_poly_t x, y, z;
        nmod_poly_init(x, n);
        nmod_poly_init(y, n);
        nmod_poly_init(z, n);
        fmpz_poly_get_nmod_poly(x, a);
        if (div_root)
        {
            fmpz_set_ui(e, nmod_poly_div_root(z, x, fmpz_get_ui(c)));
        }
        else
        {
            fmpz_poly_get_nmod_poly(y, b);
            nmod_poly_set(z, x);
            fmpz_set_ui(e, nmod_poly_remove(z, y));
        }
        fmpz_poly_set_nmod_poly_unsigned(q, z);
        if (!fmpz_poly_equal(q, q_expected) || !fmpz_equal(e, e_expected))
        {
            flint_printf("error in %s test, line %d, by nmod_poly. FLINT: ", mode_name,
                         line_number);
            fmpz_poly_print(q);
            flint_printf(", ");
            fmpz_print(e);
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
    fmpz_poly_clear(q_expected);
    fmpz_poly_clear(q);
    fmpz_clear(m);
    fmpz_clear(c);
    fmpz_clear(e_expected);
    fmpz_clear(e);
    return result;
}

static int
run(const char * arg, int pow2, int root, const char * name)
{
    checked = 0;
    nmod_lines = 0;
    power_of_2 = pow2;
    div_root = root;
    mode_name = name;
    int result = for_each_line(arg, check_line);
    if (result == 0)
        flint_printf("%s: %ld lines, all by fmpz_mod_poly, %ld of them also by nmod_poly\n", name,
                     checked, nmod_lines);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_mod_remove_power(const char * arg)
{
    return run(arg, 0, 0, "mod_remove_power");
}

int
run_mod_power_of_2_remove_power(const char * arg)
{
    return run(arg, 1, 0, "mod_power_of_2_remove_power");
}

int
run_mod_div_root(const char * arg)
{
    return run(arg, 0, 1, "mod_div_root");
}

int
run_mod_power_of_2_div_root(const char * arg)
{
    return run(arg, 1, 1, "mod_power_of_2_div_root");
}

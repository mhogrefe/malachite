/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the UnsignedPolynomial and NaturalPolynomial modular Euclidean
    division demos against fmpz_mod_poly_divrem, fmpz_mod_poly_div, and fmpz_mod_poly_rem and, when
    the modulus is at least 2 and fits in a word, against nmod_poly_divrem, nmod_poly_div, and
    nmod_poly_rem as well. Mode mod_div_rem reads the methods `mod_div_rem`, `mod_quotient`, and
    `mod_rem`, their `checked_` forms, and `mod_div_assign_rem`, `mod_quotient_assign`, and
    `mod_rem_assign`, with the modulus m, and the `_precomputed` versions of the unchecked methods,
    whose last argument is `&data`; mode mod_power_of_2_div_rem reads the `mod_power_of_2_`
    versions, with the modulus 2^k. The shapes recognized are `(A).METHOD(B, m) = R`, `p := A;
    p.METHOD(B, m); p = R`, and, for the `div_assign_rem` methods, `p := A; p.METHOD(B, m) = R; p =
    Q`, where `A`, `B`, and `m` may be written `&(...)`, and the result of a `div_rem` method is
    `(Q, R)`. Any result may be `None`. FLINT aborts when the divisor is 0 or its leading
    coefficient is not invertible, so a `None` is checked against fmpz_gcd instead, and modulo 1
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
static long none_lines;
static long preinv_lines;
static int power_of_2;
/* Whether the line being checked is of a `_precomputed` method. */
static int precomputed;
static const char * mode_name;

/* What a method returns: the quotient and remainder, the quotient, or the remainder. */
enum op { OP_DIV_REM, OP_QUOTIENT, OP_REM };

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

/* Which operation a method name is, or -1 if it is not one of the mode's. `assign_rem` is set for
   the `div_assign_rem` methods. */
static int
method_op(const char * name, int * assign_rem)
{
    const char * prefix = power_of_2 ? "mod_power_of_2_" : "mod_";
    size_t prefix_len = strlen(prefix);
    *assign_rem = 0;
    if (strncmp(name, "checked_", 8) == 0)
    {
        name += 8;
        if (strncmp(name, prefix, prefix_len) != 0)
        {
            return -1;
        }
        name += prefix_len;
        if (strcmp(name, "div_rem") == 0)
            return OP_DIV_REM;
        if (strcmp(name, "quotient") == 0)
            return OP_QUOTIENT;
        if (strcmp(name, "rem") == 0)
            return OP_REM;
        return -1;
    }
    if (strncmp(name, prefix, prefix_len) != 0)
    {
        return -1;
    }
    name += prefix_len;
    if (strcmp(name, "div_rem") == 0)
        return OP_DIV_REM;
    if (strcmp(name, "div_assign_rem") == 0)
    {
        *assign_rem = 1;
        return OP_DIV_REM;
    }
    if (strcmp(name, "quotient") == 0 || strcmp(name, "quotient_assign") == 0)
        return OP_QUOTIENT;
    if (strcmp(name, "rem") == 0 || strcmp(name, "rem_assign") == 0)
        return OP_REM;
    return -1;
}

/* Removes `_precomputed` from a method name, in place, setting `precomputed` if it was there. */
static void
remove_precomputed(char * name)
{
    char * at = strstr(name, "_precomputed");
    precomputed = at != NULL;
    if (precomputed)
    {
        memmove(at, at + 12, strlen(at + 12) + 1);
    }
}

/* Splits a line into the dividend, the divisor, the modulus or exponent, and the expected
   quotient and remainder (either of which may be NULL when the method does not return it), in
   place. Polynomials are displayed without spaces, commas, or parentheses, so the separators are
   unambiguous. Returns the operation, or -1 if the line is not recognized. */
static int
split_line(char * line, char ** a, char ** b, char ** m, char ** q, char ** r, int * is_none)
{
    line[strcspn(line, "\r\n")] = '\0';
    char * method;
    char * args;
    char * result;
    int assign_rem;
    int op;
    *q = NULL;
    *r = NULL;
    *is_none = 0;
    if (strncmp(line, "p := ", 5) == 0)
    {
        *a = line + 5;
        method = cut(*a, "; p.");
        if (method == NULL)
            return -1;
        args = cut(method, "(");
        if (args == NULL)
            return -1;
        remove_precomputed(method);
        op = method_op(method, &assign_rem);
        if (op < 0)
            return -1;
        if (assign_rem)
        {
            /* p.METHOD(B, m) = R; p = Q */
            *r = cut(args, ") = ");
            if (*r == NULL)
                return -1;
            *q = cut(*r, "; p = ");
            if (*q == NULL)
                return -1;
        }
        else
        {
            result = cut(args, "); p = ");
            if (result == NULL)
                return -1;
            if (op == OP_QUOTIENT)
                *q = result;
            else
                *r = result;
        }
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
        remove_precomputed(method);
        op = method_op(method, &assign_rem);
        if (op < 0)
            return -1;
        result = cut(args, ") = ");
        if (result == NULL)
            return -1;
        if (strcmp(result, "None") == 0)
        {
            *is_none = 1;
        }
        else if (op == OP_DIV_REM)
        {
            size_t len = strlen(result);
            if (len < 2 || result[0] != '(' || result[len - 1] != ')')
                return -1;
            result[len - 1] = '\0';
            *q = result + 1;
            *r = cut(*q, ", ");
            if (*r == NULL)
                return -1;
        }
        else if (op == OP_QUOTIENT)
        {
            *q = result;
        }
        else
        {
            *r = result;
        }
    }
    *b = args;
    *m = cut(*b, ", ");
    if (*m == NULL)
        return -1;
    if (precomputed)
    {
        char * data = cut(*m, ", ");
        if (data == NULL || strcmp(data, "&data") != 0)
            return -1;
    }
    *b = strip_ref(*b);
    *m = strip_ref(*m);
    return op;
}

/* Compares a FLINT result with the expected polynomial, if there is one, printing an error that
   names `function` if they differ. */
static int
compare(const fmpz_poly_t actual, const fmpz_poly_t expected, int present, const char * function,
        int line_number)
{
    if (!present || fmpz_poly_equal(actual, expected))
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
    char * m_str;
    char * q_str;
    char * r_str;
    int is_none;
    int op = split_line(line, &a_str, &b_str, &m_str, &q_str, &r_str, &is_none);
    if (op < 0)
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
    fmpz_poly_t a, b, q_expected, r_expected, q, r;
    fmpz_t m, g;
    fmpz_poly_init(a);
    fmpz_poly_init(b);
    fmpz_poly_init(q_expected);
    fmpz_poly_init(r_expected);
    fmpz_poly_init(q);
    fmpz_poly_init(r);
    fmpz_init(m);
    fmpz_init(g);
    if (!fmpz_poly_set_str_malachite(a, a_str) || !fmpz_poly_set_str_malachite(b, b_str)
        || (q_str != NULL && !fmpz_poly_set_str_malachite(q_expected, q_str))
        || (r_str != NULL && !fmpz_poly_set_str_malachite(r_expected, r_str)))
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
        if (is_none || (q_str != NULL && !fmpz_poly_is_zero(q_expected))
            || (r_str != NULL && !fmpz_poly_is_zero(r_expected)))
        {
            flint_printf("error in %s test, line %d: every result modulo 1 must be 0\n",
                         mode_name, line_number);
            result = 1;
        }
        goto cleanup;
    }
    /* The zero divisor is not invertible: its leading coefficient's gcd with m is taken to be m.
       */
    if (fmpz_poly_is_zero(b))
    {
        fmpz_set(g, m);
    }
    else
    {
        fmpz_gcd(g, b->coeffs + b->length - 1, m);
    }
    if (is_none)
    {
        none_lines++;
        if (fmpz_is_one(g))
        {
            flint_printf("error in %s test, line %d: None, but the divisor's leading coefficient "
                         "is invertible\n",
                         mode_name, line_number);
            result = 1;
        }
        goto cleanup;
    }
    if (!fmpz_is_one(g))
    {
        flint_printf("error in %s test, line %d: a result, but the divisor's leading coefficient "
                     "is not invertible\n",
                     mode_name, line_number);
        result = 1;
        goto cleanup;
    }
    {
        fmpz_mod_ctx_t ctx;
        fmpz_mod_ctx_init(ctx, m);
        fmpz_mod_poly_t x, y, zq, zr;
        fmpz_mod_poly_init(x, ctx);
        fmpz_mod_poly_init(y, ctx);
        fmpz_mod_poly_init(zq, ctx);
        fmpz_mod_poly_init(zr, ctx);
        fmpz_mod_poly_set_fmpz_poly(x, a, ctx);
        fmpz_mod_poly_set_fmpz_poly(y, b, ctx);
        if (op == OP_DIV_REM)
        {
            fmpz_mod_poly_divrem(zq, zr, x, y, ctx);
            fmpz_mod_poly_get_fmpz_poly(q, zq, ctx);
            fmpz_mod_poly_get_fmpz_poly(r, zr, ctx);
            result |= compare(q, q_expected, 1, "fmpz_mod_poly_divrem", line_number);
            result |= compare(r, r_expected, 1, "fmpz_mod_poly_divrem", line_number);
        }
        else if (op == OP_QUOTIENT)
        {
            fmpz_mod_poly_div(zq, x, y, ctx);
            fmpz_mod_poly_get_fmpz_poly(q, zq, ctx);
            result |= compare(q, q_expected, 1, "fmpz_mod_poly_div", line_number);
        }
        else
        {
            fmpz_mod_poly_rem(zr, x, y, ctx);
            fmpz_mod_poly_get_fmpz_poly(r, zr, ctx);
            result |= compare(r, r_expected, 1, "fmpz_mod_poly_rem", line_number);
        }
        fmpz_mod_poly_clear(x, ctx);
        fmpz_mod_poly_clear(y, ctx);
        fmpz_mod_poly_clear(zq, ctx);
        fmpz_mod_poly_clear(zr, ctx);
        fmpz_mod_ctx_clear(ctx);
    }
    if (result == 0 && fmpz_abs_fits_ui(m))
    {
        nmod_lines++;
        ulong n = fmpz_get_ui(m);
        nmod_poly_t x, y, zq, zr;
        nmod_poly_init(x, n);
        nmod_poly_init(y, n);
        nmod_poly_init(zq, n);
        nmod_poly_init(zr, n);
        fmpz_poly_get_nmod_poly(x, a);
        fmpz_poly_get_nmod_poly(y, b);
        slong len_a = nmod_poly_length(x);
        slong len_b = nmod_poly_length(y);
        if (precomputed && len_a >= len_b && len_a <= 2 * len_b - 2)
        {
            /* A precomputed division is also checked against FLINT's division by a precomputed
               inverse, where FLINT allows it: the inverse of the reversed divisor, to as many
               terms as the quotient has. */
            nmod_poly_t y_inverse;
            nmod_poly_init(y_inverse, n);
            nmod_poly_reverse(y_inverse, y, len_b);
            nmod_poly_inv_series(y_inverse, y_inverse, len_a - len_b + 1);
            if (op == OP_QUOTIENT)
            {
                nmod_poly_div_newton_n_preinv(zq, x, y, y_inverse);
                fmpz_poly_set_nmod_poly_unsigned(q, zq);
                result |= compare(q, q_expected, 1, "nmod_poly_div_newton_n_preinv", line_number);
            }
            else
            {
                nmod_poly_divrem_newton_n_preinv(zq, zr, x, y, y_inverse);
                fmpz_poly_set_nmod_poly_unsigned(q, zq);
                fmpz_poly_set_nmod_poly_unsigned(r, zr);
                result |= compare(q, q_expected, op == OP_DIV_REM,
                                  "nmod_poly_divrem_newton_n_preinv", line_number);
                result |= compare(r, r_expected, 1, "nmod_poly_divrem_newton_n_preinv",
                                  line_number);
            }
            preinv_lines++;
            nmod_poly_clear(y_inverse);
        }
        else if (op == OP_DIV_REM)
        {
            nmod_poly_divrem(zq, zr, x, y);
            fmpz_poly_set_nmod_poly_unsigned(q, zq);
            fmpz_poly_set_nmod_poly_unsigned(r, zr);
            result |= compare(q, q_expected, 1, "nmod_poly_divrem", line_number);
            result |= compare(r, r_expected, 1, "nmod_poly_divrem", line_number);
        }
        else if (op == OP_QUOTIENT)
        {
            nmod_poly_div(zq, x, y);
            fmpz_poly_set_nmod_poly_unsigned(q, zq);
            result |= compare(q, q_expected, 1, "nmod_poly_div", line_number);
        }
        else
        {
            nmod_poly_rem(zr, x, y);
            fmpz_poly_set_nmod_poly_unsigned(r, zr);
            result |= compare(r, r_expected, 1, "nmod_poly_rem", line_number);
        }
        nmod_poly_clear(x);
        nmod_poly_clear(y);
        nmod_poly_clear(zq);
        nmod_poly_clear(zr);
    }
cleanup:
    fmpz_poly_clear(a);
    fmpz_poly_clear(b);
    fmpz_poly_clear(q_expected);
    fmpz_poly_clear(r_expected);
    fmpz_poly_clear(q);
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
    preinv_lines = 0;
    power_of_2 = pow2;
    mode_name = name;
    int result = for_each_line(arg, check_line);
    if (result == 0)
    {
        flint_printf("%s: %ld lines, %ld of them None; the rest by fmpz_mod_poly, %ld of them also "
                     "by nmod_poly, %ld of those by a precomputed inverse\n",
                     name, checked, none_lines, nmod_lines, preinv_lines);
    }
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_mod_div_rem(const char * arg)
{
    return run(arg, 0, "mod_div_rem");
}

int
run_mod_power_of_2_div_rem(const char * arg)
{
    return run(arg, 1, "mod_power_of_2_div_rem");
}

/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the UnsignedPolynomial and NaturalPolynomial modular exact division
    and divisibility demos against fmpz_mod_poly_divides and, when the modulus is at least 2 and
    fits in a word, against nmod_poly_divides and nmod_poly_divexact as well. Mode mod_div_exact
    reads the methods `mod_div_exact`, `mod_div_exact_assign`, `checked_mod_div_exact`, and
    `mod_divisible_by`, with the modulus m; mode mod_power_of_2_div_exact reads the
    `mod_power_of_2_` versions, with the modulus 2^k. The shapes recognized are `(A).METHOD(B, m) =
    R` and `p := A; p.METHOD(B, m); p = R`, where `A`, `B`, and `m` may be written `&(...)`, and R
    is a polynomial, `None`, `true`, or `false`. The `mod_div_exact` demos only divide exactly, so
    FLINT must find B dividing A, with the same quotient; `None` and `false` mean that it does not,
    and modulo 1 every quotient is 0 and every polynomial divisible. Any other nonempty line is an
    error, as is an input with no lines at all.
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

/* What a method returns: an exact quotient, a quotient or None, or whether B divides A. */
enum op { OP_EXACT, OP_CHECKED, OP_DIVISIBLE };

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

/* Which operation a method name is, or -1 if it is not one of the mode's. */
static int
method_op(const char * name)
{
    const char * prefix = power_of_2 ? "mod_power_of_2_" : "mod_";
    size_t prefix_len = strlen(prefix);
    int checked_form = strncmp(name, "checked_", 8) == 0;
    if (checked_form)
        name += 8;
    if (strncmp(name, prefix, prefix_len) != 0)
        return -1;
    name += prefix_len;
    if (checked_form)
        return strcmp(name, "div_exact") == 0 ? OP_CHECKED : -1;
    if (strcmp(name, "div_exact") == 0 || strcmp(name, "div_exact_assign") == 0)
        return OP_EXACT;
    if (strcmp(name, "divisible_by") == 0)
        return OP_DIVISIBLE;
    return -1;
}

/* Splits a line into the dividend, the divisor, the modulus or exponent, and the result, in place.
   Returns the operation, or -1 if the line is not recognized. */
static int
split_line(char * line, char ** a, char ** b, char ** m, char ** r)
{
    line[strcspn(line, "\r\n")] = '\0';
    char * method;
    char * args;
    int op;
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
    op = method_op(method);
    if (*r == NULL || op < 0)
        return -1;
    *b = args;
    *m = cut(*b, ", ");
    if (*m == NULL)
        return -1;
    *b = strip_ref(*b);
    *m = strip_ref(*m);
    return op;
}

static int
check_line(char * line, int line_number)
{
    char * a_str;
    char * b_str;
    char * m_str;
    char * r_str;
    int op = split_line(line, &a_str, &b_str, &m_str, &r_str);
    if (op < 0)
    {
        if (line[0] == '\0')
            return 0;
        flint_printf("error in %s test, line %d: unrecognized line\n", mode_name, line_number);
        return 1;
    }
    checked++;
    int result = 0;
    /* What the line claims: whether B divides A and, if so, the quotient. */
    int claims_divisible;
    int has_quotient = 0;
    fmpz_poly_t a, b, expected, q;
    fmpz_t m;
    fmpz_poly_init(a);
    fmpz_poly_init(b);
    fmpz_poly_init(expected);
    fmpz_poly_init(q);
    fmpz_init(m);
    if (op == OP_DIVISIBLE)
    {
        claims_divisible = strcmp(r_str, "true") == 0;
        if (!claims_divisible && strcmp(r_str, "false") != 0)
        {
            flint_printf("error in %s test, line %d: unreadable input\n", mode_name, line_number);
            result = 1;
            goto cleanup;
        }
    }
    else if (op == OP_CHECKED && strcmp(r_str, "None") == 0)
    {
        claims_divisible = 0;
    }
    else
    {
        claims_divisible = 1;
        has_quotient = 1;
    }
    if (!fmpz_poly_set_str_malachite(a, a_str) || !fmpz_poly_set_str_malachite(b, b_str)
        || (has_quotient && !fmpz_poly_set_str_malachite(expected, r_str)))
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
        if (!claims_divisible || (has_quotient && !fmpz_poly_is_zero(expected)))
        {
            flint_printf("error in %s test, line %d: modulo 1 the quotient is 0\n", mode_name,
                         line_number);
            result = 1;
        }
        goto cleanup;
    }
    if (fmpz_poly_is_zero(b))
    {
        if (op != OP_DIVISIBLE || claims_divisible != fmpz_poly_is_zero(a))
        {
            flint_printf("error in %s test, line %d: only 0 is divisible by 0\n", mode_name,
                         line_number);
            result = 1;
        }
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
        int divides = fmpz_mod_poly_divides(z, x, y, ctx);
        fmpz_mod_poly_get_fmpz_poly(q, z, ctx);
        if (divides != claims_divisible || (has_quotient && !fmpz_poly_equal(q, expected)))
        {
            flint_printf("error in %s test, line %d, by fmpz_mod_poly_divides. FLINT: %d, ",
                         mode_name, line_number, divides);
            fmpz_poly_print(q);
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
        ulong n = fmpz_get_ui(m);
        nmod_poly_t x, y, z;
        nmod_poly_init(x, n);
        nmod_poly_init(y, n);
        nmod_poly_init(z, n);
        fmpz_poly_get_nmod_poly(x, a);
        fmpz_poly_get_nmod_poly(y, b);
        int divides = nmod_poly_divides(z, x, y);
        fmpz_poly_set_nmod_poly_unsigned(q, z);
        if (divides != claims_divisible || (has_quotient && !fmpz_poly_equal(q, expected)))
        {
            flint_printf("error in %s test, line %d, by nmod_poly_divides. FLINT: %d, ", mode_name,
                         line_number, divides);
            fmpz_poly_print(q);
            flint_printf("\n");
            result = 1;
        }
        if (result == 0 && op == OP_EXACT)
        {
            nmod_poly_divexact(z, x, y);
            fmpz_poly_set_nmod_poly_unsigned(q, z);
            if (!fmpz_poly_equal(q, expected))
            {
                flint_printf("error in %s test, line %d, by nmod_poly_divexact. FLINT: ",
                             mode_name, line_number);
                fmpz_poly_print(q);
                flint_printf("\n");
                result = 1;
            }
        }
        nmod_poly_clear(x);
        nmod_poly_clear(y);
        nmod_poly_clear(z);
    }
    if (!claims_divisible)
        none_lines++;
cleanup:
    fmpz_poly_clear(a);
    fmpz_poly_clear(b);
    fmpz_poly_clear(expected);
    fmpz_poly_clear(q);
    fmpz_clear(m);
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
        flint_printf("%s: %ld lines, %ld of them not divisible; all by fmpz_mod_poly_divides, %ld "
                     "of them also by nmod_poly\n",
                     name, checked, none_lines, nmod_lines);
    }
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_mod_div_exact(const char * arg)
{
    return run(arg, 0, "mod_div_exact");
}

int
run_mod_power_of_2_div_exact(const char * arg)
{
    return run(arg, 1, "mod_power_of_2_div_exact");
}

/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the IntegerPolynomial pseudo-division demos against FLINT. The
    methods read are `pseudo_div_rem`, `pseudo_div`, and `pseudo_rem`, and their `_assign` forms
    (`pseudo_div_assign_rem`, `pseudo_div_assign`, and `pseudo_rem_assign`). The shapes recognized
    are `(A).METHOD(B) = (X, d)` or `(A).METHOD(B) = (Q, R, d)`, and `p := A; p.METHOD(B) = d;
    p = X` or `p := A; p.pseudo_div_assign_rem(B) = (R, d); p = Q`, where `A` and `B` may be written
    `&(...)`. Malachite's exponent d is the least, which is what fmpz_poly_pseudo_divrem_basecase
    computes, so that must agree exactly; fmpz_poly_pseudo_divrem, fmpz_poly_pseudo_div,
    fmpz_poly_pseudo_rem, and fmpz_poly_pseudo_divrem_cohen may use a larger exponent e, and must
    give the same results times l^(e - d), where l is the leading coefficient of B. Any other
    nonempty line is an error, as is an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>
#include <flint/fmpz.h>
#include <flint/fmpz_poly.h>

#include "oracle.h"

static long checked;

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

enum op { OP_DIV_REM, OP_DIV, OP_REM };

/* Which operation a method name is, or -1. */
static int
method_op(const char * name)
{
    if (strcmp(name, "pseudo_div_rem") == 0 || strcmp(name, "pseudo_div_assign_rem") == 0)
        return OP_DIV_REM;
    if (strcmp(name, "pseudo_div") == 0 || strcmp(name, "pseudo_div_assign") == 0)
        return OP_DIV;
    if (strcmp(name, "pseudo_rem") == 0 || strcmp(name, "pseudo_rem_assign") == 0)
        return OP_REM;
    return -1;
}

/* Splits a line into A, B, the expected quotient and remainder (either may be NULL), and d. */
static int
split_line(char * line, char ** a, char ** b, char ** q, char ** r, char ** d)
{
    line[strcspn(line, "\r\n")] = '\0';
    char * method;
    char * args;
    char * result;
    int op;
    *q = NULL;
    *r = NULL;
    if (strncmp(line, "p := ", 5) == 0)
    {
        *a = line + 5;
        method = cut(*a, "; p.");
        if (method == NULL)
            return -1;
        args = cut(method, "(");
        if (args == NULL)
            return -1;
        op = method_op(method);
        result = cut(args, ") = ");
        if (op < 0 || result == NULL)
            return -1;
        char * p_value = cut(result, "; p = ");
        if (p_value == NULL)
            return -1;
        if (op == OP_DIV_REM)
        {
            /* (R, d); p = Q */
            size_t len = strlen(result);
            if (len < 2 || result[0] != '(' || result[len - 1] != ')')
                return -1;
            result[len - 1] = '\0';
            *r = result + 1;
            *d = cut(*r, ", ");
            if (*d == NULL)
                return -1;
            *q = p_value;
        }
        else
        {
            *d = result;
            if (op == OP_DIV)
                *q = p_value;
            else
                *r = p_value;
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
        op = method_op(method);
        result = cut(args, ") = ");
        if (op < 0 || result == NULL)
            return -1;
        size_t len = strlen(result);
        if (len < 2 || result[0] != '(' || result[len - 1] != ')')
            return -1;
        result[len - 1] = '\0';
        result++;
        if (op == OP_DIV_REM)
        {
            *q = result;
            *r = cut(*q, ", ");
            if (*r == NULL)
                return -1;
            *d = cut(*r, ", ");
        }
        else
        {
            char * x = result;
            *d = cut(x, ", ");
            if (op == OP_DIV)
                *q = x;
            else
                *r = x;
        }
        if (*d == NULL)
            return -1;
    }
    *b = strip_ref(args);
    return op;
}

/* Whether x = f * y. */
static int
equal_scaled(const fmpz_poly_t x, const fmpz_t f, const fmpz_poly_t y)
{
    fmpz_poly_t t;
    fmpz_poly_init(t);
    fmpz_poly_scalar_mul_fmpz(t, y, f);
    int result = fmpz_poly_equal(x, t);
    fmpz_poly_clear(t);
    return result;
}

static int
check_line(char * line, int line_number)
{
    char * a_str;
    char * b_str;
    char * q_str;
    char * r_str;
    char * d_str;
    int op = split_line(line, &a_str, &b_str, &q_str, &r_str, &d_str);
    if (op < 0)
    {
        if (line[0] == '\0')
            return 0;
        flint_printf("error in pseudo_div_rem test, line %d: unrecognized line\n", line_number);
        return 1;
    }
    checked++;
    int result = 0;
    ulong d = strtoul(d_str, NULL, 10);
    fmpz_poly_t a, b, q_expected, r_expected, q, r;
    fmpz_t f, lead;
    fmpz_poly_init(a);
    fmpz_poly_init(b);
    fmpz_poly_init(q_expected);
    fmpz_poly_init(r_expected);
    fmpz_poly_init(q);
    fmpz_poly_init(r);
    fmpz_init(f);
    fmpz_init(lead);
    if (!fmpz_poly_set_str_malachite(a, a_str) || !fmpz_poly_set_str_malachite(b, b_str)
        || (q_str != NULL && !fmpz_poly_set_str_malachite(q_expected, q_str))
        || (r_str != NULL && !fmpz_poly_set_str_malachite(r_expected, r_str))
        || fmpz_poly_is_zero(b))
    {
        flint_printf("error in pseudo_div_rem test, line %d: unreadable input\n", line_number);
        result = 1;
        goto cleanup;
    }
    fmpz_poly_get_coeff_fmpz(lead, b, b->length - 1);
    ulong e;
    /* The basecase exponent is the least; everything must match exactly. */
    fmpz_poly_pseudo_divrem_basecase(q, r, &e, a, b);
    if (e != d || (q_str != NULL && !fmpz_poly_equal(q, q_expected))
        || (r_str != NULL && !fmpz_poly_equal(r, r_expected)))
    {
        flint_printf("error in pseudo_div_rem test, line %d, by "
                     "fmpz_poly_pseudo_divrem_basecase. FLINT: d = %wu\n",
                     line_number, e);
        result = 1;
        goto cleanup;
    }
    /* The other functions agree up to a power of the leading coefficient. */
    fmpz_poly_pseudo_divrem(q, r, &e, a, b);
    fmpz_pow_ui(f, lead, e - d);
    if (e < d || (q_str != NULL && !equal_scaled(q, f, q_expected))
        || (r_str != NULL && !equal_scaled(r, f, r_expected)))
    {
        flint_printf("error in pseudo_div_rem test, line %d, by fmpz_poly_pseudo_divrem\n",
                     line_number);
        result = 1;
        goto cleanup;
    }
    if (op == OP_DIV)
    {
        fmpz_poly_pseudo_div(q, &e, a, b);
        fmpz_pow_ui(f, lead, e - d);
        if (e < d || !equal_scaled(q, f, q_expected))
        {
            flint_printf("error in pseudo_div_rem test, line %d, by fmpz_poly_pseudo_div\n",
                         line_number);
            result = 1;
            goto cleanup;
        }
    }
    if (op == OP_REM)
    {
        fmpz_poly_pseudo_rem(r, &e, a, b);
        fmpz_pow_ui(f, lead, e - d);
        if (e < d || !equal_scaled(r, f, r_expected))
        {
            flint_printf("error in pseudo_div_rem test, line %d, by fmpz_poly_pseudo_rem\n",
                         line_number);
            result = 1;
            goto cleanup;
        }
    }
    if (a->length >= b->length)
    {
        fmpz_poly_pseudo_divrem_cohen(q, r, a, b);
        e = a->length - b->length + 1;
        fmpz_pow_ui(f, lead, e - d);
        if ((q_str != NULL && !equal_scaled(q, f, q_expected))
            || (r_str != NULL && !equal_scaled(r, f, r_expected)))
        {
            flint_printf("error in pseudo_div_rem test, line %d, by "
                         "fmpz_poly_pseudo_divrem_cohen\n",
                         line_number);
            result = 1;
        }
    }
cleanup:
    fmpz_poly_clear(a);
    fmpz_poly_clear(b);
    fmpz_poly_clear(q_expected);
    fmpz_poly_clear(r_expected);
    fmpz_poly_clear(q);
    fmpz_poly_clear(r);
    fmpz_clear(f);
    fmpz_clear(lead);
    return result;
}

int
run_fmpz_poly_pseudo_divrem(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_line);
    if (result == 0)
        flint_printf("fmpz_poly_pseudo_divrem: %ld lines agree with FLINT\n", checked);
    return result != 0 ? result : require_some_lines("fmpz_poly_pseudo_divrem", checked);
}

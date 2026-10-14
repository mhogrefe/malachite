/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the RationalPolynomial composition demos against fmpq_poly_compose.
    The shapes recognized are `(P).compose(Q) = R`, with either operand written `&(P)` or `&(Q)`
    instead, and `p := P; p.compose_assign(Q); p = R` and `p := P; p.compose_assign(&(Q)); p = R`.
    Any other nonempty line is an error, as is an input with no lines at all.
*/

#include <string.h>
#include <flint/fmpq_poly.h>

#include "oracle.h"

static long checked;

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

/* Strips `&(` and the matching `)` from an operand written `&(Q)`, in place. */
static char *
strip_ref(char * operand)
{
    size_t len = strlen(operand);
    if (len >= 3 && operand[0] == '&' && operand[1] == '(' && operand[len - 1] == ')')
    {
        operand[len - 1] = '\0';
        return operand + 2;
    }
    return operand;
}

/* Splits a composition line into its two operands and its result, in place. Polynomials are
   displayed without spaces or parentheses, so the separators are unambiguous. Returns 1 on
   success. */
static int
split_compose_line(char * line, char ** p, char ** q, char ** r)
{
    line[strcspn(line, "\r\n")] = '\0';
    if (strncmp(line, "p := ", 5) == 0)
    {
        *p = line + 5;
        *q = cut(*p, "; p.compose_assign(");
        if (*q == NULL)
        {
            return 0;
        }
        *r = cut(*q, "); p = ");
        if (*r == NULL)
        {
            return 0;
        }
        *q = strip_ref(*q);
        return 1;
    }
    char * rest = cut(line, ").compose(");
    if (rest == NULL || line[0] != '(')
    {
        return 0;
    }
    *p = line + 1;
    /* A receiver written `(&(P)` has become `&(P` after the first parenthesis. */
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
    *r = cut(rest, ") = ");
    if (*r == NULL)
    {
        return 0;
    }
    *q = strip_ref(rest);
    return 1;
}

static int
report_mismatch(int line_number, const fmpq_poly_t r)
{
    flint_printf("error in fmpq_poly_compose test, line %d. FLINT: ", line_number);
    fmpq_poly_print_pretty(r, "x");
    flint_printf("\n");
    return 1;
}

static int
check_line(char * line, int line_number)
{
    char * p_str;
    char * q_str;
    char * r_str;
    if (!split_compose_line(line, &p_str, &q_str, &r_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in fmpq_poly_compose test, line %d: unrecognized line\n",
                     line_number);
        return 1;
    }
    checked++;
    int result = 0;
    fmpq_poly_t p, q, expected, r;
    fmpq_poly_init(p);
    fmpq_poly_init(q);
    fmpq_poly_init(expected);
    fmpq_poly_init(r);
    if (!fmpq_poly_set_str_malachite(p, p_str) || !fmpq_poly_set_str_malachite(q, q_str)
        || !fmpq_poly_set_str_malachite(expected, r_str))
    {
        flint_printf("error in fmpq_poly_compose test, line %d: unreadable input\n", line_number);
        result = 1;
    }
    else
    {
        fmpq_poly_compose(r, p, q);
        if (!fmpq_poly_equal(r, expected))
        {
            result = report_mismatch(line_number, r);
        }
    }
    fmpq_poly_clear(p);
    fmpq_poly_clear(q);
    fmpq_poly_clear(expected);
    fmpq_poly_clear(r);
    return result;
}

int
run_fmpq_poly_compose(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_line);
    if (result == 0)
    {
        flint_printf("fmpq_poly_compose: %ld lines\n", checked);
    }
    return result != 0 ? result : require_some_lines("fmpq_poly_compose", checked);
}

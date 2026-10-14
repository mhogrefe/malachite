/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the IntegerPolynomial composition demos against fmpz_poly_compose,
    fmpz_poly_compose_horner, and fmpz_poly_compose_divconquer. The shapes recognized are
    `(P).compose(Q) = R`, with either operand written `&(P)` or `&(Q)` instead, `p := P;
    p.compose_assign(Q); p = R` and `p := P; p.compose_assign(&(Q)); p = R`, and
    `compose_horner(P, Q) = R` and `compose_divide_and_conquer(P, Q) = R`. Any other nonempty
    line is an error, as is an input with no lines at all.
*/

#include <string.h>
#include <flint/fmpz_poly.h>

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
    if (strncmp(line, "compose_horner(", 15) == 0
        || strncmp(line, "compose_divide_and_conquer(", 27) == 0)
    {
        *p = strchr(line, '(') + 1;
        *q = cut(*p, ", ");
        if (*q == NULL)
        {
            return 0;
        }
        *r = cut(*q, ") = ");
        return *r != NULL;
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
report_mismatch(const char * name, int line_number, const fmpz_poly_t r)
{
    flint_printf("error in fmpz_poly_compose test, line %d, by %s. FLINT: ", line_number, name);
    fmpz_poly_print_pretty(r, "x");
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
        flint_printf("error in fmpz_poly_compose test, line %d: unrecognized line\n",
                     line_number);
        return 1;
    }
    checked++;
    int result = 0;
    fmpz_poly_t p, q, expected, r;
    fmpz_poly_init(p);
    fmpz_poly_init(q);
    fmpz_poly_init(expected);
    fmpz_poly_init(r);
    if (!fmpz_poly_set_str_malachite(p, p_str) || !fmpz_poly_set_str_malachite(q, q_str)
        || !fmpz_poly_set_str_malachite(expected, r_str))
    {
        flint_printf("error in fmpz_poly_compose test, line %d: unreadable input\n", line_number);
        result = 1;
    }
    else
    {
        fmpz_poly_compose(r, p, q);
        if (!fmpz_poly_equal(r, expected))
        {
            result = report_mismatch("fmpz_poly_compose", line_number, r);
        }
        fmpz_poly_compose_horner(r, p, q);
        if (result == 0 && !fmpz_poly_equal(r, expected))
        {
            result = report_mismatch("fmpz_poly_compose_horner", line_number, r);
        }
        fmpz_poly_compose_divconquer(r, p, q);
        if (result == 0 && !fmpz_poly_equal(r, expected))
        {
            result = report_mismatch("fmpz_poly_compose_divconquer", line_number, r);
        }
    }
    fmpz_poly_clear(p);
    fmpz_poly_clear(q);
    fmpz_poly_clear(expected);
    fmpz_poly_clear(r);
    return result;
}

int
run_fmpz_poly_compose(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_line);
    if (result == 0)
    {
        flint_printf("fmpz_poly_compose: %ld lines, each by fmpz_poly_compose, "
                     "fmpz_poly_compose_horner, and fmpz_poly_compose_divconquer\n",
                     checked);
    }
    return result != 0 ? result : require_some_lines("fmpz_poly_compose", checked);
}

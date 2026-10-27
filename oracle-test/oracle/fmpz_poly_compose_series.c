/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the IntegerPolynomial power series composition demos against
    fmpz_poly_compose_series, fmpz_poly_compose_series_horner, and
    fmpz_poly_compose_series_brent_kung. The shapes recognized are `(P).compose_truncated(Q, n) =
    R`, with either operand written `&(P)` or `&(Q)` instead, and `p := P;
    p.compose_truncated_assign(Q, n); p = R`, with `Q` possibly written `&(Q)`. Any other nonempty
    line is an error, as is an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>
#include <flint/fmpz_poly.h>

#include "oracle.h"

static long checked;

/* Cuts `line` at the last occurrence of `needle`, returning the text after it, or NULL if the
   needle is absent. */
static char *
cut_last(char * line, const char * needle)
{
    char * at = NULL;
    for (char * s = strstr(line, needle); s != NULL; s = strstr(s + 1, needle))
    {
        at = s;
    }
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

/* Splits a line into its operands, its length, and its result, in place. Polynomials are displayed
   without spaces or parentheses, so the separators are unambiguous. Returns 1 on success. */
static int
split_line(char * line, char ** p, char ** q, char ** n, char ** r)
{
    line[strcspn(line, "\r\n")] = '\0';
    char * args;
    if (strncmp(line, "p := ", 5) == 0)
    {
        *p = line + 5;
        args = strstr(*p, "; p.compose_truncated_assign(");
        if (args == NULL)
        {
            return 0;
        }
        *args = '\0';
        args += strlen("; p.compose_truncated_assign(");
        *r = cut_last(args, "); p = ");
    }
    else
    {
        if (line[0] != '(')
        {
            return 0;
        }
        args = strstr(line, ").compose_truncated(");
        if (args == NULL)
        {
            return 0;
        }
        *args = '\0';
        args += strlen(").compose_truncated(");
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
        *r = cut_last(args, ") = ");
    }
    if (*r == NULL)
    {
        return 0;
    }
    *n = cut_last(args, ", ");
    if (*n == NULL)
    {
        return 0;
    }
    *q = strip_ref(args);
    return 1;
}

static int
report_mismatch(const char * name, int line_number, const fmpz_poly_t r)
{
    flint_printf("error in fmpz_poly_compose_series test, line %d, by %s. FLINT: ", line_number,
                 name);
    fmpz_poly_print_pretty(r, "x");
    flint_printf("\n");
    return 1;
}

static int
check_line(char * line, int line_number)
{
    char * p_str;
    char * q_str;
    char * n_str;
    char * r_str;
    if (!split_line(line, &p_str, &q_str, &n_str, &r_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in fmpz_poly_compose_series test, line %d: unrecognized line\n",
                     line_number);
        return 1;
    }
    checked++;
    int result = 0;
    char * end;
    long n = strtol(n_str, &end, 10);
    fmpz_poly_t p, q, expected, r;
    fmpz_poly_init(p);
    fmpz_poly_init(q);
    fmpz_poly_init(expected);
    fmpz_poly_init(r);
    if (end == n_str || *end != '\0' || !fmpz_poly_set_str_malachite(p, p_str)
        || !fmpz_poly_set_str_malachite(q, q_str)
        || !fmpz_poly_set_str_malachite(expected, r_str))
    {
        flint_printf("error in fmpz_poly_compose_series test, line %d: unreadable input\n",
                     line_number);
        result = 1;
    }
    else
    {
        fmpz_poly_compose_series(r, p, q, n);
        if (!fmpz_poly_equal(r, expected))
        {
            result = report_mismatch("fmpz_poly_compose_series", line_number, r);
        }
        fmpz_poly_compose_series_horner(r, p, q, n);
        if (result == 0 && !fmpz_poly_equal(r, expected))
        {
            result = report_mismatch("fmpz_poly_compose_series_horner", line_number, r);
        }
        fmpz_poly_compose_series_brent_kung(r, p, q, n);
        if (result == 0 && !fmpz_poly_equal(r, expected))
        {
            result = report_mismatch("fmpz_poly_compose_series_brent_kung", line_number, r);
        }
    }
    fmpz_poly_clear(p);
    fmpz_poly_clear(q);
    fmpz_poly_clear(expected);
    fmpz_poly_clear(r);
    return result;
}

int
run_fmpz_poly_compose_series(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_line);
    if (result == 0)
    {
        flint_printf("fmpz_poly_compose_series: %ld lines, each by fmpz_poly_compose_series, "
                     "fmpz_poly_compose_series_horner, and fmpz_poly_compose_series_brent_kung\n",
                     checked);
    }
    return result != 0 ? result : require_some_lines("fmpz_poly_compose_series", checked);
}

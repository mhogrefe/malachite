/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the IntegerPolynomial multiplication and squaring demos against
    fmpz_poly_mul, fmpz_poly_mullow, fmpz_poly_sqr, and fmpz_poly_sqrlow.

    fmpz_poly_mul: `(P) * (Q) = R`, `(P) * &(Q) = R`, `&(P) * (Q) = R`, `&(P) * &(Q) = R`,
    `p := P; p *= Q; p = R`, and `p := P; p *= &Q; p = R`. When the two operands are written
    identically, FLINT is given the same polynomial for both, so that its squaring path runs too.

    fmpz_poly_mullow: `(P).mul_truncated(Q, n) = R` and the other shapes of the add_series mode.

    fmpz_poly_sqr: `(P).square() = R`, `(&(P)).square() = R`, and
    `p := P; p.square_assign(); p = R`.

    fmpz_poly_sqrlow: `(P).square_truncated(n) = R`, `(&(P)).square_truncated(n) = R`, and
    `p := P; p.square_truncated_assign(n); p = R`.

    Each mode treats any other nonempty line as an error, and an input with no lines at all.
*/

#include <stdio.h>
#include <stdlib.h>
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

/* Strips a leading `&(` or `(` and the matching trailing `)` from an operand written as
   `(P)` or `&(P)`, in place. Returns NULL if it has neither form. */
static char *
unwrap(char * operand)
{
    if (operand[0] == '&')
    {
        operand++;
    }
    size_t len = strlen(operand);
    if (len < 2 || operand[0] != '(' || operand[len - 1] != ')')
    {
        return NULL;
    }
    operand[len - 1] = '\0';
    return operand + 1;
}

/* Splits a product line into its two operands and its result, in place. Returns 1 on success. */
static int
split_product_line(char * line, char ** p, char ** q, char ** r)
{
    line[strcspn(line, "\r\n")] = '\0';
    if (strncmp(line, "p := ", 5) == 0)
    {
        *p = line + 5;
        *q = cut(*p, "; p *= ");
        if (*q == NULL)
        {
            return 0;
        }
        *r = cut(*q, "; p = ");
        if (**q == '&')
        {
            (*q)++;
        }
        return *r != NULL;
    }
    /* Polynomials are displayed without spaces, so `) * ` and ` = ` are unambiguous. */
    char * rest = cut(line, ") * ");
    if (rest == NULL)
    {
        return 0;
    }
    *r = cut(rest, " = ");
    if (*r == NULL)
    {
        return 0;
    }
    /* `cut` removed the operand's closing parenthesis along with the operator. */
    size_t len = strlen(line);
    line[len] = ')';
    line[len + 1] = '\0';
    *p = unwrap(line);
    *q = unwrap(rest);
    return *p != NULL && *q != NULL;
}

static int
report_unrecognized(char * line, const char * name, int line_number)
{
    if (line[0] == '\0')
    {
        return 0;
    }
    flint_printf("error in %s test, line %d: unrecognized line\n", name, line_number);
    return 1;
}

static int
report_mismatch(const char * name, int line_number, const fmpz_poly_t r)
{
    flint_printf("error in %s test, line %d. FLINT: ", name, line_number);
    fmpz_poly_print_pretty(r, "x");
    flint_printf("\n");
    return 1;
}

static int
check_mul_line(char * line, int line_number)
{
    char * p_str;
    char * q_str;
    char * r_str;
    if (!split_product_line(line, &p_str, &q_str, &r_str))
    {
        return report_unrecognized(line, "fmpz_poly_mul", line_number);
    }
    checked++;
    int result = 0;
    int same = strcmp(p_str, q_str) == 0;
    fmpz_poly_t p, q, expected, r;
    fmpz_poly_init(p);
    fmpz_poly_init(q);
    fmpz_poly_init(expected);
    fmpz_poly_init(r);
    if (!fmpz_poly_set_str_malachite(p, p_str) || !fmpz_poly_set_str_malachite(q, q_str)
        || !fmpz_poly_set_str_malachite(expected, r_str))
    {
        flint_printf("error in fmpz_poly_mul test, line %d: unreadable input\n", line_number);
        result = 1;
    }
    else
    {
        fmpz_poly_mul(r, p, same ? p : q);
        if (!fmpz_poly_equal(r, expected))
        {
            result = report_mismatch("fmpz_poly_mul", line_number, r);
        }
    }
    fmpz_poly_clear(p);
    fmpz_poly_clear(q);
    fmpz_poly_clear(expected);
    fmpz_poly_clear(r);
    return result;
}

int
run_fmpz_poly_mul(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_mul_line);
    return result != 0 ? result : require_some_lines("fmpz_poly_mul", checked);
}

static int
check_mullow_line(char * line, int line_number)
{
    char * p_str;
    char * arg;
    char * r_str;
    if (!split_polynomial_scalar_line(line, "mul_truncated", "mul_truncated_assign", NULL, &p_str,
                                      &arg, &r_str))
    {
        return report_unrecognized(line, "fmpz_poly_mullow", line_number);
    }
    /* The argument is `Q, n` or `&(Q), n`. Polynomials are displayed without spaces, so the last
       ", " separates the two. */
    char * comma = strrchr(arg, ',');
    if (comma == NULL || comma[1] != ' ')
    {
        flint_printf("error in fmpz_poly_mullow test, line %d: unrecognized argument\n",
                     line_number);
        return 1;
    }
    *comma = '\0';
    char * n_str = comma + 2;
    char * q_str = arg;
    if (strncmp(q_str, "&(", 2) == 0)
    {
        q_str = unwrap(q_str);
        if (q_str == NULL)
        {
            flint_printf("error in fmpz_poly_mullow test, line %d: unrecognized argument\n",
                         line_number);
            return 1;
        }
    }
    char * end;
    unsigned long long n = strtoull(n_str, &end, 10);
    checked++;
    int result = 0;
    int same = strcmp(p_str, q_str) == 0;
    fmpz_poly_t p, q, expected, r;
    fmpz_poly_init(p);
    fmpz_poly_init(q);
    fmpz_poly_init(expected);
    fmpz_poly_init(r);
    if (*end != '\0' || n > WORD_MAX || !fmpz_poly_set_str_malachite(p, p_str)
        || !fmpz_poly_set_str_malachite(q, q_str)
        || !fmpz_poly_set_str_malachite(expected, r_str))
    {
        flint_printf("error in fmpz_poly_mullow test, line %d: unreadable input\n", line_number);
        result = 1;
    }
    else
    {
        fmpz_poly_mullow(r, p, same ? p : q, (slong) n);
        if (!fmpz_poly_equal(r, expected))
        {
            result = report_mismatch("fmpz_poly_mullow", line_number, r);
        }
    }
    fmpz_poly_clear(p);
    fmpz_poly_clear(q);
    fmpz_poly_clear(expected);
    fmpz_poly_clear(r);
    return result;
}

int
run_fmpz_poly_mullow(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_mullow_line);
    return result != 0 ? result : require_some_lines("fmpz_poly_mullow", checked);
}

/* Checks a squaring line; `truncated` selects fmpz_poly_sqrlow, whose argument is the length. */
static int
check_square_line(char * line, int line_number, int truncated)
{
    const char * name = truncated ? "fmpz_poly_sqrlow" : "fmpz_poly_sqr";
    char * p_str;
    char * arg;
    char * r_str;
    if (!split_polynomial_scalar_line(line, truncated ? "square_truncated" : "square",
                                      truncated ? "square_truncated_assign" : "square_assign",
                                      NULL, &p_str, &arg, &r_str))
    {
        return report_unrecognized(line, name, line_number);
    }
    char * end = arg;
    unsigned long long n = 0;
    if (truncated)
    {
        n = strtoull(arg, &end, 10);
    }
    checked++;
    int result = 0;
    fmpz_poly_t p, expected, r;
    fmpz_poly_init(p);
    fmpz_poly_init(expected);
    fmpz_poly_init(r);
    if (*end != '\0' || (truncated && end == arg) || n > WORD_MAX
        || !fmpz_poly_set_str_malachite(p, p_str)
        || !fmpz_poly_set_str_malachite(expected, r_str))
    {
        flint_printf("error in %s test, line %d: unreadable input\n", name, line_number);
        result = 1;
    }
    else
    {
        if (truncated)
        {
            fmpz_poly_sqrlow(r, p, (slong) n);
        }
        else
        {
            fmpz_poly_sqr(r, p);
        }
        if (!fmpz_poly_equal(r, expected))
        {
            result = report_mismatch(name, line_number, r);
        }
    }
    fmpz_poly_clear(p);
    fmpz_poly_clear(expected);
    fmpz_poly_clear(r);
    return result;
}

static int
check_sqr_line(char * line, int line_number)
{
    return check_square_line(line, line_number, 0);
}

static int
check_sqrlow_line(char * line, int line_number)
{
    return check_square_line(line, line_number, 1);
}

int
run_fmpz_poly_sqr(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_sqr_line);
    return result != 0 ? result : require_some_lines("fmpz_poly_sqr", checked);
}

int
run_fmpz_poly_sqrlow(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_sqrlow_line);
    return result != 0 ? result : require_some_lines("fmpz_poly_sqrlow", checked);
}

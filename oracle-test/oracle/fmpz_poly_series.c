/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the IntegerPolynomial power series inversion and division demos
    against FLINT.

    fmpz_poly_inv_series: `(P).inverse_truncated(n) = Some(R)` or `= None`, and the same with
    `(&(P))`. The inverse must exist exactly when the constant coefficient of P is 1 or -1, where it
    must equal fmpz_poly_inv_series, fmpz_poly_inv_series_basecase, and
    fmpz_poly_inv_series_newton; FLINT does not check the constant coefficient itself. For n = 0
    the result must be 0, where FLINT throws.

    fmpz_poly_div_series: `(P).div_truncated(Q, n) = R`, `(P).checked_div_truncated(Q, n) =
    Some(R)` or `= None`, `p := P; p.div_truncated_assign(Q, n); p = R`, with `&(...)` allowed
    around either operand. FLINT throws where the quotient is not integral, so the quotient is
    first found over the rationals by fmpq_poly_div_series: `None` must be exactly where the
    divisor's constant coefficient is 0 or that quotient is not integral, and otherwise the result
    must equal fmpz_poly_div_series, fmpz_poly_div_series_basecase, and
    fmpz_poly_div_series_divconquer. For n = 0 the result must be 0, where FLINT throws.

    Each mode treats any other nonempty line as an error, and an input with no lines at all.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <flint/fmpz_poly.h>
#include <flint/fmpq_poly.h>
#include "oracle.h"

static long checked;

/* Strips a leading `&(` or `(` and the matching trailing `)` from an operand written as `(P)` or
   `&(P)`, in place, or returns it unchanged if it has neither form. */
static char *
unwrap_operand(char * operand)
{
    if (operand[0] == '&')
    {
        operand++;
    }
    size_t len = strlen(operand);
    if (len >= 2 && operand[0] == '(' && operand[len - 1] == ')')
    {
        operand[len - 1] = '\0';
        return operand + 1;
    }
    return operand;
}

/* Splits `Some(R)` or `None` in place: returns 1 and sets `*r` to R, returns 0 for `None`, and
   returns -1 for anything else. */
static int
split_option(char * s, char ** r)
{
    if (strcmp(s, "None") == 0)
    {
        return 0;
    }
    size_t len = strlen(s);
    if (len >= 6 && strncmp(s, "Some(", 5) == 0 && s[len - 1] == ')')
    {
        s[len - 1] = '\0';
        *r = s + 5;
        return 1;
    }
    return -1;
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
report_mismatch(const char * name, const char * what, int line_number, const fmpz_poly_t r)
{
    flint_printf("error in %s test, line %d. %s: ", name, line_number, what);
    fmpz_poly_print_pretty(r, "x");
    flint_printf("\n");
    return 1;
}

static int
check_inv_line(char * line, int line_number)
{
    const char * name = "fmpz_poly_inv_series";
    char * p_str;
    char * n_str;
    char * r_str;
    if (!split_polynomial_scalar_line(line, "inverse_truncated", NULL, NULL, &p_str, &n_str,
                                      &r_str))
    {
        return report_unrecognized(line, name, line_number);
    }
    char * end;
    unsigned long long n = strtoull(n_str, &end, 10);
    char * expected_str = NULL;
    int some = split_option(r_str, &expected_str);
    fmpz_poly_t p, expected, r;
    fmpz_poly_init(p);
    fmpz_poly_init(expected);
    fmpz_poly_init(r);
    int result = 0;
    checked++;
    if (*end != '\0' || n > WORD_MAX || some < 0 || !fmpz_poly_set_str_malachite(p, p_str)
        || (some && !fmpz_poly_set_str_malachite(expected, expected_str)))
    {
        flint_printf("error in %s test, line %d: unreadable input\n", name, line_number);
        result = 1;
    }
    else if (n == 0)
    {
        if (!some || !fmpz_poly_is_zero(expected))
        {
            flint_printf("error in %s test, line %d: expected Some(0)\n", name, line_number);
            result = 1;
        }
    }
    else if (p->length == 0 || !fmpz_is_pm1(p->coeffs + 0))
    {
        if (some)
        {
            flint_printf("error in %s test, line %d: expected None\n", name, line_number);
            result = 1;
        }
    }
    else if (!some)
    {
        flint_printf("error in %s test, line %d: expected an inverse\n", name, line_number);
        result = 1;
    }
    else
    {
        fmpz_poly_inv_series(r, p, (slong) n);
        if (!fmpz_poly_equal(r, expected))
        {
            result = report_mismatch(name, "fmpz_poly_inv_series", line_number, r);
        }
        fmpz_poly_inv_series_basecase(r, p, (slong) n);
        if (!result && !fmpz_poly_equal(r, expected))
        {
            result = report_mismatch(name, "fmpz_poly_inv_series_basecase", line_number, r);
        }
        fmpz_poly_inv_series_newton(r, p, (slong) n);
        if (!result && !fmpz_poly_equal(r, expected))
        {
            result = report_mismatch(name, "fmpz_poly_inv_series_newton", line_number, r);
        }
    }
    fmpz_poly_clear(p);
    fmpz_poly_clear(expected);
    fmpz_poly_clear(r);
    return result;
}

int
run_fmpz_poly_inv_series(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_inv_line);
    return result != 0 ? result : require_some_lines("fmpz_poly_inv_series", checked);
}

/* Whether the quotient of `a` by `b` as power series modulo x^n, where `b` has a nonzero constant
   coefficient, has integer coefficients. */
static int
div_series_is_integral(const fmpz_poly_t a, const fmpz_poly_t b, slong n)
{
    fmpq_poly_t qa, qb, q;
    fmpq_poly_init(qa);
    fmpq_poly_init(qb);
    fmpq_poly_init(q);
    fmpq_poly_set_fmpz_poly(qa, a);
    fmpq_poly_set_fmpz_poly(qb, b);
    fmpq_poly_div_series(q, qa, qb, n);
    int integral = fmpz_is_one(q->den);
    fmpq_poly_clear(qa);
    fmpq_poly_clear(qb);
    fmpq_poly_clear(q);
    return integral;
}

static int
check_div_line(char * line, int line_number)
{
    const char * name = "fmpz_poly_div_series";
    char * p_str;
    char * arg;
    char * r_str;
    int checked_form = 0;
    if (strstr(line, ").checked_div_truncated(") != NULL)
    {
        checked_form = 1;
        if (!split_polynomial_scalar_line(line, "checked_div_truncated", NULL, NULL, &p_str, &arg,
                                          &r_str))
        {
            return report_unrecognized(line, name, line_number);
        }
    }
    else if (!split_polynomial_scalar_line(line, "div_truncated", "div_truncated_assign", NULL,
                                           &p_str, &arg, &r_str))
    {
        return report_unrecognized(line, name, line_number);
    }
    /* The argument is `Q, n` or `&(Q), n`. Polynomials are displayed without spaces, so the last
       ", " separates the two. */
    char * comma = strrchr(arg, ',');
    if (comma == NULL || comma[1] != ' ')
    {
        flint_printf("error in %s test, line %d: unrecognized argument\n", name, line_number);
        return 1;
    }
    *comma = '\0';
    char * n_str = comma + 2;
    char * q_str = unwrap_operand(arg);
    char * expected_str = r_str;
    int some = checked_form ? split_option(r_str, &expected_str) : 1;
    char * end;
    unsigned long long n = strtoull(n_str, &end, 10);
    fmpz_poly_t p, q, expected, r;
    fmpz_poly_init(p);
    fmpz_poly_init(q);
    fmpz_poly_init(expected);
    fmpz_poly_init(r);
    int result = 0;
    checked++;
    if (*end != '\0' || n > WORD_MAX || some < 0 || !fmpz_poly_set_str_malachite(p, p_str)
        || !fmpz_poly_set_str_malachite(q, q_str)
        || (some && !fmpz_poly_set_str_malachite(expected, expected_str)))
    {
        flint_printf("error in %s test, line %d: unreadable input\n", name, line_number);
        result = 1;
    }
    else if (n == 0)
    {
        if (!some || !fmpz_poly_is_zero(expected))
        {
            flint_printf("error in %s test, line %d: expected 0\n", name, line_number);
            result = 1;
        }
    }
    else if (q->length == 0 || fmpz_is_zero(q->coeffs + 0)
             || !div_series_is_integral(p, q, (slong) n))
    {
        if (some)
        {
            flint_printf("error in %s test, line %d: expected no quotient\n", name, line_number);
            result = 1;
        }
    }
    else if (!some)
    {
        flint_printf("error in %s test, line %d: expected a quotient\n", name, line_number);
        result = 1;
    }
    else
    {
        fmpz_poly_div_series(r, p, q, (slong) n);
        if (!fmpz_poly_equal(r, expected))
        {
            result = report_mismatch(name, "fmpz_poly_div_series", line_number, r);
        }
        fmpz_poly_div_series_basecase(r, p, q, (slong) n);
        if (!result && !fmpz_poly_equal(r, expected))
        {
            result = report_mismatch(name, "fmpz_poly_div_series_basecase", line_number, r);
        }
        fmpz_poly_div_series_divconquer(r, p, q, (slong) n);
        if (!result && !fmpz_poly_equal(r, expected))
        {
            result = report_mismatch(name, "fmpz_poly_div_series_divconquer", line_number, r);
        }
    }
    fmpz_poly_clear(p);
    fmpz_poly_clear(q);
    fmpz_poly_clear(expected);
    fmpz_poly_clear(r);
    return result;
}

int
run_fmpz_poly_div_series(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_div_line);
    return result != 0 ? result : require_some_lines("fmpz_poly_div_series", checked);
}

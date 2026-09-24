/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `(&(p)).evaluate(x) = r`, `(&(p)).evaluate_horner(x) = r`, and
    `(&(p)).evaluate_divide_and_conquer(x) = r` lines, where `p` is an integer polynomial and `x`
    and `r` integers, against fmpz_poly_evaluate_fmpz, fmpz_poly_evaluate_horner_fmpz, and
    fmpz_poly_evaluate_divconquer_fmpz respectively. The last two check Malachite's translations of
    the two algorithms, which are not public, against the functions they translate; the first
    checks the public evaluation, including its choice between them. Each mode treats any other
    nonempty line as an error, and an input with no lines at all.

    The `_fmpq` modes are the same with `x` and `r` rationals, such as `-2/3`, diffed against
    fmpz_poly_evaluate_fmpq, fmpz_poly_evaluate_horner_fmpq, and fmpz_poly_evaluate_divconquer_fmpq.

    The `fmpq_poly_evaluate_fmpq` mode is the same again with `p` a rational polynomial, and
    `fmpq_poly_evaluate_fmpz` the same with `x` an integer.
*/

#include <string.h>

#include <flint/fmpq.h>
#include <flint/fmpq_poly.h>
#include <flint/fmpz_poly.h>

#include "oracle.h"

typedef void (* evaluator)(fmpz_t, const fmpz_poly_t, const fmpz_t);

static long checked;
static const char * current_method;
static const char * current_name;
static evaluator current_evaluator;

static int
check_evaluate_line(char * line, int line_number)
{
    char * receiver;
    char * point;
    char * expected_str;
    if (!split_polynomial_scalar_line(line, current_method, NULL, NULL, &receiver, &point,
                                      &expected_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in %s test, line %d: unrecognized line\n", current_name,
                     line_number);
        return 1;
    }
    checked++;
    int result = 0;
    fmpz_poly_t p;
    fmpz_t x, expected, r;
    fmpz_poly_init(p);
    fmpz_init(x);
    fmpz_init(expected);
    fmpz_init(r);
    if (!fmpz_poly_set_str_malachite(p, receiver) || fmpz_set_str(x, point, 10) != 0
        || fmpz_set_str(expected, expected_str, 10) != 0)
    {
        flint_printf("error in %s test, line %d: unreadable input\n", current_name,
                     line_number);
        result = 1;
    }
    else
    {
        current_evaluator(r, p, x);
        if (!fmpz_equal(r, expected))
        {
            flint_printf("error in %s test, line %d. FLINT: ", current_name, line_number);
            fmpz_print(r);
            flint_printf("\n");
            result = 1;
        }
    }
    fmpz_poly_clear(p);
    fmpz_clear(x);
    fmpz_clear(expected);
    fmpz_clear(r);
    return result;
}

static int
run_evaluate(const char * arg, const char * method, const char * name, evaluator f)
{
    checked = 0;
    current_method = method;
    current_name = name;
    current_evaluator = f;
    int result = for_each_line(arg, check_evaluate_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_fmpz_poly_evaluate_fmpz(const char * arg)
{
    return run_evaluate(arg, "evaluate", "fmpz_poly_evaluate_fmpz", fmpz_poly_evaluate_fmpz);
}

int
run_fmpz_poly_evaluate_horner_fmpz(const char * arg)
{
    return run_evaluate(arg, "evaluate_horner", "fmpz_poly_evaluate_horner_fmpz",
                        fmpz_poly_evaluate_horner_fmpz);
}

int
run_fmpz_poly_evaluate_divconquer_fmpz(const char * arg)
{
    return run_evaluate(arg, "evaluate_divide_and_conquer", "fmpz_poly_evaluate_divconquer_fmpz",
                        fmpz_poly_evaluate_divconquer_fmpz);
}

typedef void (* rational_evaluator)(fmpq_t, const fmpz_poly_t, const fmpq_t);

static rational_evaluator current_rational_evaluator;

static int
check_evaluate_fmpq_line(char * line, int line_number)
{
    char * receiver;
    char * point;
    char * expected_str;
    if (!split_polynomial_scalar_line(line, current_method, NULL, NULL, &receiver, &point,
                                      &expected_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in %s test, line %d: unrecognized line\n", current_name,
                     line_number);
        return 1;
    }
    checked++;
    int result = 0;
    fmpz_poly_t p;
    fmpq_t x, expected, r;
    fmpz_poly_init(p);
    fmpq_init(x);
    fmpq_init(expected);
    fmpq_init(r);
    if (!fmpz_poly_set_str_malachite(p, receiver) || fmpq_set_str(x, point, 10) != 0
        || fmpq_set_str(expected, expected_str, 10) != 0)
    {
        flint_printf("error in %s test, line %d: unreadable input\n", current_name,
                     line_number);
        result = 1;
    }
    else
    {
        /* Malachite prints rationals in lowest terms, so what was read must already be canonical;
           otherwise the comparison below would pass for an unreduced result. */
        fmpq_t canonical;
        fmpq_init(canonical);
        fmpq_set(canonical, expected);
        fmpq_canonicalise(canonical);
        if (!fmpq_equal(canonical, expected))
        {
            flint_printf("error in %s test, line %d: result not in lowest terms\n",
                         current_name, line_number);
            result = 1;
        }
        fmpq_clear(canonical);
        current_rational_evaluator(r, p, x);
        if (result == 0 && !fmpq_equal(r, expected))
        {
            flint_printf("error in %s test, line %d. FLINT: ", current_name, line_number);
            fmpq_print(r);
            flint_printf("\n");
            result = 1;
        }
    }
    fmpz_poly_clear(p);
    fmpq_clear(x);
    fmpq_clear(expected);
    fmpq_clear(r);
    return result;
}

static int
run_evaluate_fmpq(const char * arg, const char * method, const char * name, rational_evaluator f)
{
    checked = 0;
    current_method = method;
    current_name = name;
    current_rational_evaluator = f;
    int result = for_each_line(arg, check_evaluate_fmpq_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_fmpz_poly_evaluate_fmpq(const char * arg)
{
    return run_evaluate_fmpq(arg, "evaluate", "fmpz_poly_evaluate_fmpq", fmpz_poly_evaluate_fmpq);
}

int
run_fmpz_poly_evaluate_horner_fmpq(const char * arg)
{
    return run_evaluate_fmpq(arg, "evaluate_horner", "fmpz_poly_evaluate_horner_fmpq",
                             fmpz_poly_evaluate_horner_fmpq);
}

int
run_fmpz_poly_evaluate_divconquer_fmpq(const char * arg)
{
    return run_evaluate_fmpq(arg, "evaluate_divide_and_conquer",
                             "fmpz_poly_evaluate_divconquer_fmpq",
                             fmpz_poly_evaluate_divconquer_fmpq);
}

/* Whether the point is an integer, for fmpq_poly_evaluate_fmpz, or a rational. */
static int point_is_integer;

static int
check_fmpq_poly_evaluate_line(char * line, int line_number)
{
    char * receiver;
    char * point;
    char * expected_str;
    if (!split_polynomial_scalar_line(line, "evaluate", NULL, NULL, &receiver, &point,
                                      &expected_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in %s test, line %d: unrecognized line\n", current_name,
                     line_number);
        return 1;
    }
    checked++;
    int result = 0;
    fmpq_poly_t p;
    fmpq_t x, expected, r, canonical;
    fmpz_t n;
    fmpq_poly_init(p);
    fmpz_init(n);
    fmpq_init(x);
    fmpq_init(expected);
    fmpq_init(r);
    fmpq_init(canonical);
    int point_ok = point_is_integer ? fmpz_set_str(n, point, 10) == 0
                                    : fmpq_set_str(x, point, 10) == 0;
    if (!fmpq_poly_set_str_malachite(p, receiver) || !point_ok
        || fmpq_set_str(expected, expected_str, 10) != 0)
    {
        flint_printf("error in %s test, line %d: unreadable input\n", current_name, line_number);
        result = 1;
    }
    else
    {
        fmpq_set(canonical, expected);
        fmpq_canonicalise(canonical);
        if (!fmpq_equal(canonical, expected))
        {
            flint_printf("error in %s test, line %d: result not in lowest terms\n", current_name,
                         line_number);
            result = 1;
        }
        else
        {
            if (point_is_integer)
            {
                fmpq_poly_evaluate_fmpz(r, p, n);
            }
            else
            {
                fmpq_poly_evaluate_fmpq(r, p, x);
            }
            if (!fmpq_equal(r, expected))
            {
                flint_printf("error in %s test, line %d. FLINT: ", current_name, line_number);
                fmpq_print(r);
                flint_printf("\n");
                result = 1;
            }
        }
    }
    fmpq_poly_clear(p);
    fmpz_clear(n);
    fmpq_clear(x);
    fmpq_clear(expected);
    fmpq_clear(r);
    fmpq_clear(canonical);
    return result;
}

static int
run_fmpq_poly_evaluate(const char * arg, int integer, const char * name)
{
    checked = 0;
    point_is_integer = integer;
    current_name = name;
    int result = for_each_line(arg, check_fmpq_poly_evaluate_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_fmpq_poly_evaluate_fmpq(const char * arg)
{
    return run_fmpq_poly_evaluate(arg, 0, "fmpq_poly_evaluate_fmpq");
}

int
run_fmpq_poly_evaluate_fmpz(const char * arg)
{
    return run_fmpq_poly_evaluate(arg, 1, "fmpq_poly_evaluate_fmpz");
}

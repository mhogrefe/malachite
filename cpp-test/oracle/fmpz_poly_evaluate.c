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
*/

#include <string.h>

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

/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `Rational::reconstruct(a, m) = ...` lines against fmpq_reconstruct_fmpz, and
    `Rational::reconstruct_with_bounds(a, m, N, D) = ...` lines against fmpq_reconstruct_fmpz_2.
    The result is either `None` or `Some(q)`; both the success flag and the value must agree.
*/

#include <string.h>

#include <flint/fmpq.h>

#include "oracle.h"

/* parses "x, " or "x)" starting at *line, writing x and advancing *line past the separator */
static int
parse_fmpz_arg(char ** line, fmpz_t x)
{
    char * end = *line;
    while (*end != '\0' && *end != ',' && *end != ')')
    {
        end++;
    }
    if (*end == '\0')
    {
        return 0;
    }
    char saved = *end;
    *end = '\0';
    if (fmpz_set_str(x, *line, 10) != 0)
    {
        return 0;
    }
    *line = end + (saved == ',' ? 2 : 1);
    return 1;
}

/* parses " = None" or " = Some(q)" in place; returns 0 on parse failure */
static int
parse_result(char * rest, int * success, fmpq_t q)
{
    if (strncmp(rest, " = None", 7) == 0)
    {
        *success = 0;
        return 1;
    }
    if (strncmp(rest, " = Some(", 8) != 0)
    {
        return 0;
    }
    char * q_str = rest + 8;
    char * close = strrchr(q_str, ')');
    if (close == NULL)
    {
        return 0;
    }
    *close = '\0';
    *success = 1;
    return fmpq_set_str(q, q_str, 10) == 0;
}

static int
check_reconstruct_line(char * line, int line_number)
{
    fmpq_t expected, actual;
    fmpz_t a, m;
    fmpq_init(expected);
    fmpq_init(actual);
    fmpz_init(a);
    fmpz_init(m);
    int result = 0;
    char * start = strstr(line, "Rational::reconstruct(");
    if (start == NULL)
    {
        start = strstr(line, "Rational::reconstruct_ref(");
        if (start != NULL)
        {
            start += strlen("Rational::reconstruct_ref(");
        }
    }
    else
    {
        start += strlen("Rational::reconstruct(");
    }
    if (start != NULL)
    {
        int expected_success;
        if (!parse_fmpz_arg(&start, a) || !parse_fmpz_arg(&start, m)
            || !parse_result(start, &expected_success, expected))
        {
            flint_printf("error in fmpq_reconstruct test, line %d: unreadable line\n",
                         line_number);
            result = 1;
        }
        else
        {
            int success = fmpq_reconstruct_fmpz(actual, a, m);
            if (success != expected_success
                || (success && !fmpq_equal(actual, expected)))
            {
                flint_printf("error in fmpq_reconstruct test, line %d. FLINT: success=%d ",
                             line_number, success);
                fmpq_print(actual);
                flint_printf("\n");
                result = 1;
            }
        }
    }
    fmpq_clear(expected);
    fmpq_clear(actual);
    fmpz_clear(a);
    fmpz_clear(m);
    return result;
}

static int
check_reconstruct_2_line(char * line, int line_number)
{
    fmpq_t expected, actual;
    fmpz_t a, m, big_n, big_d;
    fmpq_init(expected);
    fmpq_init(actual);
    fmpz_init(a);
    fmpz_init(m);
    fmpz_init(big_n);
    fmpz_init(big_d);
    int result = 0;
    char * start = strstr(line, "Rational::reconstruct_with_bounds(");
    if (start == NULL)
    {
        start = strstr(line, "Rational::reconstruct_with_bounds_ref(");
        if (start != NULL)
        {
            start += strlen("Rational::reconstruct_with_bounds_ref(");
        }
    }
    else
    {
        start += strlen("Rational::reconstruct_with_bounds(");
    }
    if (start != NULL)
    {
        int expected_success;
        fmpz_t two_n_d;
        fmpz_init(two_n_d);
        if (!parse_fmpz_arg(&start, a) || !parse_fmpz_arg(&start, m)
            || !parse_fmpz_arg(&start, big_n) || !parse_fmpz_arg(&start, big_d)
            || !parse_result(start, &expected_success, expected))
        {
            flint_printf("error in fmpq_reconstruct_2 test, line %d: unreadable line\n",
                         line_number);
            result = 1;
        }
        else
        {
            /*
                Outside the documented precondition 2*N*D < m, FLINT's size-dispatched kernels
                disagree with each other: the two-limb kernel reads N and D through
                fmpz_get_uiui, which drops all limbs beyond the second, and can spuriously fail
                where FLINT's own reference implementation (and Malachite) succeeds. Only the
                in-contract domain is diffed.
            */
            fmpz_mul(two_n_d, big_n, big_d);
            fmpz_mul_2exp(two_n_d, two_n_d, 1);
            if (fmpz_cmp(two_n_d, m) < 0)
            {
                int success = fmpq_reconstruct_fmpz_2(actual, a, m, big_n, big_d);
                if (success != expected_success
                    || (success && !fmpq_equal(actual, expected)))
                {
                    flint_printf("error in fmpq_reconstruct_2 test, line %d. FLINT: success=%d ",
                                 line_number, success);
                    fmpq_print(actual);
                    flint_printf("\n");
                    result = 1;
                }
            }
        }
        fmpz_clear(two_n_d);
    }
    fmpq_clear(expected);
    fmpq_clear(actual);
    fmpz_clear(a);
    fmpz_clear(m);
    fmpz_clear(big_n);
    fmpz_clear(big_d);
    return result;
}

int
run_fmpq_reconstruct(const char * arg)
{
    return for_each_line(arg, check_reconstruct_line);
}

int
run_fmpq_reconstruct_2(const char * arg)
{
    return for_each_line(arg, check_reconstruct_2_line);
}

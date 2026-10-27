/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
    Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Oracles for Malachite's characteristic polynomials of `IntegerMatrix`es and `RationalMatrix`es.
    The demos print `A.characteristic_polynomial() = p`, or `(&A).characteristic_polynomial() = p`
    for the `_ref` demos; the `fmpz_mat_charpoly` and `fmpq_mat_charpoly` modes recompute p with
    `fmpz_mat_charpoly` and `fmpq_mat_charpoly`. The bound demo prints
    `A.characteristic_polynomial_bound() = b`; the `fmpz_mat_charpoly_bound` mode requires b to
    equal `fmpz_mat_charpoly_bound` exactly, since Malachite rounds its floating-point magnitudes as
    FLINT does. The mod-2^k demos print `A.mod_power_of_2_characteristic_polynomial(k) = p`; the
    `fmpz_mod_mat_charpoly_power_of_2` mode recomputes p with `fmpz_mod_mat_charpoly` modulo 2^k,
    and for k = 0 requires p = 0. The general-modulus demos print
    `A.mod_characteristic_polynomial(m) = p`, possibly with `&m` for m; the `fmpz_mod_mat_charpoly`
    mode recomputes p with `fmpz_mod_mat_charpoly` modulo m. Malachite panics rather than printing
    a line for a non-square matrix.

    Matrices are read by fmpz_mat_init_set_str_malachite or fmpq_mat_init_set_str_malachite and
    polynomials by fmpz_poly_set_str_malachite or fmpq_poly_set_str_malachite in util.c. Any other
    nonempty line is an error, and so is an input with no lines at all.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <flint/fmpz.h>
#include <flint/fmpz_mat.h>
#include <flint/fmpz_poly.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_mat.h>
#include <flint/fmpz_mod_poly.h>
#include <flint/fmpq_mat.h>
#include <flint/fmpq_poly.h>

#include "oracle.h"

static long checked;
static const char * current_name;
static int bound_mode;
static int rational_mode;
static int power_of_2_mode;
static int mod_mode;

static int
error(const char * message, int line_number)
{
    flint_printf("error in %s test, line %d: %s\n", current_name, line_number, message);
    return 1;
}

static int
check_rational(const char * input, const char * output, int line_number)
{
    fmpq_mat_t a;
    if (!fmpq_mat_init_set_str_malachite(a, input))
    {
        return error("unreadable input", line_number);
    }
    int result = 0;
    fmpq_poly_t expected, actual;
    fmpq_poly_init(expected);
    fmpq_poly_init(actual);
    if (fmpq_mat_nrows(a) != fmpq_mat_ncols(a))
    {
        result = error("the input is not square", line_number);
    }
    else if (!fmpq_poly_set_str_malachite(expected, output))
    {
        result = error("unreadable result", line_number);
    }
    else
    {
        fmpq_mat_charpoly(actual, a);
        if (!fmpq_poly_equal(actual, expected))
        {
            result = error("the characteristic polynomials differ", line_number);
        }
    }
    fmpq_poly_clear(expected);
    fmpq_poly_clear(actual);
    fmpq_mat_clear(a);
    return result;
}

/* Checks `input.mod_power_of_2_characteristic_polynomial(k) = p`, or, in the general-modulus
   mode, `input.mod_characteristic_polynomial(m) = p`, where `rest` is `k) = p` or `m) = p`; the
   `_ref` demos may print `&m` for m. */
static int
check_modular(const char * input, char * rest, int line_number)
{
    char * close = strstr(rest, ") = ");
    if (close == NULL)
    {
        return error("unrecognized line", line_number);
    }
    *close = '\0';
    const char * output = close + 4;
    if (rest[0] == '&')
    {
        rest++;
    }
    fmpz_t m;
    fmpz_init(m);
    int pow_zero = 0;
    if (power_of_2_mode)
    {
        char * end;
        unsigned long long pow = strtoull(rest, &end, 10);
        if (end == rest || *end != '\0')
        {
            fmpz_clear(m);
            return error("unrecognized line", line_number);
        }
        pow_zero = pow == 0;
        fmpz_one(m);
        fmpz_mul_2exp(m, m, pow);
    }
    else if (fmpz_set_str(m, rest, 10) != 0 || fmpz_sgn(m) <= 0)
    {
        fmpz_clear(m);
        return error("unreadable modulus", line_number);
    }
    fmpz_mat_t a;
    if (!fmpz_mat_init_set_str_malachite(a, input))
    {
        fmpz_clear(m);
        return error("unreadable input", line_number);
    }
    int result = 0;
    fmpz_poly_t expected;
    fmpz_poly_init(expected);
    if (fmpz_mat_nrows(a) != fmpz_mat_ncols(a))
    {
        result = error("the input is not square", line_number);
    }
    else if (!fmpz_poly_set_str_malachite(expected, output))
    {
        result = error("unreadable result", line_number);
    }
    else if (pow_zero)
    {
        if (!fmpz_poly_is_zero(expected))
        {
            result = error("the characteristic polynomial modulo 1 must be 0", line_number);
        }
    }
    else
    {
        fmpz_mod_ctx_t ctx;
        fmpz_mod_ctx_init(ctx, m);
        fmpz_mod_mat_t am;
        fmpz_mod_mat_init(am, fmpz_mat_nrows(a), fmpz_mat_ncols(a), ctx);
        fmpz_mod_mat_set_fmpz_mat(am, a, ctx);
        fmpz_mod_poly_t p;
        fmpz_mod_poly_init(p, ctx);
        fmpz_mod_mat_charpoly(p, am, ctx);
        fmpz_poly_t actual;
        fmpz_poly_init(actual);
        fmpz_mod_poly_get_fmpz_poly(actual, p, ctx);
        if (!fmpz_poly_equal(actual, expected))
        {
            result = error("the characteristic polynomials differ", line_number);
        }
        fmpz_poly_clear(actual);
        fmpz_mod_poly_clear(p, ctx);
        fmpz_mod_mat_clear(am, ctx);
        fmpz_mod_ctx_clear(ctx);
    }
    fmpz_clear(m);
    fmpz_poly_clear(expected);
    fmpz_mat_clear(a);
    return result;
}

static int
check_line(char * line, int line_number)
{
    line[strcspn(line, "\r\n")] = '\0';
    if (line[0] == '\0')
    {
        return 0;
    }
    const char * method = power_of_2_mode ? ".mod_power_of_2_characteristic_polynomial("
                          : mod_mode      ? ".mod_characteristic_polynomial("
                          : bound_mode    ? ".characteristic_polynomial_bound() = "
                                          : ".characteristic_polynomial() = ";
    char * at = strstr(line, method);
    if (at == NULL)
    {
        return error("unrecognized line", line_number);
    }
    *at = '\0';
    char * rest = at + strlen(method);
    checked++;
    /* The `_ref` demos print `(&A)` in place of `A`. */
    size_t input_len = strlen(line);
    if (strncmp(line, "(&", 2) == 0 && input_len > 2 && line[input_len - 1] == ')')
    {
        line[input_len - 1] = '\0';
        line += 2;
    }
    if (rational_mode)
    {
        return check_rational(line, rest, line_number);
    }
    if (power_of_2_mode || mod_mode)
    {
        return check_modular(line, rest, line_number);
    }
    fmpz_mat_t a;
    if (!fmpz_mat_init_set_str_malachite(a, line))
    {
        return error("unreadable input", line_number);
    }
    int result = 0;
    if (fmpz_mat_nrows(a) != fmpz_mat_ncols(a))
    {
        result = error("the input is not square", line_number);
    }
    else if (bound_mode)
    {
        fmpz_t expected, actual;
        fmpz_init(expected);
        fmpz_init(actual);
        if (fmpz_set_str(expected, rest, 10) != 0)
        {
            result = error("unreadable result", line_number);
        }
        else
        {
            fmpz_mat_charpoly_bound(actual, a);
            if (!fmpz_equal(actual, expected))
            {
                result = error("the bounds differ", line_number);
            }
        }
        fmpz_clear(expected);
        fmpz_clear(actual);
    }
    else
    {
        fmpz_poly_t expected, actual;
        fmpz_poly_init(expected);
        fmpz_poly_init(actual);
        if (!fmpz_poly_set_str_malachite(expected, rest))
        {
            result = error("unreadable result", line_number);
        }
        else
        {
            fmpz_mat_charpoly(actual, a);
            if (!fmpz_poly_equal(actual, expected))
            {
                result = error("the characteristic polynomials differ", line_number);
            }
        }
        fmpz_poly_clear(expected);
        fmpz_poly_clear(actual);
    }
    fmpz_mat_clear(a);
    return result;
}

static int
run(const char * arg, const char * name, int bound, int rational, int power_of_2, int mod)
{
    power_of_2_mode = power_of_2;
    mod_mode = mod;
    checked = 0;
    current_name = name;
    bound_mode = bound;
    rational_mode = rational;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_fmpz_mat_charpoly(const char * arg)
{
    return run(arg, "fmpz_mat_charpoly", 0, 0, 0, 0);
}

int
run_fmpz_mat_charpoly_bound(const char * arg)
{
    return run(arg, "fmpz_mat_charpoly_bound", 1, 0, 0, 0);
}

int
run_fmpq_mat_charpoly(const char * arg)
{
    return run(arg, "fmpq_mat_charpoly", 0, 1, 0, 0);
}

int
run_fmpz_mod_mat_charpoly_power_of_2(const char * arg)
{
    return run(arg, "fmpz_mod_mat_charpoly_power_of_2", 0, 0, 1, 0);
}

int
run_fmpz_mod_mat_charpoly(const char * arg)
{
    return run(arg, "fmpz_mod_mat_charpoly", 0, 0, 0, 1);
}

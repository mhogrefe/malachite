/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
    Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Oracles for Malachite's similarity transforms. The `IntegerMatrix` and `RationalMatrix` demos
    print `A.similarity_transform(r, d) = B`; the `fmpz_mat_similarity` and `fmpq_mat_similarity`
    modes read A, r, and d, apply `fmpz_mat_similarity` or `fmpq_mat_similarity` to a copy of A, and
    require the result to equal B. The modular demos print `A.mod_similarity_transform(r, d, m) =
    B`; the `nmod_mat_similarity` mode (for word-sized moduli) and the `fmpz_mod_mat_similarity`
    mode apply the corresponding FLINT function modulo m. The demos modulo a power of 2 print
    `A.mod_power_of_2_similarity_transform(r, d, k) = B`; the `fmpz_mod_mat_similarity_power_of_2`
    mode applies `fmpz_mod_mat_similarity` modulo 2^k, and for k = 0 requires B to be zero. Malachite
    panics rather than printing a line when A is not square, r is not a row index, or an input is
    not reduced, so FLINT, which checks none of these, only sees valid inputs.

    Matrices are read by fmpz_mat_init_set_str_malachite in util.c. Any other nonempty line is an
    error, and so is an input with no lines at all.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <flint/fmpz.h>
#include <flint/fmpq.h>
#include <flint/fmpq_mat.h>
#include <flint/fmpz_mat.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_mat.h>
#include <flint/nmod_mat.h>

#include "oracle.h"

static long checked;
static const char * current_name;

static int
error(const char * message, int line_number)
{
    flint_printf("error in %s test, line %d: %s\n", current_name, line_number, message);
    return 1;
}

static int
check_line(char * line, int line_number)
{
    line[strcspn(line, "\r\n")] = '\0';
    if (line[0] == '\0')
    {
        return 0;
    }
    char * pieces[3];
    char * rest;
    if (!split_method_call(line, ".similarity_transform(", pieces, 3, &rest)
        || strncmp(rest, " = ", 3) != 0)
    {
        return error("unrecognized line", line_number);
    }
    rest += 3;
    checked++;
    char * end;
    long long r = strtoll(pieces[1], &end, 10);
    fmpz_t d;
    fmpz_init(d);
    fmpz_mat_t a, expected;
    if (end == pieces[1] || *end != '\0' || fmpz_set_str(d, pieces[2], 10) != 0)
    {
        fmpz_clear(d);
        return error("unreadable r or d", line_number);
    }
    if (!fmpz_mat_init_set_str_malachite(a, pieces[0]))
    {
        fmpz_clear(d);
        return error("unreadable input", line_number);
    }
    if (!fmpz_mat_init_set_str_malachite(expected, rest))
    {
        fmpz_clear(d);
        fmpz_mat_clear(a);
        return error("unreadable result", line_number);
    }
    int result = 0;
    if (fmpz_mat_nrows(a) != fmpz_mat_ncols(a) || r < 0 || r >= fmpz_mat_nrows(a))
    {
        result = error("the input is not a square matrix with a valid row index", line_number);
    }
    else
    {
        fmpz_mat_similarity(a, r, d);
        if (!fmpz_mat_equal(a, expected))
        {
            result = error("the transforms differ", line_number);
        }
    }
    fmpz_clear(d);
    fmpz_mat_clear(a);
    fmpz_mat_clear(expected);
    return result;
}

/* Reads `r` from `piece`, returning 1 on success. */
static int
read_r(const char * piece, slong * r)
{
    char * end;
    long long x = strtoll(piece, &end, 10);
    if (end == piece || *end != '\0')
    {
        return 0;
    }
    *r = x;
    return 1;
}

static int
check_rational_line(char * line, int line_number)
{
    line[strcspn(line, "\r\n")] = '\0';
    if (line[0] == '\0')
    {
        return 0;
    }
    char * pieces[3];
    char * rest;
    if (!split_method_call(line, ".similarity_transform(", pieces, 3, &rest)
        || strncmp(rest, " = ", 3) != 0)
    {
        return error("unrecognized line", line_number);
    }
    rest += 3;
    checked++;
    slong r;
    fmpq_t d;
    fmpq_init(d);
    fmpq_mat_t a, expected;
    if (!read_r(pieces[1], &r) || fmpq_set_str(d, pieces[2], 10) != 0)
    {
        fmpq_clear(d);
        return error("unreadable r or d", line_number);
    }
    if (!fmpq_mat_init_set_str_malachite(a, pieces[0]))
    {
        fmpq_clear(d);
        return error("unreadable input", line_number);
    }
    if (!fmpq_mat_init_set_str_malachite(expected, rest))
    {
        fmpq_clear(d);
        fmpq_mat_clear(a);
        return error("unreadable result", line_number);
    }
    int result = 0;
    if (fmpq_mat_nrows(a) != fmpq_mat_ncols(a) || r < 0 || r >= fmpq_mat_nrows(a))
    {
        result = error("the input is not a square matrix with a valid row index", line_number);
    }
    else
    {
        fmpq_mat_similarity(a, r, d);
        if (!fmpq_mat_equal(a, expected))
        {
            result = error("the transforms differ", line_number);
        }
    }
    fmpq_clear(d);
    fmpq_mat_clear(a);
    fmpq_mat_clear(expected);
    return result;
}

static int power_of_2;
static int use_nmod;

static int
check_mod_line(char * line, int line_number)
{
    line[strcspn(line, "\r\n")] = '\0';
    if (line[0] == '\0')
    {
        return 0;
    }
    char * pieces[4];
    char * rest;
    const char * method = power_of_2 ? ".mod_power_of_2_similarity_transform("
                                     : ".mod_similarity_transform(";
    if (!split_method_call(line, method, pieces, 4, &rest) || strncmp(rest, " = ", 3) != 0)
    {
        return error("unrecognized line", line_number);
    }
    rest += 3;
    checked++;
    slong r;
    fmpz_t d, m;
    fmpz_init(d);
    fmpz_init(m);
    int ok = read_r(pieces[1], &r) && fmpz_set_str(d, pieces[2], 10) == 0;
    if (ok && power_of_2)
    {
        slong pow;
        ok = read_r(pieces[3], &pow) && pow >= 0;
        if (ok)
        {
            fmpz_one(m);
            fmpz_mul_2exp(m, m, pow);
        }
    }
    else if (ok)
    {
        ok = fmpz_set_str(m, pieces[3], 10) == 0 && fmpz_sgn(m) > 0;
    }
    if (!ok)
    {
        fmpz_clear(d);
        fmpz_clear(m);
        return error("unreadable r, d, or modulus", line_number);
    }
    fmpz_mat_t a, expected;
    if (!fmpz_mat_init_set_str_malachite(a, pieces[0]))
    {
        fmpz_clear(d);
        fmpz_clear(m);
        return error("unreadable input", line_number);
    }
    if (!fmpz_mat_init_set_str_malachite(expected, rest))
    {
        fmpz_clear(d);
        fmpz_clear(m);
        fmpz_mat_clear(a);
        return error("unreadable result", line_number);
    }
    int result = 0;
    slong n = fmpz_mat_nrows(a);
    if (n != fmpz_mat_ncols(a) || r < 0 || r >= n)
    {
        result = error("the input is not a square matrix with a valid row index", line_number);
    }
    else if (fmpz_is_one(m))
    {
        /* Modulo 1 everything is 0. */
        if (!fmpz_mat_is_zero(expected))
        {
            result = error("the transform modulo 1 must be 0", line_number);
        }
    }
    else if (use_nmod)
    {
        if (!fmpz_abs_fits_ui(m))
        {
            result = error("the modulus does not fit in a word", line_number);
        }
        else
        {
            nmod_mat_t am;
            nmod_mat_init(am, n, n, fmpz_get_ui(m));
            fmpz_mat_get_nmod_mat(am, a);
            nmod_mat_similarity(am, r, fmpz_get_ui(d));
            fmpz_mat_set_nmod_mat_unsigned(a, am);
            if (!fmpz_mat_equal(a, expected))
            {
                result = error("the transforms differ", line_number);
            }
            nmod_mat_clear(am);
        }
    }
    else
    {
        fmpz_mod_ctx_t ctx;
        fmpz_mod_ctx_init(ctx, m);
        fmpz_mod_mat_t am;
        fmpz_mod_mat_init(am, n, n, ctx);
        fmpz_mod_mat_set_fmpz_mat(am, a, ctx);
        fmpz_mod_mat_similarity(am, r, d, ctx);
        fmpz_mod_mat_get_fmpz_mat(a, am, ctx);
        if (!fmpz_mat_equal(a, expected))
        {
            result = error("the transforms differ", line_number);
        }
        fmpz_mod_mat_clear(am, ctx);
        fmpz_mod_ctx_clear(ctx);
    }
    fmpz_clear(d);
    fmpz_clear(m);
    fmpz_mat_clear(a);
    fmpz_mat_clear(expected);
    return result;
}

static int
run(const char * arg, const char * name, int (* handler)(char * line, int line_number))
{
    checked = 0;
    current_name = name;
    int result = for_each_line(arg, handler);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_fmpz_mat_similarity(const char * arg)
{
    return run(arg, "fmpz_mat_similarity", check_line);
}

int
run_fmpq_mat_similarity(const char * arg)
{
    return run(arg, "fmpq_mat_similarity", check_rational_line);
}

int
run_nmod_mat_similarity(const char * arg)
{
    power_of_2 = 0;
    use_nmod = 1;
    return run(arg, "nmod_mat_similarity", check_mod_line);
}

int
run_fmpz_mod_mat_similarity(const char * arg)
{
    power_of_2 = 0;
    use_nmod = 0;
    return run(arg, "fmpz_mod_mat_similarity", check_mod_line);
}

int
run_fmpz_mod_mat_similarity_power_of_2(const char * arg)
{
    power_of_2 = 1;
    use_nmod = 0;
    return run(arg, "fmpz_mod_mat_similarity_power_of_2", check_mod_line);
}

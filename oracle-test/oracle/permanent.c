/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the permanent, determinant-sign, and determinant-mod-2^k demos
    against FLINT.

    The permanent demos print `permanent(A) = p` (or `permanent_cofactor`, `permanent_glynn`, or
    `permanent_dense` for the demos of individual algorithms); the `fmpz_mat_permanent` and
    `fmpq_mat_permanent` modes recompute p with `fmpz_mat_permanent` and `fmpq_mat_permanent`, and report an error if
    FLINT cannot compute it (above dimension 62), since the demos' matrices are small. The
    determinant-sign demos print `determinant_sign(A) = Less`, `Equal`, or `Greater`; the
    `fmpz_mat_det_sign` and `fmpq_mat_det_sign` modes compare it with the sign of `fmpz_mat_det` and
    `fmpq_mat_det`. The determinant-mod-2^k demos print `A.mod_power_of_2_determinant(k) = d`; the
    `fmpz_mod_mat_det_power_of_2` mode recomputes d with `fmpz_mod_mat_det` modulo 2^k, and for k = 0
    requires d = 0.

    Matrices are read by fmpz_mat_init_set_str_malachite and fmpq_mat_init_set_str_malachite in
    util.c. Each mode treats any other nonempty line as an error, and an input with no lines at all.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <flint/fmpq.h>
#include <flint/fmpq_mat.h>
#include <flint/fmpz.h>
#include <flint/fmpz_mat.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_mat.h>

#include "oracle.h"

static long checked;
static const char * current_name;
static int current_kind;

enum
{
    KIND_FMPZ_PERMANENT,
    KIND_FMPQ_PERMANENT,
    KIND_FMPZ_DET_SIGN,
    KIND_FMPQ_DET_SIGN,
    KIND_DET_POWER_OF_2
};

static int
error(const char * message, int line_number)
{
    flint_printf("error in %s test, line %d: %s\n", current_name, line_number, message);
    return 1;
}

/* Splits `op(A) = rest` into A and rest, in place. Returns 1 on success. */
static int
split_function_call(char * line, const char * op, char ** a, char ** rest)
{
    size_t op_len = strlen(op);
    char * eq = strstr(line, ") = ");
    if (strncmp(line, op, op_len) != 0 || line[op_len] != '(' || eq == NULL)
    {
        return 0;
    }
    *eq = '\0';
    *a = line + op_len + 1;
    *rest = eq + 4;
    return 1;
}

static const char *
sign_name(int sign)
{
    return sign < 0 ? "Less" : (sign == 0 ? "Equal" : "Greater");
}

static int
check_permanent_or_sign(char * line, int line_number)
{
    /* The permanent modes also read the lines of the demos of Malachite's individual algorithms. */
    static const char * const permanent_ops[] = {"permanent_cofactor", "permanent_glynn",
                                                 "permanent_dense", "permanent"};
    static const char * const sign_ops[] = {"determinant_sign"};
    int permanent = current_kind == KIND_FMPZ_PERMANENT || current_kind == KIND_FMPQ_PERMANENT;
    const char * const * ops = permanent ? permanent_ops : sign_ops;
    int op_count = permanent ? 4 : 1;
    char * a_str;
    char * rest;
    int found = 0;
    for (int i = 0; i < op_count && !found; i++)
    {
        found = split_function_call(line, ops[i], &a_str, &rest);
    }
    if (!found)
    {
        return error("unrecognized line", line_number);
    }
    checked++;
    int result = 0;
    if (current_kind == KIND_FMPZ_PERMANENT || current_kind == KIND_FMPZ_DET_SIGN)
    {
        fmpz_mat_t a;
        fmpz_t actual;
        fmpz_init(actual);
        if (!fmpz_mat_init_set_str_malachite(a, a_str) || fmpz_mat_nrows(a) != fmpz_mat_ncols(a))
        {
            fmpz_clear(actual);
            return error("unreadable input", line_number);
        }
        if (current_kind == KIND_FMPZ_PERMANENT)
        {
            fmpz_t expected;
            fmpz_init(expected);
            if (!fmpz_mat_permanent(actual, a))
            {
                result = error("FLINT cannot compute this permanent", line_number);
            }
            else if (fmpz_set_str(expected, rest, 10) != 0)
            {
                result = error("unreadable result", line_number);
            }
            else if (!fmpz_equal(actual, expected))
            {
                result = error("the permanents differ", line_number);
            }
            fmpz_clear(expected);
        }
        else
        {
            fmpz_mat_det(actual, a);
            if (strcmp(rest, sign_name(fmpz_sgn(actual))) != 0)
            {
                result = error("the determinant signs differ", line_number);
            }
        }
        fmpz_clear(actual);
        fmpz_mat_clear(a);
    }
    else
    {
        fmpq_mat_t a;
        fmpq_t actual;
        fmpq_init(actual);
        if (!fmpq_mat_init_set_str_malachite(a, a_str) || fmpq_mat_nrows(a) != fmpq_mat_ncols(a))
        {
            fmpq_clear(actual);
            return error("unreadable input", line_number);
        }
        if (current_kind == KIND_FMPQ_PERMANENT)
        {
            fmpq_t expected;
            fmpq_init(expected);
            if (!fmpq_mat_permanent(actual, a))
            {
                result = error("FLINT cannot compute this permanent", line_number);
            }
            else if (fmpq_set_str(expected, rest, 10) != 0)
            {
                result = error("unreadable result", line_number);
            }
            else if (!fmpq_equal(actual, expected))
            {
                result = error("the permanents differ", line_number);
            }
            fmpq_clear(expected);
        }
        else
        {
            fmpq_mat_det(actual, a);
            if (strcmp(rest, sign_name(fmpq_sgn(actual))) != 0)
            {
                result = error("the determinant signs differ", line_number);
            }
        }
        fmpq_clear(actual);
        fmpq_mat_clear(a);
    }
    return result;
}

static int
check_det_power_of_2(char * line, int line_number)
{
    char * pieces[2];
    char * rest;
    if (!split_method_call(line, ".mod_power_of_2_determinant(", pieces, 2, &rest)
        || strncmp(rest, " = ", 3) != 0)
    {
        return error("unrecognized line", line_number);
    }
    rest += 3;
    checked++;
    fmpz_mat_t a;
    fmpz_t expected;
    fmpz_init(expected);
    char * end;
    unsigned long long pow = strtoull(pieces[1], &end, 10);
    if (end == pieces[1] || *end != '\0' || !fmpz_mat_init_set_str_malachite(a, pieces[0])
        || fmpz_mat_nrows(a) != fmpz_mat_ncols(a))
    {
        fmpz_clear(expected);
        return error("unreadable input", line_number);
    }
    int result = 0;
    if (fmpz_set_str(expected, rest, 10) != 0)
    {
        result = error("unreadable result", line_number);
    }
    else if (pow == 0)
    {
        if (!fmpz_is_zero(expected))
        {
            result = error("the determinant modulo 1 must be 0", line_number);
        }
    }
    else
    {
        fmpz_t m, actual;
        fmpz_init(m);
        fmpz_init(actual);
        fmpz_one(m);
        fmpz_mul_2exp(m, m, pow);
        fmpz_mod_ctx_t ctx;
        fmpz_mod_ctx_init(ctx, m);
        fmpz_mod_mat_t am;
        fmpz_mod_mat_init(am, fmpz_mat_nrows(a), fmpz_mat_ncols(a), ctx);
        fmpz_mod_mat_set_fmpz_mat(am, a, ctx);
        fmpz_mod_mat_det(actual, am, ctx);
        if (!fmpz_equal(actual, expected))
        {
            result = error("the determinants differ", line_number);
        }
        fmpz_mod_mat_clear(am, ctx);
        fmpz_mod_ctx_clear(ctx);
        fmpz_clear(m);
        fmpz_clear(actual);
    }
    fmpz_clear(expected);
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
    return current_kind == KIND_DET_POWER_OF_2 ? check_det_power_of_2(line, line_number)
                                               : check_permanent_or_sign(line, line_number);
}

static int
run(const char * arg, const char * name, int kind)
{
    checked = 0;
    current_name = name;
    current_kind = kind;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_fmpz_mat_permanent(const char * arg)
{
    return run(arg, "fmpz_mat_permanent", KIND_FMPZ_PERMANENT);
}

int
run_fmpq_mat_permanent(const char * arg)
{
    return run(arg, "fmpq_mat_permanent", KIND_FMPQ_PERMANENT);
}

int
run_fmpz_mat_det_sign(const char * arg)
{
    return run(arg, "fmpz_mat_det_sign", KIND_FMPZ_DET_SIGN);
}

int
run_fmpq_mat_det_sign(const char * arg)
{
    return run(arg, "fmpq_mat_det_sign", KIND_FMPQ_DET_SIGN);
}

int
run_fmpz_mod_mat_det_power_of_2(const char * arg)
{
    return run(arg, "fmpz_mod_mat_det_power_of_2", KIND_DET_POWER_OF_2);
}

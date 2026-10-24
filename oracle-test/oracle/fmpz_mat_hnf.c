/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the IntegerMatrix Hermite normal form demos against FLINT's
    fmpz_mat functions.

    The demos print `op(A) = H`, `op(A, D) = H` for the modular algorithms, `op(A) = (H, U)` for
    the transforms, and `is_in_hermite_normal_form(A) = b`, where `b` is `true` or `false`. The
    modes recompute the results with `fmpz_mat_hnf`, `fmpz_mat_hnf_classical`,
    `fmpz_mat_hnf_xgcd`, `fmpz_mat_hnf_minors`, `fmpz_mat_hnf_pernet_stein`,
    `fmpz_mat_hnf_modular`, `fmpz_mat_hnf_modular_eldiv`, `fmpz_mat_hnf_transform`,
    `fmpz_mat_hnf_minors_transform`, and `fmpz_mat_is_in_hnf`. The `fmpz_mat_hnf_transform_naive`
    mode reads H and U off the Hermite normal form of [A | I], computed with `fmpz_mat_hnf`, as
    FLINT's static `_fmpz_mat_hnf_transform_naive` does.

    The `fmpz_xgcd` mode checks lines `extended_gcd_reduced_cofactor(f, g) = (d, a, b)` against
    `fmpz_xgcd`.

    Matrices are read by fmpz_mat_init_set_str_malachite in util.c. Each mode treats any other
    nonempty line as an error, and an input with no lines at all.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <flint/fmpz.h>
#include <flint/fmpz_mat.h>

#include "oracle.h"

enum { KIND_HNF, KIND_CLASSICAL, KIND_XGCD, KIND_MINORS, KIND_PERNET_STEIN, KIND_MODULAR,
       KIND_MODULAR_ELDIV, KIND_TRANSFORM, KIND_MINORS_TRANSFORM, KIND_TRANSFORM_NAIVE,
       KIND_IS_IN_HNF, KIND_FMPZ_XGCD };

static long checked;
static const char * current_name;
static const char * current_op;
static int current_kind;

static int
unrecognized(int line_number)
{
    flint_printf("error in %s test, line %d: unrecognized line\n", current_name, line_number);
    return 1;
}

static int
unreadable(int line_number)
{
    flint_printf("error in %s test, line %d: unreadable line\n", current_name, line_number);
    return 1;
}

/* The length of the matrix string at the start of `s`: `[]_{h,w}` or `[...]`. */
static size_t
matrix_length(const char * s)
{
    const char * end = strncmp(s, "[]_{", 4) == 0 ? strchr(s, '}') : strchr(s, ']');
    return end == NULL ? 0 : (size_t) (end - s + 1);
}

/* Compares `actual` with the matrix written in `expected_str`. */
static int
compare_mat(const fmpz_mat_t actual, const char * expected_str, const char * what,
            int line_number)
{
    fmpz_mat_t expected;
    int result = 0;
    if (!fmpz_mat_init_set_str_malachite(expected, expected_str))
    {
        result = unreadable(line_number);
    }
    else if (!fmpz_mat_equal(actual, expected))
    {
        flint_printf("error in %s test, line %d: the %s differ; FLINT gives\n", current_name,
                     line_number, what);
        fmpz_mat_print_pretty(actual);
        flint_printf("\n");
        result = 1;
    }
    fmpz_mat_clear(expected);
    return result;
}

/* Splits `(X, Y)` in place into the strings of X and Y, which are matrices. */
static int
split_matrix_pair(char * s, char ** x, char ** y)
{
    size_t len = strlen(s);
    if (len < 2 || s[0] != '(' || s[len - 1] != ')')
    {
        return 0;
    }
    s[len - 1] = '\0';
    *x = s + 1;
    size_t x_len = matrix_length(*x);
    if (x_len == 0 || strncmp(*x + x_len, ", ", 2) != 0)
    {
        return 0;
    }
    (*x)[x_len] = '\0';
    *y = *x + x_len + 2;
    return 1;
}

static int
check_xgcd(char * args, char * rest, int line_number)
{
    char * comma = strstr(args, ", ");
    size_t len = strlen(rest);
    if (comma == NULL || len < 2 || rest[0] != '(' || rest[len - 1] != ')')
    {
        return unrecognized(line_number);
    }
    *comma = '\0';
    rest[len - 1] = '\0';
    fmpz_t f, g, d, a, b, d2, a2, b2;
    fmpz_init(f);
    fmpz_init(g);
    fmpz_init(d);
    fmpz_init(a);
    fmpz_init(b);
    fmpz_init(d2);
    fmpz_init(a2);
    fmpz_init(b2);
    char * d_str = rest + 1;
    char * a_str = strstr(d_str, ", ");
    char * b_str = a_str == NULL ? NULL : strstr(a_str + 2, ", ");
    int result = 0;
    if (b_str == NULL)
    {
        result = unrecognized(line_number);
    }
    else
    {
        *a_str = '\0';
        *b_str = '\0';
        if (fmpz_set_str(f, args, 10) != 0 || fmpz_set_str(g, comma + 2, 10) != 0
            || fmpz_set_str(d2, d_str, 10) != 0 || fmpz_set_str(a2, a_str + 2, 10) != 0
            || fmpz_set_str(b2, b_str + 2, 10) != 0)
        {
            result = unreadable(line_number);
        }
        else
        {
            fmpz_xgcd(d, a, b, f, g);
            if (!fmpz_equal(d, d2) || !fmpz_equal(a, a2) || !fmpz_equal(b, b2))
            {
                flint_printf("error in %s test, line %d: FLINT gives (", current_name,
                             line_number);
                fmpz_print(d);
                flint_printf(", ");
                fmpz_print(a);
                flint_printf(", ");
                fmpz_print(b);
                flint_printf(")\n");
                result = 1;
            }
        }
    }
    fmpz_clear(f);
    fmpz_clear(g);
    fmpz_clear(d);
    fmpz_clear(a);
    fmpz_clear(b);
    fmpz_clear(d2);
    fmpz_clear(a2);
    fmpz_clear(b2);
    return result;
}

/* Sets `h` and `u` to the Hermite normal form of [A | I], split, as
   `_fmpz_mat_hnf_transform_naive` does. */
static void
hnf_transform_naive(fmpz_mat_t h, fmpz_mat_t u, const fmpz_mat_t a)
{
    slong m = fmpz_mat_nrows(a), n = fmpz_mat_ncols(a);
    fmpz_mat_t a2, h2;
    fmpz_mat_init(a2, m, n + m);
    fmpz_mat_init(h2, m, n + m);
    for (slong i = 0; i < m; i++)
    {
        for (slong j = 0; j < n; j++)
        {
            fmpz_set(fmpz_mat_entry(a2, i, j), fmpz_mat_entry(a, i, j));
        }
        fmpz_one(fmpz_mat_entry(a2, i, n + i));
    }
    fmpz_mat_hnf(h2, a2);
    for (slong i = 0; i < m; i++)
    {
        for (slong j = 0; j < n; j++)
        {
            fmpz_set(fmpz_mat_entry(h, i, j), fmpz_mat_entry(h2, i, j));
        }
        for (slong j = 0; j < m; j++)
        {
            fmpz_set(fmpz_mat_entry(u, i, j), fmpz_mat_entry(h2, i, n + j));
        }
    }
    fmpz_mat_clear(a2);
    fmpz_mat_clear(h2);
}

static int
check_line(char * line, int line_number)
{
    line[strcspn(line, "\r\n")] = '\0';
    if (line[0] == '\0')
    {
        return 0;
    }
    char * eq;
    size_t op_len = strlen(current_op);
    if (strncmp(line, current_op, op_len) != 0 || line[op_len] != '('
        || (eq = strstr(line, ") = ")) == NULL)
    {
        return unrecognized(line_number);
    }
    *eq = '\0';
    char * args = line + op_len + 1;
    char * rest = eq + 4;
    checked++;
    if (current_kind == KIND_FMPZ_XGCD)
    {
        return check_xgcd(args, rest, line_number);
    }
    char * d_str = NULL;
    if (current_kind == KIND_MODULAR || current_kind == KIND_MODULAR_ELDIV)
    {
        size_t a_len = matrix_length(args);
        if (a_len == 0 || strncmp(args + a_len, ", ", 2) != 0)
        {
            return unrecognized(line_number);
        }
        args[a_len] = '\0';
        d_str = args + a_len + 2;
    }
    fmpz_mat_t a;
    if (!fmpz_mat_init_set_str_malachite(a, args))
    {
        fmpz_mat_clear(a);
        return unreadable(line_number);
    }
    slong m = fmpz_mat_nrows(a), n = fmpz_mat_ncols(a);
    int result = 0;
    if (current_kind == KIND_IS_IN_HNF)
    {
        int expected = strcmp(rest, "true") == 0;
        if (!expected && strcmp(rest, "false") != 0)
        {
            result = unrecognized(line_number);
        }
        else if (fmpz_mat_is_in_hnf(a) != expected)
        {
            flint_printf("error in %s test, line %d: Malachite gives %s\n", current_name,
                         line_number, rest);
            result = 1;
        }
    }
    else if (current_kind >= KIND_TRANSFORM)
    {
        char * h_str;
        char * u_str;
        if (!split_matrix_pair(rest, &h_str, &u_str))
        {
            result = unrecognized(line_number);
        }
        else
        {
            fmpz_mat_t h, u;
            fmpz_mat_init(h, m, n);
            fmpz_mat_init(u, m, m);
            if (current_kind == KIND_TRANSFORM)
            {
                fmpz_mat_hnf_transform(h, u, a);
            }
            else if (current_kind == KIND_MINORS_TRANSFORM)
            {
                fmpz_mat_hnf_minors_transform(h, u, a);
            }
            else
            {
                hnf_transform_naive(h, u, a);
            }
            result = compare_mat(h, h_str, "Hermite normal forms", line_number);
            if (result == 0)
            {
                result = compare_mat(u, u_str, "transforms", line_number);
            }
            fmpz_mat_clear(h);
            fmpz_mat_clear(u);
        }
    }
    else
    {
        fmpz_mat_t h;
        fmpz_mat_init(h, m, n);
        if (current_kind == KIND_HNF)
        {
            fmpz_mat_hnf(h, a);
        }
        else if (current_kind == KIND_CLASSICAL)
        {
            fmpz_mat_hnf_classical(h, a);
        }
        else if (current_kind == KIND_XGCD)
        {
            fmpz_mat_hnf_xgcd(h, a);
        }
        else if (current_kind == KIND_MINORS)
        {
            fmpz_mat_hnf_minors(h, a);
        }
        else if (current_kind == KIND_PERNET_STEIN)
        {
            flint_rand_t state;
            flint_rand_init(state);
            fmpz_mat_hnf_pernet_stein(h, a, state);
            flint_rand_clear(state);
        }
        else
        {
            fmpz_t d;
            fmpz_init(d);
            if (fmpz_set_str(d, d_str, 10) != 0)
            {
                result = unreadable(line_number);
            }
            else if (current_kind == KIND_MODULAR)
            {
                fmpz_mat_hnf_modular(h, a, d);
            }
            else
            {
                fmpz_mat_set(h, a);
                fmpz_mat_hnf_modular_eldiv(h, d);
            }
            fmpz_clear(d);
        }
        if (result == 0)
        {
            result = compare_mat(h, rest, "Hermite normal forms", line_number);
        }
        fmpz_mat_clear(h);
    }
    fmpz_mat_clear(a);
    return result;
}

static int
run(const char * arg, const char * name, const char * op, int kind)
{
    checked = 0;
    current_name = name;
    current_op = op;
    current_kind = kind;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_fmpz_mat_hnf(const char * arg)
{
    return run(arg, "fmpz_mat_hnf", "hermite_normal_form", KIND_HNF);
}

int
run_fmpz_mat_hnf_classical(const char * arg)
{
    return run(arg, "fmpz_mat_hnf_classical", "hnf_classical", KIND_CLASSICAL);
}

int
run_fmpz_mat_hnf_xgcd(const char * arg)
{
    return run(arg, "fmpz_mat_hnf_xgcd", "hnf_xgcd", KIND_XGCD);
}

int
run_fmpz_mat_hnf_minors(const char * arg)
{
    return run(arg, "fmpz_mat_hnf_minors", "hnf_minors", KIND_MINORS);
}

int
run_fmpz_mat_hnf_pernet_stein(const char * arg)
{
    return run(arg, "fmpz_mat_hnf_pernet_stein", "hnf_pernet_stein", KIND_PERNET_STEIN);
}

int
run_fmpz_mat_hnf_modular(const char * arg)
{
    return run(arg, "fmpz_mat_hnf_modular", "hnf_modular", KIND_MODULAR);
}

int
run_fmpz_mat_hnf_modular_eldiv(const char * arg)
{
    return run(arg, "fmpz_mat_hnf_modular_eldiv", "hnf_modular_eldiv", KIND_MODULAR_ELDIV);
}

int
run_fmpz_mat_hnf_transform(const char * arg)
{
    return run(arg, "fmpz_mat_hnf_transform", "hermite_normal_form_and_transform",
               KIND_TRANSFORM);
}

int
run_fmpz_mat_hnf_minors_transform(const char * arg)
{
    return run(arg, "fmpz_mat_hnf_minors_transform", "hnf_minors_transform",
               KIND_MINORS_TRANSFORM);
}

int
run_fmpz_mat_hnf_transform_naive(const char * arg)
{
    return run(arg, "fmpz_mat_hnf_transform_naive", "hnf_transform_naive",
               KIND_TRANSFORM_NAIVE);
}

int
run_fmpz_mat_is_in_hnf(const char * arg)
{
    return run(arg, "fmpz_mat_is_in_hnf", "is_in_hermite_normal_form", KIND_IS_IN_HNF);
}

int
run_fmpz_xgcd(const char * arg)
{
    return run(arg, "fmpz_xgcd", "extended_gcd_reduced_cofactor", KIND_FMPZ_XGCD);
}

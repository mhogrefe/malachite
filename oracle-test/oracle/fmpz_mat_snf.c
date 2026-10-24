/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the IntegerMatrix Smith normal form demos against FLINT's fmpz_mat
    functions.

    The demos print `op(A) = S`, `snf_iliopoulos(A, D) = S`, `smith_normal_form_and_transforms(A)
    = (S, U, V)`, `is_in_smith_normal_form(A) = b`, where `b` is `true` or `false`, and
    `elementary_divisors(A) = [d_1, d_2, ...]`. The modes recompute the results with
    `fmpz_mat_snf`, `fmpz_mat_snf_diagonal`, `fmpz_mat_snf_kannan_bachem`,
    `fmpz_mat_snf_iliopoulos`, `fmpz_mat_snf_transform`, `fmpz_mat_is_in_snf`, and
    `fmpz_mat_elementary_divisors`. The `fmpz_mat_snf_iterative_hermite` mode checks lines
    `snf_iterative_hermite(A) = S` against `fmpz_mat_snf`, since the form is unique and FLINT's
    `_fmpz_mat_snf_iterative_hermite` is static.

    The `fmpz_xgcd_canonical_bezout` mode checks the lines `(f).extended_gcd(g) = (d, a, b)` of
    Malachite's `Integer` extended GCD demo against `fmpz_xgcd_canonical_bezout`.

    Matrices are read by fmpz_mat_init_set_str_malachite in util.c. Each mode treats any other
    nonempty line as an error, and an input with no lines at all.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <flint/fmpz.h>
#include <flint/fmpz_mat.h>
#include <flint/fmpz_vec.h>

#include "oracle.h"

enum { KIND_SNF, KIND_DIAGONAL, KIND_ITERATIVE, KIND_KANNAN_BACHEM, KIND_ILIOPOULOS,
       KIND_TRANSFORM, KIND_IS_IN_SNF, KIND_ELEMENTARY_DIVISORS, KIND_XGCD };

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

/* Splits `(X, Y, Z)` in place into the strings of the three matrices. */
static int
split_matrix_triple(char * s, char ** x, char ** y, char ** z)
{
    size_t len = strlen(s);
    if (len < 2 || s[0] != '(' || s[len - 1] != ')')
    {
        return 0;
    }
    s[len - 1] = '\0';
    char * parts[3];
    char * p = s + 1;
    for (int i = 0; i < 3; i++)
    {
        size_t l = matrix_length(p);
        if (l == 0)
        {
            return 0;
        }
        parts[i] = p;
        if (i < 2)
        {
            if (strncmp(p + l, ", ", 2) != 0)
            {
                return 0;
            }
            p[l] = '\0';
            p += l + 2;
        }
        else if (p[l] != '\0')
        {
            return 0;
        }
    }
    *x = parts[0];
    *y = parts[1];
    *z = parts[2];
    return 1;
}

static int
check_xgcd(char * line, int line_number)
{
    /* (f).extended_gcd(g) = (d, a, b) */
    char * mid = strstr(line, ").extended_gcd(");
    char * eq = strstr(line, ") = (");
    if (line[0] != '(' || mid == NULL || eq == NULL || eq < mid)
    {
        return unrecognized(line_number);
    }
    *mid = '\0';
    *eq = '\0';
    char * f_str = line + 1;
    char * g_str = mid + strlen(").extended_gcd(");
    char * rest = eq + 5;
    size_t len = strlen(rest);
    if (len == 0 || rest[len - 1] != ')')
    {
        return unrecognized(line_number);
    }
    rest[len - 1] = '\0';
    char * a_str = strstr(rest, ", ");
    char * b_str = a_str == NULL ? NULL : strstr(a_str + 2, ", ");
    if (b_str == NULL)
    {
        return unrecognized(line_number);
    }
    *a_str = '\0';
    *b_str = '\0';
    fmpz_t f, g, d, a, b, d2, a2, b2;
    fmpz_init(f);
    fmpz_init(g);
    fmpz_init(d);
    fmpz_init(a);
    fmpz_init(b);
    fmpz_init(d2);
    fmpz_init(a2);
    fmpz_init(b2);
    int result = 0;
    if (fmpz_set_str(f, f_str, 10) != 0 || fmpz_set_str(g, g_str, 10) != 0
        || fmpz_set_str(d2, rest, 10) != 0 || fmpz_set_str(a2, a_str + 2, 10) != 0
        || fmpz_set_str(b2, b_str + 2, 10) != 0)
    {
        result = unreadable(line_number);
    }
    else
    {
        fmpz_xgcd_canonical_bezout(d, a, b, f, g);
        if (!fmpz_equal(d, d2) || !fmpz_equal(a, a2) || !fmpz_equal(b, b2))
        {
            flint_printf("error in %s test, line %d: FLINT gives (", current_name, line_number);
            fmpz_print(d);
            flint_printf(", ");
            fmpz_print(a);
            flint_printf(", ");
            fmpz_print(b);
            flint_printf(")\n");
            result = 1;
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

/* Compares FLINT's elementary divisors with `[d_1, d_2, ...]`. */
static int
check_elementary_divisors(const fmpz_mat_t a, char * rest, int line_number)
{
    slong m = fmpz_mat_nrows(a), n = fmpz_mat_ncols(a);
    slong cap = FLINT_MAX(FLINT_MIN(m, n), 1);
    fmpz * ed = _fmpz_vec_init(cap);
    slong r = fmpz_mat_elementary_divisors(ed, a);
    size_t len = strlen(rest);
    int result = 0;
    if (len < 2 || rest[0] != '[' || rest[len - 1] != ']')
    {
        result = unrecognized(line_number);
    }
    else
    {
        rest[len - 1] = '\0';
        char * p = rest + 1;
        slong count = 0;
        fmpz_t x;
        fmpz_init(x);
        while (*p != '\0' && result == 0)
        {
            char * comma = strstr(p, ", ");
            if (comma != NULL)
            {
                *comma = '\0';
            }
            if (fmpz_set_str(x, p, 10) != 0)
            {
                result = unreadable(line_number);
            }
            else if (count >= r || !fmpz_equal(x, ed + count))
            {
                result = 1;
            }
            count++;
            p = comma == NULL ? p + strlen(p) : comma + 2;
        }
        fmpz_clear(x);
        if (result == 0 && count != r)
        {
            result = 1;
        }
        if (result == 1)
        {
            flint_printf("error in %s test, line %d: FLINT gives [", current_name, line_number);
            for (slong i = 0; i < r; i++)
            {
                if (i != 0)
                {
                    flint_printf(", ");
                }
                fmpz_print(ed + i);
            }
            flint_printf("]\n");
        }
    }
    _fmpz_vec_clear(ed, cap);
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
    if (current_kind == KIND_XGCD)
    {
        checked++;
        return check_xgcd(line, line_number);
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
    char * d_str = NULL;
    if (current_kind == KIND_ILIOPOULOS)
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
    if (current_kind == KIND_IS_IN_SNF)
    {
        int expected = strcmp(rest, "true") == 0;
        if (!expected && strcmp(rest, "false") != 0)
        {
            result = unrecognized(line_number);
        }
        else if (fmpz_mat_is_in_snf(a) != expected)
        {
            flint_printf("error in %s test, line %d: Malachite gives %s\n", current_name,
                         line_number, rest);
            result = 1;
        }
    }
    else if (current_kind == KIND_ELEMENTARY_DIVISORS)
    {
        result = check_elementary_divisors(a, rest, line_number);
    }
    else if (current_kind == KIND_TRANSFORM)
    {
        char * s_str;
        char * u_str;
        char * v_str;
        if (!split_matrix_triple(rest, &s_str, &u_str, &v_str))
        {
            result = unrecognized(line_number);
        }
        else
        {
            fmpz_mat_t s, u, v;
            fmpz_mat_init(s, m, n);
            fmpz_mat_init(u, m, m);
            fmpz_mat_init(v, n, n);
            fmpz_mat_snf_transform(s, u, v, a);
            result = compare_mat(s, s_str, "Smith normal forms", line_number);
            if (result == 0)
            {
                result = compare_mat(u, u_str, "left transforms", line_number);
            }
            if (result == 0)
            {
                result = compare_mat(v, v_str, "right transforms", line_number);
            }
            fmpz_mat_clear(s);
            fmpz_mat_clear(u);
            fmpz_mat_clear(v);
        }
    }
    else
    {
        fmpz_mat_t s;
        fmpz_mat_init(s, m, n);
        if (current_kind == KIND_SNF || current_kind == KIND_ITERATIVE)
        {
            fmpz_mat_snf(s, a);
        }
        else if (current_kind == KIND_DIAGONAL)
        {
            fmpz_mat_snf_diagonal(s, a);
        }
        else if (current_kind == KIND_KANNAN_BACHEM)
        {
            fmpz_mat_snf_kannan_bachem(s, a);
        }
        else
        {
            fmpz_t d;
            fmpz_init(d);
            if (fmpz_set_str(d, d_str, 10) != 0)
            {
                result = unreadable(line_number);
            }
            else
            {
                fmpz_mat_snf_iliopoulos(s, a, d);
            }
            fmpz_clear(d);
        }
        if (result == 0)
        {
            result = compare_mat(s, rest, "Smith normal forms", line_number);
        }
        fmpz_mat_clear(s);
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
run_fmpz_mat_snf(const char * arg)
{
    return run(arg, "fmpz_mat_snf", "smith_normal_form", KIND_SNF);
}

int
run_fmpz_mat_snf_diagonal(const char * arg)
{
    return run(arg, "fmpz_mat_snf_diagonal", "snf_diagonal", KIND_DIAGONAL);
}

int
run_fmpz_mat_snf_iterative_hermite(const char * arg)
{
    return run(arg, "fmpz_mat_snf_iterative_hermite", "snf_iterative_hermite", KIND_ITERATIVE);
}

int
run_fmpz_mat_snf_kannan_bachem(const char * arg)
{
    return run(arg, "fmpz_mat_snf_kannan_bachem", "snf_kannan_bachem", KIND_KANNAN_BACHEM);
}

int
run_fmpz_mat_snf_iliopoulos(const char * arg)
{
    return run(arg, "fmpz_mat_snf_iliopoulos", "snf_iliopoulos", KIND_ILIOPOULOS);
}

int
run_fmpz_mat_snf_transform(const char * arg)
{
    return run(arg, "fmpz_mat_snf_transform", "smith_normal_form_and_transforms",
               KIND_TRANSFORM);
}

int
run_fmpz_mat_is_in_snf(const char * arg)
{
    return run(arg, "fmpz_mat_is_in_snf", "is_in_smith_normal_form", KIND_IS_IN_SNF);
}

int
run_fmpz_mat_elementary_divisors(const char * arg)
{
    return run(arg, "fmpz_mat_elementary_divisors", "elementary_divisors",
               KIND_ELEMENTARY_DIVISORS);
}

int
run_fmpz_xgcd_canonical_bezout(const char * arg)
{
    return run(arg, "fmpz_xgcd_canonical_bezout", "", KIND_XGCD);
}

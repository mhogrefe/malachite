/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the IntegerMatrix linear algebra demos against FLINT's fmpz_mat
    functions.

    The fraction-free LU decomposition demo prints `A.fraction_free_lu_decomposition() = ([p_0,
    p_1, ...], L, U, den)`; the `fmpz_mat_fflu` mode recomputes the packed decomposition with
    `fmpz_mat_fflu` and unpacks it as Malachite does: L has the elements below and at each pivot in
    the pivot columns, and U has the pivot rows from each pivot rightward.

    The determinant demos print `determinant_bareiss(A) = d` and `determinant_cofactor(A) = d`, and
    the rref check demo prints `is_in_reduced_row_echelon_form_with_rank(A, den, rank) = b`, where
    `b` is `true` or `false`; the modes recompute the results with `fmpz_mat_det_bareiss`,
    `fmpz_mat_det_cofactor`, and `fmpz_mat_is_in_rref_with_rank`.

    Matrices are read by fmpz_mat_init_set_str_malachite in util.c. Each mode treats any other
    nonempty line as an error, and an input with no lines at all.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <flint/fmpz.h>
#include <flint/fmpz_mat.h>

#include "oracle.h"

static long checked;
static const char * current_name;
static const char * current_op;
static int current_kind;

enum { KIND_FFLU, KIND_DET_BAREISS, KIND_DET_COFACTOR, KIND_IS_IN_RREF };

static int
unrecognized(int line_number)
{
    flint_printf("error in %s test, line %d: unrecognized line\n", current_name, line_number);
    return 1;
}

static int
report_mat(const fmpz_mat_t actual, const fmpz_mat_t expected, int line_number)
{
    if (fmpz_mat_equal(actual, expected))
    {
        return 0;
    }
    flint_printf("error in %s test, line %d: the results differ\n", current_name, line_number);
    return 1;
}

static int
compare_fmpz(const fmpz_t actual, const char * expected_str, int line_number)
{
    fmpz_t expected;
    fmpz_init(expected);
    int result = 0;
    if (fmpz_set_str(expected, expected_str, 10) != 0)
    {
        flint_printf("error in %s test, line %d: unreadable result\n", current_name, line_number);
        result = 1;
    }
    else if (!fmpz_equal(actual, expected))
    {
        flint_printf("error in %s test, line %d: Malachite gives ", current_name, line_number);
        fmpz_print(expected);
        flint_printf(", but FLINT gives ");
        fmpz_print(actual);
        flint_printf("\n");
        result = 1;
    }
    fmpz_clear(expected);
    return result;
}

/* The length of the matrix string at the start of `s`: `[]_{h,w}` or `[...]`. */
static size_t
matrix_length(const char * s)
{
    const char * end = strncmp(s, "[]_{", 4) == 0 ? strchr(s, '}') : strchr(s, ']');
    return end == NULL ? 0 : (size_t) (end - s + 1);
}

static int
check_fflu(char * a_str, char * rest, int line_number)
{
    fmpz_mat_t a;
    if (!fmpz_mat_init_set_str_malachite(a, a_str))
    {
        flint_printf("error in %s test, line %d: unreadable input\n", current_name, line_number);
        return 1;
    }
    slong h = fmpz_mat_nrows(a), w = fmpz_mat_ncols(a);
    int result = 0;
    size_t len = strlen(rest);
    char * p_end;
    if (len < 2 || rest[0] != '(' || rest[len - 1] != ')' || rest[1] != '['
        || (p_end = strchr(rest + 2, ']')) == NULL || strncmp(p_end, "], ", 3) != 0)
    {
        fmpz_mat_clear(a);
        return unrecognized(line_number);
    }
    rest[len - 1] = '\0';
    *p_end = '\0';
    char * p_str = rest + 2;
    char * l_str = p_end + 3;
    size_t l_len = matrix_length(l_str);
    if (l_len == 0 || strncmp(l_str + l_len, ", ", 2) != 0)
    {
        fmpz_mat_clear(a);
        return unrecognized(line_number);
    }
    l_str[l_len] = '\0';
    char * u_str = l_str + l_len + 2;
    size_t u_len = matrix_length(u_str);
    if (u_len == 0 || strncmp(u_str + u_len, ", ", 2) != 0)
    {
        fmpz_mat_clear(a);
        return unrecognized(line_number);
    }
    u_str[u_len] = '\0';
    char * den_str = u_str + u_len + 2;
    fmpz_mat_t b;
    fmpz_t den;
    fmpz_mat_init(b, h, w);
    fmpz_init(den);
    slong * perm = flint_malloc(sizeof(slong) * FLINT_MAX(h, 1));
    for (slong i = 0; i < h; i++)
    {
        perm[i] = i;
    }
    slong rank = fmpz_mat_fflu(b, den, perm, a, 0);
    char * s = p_str;
    for (slong i = 0; i < h && result == 0; i++)
    {
        char * end;
        long long q = strtoll(s, &end, 10);
        if (end == s || q != perm[i])
        {
            flint_printf("error in %s test, line %d: permutation element %wd differs\n",
                         current_name, line_number, i);
            result = 1;
        }
        s = end;
        if (*s == ',')
        {
            s += 2;
        }
    }
    if (result == 0 && *s != '\0')
    {
        flint_printf("error in %s test, line %d: permutation too long\n", current_name,
                     line_number);
        result = 1;
    }
    if (result == 0)
    {
        slong * pivots = flint_malloc(sizeof(slong) * FLINT_MAX(rank, 1));
        slong col = 0;
        for (slong k = 0; k < rank; k++)
        {
            while (fmpz_is_zero(fmpz_mat_entry(b, k, col)))
            {
                col++;
            }
            pivots[k] = col++;
        }
        fmpz_mat_t l, u, l_expected, u_expected;
        fmpz_mat_init(l, h, rank);
        fmpz_mat_init(u, rank, w);
        for (slong i = 0; i < h; i++)
        {
            for (slong k = 0; k < rank && k <= i; k++)
            {
                fmpz_set(fmpz_mat_entry(l, i, k), fmpz_mat_entry(b, i, pivots[k]));
            }
        }
        for (slong k = 0; k < rank; k++)
        {
            for (slong j = pivots[k]; j < w; j++)
            {
                fmpz_set(fmpz_mat_entry(u, k, j), fmpz_mat_entry(b, k, j));
            }
        }
        int ok = fmpz_mat_init_set_str_malachite(l_expected, l_str);
        ok &= fmpz_mat_init_set_str_malachite(u_expected, u_str);
        if (!ok)
        {
            flint_printf("error in %s test, line %d: unreadable result\n", current_name,
                         line_number);
            result = 1;
        }
        else
        {
            result = report_mat(l, l_expected, line_number)
                || report_mat(u, u_expected, line_number)
                || compare_fmpz(den, den_str, line_number);
        }
        fmpz_mat_clear(l);
        fmpz_mat_clear(u);
        fmpz_mat_clear(l_expected);
        fmpz_mat_clear(u_expected);
        flint_free(pivots);
    }
    flint_free(perm);
    fmpz_clear(den);
    fmpz_mat_clear(b);
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
    char * eq;
    if (current_kind == KIND_FFLU)
    {
        char needle[64];
        snprintf(needle, sizeof(needle), ".%s() = ", current_op);
        char * at = strstr(line, needle);
        if (at == NULL)
        {
            return unrecognized(line_number);
        }
        *at = '\0';
        checked++;
        return check_fflu(line, at + strlen(needle), line_number);
    }
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
    int result = 0;
    if (current_kind == KIND_IS_IN_RREF)
    {
        size_t a_len = matrix_length(args);
        char * den_str;
        char * rank_str;
        if (a_len == 0 || strncmp(args + a_len, ", ", 2) != 0
            || (rank_str = strrchr(args, ',')) == args + a_len)
        {
            return unrecognized(line_number);
        }
        args[a_len] = '\0';
        den_str = args + a_len + 2;
        *rank_str = '\0';
        rank_str += 2;
        fmpz_mat_t a;
        fmpz_t den;
        fmpz_init(den);
        if (!fmpz_mat_init_set_str_malachite(a, args) || fmpz_set_str(den, den_str, 10) != 0)
        {
            fmpz_clear(den);
            flint_printf("error in %s test, line %d: unreadable input\n", current_name,
                         line_number);
            return 1;
        }
        int actual = fmpz_mat_is_in_rref_with_rank(a, den, atol(rank_str));
        int expected = strcmp(rest, "true") == 0;
        if (!expected && strcmp(rest, "false") != 0)
        {
            result = unrecognized(line_number);
        }
        else if (actual != expected)
        {
            flint_printf("error in %s test, line %d: Malachite gives %s, but FLINT gives %s\n",
                         current_name, line_number, rest, actual ? "true" : "false");
            result = 1;
        }
        fmpz_mat_clear(a);
        fmpz_clear(den);
        return result;
    }
    fmpz_mat_t a;
    if (!fmpz_mat_init_set_str_malachite(a, args))
    {
        flint_printf("error in %s test, line %d: unreadable input\n", current_name, line_number);
        return 1;
    }
    fmpz_t det;
    fmpz_init(det);
    if (current_kind == KIND_DET_BAREISS)
    {
        fmpz_mat_det_bareiss(det, a);
    }
    else
    {
        fmpz_mat_det_cofactor(det, a);
    }
    result = compare_fmpz(det, rest, line_number);
    fmpz_clear(det);
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
run_fmpz_mat_fflu(const char * arg)
{
    return run(arg, "fmpz_mat_fflu", "fraction_free_lu_decomposition", KIND_FFLU);
}

int
run_fmpz_mat_det_bareiss(const char * arg)
{
    return run(arg, "fmpz_mat_det_bareiss", "determinant_bareiss", KIND_DET_BAREISS);
}

int
run_fmpz_mat_det_cofactor(const char * arg)
{
    return run(arg, "fmpz_mat_det_cofactor", "determinant_cofactor", KIND_DET_COFACTOR);
}

int
run_fmpz_mat_is_in_rref_with_rank(const char * arg)
{
    return run(arg, "fmpz_mat_is_in_rref_with_rank", "is_in_reduced_row_echelon_form_with_rank",
               KIND_IS_IN_RREF);
}

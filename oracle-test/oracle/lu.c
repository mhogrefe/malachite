/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the LU decomposition, rank, and determinant demos against FLINT.

    The demos print `A.mod_lu_decomposition(m) = ([p_0, p_1, ...], L, U)`, `A.mod_rank(m) = r`, and
    `A.mod_determinant(m) = d`. The `nmod_mat_lu` mode recomputes the packed decomposition with
    `nmod_mat_lu` and unpacks it as Malachite does: L is made of the first `rank` columns of the
    packed matrix below the diagonal, with ones on it, and U of its first `rank` rows on and above
    the diagonal. The `nmod_mat_rank` and `nmod_mat_det` modes recompute the result with
    `nmod_mat_rank` and `nmod_mat_det`.

    Matrices are read by fmpz_mat_init_set_str_malachite in util.c. Each mode treats any other
    nonempty line as an error, and an input with no lines at all.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <flint/fmpz.h>
#include <flint/fmpz_mat.h>
#include <flint/nmod_mat.h>

#include "oracle.h"

static long checked;
static const char * current_name;
static const char * current_op;
static int current_kind;

enum { KIND_LU, KIND_RANK, KIND_DET };

/* Splits `A.<op>(m) = rest` into its parts, in place. Returns 1 on success. */
static int
split_line(char * line, char ** a, char ** m, char ** rest)
{
    char needle[64];
    line[strcspn(line, "\r\n")] = '\0';
    snprintf(needle, sizeof(needle), ".%s(", current_op);
    char * at = strstr(line, needle);
    if (at == NULL)
    {
        return 0;
    }
    *at = '\0';
    *a = line;
    *m = at + strlen(needle);
    at = strstr(*m, ") = ");
    if (at == NULL)
    {
        return 0;
    }
    *at = '\0';
    *rest = at + 4;
    return 1;
}

/* The length of the matrix string at the start of `s`: `[]_{h,w}` or `[...]`. */
static size_t
matrix_length(const char * s)
{
    if (strncmp(s, "[]_{", 4) == 0)
    {
        const char * end = strchr(s, '}');
        return end == NULL ? 0 : (size_t) (end - s + 1);
    }
    const char * end = strchr(s, ']');
    return end == NULL ? 0 : (size_t) (end - s + 1);
}

static int
report_matrix_difference(const char * what, const fmpz_mat_t actual, const fmpz_mat_t expected,
                         int line_number)
{
    if (fmpz_mat_nrows(actual) != fmpz_mat_nrows(expected)
        || fmpz_mat_ncols(actual) != fmpz_mat_ncols(expected))
    {
        flint_printf("error in %s test, line %d: FLINT's %s is %wd by %wd, Malachite's %wd by "
                     "%wd\n", current_name, line_number, what, fmpz_mat_nrows(actual),
                     fmpz_mat_ncols(actual), fmpz_mat_nrows(expected), fmpz_mat_ncols(expected));
        return 1;
    }
    if (!fmpz_mat_equal(actual, expected))
    {
        flint_printf("error in %s test, line %d: %s differs\n", current_name, line_number, what);
        return 1;
    }
    return 0;
}

static int
check_lu(const nmod_mat_t a, char * rest, int line_number)
{
    slong h = nmod_mat_nrows(a), w = nmod_mat_ncols(a);
    /* rest is ([p_0, ...], L, U) */
    size_t len = strlen(rest);
    if (len < 2 || rest[0] != '(' || rest[len - 1] != ')' || rest[1] != '[')
    {
        flint_printf("error in %s test, line %d: unrecognized result\n", current_name, line_number);
        return 1;
    }
    rest[len - 1] = '\0';
    char * p_str = rest + 2;
    char * p_end = strchr(p_str, ']');
    if (p_end == NULL || strncmp(p_end, "], ", 3) != 0)
    {
        flint_printf("error in %s test, line %d: unrecognized result\n", current_name, line_number);
        return 1;
    }
    *p_end = '\0';
    char * l_str = p_end + 3;
    size_t l_len = matrix_length(l_str);
    if (l_len == 0 || strncmp(l_str + l_len, ", ", 2) != 0)
    {
        flint_printf("error in %s test, line %d: unrecognized result\n", current_name, line_number);
        return 1;
    }
    l_str[l_len] = '\0';
    char * u_str = l_str + l_len + 2;
    nmod_mat_t lu;
    nmod_mat_init_set(lu, a);
    slong * perm = flint_malloc(sizeof(slong) * FLINT_MAX(h, 1));
    slong rank = nmod_mat_lu(perm, lu, 0);
    int result = 0;
    /* Compare the permutations. */
    char * s = p_str;
    for (slong i = 0; i < h && result == 0; i++)
    {
        char * end;
        long long p = strtoll(s, &end, 10);
        if (end == s || p != perm[i])
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
        flint_printf("error in %s test, line %d: permutation too long\n", current_name, line_number);
        result = 1;
    }
    if (result == 0)
    {
        fmpz_mat_t l, u, l_expected, u_expected;
        fmpz_mat_init(l, h, rank);
        fmpz_mat_init(u, rank, w);
        for (slong i = 0; i < h; i++)
        {
            for (slong c = 0; c < rank; c++)
            {
                ulong x = i > c ? nmod_mat_entry(lu, i, c) : (i == c ? 1 : 0);
                fmpz_set_ui(fmpz_mat_entry(l, i, c), x);
            }
        }
        for (slong i = 0; i < rank; i++)
        {
            for (slong j = 0; j < w; j++)
            {
                fmpz_set_ui(fmpz_mat_entry(u, i, j), j >= i ? nmod_mat_entry(lu, i, j) : 0);
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
            result = report_matrix_difference("L", l, l_expected, line_number)
                || report_matrix_difference("U", u, u_expected, line_number);
        }
        fmpz_mat_clear(l);
        fmpz_mat_clear(u);
        fmpz_mat_clear(l_expected);
        fmpz_mat_clear(u_expected);
    }
    flint_free(perm);
    nmod_mat_clear(lu);
    return result;
}

static int
check_line(char * line, int line_number)
{
    char * a_str;
    char * m_str;
    char * rest;
    if (!split_line(line, &a_str, &m_str, &rest))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in %s test, line %d: unrecognized line\n", current_name, line_number);
        return 1;
    }
    checked++;
    int result = 0;
    fmpz_mat_t a;
    fmpz_t m;
    fmpz_init(m);
    int ok = fmpz_mat_init_set_str_malachite(a, a_str);
    ok &= fmpz_set_str(m, m_str, 10) == 0 && fmpz_sgn(m) > 0 && fmpz_abs_fits_ui(m);
    if (!ok)
    {
        flint_printf("error in %s test, line %d: unreadable input\n", current_name, line_number);
        result = 1;
    }
    else
    {
        nmod_mat_t an;
        nmod_mat_init(an, fmpz_mat_nrows(a), fmpz_mat_ncols(a), fmpz_get_ui(m));
        fmpz_mat_get_nmod_mat(an, a);
        if (current_kind == KIND_LU)
        {
            result = check_lu(an, rest, line_number);
        }
        else
        {
            ulong actual = current_kind == KIND_RANK ? (ulong) nmod_mat_rank(an) : nmod_mat_det(an);
            char * end;
            ulong expected = strtoull(rest, &end, 10);
            if (end == rest || *end != '\0')
            {
                flint_printf("error in %s test, line %d: unreadable result\n", current_name,
                             line_number);
                result = 1;
            }
            else if (actual != expected)
            {
                flint_printf("error in %s test, line %d: Malachite gives %wu, but FLINT gives "
                             "%wu\n", current_name, line_number, expected, actual);
                result = 1;
            }
        }
        nmod_mat_clear(an);
    }
    fmpz_clear(m);
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
run_nmod_mat_lu(const char * arg)
{
    return run(arg, "nmod_mat_lu", "mod_lu_decomposition", KIND_LU);
}

int
run_nmod_mat_rank(const char * arg)
{
    return run(arg, "nmod_mat_rank", "mod_rank", KIND_RANK);
}

int
run_nmod_mat_det(const char * arg)
{
    return run(arg, "nmod_mat_det", "mod_determinant", KIND_DET);
}

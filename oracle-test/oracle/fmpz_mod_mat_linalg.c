/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the NaturalMatrix linear algebra demos against FLINT's fmpz_mod_mat
    functions.

    The triangular solving demos print `A.<op>(B, m, unit) = X`, where `<op>` is
    `mod_solve_lower_triangular` or `mod_solve_upper_triangular` and `unit` is `true` or `false`;
    the modes recompute `X` with `fmpz_mod_mat_solve_tril` or `fmpz_mod_mat_solve_triu`.

    The LU decomposition, rank, and determinant demos print `A.mod_lu_decomposition(m) = ([p_0,
    p_1, ...], L, U)`, `A.mod_rank(m) = r`, and `A.mod_determinant(m) = d`. The `fmpz_mod_mat_lu`
    mode recomputes the packed decomposition with `fmpz_mod_mat_lu` and unpacks it as Malachite
    does, and the `fmpz_mod_mat_rank` and `fmpz_mod_mat_det` modes recompute the result with
    `fmpz_mod_mat_rank` and `fmpz_mod_mat_det`.

    Matrices are read by fmpz_mat_init_set_str_malachite in util.c. Each mode treats any other
    nonempty line as an error, and an input with no lines at all.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <flint/fmpz.h>
#include <flint/fmpz_mat.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_mat.h>

#include "oracle.h"

static long checked;
static const char * current_name;
static const char * current_op;
static int current_kind;

enum { KIND_TRIL, KIND_TRIU, KIND_LU, KIND_RANK, KIND_DET };

static int
report(const fmpz_mat_t actual, const fmpz_mat_t expected, int line_number)
{
    if (fmpz_mat_nrows(actual) != fmpz_mat_nrows(expected)
        || fmpz_mat_ncols(actual) != fmpz_mat_ncols(expected))
    {
        flint_printf("error in %s test, line %d: FLINT's result is %wd by %wd, Malachite's %wd by "
                     "%wd\n", current_name, line_number, fmpz_mat_nrows(actual),
                     fmpz_mat_ncols(actual), fmpz_mat_nrows(expected), fmpz_mat_ncols(expected));
        return 1;
    }
    if (!fmpz_mat_equal(actual, expected))
    {
        flint_printf("error in %s test, line %d: the results differ\n", current_name, line_number);
        return 1;
    }
    return 0;
}

static int
check_triangular_line(char * line, int line_number)
{
    char needle[64];
    line[strcspn(line, "\r\n")] = '\0';
    snprintf(needle, sizeof(needle), ".%s(", current_op);
    char * at = strstr(line, needle);
    if (at == NULL)
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in %s test, line %d: unrecognized line\n", current_name, line_number);
        return 1;
    }
    *at = '\0';
    char * a_str = line;
    char * args = at + strlen(needle);
    char * eq = strstr(args, ") = ");
    if (eq == NULL)
    {
        flint_printf("error in %s test, line %d: unrecognized line\n", current_name, line_number);
        return 1;
    }
    *eq = '\0';
    char * x_str = eq + 4;
    /* args is "B, m, unit" */
    char * last = strrchr(args, ',');
    if (last == NULL)
    {
        flint_printf("error in %s test, line %d: unrecognized line\n", current_name, line_number);
        return 1;
    }
    *last = '\0';
    int unit = strcmp(last + 2, "true") == 0;
    char * mid = strrchr(args, ',');
    if (mid == NULL)
    {
        flint_printf("error in %s test, line %d: unrecognized line\n", current_name, line_number);
        return 1;
    }
    *mid = '\0';
    char * b_str = args;
    char * m_str = mid + 2;
    checked++;
    int result = 0;
    fmpz_mat_t a, b, expected;
    fmpz_t m;
    fmpz_init(m);
    int ok = fmpz_mat_init_set_str_malachite(a, a_str);
    ok &= fmpz_mat_init_set_str_malachite(b, b_str);
    ok &= fmpz_mat_init_set_str_malachite(expected, x_str);
    ok &= fmpz_set_str(m, m_str, 10) == 0 && fmpz_sgn(m) > 0;
    if (!ok)
    {
        flint_printf("error in %s test, line %d: unreadable line\n", current_name, line_number);
        result = 1;
    }
    else
    {
        fmpz_mod_ctx_t ctx;
        fmpz_mod_ctx_init(ctx, m);
        slong n = fmpz_mat_nrows(a), k = fmpz_mat_ncols(b);
        fmpz_mod_mat_t am, bm, xm;
        fmpz_mod_mat_init(am, n, n, ctx);
        fmpz_mod_mat_init(bm, n, k, ctx);
        fmpz_mod_mat_init(xm, n, k, ctx);
        fmpz_mod_mat_set_fmpz_mat(am, a, ctx);
        fmpz_mod_mat_set_fmpz_mat(bm, b, ctx);
        if (current_kind == KIND_TRIL)
        {
            fmpz_mod_mat_solve_tril(xm, am, bm, unit, ctx);
        }
        else
        {
            fmpz_mod_mat_solve_triu(xm, am, bm, unit, ctx);
        }
        fmpz_mat_t actual;
        fmpz_mat_init(actual, n, k);
        fmpz_mod_mat_get_fmpz_mat(actual, xm, ctx);
        result = report(actual, expected, line_number);
        fmpz_mat_clear(actual);
        fmpz_mod_mat_clear(am, ctx);
        fmpz_mod_mat_clear(bm, ctx);
        fmpz_mod_mat_clear(xm, ctx);
        fmpz_mod_ctx_clear(ctx);
    }
    fmpz_clear(m);
    fmpz_mat_clear(a);
    fmpz_mat_clear(b);
    fmpz_mat_clear(expected);
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
check_unary_line(char * line, int line_number)
{
    char needle[64];
    line[strcspn(line, "\r\n")] = '\0';
    snprintf(needle, sizeof(needle), ".%s(", current_op);
    char * at = strstr(line, needle);
    if (at == NULL)
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in %s test, line %d: unrecognized line\n", current_name, line_number);
        return 1;
    }
    *at = '\0';
    char * a_str = line;
    char * m_str = at + strlen(needle);
    char * eq = strstr(m_str, ") = ");
    if (eq == NULL)
    {
        flint_printf("error in %s test, line %d: unrecognized line\n", current_name, line_number);
        return 1;
    }
    *eq = '\0';
    char * rest = eq + 4;
    checked++;
    int result = 0;
    fmpz_mat_t a;
    fmpz_t m;
    fmpz_init(m);
    int ok = fmpz_mat_init_set_str_malachite(a, a_str);
    ok &= fmpz_set_str(m, m_str, 10) == 0 && fmpz_sgn(m) > 0;
    if (!ok)
    {
        flint_printf("error in %s test, line %d: unreadable input\n", current_name, line_number);
        fmpz_clear(m);
        fmpz_mat_clear(a);
        return 1;
    }
    fmpz_mod_ctx_t ctx;
    fmpz_mod_ctx_init(ctx, m);
    slong h = fmpz_mat_nrows(a), w = fmpz_mat_ncols(a);
    fmpz_mod_mat_t am;
    fmpz_mod_mat_init(am, h, w, ctx);
    fmpz_mod_mat_set_fmpz_mat(am, a, ctx);
    if (current_kind == KIND_LU)
    {
        size_t len = strlen(rest);
        char * p_end;
        char * l_str;
        size_t l_len;
        if (len < 2 || rest[0] != '(' || rest[len - 1] != ')' || rest[1] != '['
            || (p_end = strchr(rest + 2, ']')) == NULL || strncmp(p_end, "], ", 3) != 0)
        {
            flint_printf("error in %s test, line %d: unrecognized result\n", current_name,
                         line_number);
            result = 1;
        }
        else
        {
            rest[len - 1] = '\0';
            *p_end = '\0';
            char * p_str = rest + 2;
            l_str = p_end + 3;
            l_len = matrix_length(l_str);
            if (l_len == 0 || strncmp(l_str + l_len, ", ", 2) != 0)
            {
                flint_printf("error in %s test, line %d: unrecognized result\n", current_name,
                             line_number);
                result = 1;
            }
            else
            {
                l_str[l_len] = '\0';
                char * u_str = l_str + l_len + 2;
                slong * perm = flint_malloc(sizeof(slong) * FLINT_MAX(h, 1));
                slong rank = fmpz_mod_mat_lu(perm, am, 0, ctx);
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
                    flint_printf("error in %s test, line %d: permutation too long\n",
                                 current_name, line_number);
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
                            if (i > c)
                            {
                                fmpz_set(fmpz_mat_entry(l, i, c), fmpz_mod_mat_entry(am, i, c));
                            }
                            else if (i == c)
                            {
                                fmpz_one(fmpz_mat_entry(l, i, c));
                            }
                        }
                    }
                    for (slong i = 0; i < rank; i++)
                    {
                        for (slong j = i; j < w; j++)
                        {
                            fmpz_set(fmpz_mat_entry(u, i, j), fmpz_mod_mat_entry(am, i, j));
                        }
                    }
                    int ok2 = fmpz_mat_init_set_str_malachite(l_expected, l_str);
                    ok2 &= fmpz_mat_init_set_str_malachite(u_expected, u_str);
                    if (!ok2)
                    {
                        flint_printf("error in %s test, line %d: unreadable result\n",
                                     current_name, line_number);
                        result = 1;
                    }
                    else
                    {
                        result = report(l, l_expected, line_number)
                            || report(u, u_expected, line_number);
                    }
                    fmpz_mat_clear(l);
                    fmpz_mat_clear(u);
                    fmpz_mat_clear(l_expected);
                    fmpz_mat_clear(u_expected);
                }
                flint_free(perm);
            }
        }
    }
    else
    {
        fmpz_t actual, expected;
        fmpz_init(actual);
        fmpz_init(expected);
        if (current_kind == KIND_RANK)
        {
            fmpz_set_si(actual, fmpz_mod_mat_rank(am, ctx));
        }
        else
        {
            fmpz_mod_mat_det(actual, am, ctx);
        }
        if (fmpz_set_str(expected, rest, 10) != 0)
        {
            flint_printf("error in %s test, line %d: unreadable result\n", current_name,
                         line_number);
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
        fmpz_clear(actual);
        fmpz_clear(expected);
    }
    fmpz_mod_mat_clear(am, ctx);
    fmpz_mod_ctx_clear(ctx);
    fmpz_clear(m);
    fmpz_mat_clear(a);
    return result;
}

static int
run(const char * arg, const char * name, const char * op, int kind,
    int (*check)(char *, int))
{
    checked = 0;
    current_name = name;
    current_op = op;
    current_kind = kind;
    int result = for_each_line(arg, check);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_fmpz_mod_mat_solve_tril(const char * arg)
{
    return run(arg, "fmpz_mod_mat_solve_tril", "mod_solve_lower_triangular", KIND_TRIL,
               check_triangular_line);
}

int
run_fmpz_mod_mat_solve_triu(const char * arg)
{
    return run(arg, "fmpz_mod_mat_solve_triu", "mod_solve_upper_triangular", KIND_TRIU,
               check_triangular_line);
}

int
run_fmpz_mod_mat_lu(const char * arg)
{
    return run(arg, "fmpz_mod_mat_lu", "mod_lu_decomposition", KIND_LU, check_unary_line);
}

int
run_fmpz_mod_mat_rank(const char * arg)
{
    return run(arg, "fmpz_mod_mat_rank", "mod_rank", KIND_RANK, check_unary_line);
}

int
run_fmpz_mod_mat_det(const char * arg)
{
    return run(arg, "fmpz_mod_mat_det", "mod_determinant", KIND_DET, check_unary_line);
}

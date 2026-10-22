/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the solving and inverse demos against FLINT.

    The demos print `A.mod_solve(B, m) = R` and `A.mod_inverse(m) = R`, where `R` is `None` or
    `Some(X)`, and `B` and `X` are matrices, or, in the `nmod_mat_can_solve_vec` mode, vectors. The
    solving modes recompute the solution with `nmod_mat_can_solve`, treating a vector as a matrix
    with one column; like Malachite, FLINT returns the solution whose elements in the rows of the
    non-pivot columns are zero, so the solutions must be equal. The `nmod_mat_inv` mode recomputes
    the inverse with `nmod_mat_inv`. Modulo 1, every matrix is invertible, its inverse being itself,
    the zero matrix; `nmod_mat_inv` reports failure for such matrices of dimension at least 1, so
    the expected result there is computed here instead.

    Matrices are read by fmpz_mat_init_set_str_malachite and vectors by fmpz_vec_set_str_malachite,
    in util.c. Each mode treats any other nonempty line as an error, and an input with no lines at
    all.
*/

#include <stdio.h>
#include <string.h>

#include <flint/fmpz.h>
#include <flint/fmpz_mat.h>
#include <flint/fmpz_vec.h>
#include <flint/nmod_mat.h>

#include "oracle.h"

static long checked;
static const char * current_name;
static int current_inverse;
static int current_vector;

/* Reads a matrix, or, if `current_vector`, a vector as a matrix with one column. */
static int
read_matrix(fmpz_mat_t mat, const char * s)
{
    if (!current_vector)
    {
        return fmpz_mat_init_set_str_malachite(mat, s);
    }
    slong len;
    fmpz * vec = fmpz_vec_set_str_malachite(&len, s);
    if (vec == NULL)
    {
        fmpz_mat_init(mat, 0, 1);
        return 0;
    }
    fmpz_mat_init(mat, len, 1);
    for (slong i = 0; i < len; i++)
    {
        fmpz_set(fmpz_mat_entry(mat, i, 0), vec + i);
    }
    _fmpz_vec_clear(vec, len);
    return 1;
}

/* Reads `None` or `Some(X)`. Returns 1 on success, setting `*some`. */
static int
read_result(fmpz_mat_t mat, char * s, int * some)
{
    if (strcmp(s, "None") == 0)
    {
        *some = 0;
        fmpz_mat_init(mat, 0, 0);
        return 1;
    }
    size_t len = strlen(s);
    if (strncmp(s, "Some(", 5) != 0 || len < 6 || s[len - 1] != ')')
    {
        fmpz_mat_init(mat, 0, 0);
        return 0;
    }
    s[len - 1] = '\0';
    *some = 1;
    return read_matrix(mat, s + 5);
}

static int
check_line(char * line, int line_number)
{
    line[strcspn(line, "\r\n")] = '\0';
    const char * needle = current_inverse ? ".mod_inverse(" : ".mod_solve(";
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
    char * result_str = eq + 4;
    char * b_str = NULL;
    char * m_str = args;
    if (!current_inverse)
    {
        char * comma = strrchr(args, ',');
        if (comma == NULL || comma[1] != ' ')
        {
            flint_printf("error in %s test, line %d: unrecognized line\n", current_name,
                         line_number);
            return 1;
        }
        *comma = '\0';
        b_str = args;
        m_str = comma + 2;
    }
    checked++;
    int result = 0;
    fmpz_t m;
    fmpz_init(m);
    fmpz_mat_t a, b, expected;
    int some = 0;
    int ok = fmpz_mat_init_set_str_malachite(a, a_str);
    if (current_inverse)
    {
        fmpz_mat_init(b, 0, 0);
    }
    else
    {
        ok &= read_matrix(b, b_str);
    }
    ok &= read_result(expected, result_str, &some);
    ok &= fmpz_set_str(m, m_str, 10) == 0 && fmpz_sgn(m) > 0 && fmpz_abs_fits_ui(m);
    if (!ok)
    {
        flint_printf("error in %s test, line %d: unreadable line\n", current_name, line_number);
        result = 1;
    }
    else
    {
        ulong n = fmpz_get_ui(m);
        slong h = fmpz_mat_nrows(a), w = fmpz_mat_ncols(a);
        nmod_mat_t an, x;
        nmod_mat_init(an, h, w, n);
        fmpz_mat_get_nmod_mat(an, a);
        int solvable;
        if (current_inverse)
        {
            nmod_mat_init(x, h, w, n);
            if (n == 1)
            {
                solvable = 1;
            }
            else
            {
                solvable = nmod_mat_inv(x, an);
            }
        }
        else
        {
            slong k = fmpz_mat_ncols(b);
            nmod_mat_t bn;
            nmod_mat_init(bn, fmpz_mat_nrows(b), k, n);
            fmpz_mat_get_nmod_mat(bn, b);
            nmod_mat_init(x, w, k, n);
            solvable = nmod_mat_can_solve(x, an, bn);
            nmod_mat_clear(bn);
        }
        if (solvable != some)
        {
            flint_printf("error in %s test, line %d: FLINT %s a solution, but Malachite %s\n",
                         current_name, line_number, solvable ? "finds" : "does not find",
                         some ? "does" : "does not");
            result = 1;
        }
        else if (solvable)
        {
            fmpz_mat_t actual;
            fmpz_mat_init(actual, nmod_mat_nrows(x), nmod_mat_ncols(x));
            fmpz_mat_set_nmod_mat_unsigned(actual, x);
            if (!fmpz_mat_equal(actual, expected))
            {
                flint_printf("error in %s test, line %d: the solutions differ\n", current_name,
                             line_number);
                result = 1;
            }
            fmpz_mat_clear(actual);
        }
        nmod_mat_clear(x);
        nmod_mat_clear(an);
    }
    fmpz_clear(m);
    fmpz_mat_clear(a);
    fmpz_mat_clear(b);
    fmpz_mat_clear(expected);
    return result;
}

static int
run(const char * arg, const char * name, int inverse, int vector)
{
    checked = 0;
    current_name = name;
    current_inverse = inverse;
    current_vector = vector;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_nmod_mat_can_solve(const char * arg)
{
    return run(arg, "nmod_mat_can_solve", 0, 0);
}

int
run_nmod_mat_can_solve_vec(const char * arg)
{
    return run(arg, "nmod_mat_can_solve_vec", 0, 1);
}

int
run_nmod_mat_inv(const char * arg)
{
    return run(arg, "nmod_mat_inv", 1, 0);
}

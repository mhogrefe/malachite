/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the reduced row echelon form and nullspace demos against FLINT.

    The demos print `A.<op>(m) = C`, `(&A).<op>(m) = C`, and `a := A; a.<op>_assign(m); a = C`. The
    `nmod_mat_rref` mode recomputes `C` with `nmod_mat_rref`. The `nmod_mat_nullspace` mode
    recomputes it with `nmod_mat_nullspace`, keeping the first `nullity` columns of FLINT's square
    result, and the `nmod_mat_left_nullspace` mode with `nmod_mat_left_nullspace`, keeping the first
    `nullity` rows. Modulo 1, FLINT's basis vectors have elements equal to 1, which are not reduced;
    Malachite's are 0, so FLINT's are reduced here before comparing.

    Matrices are read by fmpz_mat_init_set_str_malachite in util.c. Each mode treats any other
    nonempty line as an error, and an input with no lines at all.
*/

#include <stdio.h>
#include <string.h>

#include <flint/fmpz.h>
#include <flint/fmpz_mat.h>
#include <flint/nmod_mat.h>

#include "oracle.h"

static long checked;
static const char * current_name;
static const char * current_op;
static int current_kind;

enum { KIND_RREF, KIND_NULLSPACE, KIND_LEFT_NULLSPACE };

static char *
cut(char * line, const char * needle)
{
    char * at = strstr(line, needle);
    if (at == NULL)
    {
        return NULL;
    }
    *at = '\0';
    return at + strlen(needle);
}

/* Splits a line into its matrix, its modulus, and its result, in place. Returns 1 on success. */
static int
split_line(char * line, char ** a, char ** m, char ** c)
{
    char needle[64];
    line[strcspn(line, "\r\n")] = '\0';
    if (strncmp(line, "a := ", 5) == 0)
    {
        *a = line + 5;
        snprintf(needle, sizeof(needle), "; a.%s_assign(", current_op);
        *m = cut(*a, needle);
        if (*m == NULL)
        {
            return 0;
        }
        *c = cut(*m, "); a = ");
        return *c != NULL;
    }
    snprintf(needle, sizeof(needle), ".%s(", current_op);
    *m = cut(line, needle);
    if (*m == NULL)
    {
        return 0;
    }
    *c = cut(*m, ") = ");
    *a = line;
    if (strncmp(*a, "(&", 2) == 0)
    {
        size_t len = strlen(*a);
        if (len < 3 || (*a)[len - 1] != ')')
        {
            return 0;
        }
        (*a)[len - 1] = '\0';
        *a += 2;
    }
    return *c != NULL;
}

static int
check_line(char * line, int line_number)
{
    char * a_str;
    char * m_str;
    char * c_str;
    if (!split_line(line, &a_str, &m_str, &c_str))
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
    fmpz_mat_t a, expected;
    fmpz_t m;
    fmpz_init(m);
    int ok = fmpz_mat_init_set_str_malachite(a, a_str);
    ok &= fmpz_mat_init_set_str_malachite(expected, c_str);
    ok &= fmpz_set_str(m, m_str, 10) == 0 && fmpz_sgn(m) > 0 && fmpz_abs_fits_ui(m);
    if (!ok)
    {
        flint_printf("error in %s test, line %d: unreadable input\n", current_name, line_number);
        result = 1;
    }
    else
    {
        ulong n = fmpz_get_ui(m);
        slong h = fmpz_mat_nrows(a), w = fmpz_mat_ncols(a);
        nmod_mat_t an;
        nmod_mat_init(an, h, w, n);
        fmpz_mat_get_nmod_mat(an, a);
        fmpz_mat_t actual;
        if (current_kind == KIND_RREF)
        {
            nmod_mat_rref(an);
            fmpz_mat_init(actual, h, w);
            fmpz_mat_set_nmod_mat_unsigned(actual, an);
        }
        else if (current_kind == KIND_NULLSPACE)
        {
            nmod_mat_t x;
            nmod_mat_init(x, w, w, n);
            slong nullity = nmod_mat_nullspace(x, an);
            fmpz_mat_init(actual, w, nullity);
            for (slong i = 0; i < w; i++)
            {
                for (slong j = 0; j < nullity; j++)
                {
                    fmpz_set_ui(fmpz_mat_entry(actual, i, j), nmod_mat_entry(x, i, j) % n);
                }
            }
            nmod_mat_clear(x);
        }
        else
        {
            nmod_mat_t x;
            nmod_mat_init(x, h, h, n);
            slong nullity = nmod_mat_left_nullspace(x, an);
            fmpz_mat_init(actual, nullity, h);
            for (slong i = 0; i < nullity; i++)
            {
                for (slong j = 0; j < h; j++)
                {
                    fmpz_set_ui(fmpz_mat_entry(actual, i, j), nmod_mat_entry(x, i, j) % n);
                }
            }
            nmod_mat_clear(x);
        }
        if (fmpz_mat_nrows(actual) != fmpz_mat_nrows(expected)
            || fmpz_mat_ncols(actual) != fmpz_mat_ncols(expected))
        {
            flint_printf("error in %s test, line %d: FLINT's result is %wd by %wd, Malachite's %wd "
                         "by %wd\n", current_name, line_number, fmpz_mat_nrows(actual),
                         fmpz_mat_ncols(actual), fmpz_mat_nrows(expected),
                         fmpz_mat_ncols(expected));
            result = 1;
        }
        else if (!fmpz_mat_equal(actual, expected))
        {
            flint_printf("error in %s test, line %d: the results differ\n", current_name,
                         line_number);
            result = 1;
        }
        fmpz_mat_clear(actual);
        nmod_mat_clear(an);
    }
    fmpz_clear(m);
    fmpz_mat_clear(a);
    fmpz_mat_clear(expected);
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
run_nmod_mat_rref(const char * arg)
{
    return run(arg, "nmod_mat_rref", "mod_reduced_row_echelon_form", KIND_RREF);
}

int
run_nmod_mat_nullspace(const char * arg)
{
    return run(arg, "nmod_mat_nullspace", "mod_nullspace", KIND_NULLSPACE);
}

int
run_nmod_mat_left_nullspace(const char * arg)
{
    return run(arg, "nmod_mat_left_nullspace", "mod_left_nullspace", KIND_LEFT_NULLSPACE);
}

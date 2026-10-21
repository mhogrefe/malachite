/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the Howell and strong echelon form demos against FLINT.

    The demos print `A.<op>(m) = C`, `(&A).<op>(m) = C`, and `a := A; a.<op>_assign(m); a = C`,
    where `<op>` is `mod_howell_form` or `mod_strong_echelon_form`. The `nmod_mat_*` modes recompute
    `C` with `nmod_mat_howell_form` or `nmod_mat_strong_echelon_form`, and the `fmpz_mod_mat_*`
    modes with `fmpz_mat_howell_form_mod` or `fmpz_mat_strong_echelon_form_mod`, which
    `fmpz_mod_mat_howell_form` and `fmpz_mod_mat_strong_echelon_form` call. FLINT requires at least
    as many rows as columns; Malachite appends zero rows to a wider matrix, which leaves its row span
    unchanged, so the same padding is applied here before calling FLINT.

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
static int current_howell;
static int current_nmod;

/* Cuts `line` at `needle`, returning the text after it, or NULL if the needle is absent. */
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

/* Reports the first element in which `actual` differs from `expected`, or a shape mismatch.
   Returns 1 if they differ. */
static int
report_difference(const fmpz_mat_t actual, const fmpz_mat_t expected, int line_number)
{
    if (fmpz_mat_nrows(actual) != fmpz_mat_nrows(expected)
        || fmpz_mat_ncols(actual) != fmpz_mat_ncols(expected))
    {
        flint_printf("error in %s test, line %d: FLINT's form is %wd by %wd, Malachite's %wd by "
                     "%wd\n", current_name, line_number, fmpz_mat_nrows(actual),
                     fmpz_mat_ncols(actual), fmpz_mat_nrows(expected), fmpz_mat_ncols(expected));
        return 1;
    }
    for (slong i = 0; i < fmpz_mat_nrows(actual); i++)
    {
        for (slong j = 0; j < fmpz_mat_ncols(actual); j++)
        {
            if (!fmpz_equal(fmpz_mat_entry(actual, i, j), fmpz_mat_entry(expected, i, j)))
            {
                flint_printf("error in %s test, line %d: element (%wd, %wd) is ", current_name,
                             line_number, i, j);
                fmpz_print(fmpz_mat_entry(expected, i, j));
                flint_printf(", but FLINT gives ");
                fmpz_print(fmpz_mat_entry(actual, i, j));
                flint_printf("\n");
                return 1;
            }
        }
    }
    return 0;
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
    ok &= fmpz_set_str(m, m_str, 10) == 0 && fmpz_sgn(m) > 0;
    if (current_nmod)
    {
        ok &= fmpz_abs_fits_ui(m);
    }
    if (!ok)
    {
        flint_printf("error in %s test, line %d: unreadable input\n", current_name, line_number);
        result = 1;
    }
    else
    {
        slong rows = FLINT_MAX(fmpz_mat_nrows(a), fmpz_mat_ncols(a));
        slong cols = fmpz_mat_ncols(a);
        fmpz_mat_t padded;
        fmpz_mat_init(padded, rows, cols);
        for (slong i = 0; i < fmpz_mat_nrows(a); i++)
        {
            for (slong j = 0; j < cols; j++)
            {
                fmpz_set(fmpz_mat_entry(padded, i, j), fmpz_mat_entry(a, i, j));
            }
        }
        if (current_nmod)
        {
            nmod_mat_t an;
            nmod_mat_init(an, rows, cols, fmpz_get_ui(m));
            fmpz_mat_get_nmod_mat(an, padded);
            if (current_howell)
            {
                nmod_mat_howell_form(an);
            }
            else
            {
                nmod_mat_strong_echelon_form(an);
            }
            fmpz_mat_set_nmod_mat_unsigned(padded, an);
            nmod_mat_clear(an);
        }
        else
        {
            if (current_howell)
            {
                fmpz_mat_howell_form_mod(padded, m);
            }
            else
            {
                fmpz_mat_strong_echelon_form_mod(padded, m);
            }
        }
        result = report_difference(padded, expected, line_number);
        fmpz_mat_clear(padded);
    }
    fmpz_clear(m);
    fmpz_mat_clear(a);
    fmpz_mat_clear(expected);
    return result;
}

static int
run(const char * arg, const char * name, const char * op, int howell, int nmod)
{
    checked = 0;
    current_name = name;
    current_op = op;
    current_howell = howell;
    current_nmod = nmod;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_nmod_mat_howell_form(const char * arg)
{
    return run(arg, "nmod_mat_howell_form", "mod_howell_form", 1, 1);
}

int
run_nmod_mat_strong_echelon_form(const char * arg)
{
    return run(arg, "nmod_mat_strong_echelon_form", "mod_strong_echelon_form", 0, 1);
}

int
run_fmpz_mod_mat_howell_form(const char * arg)
{
    return run(arg, "fmpz_mod_mat_howell_form", "mod_howell_form", 1, 0);
}

int
run_fmpz_mod_mat_strong_echelon_form(const char * arg)
{
    return run(arg, "fmpz_mod_mat_strong_echelon_form", "mod_strong_echelon_form", 0, 0);
}

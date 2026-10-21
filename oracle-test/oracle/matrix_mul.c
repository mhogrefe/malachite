/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the matrix multiplication demos against FLINT.

    The `fmpz_mat_mul` mode reads the IntegerMatrix and NaturalMatrix product demos, whose shapes
    are `A * B = C`, `A * &B = C`, `&A * B = C`, `&A * &B = C`, `a := A; a *= B; a = C`, and
    `a := A; a *= &B; a = C`, and recomputes each product with fmpz_mat_mul.

    The `nmod_mat_mul` mode reads the UnsignedMatrix modular product demos, whose shapes are
    `A.mod_mul(B, m) = C` and `a := A; a.mod_mul_assign(B, m); a = C`, and recomputes each product
    with nmod_mat_mul.

    Matrices are read by fmpz_mat_init_set_str_malachite in util.c. Each mode treats any other
    nonempty line as an error, and an input with no lines at all.
*/

#include <stdio.h>
#include <string.h>

#include <flint/fmpz_mat.h>
#include <flint/nmod_mat.h>

#include "oracle.h"

static long checked;
static const char * current_name;

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

/* Skips a leading `&`, which marks an operand taken by reference. */
static char *
skip_ref(char * s)
{
    return s[0] == '&' ? s + 1 : s;
}

/* Reports the first element in which `actual` differs from `expected`, or a shape mismatch.
   Returns 1 if they differ. */
static int
report_difference(const fmpz_mat_t actual, const fmpz_mat_t expected, int line_number)
{
    if (fmpz_mat_nrows(actual) != fmpz_mat_nrows(expected)
        || fmpz_mat_ncols(actual) != fmpz_mat_ncols(expected))
    {
        flint_printf("error in %s test, line %d: FLINT's product is %wd by %wd, Malachite's %wd "
                     "by %wd\n", current_name, line_number, fmpz_mat_nrows(actual),
                     fmpz_mat_ncols(actual), fmpz_mat_nrows(expected),
                     fmpz_mat_ncols(expected));
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

/* Splits a product line into its two operands and its result, in place. Returns 1 on success. */
static int
split_mul_line(char * line, char ** a, char ** b, char ** c)
{
    line[strcspn(line, "\r\n")] = '\0';
    if (strncmp(line, "a := ", 5) == 0)
    {
        *a = line + 5;
        *b = cut(*a, "; a *= ");
        if (*b == NULL)
        {
            return 0;
        }
        *c = cut(*b, "; a = ");
        *b = skip_ref(*b);
        return *c != NULL;
    }
    /* Matrices are displayed without ` * ` or ` = ` inside them. */
    *b = cut(line, " * ");
    if (*b == NULL)
    {
        return 0;
    }
    *c = cut(*b, " = ");
    *a = skip_ref(line);
    *b = skip_ref(*b);
    return *c != NULL;
}

static int
check_fmpz_mat_mul_line(char * line, int line_number)
{
    char * a_str;
    char * b_str;
    char * c_str;
    if (!split_mul_line(line, &a_str, &b_str, &c_str))
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
    fmpz_mat_t a, b, expected;
    int ok = fmpz_mat_init_set_str_malachite(a, a_str);
    ok &= fmpz_mat_init_set_str_malachite(b, b_str);
    ok &= fmpz_mat_init_set_str_malachite(expected, c_str);
    if (!ok || fmpz_mat_ncols(a) != fmpz_mat_nrows(b))
    {
        flint_printf("error in %s test, line %d: unreadable input\n", current_name, line_number);
        result = 1;
    }
    else
    {
        fmpz_mat_t c;
        fmpz_mat_init(c, fmpz_mat_nrows(a), fmpz_mat_ncols(b));
        fmpz_mat_mul(c, a, b);
        result = report_difference(c, expected, line_number);
        fmpz_mat_clear(c);
    }
    fmpz_mat_clear(a);
    fmpz_mat_clear(b);
    fmpz_mat_clear(expected);
    return result;
}

/* Splits a modular product line into its operands, its modulus, and its result, in place.
   Returns 1 on success. */
static int
split_mod_mul_line(char * line, char ** a, char ** b, char ** m, char ** c)
{
    line[strcspn(line, "\r\n")] = '\0';
    if (strncmp(line, "a := ", 5) == 0)
    {
        *a = line + 5;
        *b = cut(*a, "; a.mod_mul_assign(");
        if (*b == NULL)
        {
            return 0;
        }
        *c = cut(*b, "); a = ");
    }
    else
    {
        *b = cut(line, ".mod_mul(");
        if (*b == NULL)
        {
            return 0;
        }
        *c = cut(*b, ") = ");
        *a = line;
    }
    if (*c == NULL)
    {
        return 0;
    }
    /* The modulus follows the last `, ` of the arguments; the matrix has none after its `]`. */
    char * comma = strrchr(*b, ',');
    if (comma == NULL || comma[1] != ' ')
    {
        return 0;
    }
    *comma = '\0';
    *m = comma + 2;
    *a = skip_ref(*a);
    *b = skip_ref(*b);
    return 1;
}

static int
check_nmod_mat_mul_line(char * line, int line_number)
{
    char * a_str;
    char * b_str;
    char * m_str;
    char * c_str;
    if (!split_mod_mul_line(line, &a_str, &b_str, &m_str, &c_str))
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
    fmpz_mat_t a, b, expected;
    fmpz_t m;
    fmpz_init(m);
    int ok = fmpz_mat_init_set_str_malachite(a, a_str);
    ok &= fmpz_mat_init_set_str_malachite(b, b_str);
    ok &= fmpz_mat_init_set_str_malachite(expected, c_str);
    ok &= fmpz_set_str(m, m_str, 10) == 0 && fmpz_sgn(m) > 0 && fmpz_abs_fits_ui(m);
    if (!ok || fmpz_mat_ncols(a) != fmpz_mat_nrows(b))
    {
        flint_printf("error in %s test, line %d: unreadable input\n", current_name, line_number);
        result = 1;
    }
    else
    {
        ulong n = fmpz_get_ui(m);
        nmod_mat_t an, bn, cn;
        nmod_mat_init(an, fmpz_mat_nrows(a), fmpz_mat_ncols(a), n);
        nmod_mat_init(bn, fmpz_mat_nrows(b), fmpz_mat_ncols(b), n);
        nmod_mat_init(cn, fmpz_mat_nrows(a), fmpz_mat_ncols(b), n);
        fmpz_mat_get_nmod_mat(an, a);
        fmpz_mat_get_nmod_mat(bn, b);
        nmod_mat_mul(cn, an, bn);
        fmpz_mat_t c;
        fmpz_mat_init(c, fmpz_mat_nrows(a), fmpz_mat_ncols(b));
        fmpz_mat_set_nmod_mat_unsigned(c, cn);
        result = report_difference(c, expected, line_number);
        fmpz_mat_clear(c);
        nmod_mat_clear(an);
        nmod_mat_clear(bn);
        nmod_mat_clear(cn);
    }
    fmpz_clear(m);
    fmpz_mat_clear(a);
    fmpz_mat_clear(b);
    fmpz_mat_clear(expected);
    return result;
}

static int
run(const char * arg, int (* handler)(char * line, int line_number), const char * name)
{
    checked = 0;
    current_name = name;
    int result = for_each_line(arg, handler);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_fmpz_mat_mul(const char * arg)
{
    return run(arg, check_fmpz_mat_mul_line, "fmpz_mat_mul");
}

int
run_nmod_mat_mul(const char * arg)
{
    return run(arg, check_nmod_mat_mul_line, "nmod_mat_mul");
}

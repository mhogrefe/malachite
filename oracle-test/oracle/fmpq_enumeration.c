/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `(x).farey_neighbors(n) = (l, r)` lines against fmpq_farey_neighbors, and the terms of
    the minimal-height enumerations against fmpq_next_minimal and fmpq_next_signed_minimal. The
    enumeration modes are stateful: the terms arrive in order, and FLINT is stepped alongside
    them, so term i is compared against FLINT's ith successor of zero.
*/

#include <string.h>

#include <flint/fmpq.h>

#include "oracle.h"

static int
check_farey_line(char * line, int line_number)
{
    fmpq_t x, l, r, el, er;
    fmpz_t big_n;
    fmpq_init(x); fmpq_init(l); fmpq_init(r); fmpq_init(el); fmpq_init(er);
    fmpz_init(big_n);
    int result = 0;
    char * needle = strstr(line, ").farey_neighbors(");
    if (line[0] == '(' && needle != NULL)
    {
        *needle = '\0';
        char * n_str = needle + strlen(").farey_neighbors(");
        char * close = strstr(n_str, ") = (");
        char * l_str = NULL;
        char * r_str = NULL;
        int ok = close != NULL;
        if (ok)
        {
            *close = '\0';
            l_str = close + strlen(") = (");
            char * sep = strstr(l_str, ", ");
            ok = sep != NULL;
            if (ok)
            {
                *sep = '\0';
                r_str = sep + 2;
                char * end = strrchr(r_str, ')');
                ok = end != NULL;
                if (ok)
                {
                    *end = '\0';
                }
            }
        }
        ok = ok && fmpq_set_str(x, line + 1, 10) == 0
            && fmpz_set_str(big_n, n_str, 10) == 0
            && fmpq_set_str(el, l_str, 10) == 0
            && fmpq_set_str(er, r_str, 10) == 0;
        if (!ok)
        {
            flint_printf("error in fmpq_farey_neighbors test, line %d: unreadable line\n",
                         line_number);
            result = 1;
        }
        else
        {
            fmpq_farey_neighbors(l, r, x, big_n);
            if (!fmpq_equal(l, el) || !fmpq_equal(r, er))
            {
                flint_printf("error in fmpq_farey_neighbors test, line %d. FLINT: ", line_number);
                fmpq_print(l); flint_printf("  "); fmpq_print(r); flint_printf("\n");
                result = 1;
            }
        }
    }
    fmpq_clear(x); fmpq_clear(l); fmpq_clear(r); fmpq_clear(el); fmpq_clear(er);
    fmpz_clear(big_n);
    return result;
}

/* the term of the enumeration that FLINT is holding, advanced once per matched line */
static fmpq_t enum_state;
static int enum_started;
static int enum_signed;

static int
check_enum_line(char * line, int line_number)
{
    const char * prefix = enum_signed ? "signed_minimal[" : "minimal[";
    if (strncmp(line, prefix, strlen(prefix)) != 0)
    {
        return 0;
    }
    char * eq = strstr(line, "] = ");
    if (eq == NULL)
    {
        flint_printf("error in enumeration test, line %d: unreadable line\n", line_number);
        return 1;
    }
    char * x_str = eq + strlen("] = ");
    fmpq_t x;
    fmpq_init(x);
    int result = 0;
    if (fmpq_set_str(x, x_str, 10) != 0)
    {
        flint_printf("error in enumeration test, line %d: unreadable term\n", line_number);
        result = 1;
    }
    else
    {
        if (!enum_started)
        {
            fmpq_init(enum_state);
            fmpq_zero(enum_state);
            enum_started = 1;
        }
        if (!fmpq_equal(x, enum_state))
        {
            flint_printf("error in enumeration test, line %d. FLINT: ", line_number);
            fmpq_print(enum_state);
            flint_printf("\n");
            result = 1;
        }
        if (enum_signed)
        {
            fmpq_next_signed_minimal(enum_state, enum_state);
        }
        else
        {
            fmpq_next_minimal(enum_state, enum_state);
        }
    }
    fmpq_clear(x);
    return result;
}

int
run_fmpq_farey_neighbors(const char * arg)
{
    return for_each_line(arg, check_farey_line);
}

int
run_fmpq_next_minimal(const char * arg)
{
    enum_started = 0;
    enum_signed = 0;
    return for_each_line(arg, check_enum_line);
}

int
run_fmpq_next_signed_minimal(const char * arg)
{
    enum_started = 0;
    enum_signed = 1;
    return for_each_line(arg, check_enum_line);
}

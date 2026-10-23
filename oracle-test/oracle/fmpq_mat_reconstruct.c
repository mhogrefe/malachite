/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the RationalMatrix reconstruction demos against FLINT's
    fmpq_mat_set_fmpz_mat_mod_fmpz.

    The demos print `RationalMatrix::reconstruct(A, m) = R`, where `A` is a matrix of residues and
    `R` is `None` or `Some(X)` for a rational matrix `X`; the mode recomputes the result with
    `fmpq_mat_set_fmpz_mat_mod_fmpz`.

    Integer matrices are read by fmpz_mat_init_set_str_malachite in util.c, and rational matrices,
    written the same way with elements such as `-2/3`, by fmpq_mat_init_set_str_malachite below.
    The mode treats any other nonempty line as an error, and an input with no lines at all.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <flint/fmpz.h>
#include <flint/fmpz_mat.h>
#include <flint/fmpq.h>
#include <flint/fmpq_mat.h>

#include "oracle.h"

static long checked;

/* Reads a rational matrix written by Malachite into `mat`, returning 0 if it is malformed. */
static int
fmpq_mat_init_set_str_malachite(fmpq_mat_t mat, const char * s)
{
    slong height;
    slong width;
    char tail;
    if (sscanf(s, "[]_{%ld,%ld}%c", &height, &width, &tail) == 2)
    {
        fmpq_mat_init(mat, height, width);
        return 1;
    }
    size_t len = strlen(s);
    if (len < 2 || s[0] != '[' || s[len - 1] != ']')
    {
        fmpq_mat_init(mat, 0, 0);
        return 0;
    }
    height = 1;
    width = 1;
    int first_row = 1;
    for (size_t i = 1; i + 1 < len; i++)
    {
        if (s[i] == ';')
        {
            height++;
            first_row = 0;
        }
        else if (s[i] == ',' && first_row)
        {
            width++;
        }
    }
    fmpq_mat_init(mat, height, width);
    char * buffer = flint_malloc(len);
    memcpy(buffer, s + 1, len - 2);
    buffer[len - 2] = '\0';
    char * cursor = buffer;
    int ok = 1;
    for (slong i = 0; i < height && ok; i++)
    {
        for (slong j = 0; j < width && ok; j++)
        {
            while (*cursor == ' ')
            {
                cursor++;
            }
            size_t token_len = strcspn(cursor, ",;");
            char saved = cursor[token_len];
            cursor[token_len] = '\0';
            ok = fmpq_set_str(fmpq_mat_entry(mat, i, j), cursor, 10) == 0;
            cursor[token_len] = saved;
            int last = i == height - 1 && j == width - 1;
            char expected = last ? '\0' : (j == width - 1 ? ';' : ',');
            if (ok && saved != expected)
            {
                ok = 0;
            }
            cursor += token_len + (saved != '\0');
        }
    }
    flint_free(buffer);
    return ok;
}

static int
check_line(char * line, int line_number)
{
    static const char prefix[] = "RationalMatrix::reconstruct(";
    line[strcspn(line, "\r\n")] = '\0';
    if (line[0] == '\0')
    {
        return 0;
    }
    char * eq = strstr(line, ") = ");
    char * comma = eq == NULL ? NULL : strstr(line, "], ");
    if (strncmp(line, prefix, strlen(prefix)) != 0 || eq == NULL)
    {
        flint_printf("error in fmpq_mat_set_fmpz_mat_mod_fmpz test, line %d: unrecognized line\n",
                     line_number);
        return 1;
    }
    char * a_str = line + strlen(prefix);
    char * m_str;
    if (strncmp(a_str, "[]_{", 4) == 0)
    {
        comma = strstr(a_str, "}, ");
        if (comma == NULL)
        {
            return 1;
        }
        m_str = comma + 3;
        comma[1] = '\0';
    }
    else
    {
        if (comma == NULL)
        {
            return 1;
        }
        m_str = comma + 3;
        comma[1] = '\0';
    }
    *eq = '\0';
    char * rest = eq + 4;
    checked++;
    fmpz_mat_t a;
    fmpz_t m;
    fmpz_init(m);
    if (!fmpz_mat_init_set_str_malachite(a, a_str) || fmpz_set_str(m, m_str, 10) != 0)
    {
        flint_printf("error in fmpq_mat_set_fmpz_mat_mod_fmpz test, line %d: unreadable input\n",
                     line_number);
        fmpz_clear(m);
        return 1;
    }
    fmpq_mat_t x;
    fmpq_mat_init(x, fmpz_mat_nrows(a), fmpz_mat_ncols(a));
    int success = fmpq_mat_set_fmpz_mat_mod_fmpz(x, a, m);
    int result = 0;
    if (strcmp(rest, "None") == 0)
    {
        if (success)
        {
            flint_printf("error in fmpq_mat_set_fmpz_mat_mod_fmpz test, line %d: Malachite gives "
                         "None, but FLINT succeeds\n", line_number);
            result = 1;
        }
    }
    else
    {
        size_t len = strlen(rest);
        if (len < 6 || strncmp(rest, "Some(", 5) != 0 || rest[len - 1] != ')')
        {
            flint_printf("error in fmpq_mat_set_fmpz_mat_mod_fmpz test, line %d: unrecognized "
                         "result\n", line_number);
            result = 1;
        }
        else if (!success)
        {
            flint_printf("error in fmpq_mat_set_fmpz_mat_mod_fmpz test, line %d: Malachite "
                         "succeeds, but FLINT does not\n", line_number);
            result = 1;
        }
        else
        {
            rest[len - 1] = '\0';
            fmpq_mat_t expected;
            if (!fmpq_mat_init_set_str_malachite(expected, rest + 5))
            {
                flint_printf("error in fmpq_mat_set_fmpz_mat_mod_fmpz test, line %d: unreadable "
                             "result\n", line_number);
                result = 1;
            }
            else if (!fmpq_mat_equal(x, expected))
            {
                flint_printf("error in fmpq_mat_set_fmpz_mat_mod_fmpz test, line %d: the results "
                             "differ\n", line_number);
                result = 1;
            }
            fmpq_mat_clear(expected);
        }
    }
    fmpq_mat_clear(x);
    fmpz_mat_clear(a);
    fmpz_clear(m);
    return result;
}

int
run_fmpq_mat_set_fmpz_mat_mod_fmpz(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("fmpq_mat_set_fmpz_mat_mod_fmpz", checked);
}

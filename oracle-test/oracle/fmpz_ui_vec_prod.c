/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `product([x, y, ...]) = v` lines, a product of words printed by Malachite's demos of
    `Product` for `Natural` over `u64`s, against _fmpz_ui_vec_prod.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/fmpz_vec.h>

#include "oracle.h"

static int
check_ui_vec_prod_line(char * line, int line_number)
{
    const char * prefix = "product([";
    size_t prefix_len = strlen(prefix);
    if (strncmp(line, prefix, prefix_len) != 0)
    {
        return 0;
    }
    char * list = line + prefix_len;
    char * close = strstr(list, "]) = ");
    if (close == NULL)
    {
        return 0;
    }
    *close = '\0';
    char * rest = close + 5;
    /* the list has at most one more element than it has commas */
    slong capacity = 1;
    for (char * c = list; *c != '\0'; c++)
    {
        if (*c == ',')
        {
            capacity++;
        }
    }
    ulong * vec = flint_malloc(capacity * sizeof(ulong));
    slong len = 0;
    char * p = list;
    while (*p != '\0')
    {
        char * end;
        vec[len++] = strtoull(p, &end, 10);
        p = end;
        while (*p == ',' || *p == ' ')
        {
            p++;
        }
    }
    int result = 0;
    fmpz_t out, expected;
    fmpz_init(out);
    fmpz_init(expected);
    fmpz_set_str(expected, rest, 10);
    _fmpz_ui_vec_prod(out, vec, len);
    if (!fmpz_equal(out, expected))
    {
        flint_printf("error in _fmpz_ui_vec_prod test, line %d. FLINT: out=", line_number);
        fmpz_print(out);
        flint_printf("\n");
        result = 1;
    }
    fmpz_clear(out);
    fmpz_clear(expected);
    flint_free(vec);
    return result;
}

int
run_fmpz_ui_vec_prod(const char * arg)
{
    return for_each_line(arg, check_ui_vec_prod_line);
}

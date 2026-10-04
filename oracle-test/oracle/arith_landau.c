/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `landau_function_prefix(n) = [...]` lines against arith_landau_function_vec.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/arith.h>
#include <flint/fmpz.h>
#include <flint/fmpz_vec.h>

#include "oracle.h"

static int
check_landau_line(char * line, int line_number)
{
    fmpz * vec;
    fmpz_t entry;
    slong n, k;
    int result = 1;
    const char * prefix = "landau_function_prefix(";
    char * needle = strstr(line, ") = [");
    if (strncmp(line, prefix, strlen(prefix)) == 0 && needle != NULL)
    {
        *needle = '\0';
        n = strtol(line + strlen(prefix), NULL, 10);
        char * cursor = needle + strlen(") = [");
        vec = _fmpz_vec_init(n);
        fmpz_init(entry);
        arith_landau_function_vec(vec, n);
        result = 0;
        for (k = 0; k < n; k++)
        {
            char * end = strpbrk(cursor, ",]");
            if (end == NULL)
            {
                flint_printf("line %d: truncated vector\n", line_number);
                result = 1;
                break;
            }
            *end = '\0';
            if (fmpz_set_str(entry, cursor, 10) != 0 || !fmpz_equal(entry, vec + k))
            {
                flint_printf("line %d: landau entry %wd mismatch\n", line_number, k);
                result = 1;
                break;
            }
            cursor = end + 1;
            while (*cursor == ' ')
                cursor++;
        }
        fmpz_clear(entry);
        _fmpz_vec_clear(vec, n);
    }
    else
    {
        flint_printf("line %d: unreadable line\n", line_number);
    }
    return result;
}

int
run_arith_landau_function_vec(const char * arg)
{
    return for_each_line(arg, check_landau_line);
}

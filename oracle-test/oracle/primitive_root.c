/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `primitive_root_prime(n) = out` lines against n_primitive_root_prime.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/ulong_extras.h>

#include "oracle.h"

static int
check_primitive_root_line(char * line, int line_number)
{
    const char * prefix = "primitive_root_prime(";
    if (strncmp(line, prefix, strlen(prefix)) != 0)
    {
        return 0;
    }
    char * suffix = line + strlen(prefix);
    ulong n = strtoul(suffix, &suffix, 10);
    suffix = strchr(suffix, '=') + 1;
    ulong output = strtoul(suffix, &suffix, 10);
    ulong flint_output = n_primitive_root_prime(n);
    if (flint_output != output)
    {
        flint_printf(
            "error in primitive_root_prime test, line %d, on input %lu. Malachite: %lu, "
            "FLINT: %lu\n",
            line_number, n, output, flint_output);
        return 1;
    }
    return 0;
}

int
run_n_primitive_root_prime(const char * arg)
{
    return for_each_line(arg, check_primitive_root_line);
}

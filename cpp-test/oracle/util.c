/*
    Copyright © 2026 Mikhail Hogrefe

    Uses code adapted from the FLINT Library examples.

        Copyright © 2022 Fredrik Johansson

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Line iteration and parsing helpers shared by the oracle modes. The demos print one
    `input = output` line per case; these helpers split such lines into their pieces, leaving the
    numeral conversions (fmpz_set_str, strtoul) to each mode.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "oracle.h"

int
for_each_line(const char * path, int (* handler)(char * line, int line_number))
{
    FILE * fp = fopen(path, "r");
    if (fp == NULL)
    {
        flint_printf("cannot open %s\n", path);
        return 2;
    }
    char * line = NULL;
    size_t len = 0;
    int line_number = 1;
    int result = 0;
    while (getline(&line, &len, fp) != -1)
    {
        result = handler(line, line_number);
        if (result != 0)
        {
            break;
        }
        line_number += 1;
    }
    free(line);
    fclose(fp);
    return result;
}

int
split_method_call(char * line, const char * method, char ** pieces, int n_pieces, char ** rest)
{
    char * dot = strstr(line, method);
    if (dot == NULL)
    {
        return 0;
    }
    *dot = '\0';
    pieces[0] = line;
    char * p = dot + strlen(method);
    for (int i = 1; i < n_pieces; i++)
    {
        pieces[i] = p;
        int last = i == n_pieces - 1;
        char * sep = last ? strchr(p, ')') : strstr(p, ", ");
        if (sep == NULL)
        {
            return 0;
        }
        *sep = '\0';
        p = sep + (last ? 1 : 2);
    }
    *rest = p;
    return 1;
}

int
parse_option_fmpz(char * rest, fmpz_t expected)
{
    char * some = strstr(rest, "Some(");
    if (some == NULL)
    {
        return 0;
    }
    char * start = some + strlen("Some(");
    char * end = strchr(start, ')');
    *end = '\0';
    fmpz_set_str(expected, start, 10);
    return 1;
}

int
parse_option_ulong(const char * rest, ulong * expected)
{
    const char * some = strstr(rest, "Some(");
    if (some == NULL)
    {
        return 0;
    }
    char * end;
    *expected = strtoul(some + strlen("Some("), &end, 10);
    return 1;
}

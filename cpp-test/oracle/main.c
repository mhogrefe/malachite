/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    A differential-testing oracle: reads lines that a Malachite demo printed, recomputes each one
    with FLINT, and exits nonzero on the first disagreement. The first argument selects the mode;
    the second is the mode's input (a file of demo output, or an iteration count for the stress
    modes). See README.md.
*/

#include <string.h>

#include "oracle.h"

typedef struct
{
    const char * name;
    int (* run)(const char * arg);
} oracle_mode;

static const oracle_mode modes[] = {
    {"n_primitive_root_prime", run_n_primitive_root_prime},
    {"fmpz_sqrtmod", run_fmpz_sqrtmod},
    {"n_sqrtmod", run_n_sqrtmod},
    {"sqrtmod_stress", run_sqrtmod_stress},
    {"fmpz_mod_divides", run_fmpz_mod_divides},
    {"fmpz_divides_mod_list", run_fmpz_divides_mod_list},
    {"fmpz_CRT", run_fmpz_CRT},
    {"fmpz_CRT_balanced", run_fmpz_CRT_balanced},
    {"fmpz_multi_CRT", run_fmpz_multi_CRT},
    {"fmpz_multi_CRT_balanced", run_fmpz_multi_CRT_balanced},
    {"fmpz_multi_mod_ui", run_fmpz_multi_mod_ui},
    {"fmpz_multi_CRT_ui", run_fmpz_multi_CRT_ui},
    {"fmpz_multi_CRT_ui_balanced", run_fmpz_multi_CRT_ui_balanced},
    {"fmpz_rfac", run_fmpz_rfac},
    {"fmpz_xgcd_partial", run_fmpz_xgcd_partial},
};

int
main(int argc, char * argv[])
{
    if (argc < 3)
    {
        flint_printf("usage: flint-oracle <mode> <input-file-or-count>\n");
        return 2;
    }
    for (size_t i = 0; i < sizeof(modes) / sizeof(modes[0]); i++)
    {
        if (strcmp(argv[1], modes[i].name) == 0)
        {
            return modes[i].run(argv[2]);
        }
    }
    flint_printf("unknown mode %s\n", argv[1]);
    return 2;
}

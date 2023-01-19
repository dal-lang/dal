/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "cli/cli.hh"
#include "config.h"

using namespace dal::cli;

int main(int argc, char *argv[]) {
    cli_app app(DAL_PROJECT_NAME, DAL_VERSION, "Compiler for the Dal programming language.");
    return app.parse(argc, argv);
}
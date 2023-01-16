/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <config.h>
#include <libcli/CLI.hpp>
#include <libcli/Command.hpp>

void buildHandler(Context *ctx) {
    printf("Hello world! %s\n", ctx->getStringArg("output")->c_str());
}

int main(int argc, char *argv[]) {
    // Create a new CLI object
    CLI cli(DAL_PROJECT_NAME, DAL_VERSION, "DAL is a compiler for the DAL language");

    // Build command.
    Command build("build", "Compile the Dal source code", "dal build [options]");
    build.addBooleanArgument("release", "Build the project in release mode", "dal build --release");
    build.addBooleanArgument("strip", "Strip the binary after building", "dal build --strip");
    build.addBooleanArgument("verbose", "Print verbose output", "dal build --verbose");
    build.addBooleanArgument("no-color", "Disable colored output", "dal build --no-color");
    build.addStringArgument("output", "Set output file", "dal build --output <filename>");
    build.addStringArgument("input", "Set input file", "dal build --input <filename>", true);
    build.setHandler(buildHandler);

    // Assign build command to cli.
    cli.addCommand(&build);

    // Format command.
    Command format("format", "Format the Dal source code", "dal format [options] <file>");

    // Assign format command to cli.
    cli.addCommand(&format);

    // parse command.
    return cli.parse(argc, argv);
}
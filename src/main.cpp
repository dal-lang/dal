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
#include <libutils/Os.hpp>
#include <libutils/Fmt.hpp>
#include <liblexer/Lexer.hpp>
#include <libparser/Parser.hpp>

void buildHandler(Context *ctx) {
    std::error_code ec;
    auto path = Os::getAbsolutePath(*ctx->getStringArg("input"), ec);
    if (ec)
    {
        Fmt::panic("%s: %s\n", Fmt::redBold("Failed finding `%s`"), *ctx->getStringArg("input"), ec.message());
    }
    auto content = Os::readFile(path, ec);
    if (ec)
    {
        Fmt::panic("%s: %s", Fmt::redBold("Input file not found"), ec.message());
    }

    Fmt::println("Content: %s", content);

    dal::Lexer lexer(content);
    std::vector<dal::Token> tokens = lexer.lex();
    for (auto token : tokens)
    {
        Fmt::println("%s", token.toString(content));
    }

    dal::Parser parser(content, tokens);
    auto ast = parser.parse();
    Fmt::println("%s", ast->to_string());
}

int main(int argc, char *argv[])
{
    CLI cli(DAL_PROJECT_NAME, DAL_VERSION, "DAL is a compiler for the DAL language");

    Command build("build", "Compile the Dal source code", "dal build [options]");
    build.addBooleanArgument("release", "Build the project in release mode", "dal build --release");
    build.addBooleanArgument("strip", "Strip the binary after building", "dal build --strip");
    build.addBooleanArgument("verbose", "Print verbose output", "dal build --verbose");
    build.addBooleanArgument("no-color", "Disable colored output", "dal build --no-color");
    build.addStringArgument("output", "Set output file", "dal build --output <filename>");
    build.addStringArgument("input", "Set input file", "dal build --input <filename>", true);
    build.setHandler(buildHandler);

    cli.addCommand(&build);

    Command format("format", "Format the Dal source code", "dal format [options] <file>");

    cli.addCommand(&format);

    return cli.parse(argc, argv);
}
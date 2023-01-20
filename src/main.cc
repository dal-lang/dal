/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <cli/cli_app.hh>
#include <config.h>

using namespace dal::cli;

int build_handler(context ctx) {
  fmt::println("file: {}", ctx.get_string("input"));
  fmt::println("output: {}", ctx.get_string("type"));
  return 0;
}

int main(int argc, char **argv) {
  cli_app app;
  app.set_name(DAL_PROJECT_NAME);
  app.set_description("Compiler for the Dal programming language.");
  app.set_version(DAL_VERSION);
  app.set_usage("dal <command> [options]");
  app.set_license(DAL_LICENSE);

  cli_command build;

  build.set_name("build");
  build.set_description("Build executable|library|object file.");
  build.set_usage("dal build [options]");

  arg_builder b;

  b.set_name("input").set_description("Input file").set_usage("dal build --input <file>").set_required(true);
  build.add_arg(b.build_string());

  b.set_name("type").set_description("Output type").set_usage("dal build --type <exe|lib|obj>").set_required(true);
  b.set_str_default("exe");
  build.add_arg(b.build_string());

  b.set_name("output").set_description("Output file").set_usage("dal build --output <file>").set_required(true);
  b.set_str_default("a.out");
  build.add_arg(b.build_string());

  b.set_name("name").set_description("Output name").set_usage("dal build --name <name>").set_required(true);
  b.set_str_default("a");
  build.add_arg(b.build_string());

  b.set_name("verbose").set_description("Verbose output").set_usage("dal build --verbose");
  build.add_arg(b.build_bool());

  b.set_name("static").set_description("Output will be statically linked").set_usage("dal build --static");
  build.add_arg(b.build_bool());

  b.set_name("strip").set_description("Strip debug symbols").set_usage("dal build --strip");
  build.add_arg(b.build_bool());

  b.set_name("release").set_description("Build in release mode").set_usage("dal build --release");
  build.add_arg(b.build_bool());

  build.set_handler(build_handler);

  app.add_command(build);

  return app.parse(argc, argv);
}

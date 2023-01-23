/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <config.h>
#include <cli/cli_app.hh>
#include <core/codegen.hh>
#include <core/os.hh>

using namespace dal::cli;
using namespace dal;

int build_handler(context ctx) {
  core::codegen gen;

  auto in_file = ctx.get_string("input");
  if (in_file.empty()) {
    fmt::panic("{}: no input file provided", fmt::red_bold("error"));
  }

  std::error_code ec;
  std::string dir, file;
  core::os::split_path(in_file, dir, file, ec);
  if (ec) {
    fmt::panic("{}: {}", fmt::red_bold("error"), ec.message());
  }

  gen.set_root_dir(dir);
  gen.set_build_type(ctx.get_bool("release") ? core::build_type::release
                                             : core::build_type::debug);
  if (ctx.get_bool("strip")) {
    gen.strip_debug_symbols();
  }
  if (ctx.get_bool("static")) {
    gen.link_static();
  }

  auto out_type = ctx.get_string("type");
  if (out_type == "exe") {
    gen.set_out_type(core::out_type::exe);
  } else if (out_type == "lib") {
    gen.set_out_type(core::out_type::lib);
  } else if (out_type == "obj") {
    gen.set_out_type(core::out_type::obj);
  } else {
    fmt::panic("{}: invalid output type '{}'", fmt::red_bold("error"),
               out_type);
  }

  gen.set_out_path(ctx.get_string("output"));
  if (ctx.get_bool("verbose")) {
    gen.verbose();
  }

  auto abs_path = core::os::to_abs_path(in_file, ec);
  if (ec) {
    fmt::panic("{}: {}", fmt::red_bold("error"), ec.message());
  }
  return gen.gen(abs_path);
}

int main(int argc, char** argv) {
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

  b.set_name("input")
      .set_description("Input file")
      .set_usage("dal build --input <file>")
      .set_required(true);
  build.add_arg(b.build_string());

  b.set_name("type")
      .set_description("Output type")
      .set_usage("dal build --type <exe|lib|obj>")
      .set_required(true);
  b.set_str_default("exe");
  build.add_arg(b.build_string());

  b.set_name("output")
      .set_description("Output file")
      .set_usage("dal build --output <file>")
      .set_required(true);
  b.set_str_default("a.out");
  build.add_arg(b.build_string());

  b.set_name("verbose")
      .set_description("Verbose output")
      .set_usage("dal build --verbose");
  build.add_arg(b.build_bool());

  b.set_name("static")
      .set_description("Output will be statically linked")
      .set_usage("dal build --static");
  build.add_arg(b.build_bool());

  b.set_name("strip")
      .set_description("Strip debug symbols")
      .set_usage("dal build --strip");
  build.add_arg(b.build_bool());

  b.set_name("release")
      .set_description("Build in release mode")
      .set_usage("dal build --release");
  build.add_arg(b.build_bool());

  build.set_handler(build_handler);

  app.add_command(build);

  return app.parse(argc, argv);
}

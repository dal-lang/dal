/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef _DAL_CLI_CMD_HH_
#define _DAL_CLI_CMD_HH_

#include <string>
#include <map>
#include <vector>
#include "arg.hh"
#include "ctx.hh"

namespace dal::cli {

    // Command handler function
    typedef void (*cmd_handler)(context *ctx);

    class cli_cmd {
    private:
        std::string m_name;
        std::string m_desc;
        std::string m_usage;
        std::map<std::string, cli_arg *> m_args;
        cmd_handler m_handler = {};

        static void fallback(const std::string &msg);

        void run(context *ctx);

    public:
        cli_cmd(std::string name, std::string description, std::string usage);

        ~cli_cmd() = default;

        void add_arg(cli_arg *argument);

        void
        add_string_arg(std::string arg_name, std::string arg_desc, std::string arg_usage, bool is_required = false);

        void
        add_bool_arg(std::string arg_name, std::string arg_desc, std::string arg_usage, bool is_required = false);

        void set_handler(cmd_handler command_handler);

        std::string get_name();

        std::string get_desc();

        std::string get_usage();

        void parse(std::vector<std::string> args);

        void print_help();
    };

} // dal::cli

#endif //_DAL_CLI_CMD_HH_

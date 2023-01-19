/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */


#ifndef _DAL_CLI_CLI_HH_
#define _DAL_CLI_CLI_HH_

#include <string>
#include <map>
#include "cmd.hh"

namespace dal::cli {

    class cli_app {
    private:
        std::string m_name;
        std::string m_ver;
        std::string m_desc;
        std::map<std::string, cli_cmd *> m_commands;
        std::map<std::string, std::string> m_additional;

        int fallback(const std::string &command = "", const std::string &msg = "", int exit_code = 0);

        int print_version();

    public:
        cli_app(const std::string &app_name, const std::string &app_version, const std::string &app_description);

        ~cli_app() = default;

        void add_command(cli_cmd *command);

        int parse(int argc, char *argv[]);
    };

} // dal::cli


#endif //_DAL_CLI_CLI_HH_

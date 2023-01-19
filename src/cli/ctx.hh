/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef _DAL_CLI_CTX_HH_
#define _DAL_CLI_CTX_HH_

#include <string>
#include <map>
#include "arg.hh"

namespace dal::cli {

    class context {
    private:
        std::map<std::string, cli_arg *> arguments;
    public:
        explicit context(std::map<std::string, cli_arg *> arguments);

        std::string *get_string_arg(const std::string &key);

        bool *get_bool_arg(const std::string &key);
    };

} // dal::cli


#endif //_DAL_CLI_CTX_HH_

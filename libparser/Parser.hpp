/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_PARSER_HPP
#define DAL_PARSER_HPP

#include <string>
#include <vector>
#include <liblexer/Token.hpp>
#include <libast/Ast.hpp>

namespace dal {

    class Parser {
    private:
        std::string m_source;
        std::vector<Token> m_tokens;
        int m_index = 0;
        std::vector<std::shared_ptr<NodeAttribute>> m_attributes;

        //----- Parse functions -----//
        void parse_attributes();

        void parse_attribute();

        std::shared_ptr<NodeFnDef> parse_fn_def(bool is_required);

        std::shared_ptr<NodeExternBlock> parse_extern_block(bool is_required);

        std::shared_ptr<NodeImport> parse_import();

        std::shared_ptr<NodeFnProto> parse_fn_proto(bool is_required);

        std::shared_ptr<NodeBlock> parse_block(bool is_required);

        std::shared_ptr<NodeFnDecl> parse_fn_decl();

        std::shared_ptr<NodeParamDecl> parse_param_decl();

        std::vector<std::shared_ptr<NodeParamDecl>> parse_param_decls(bool *is_variadic);

        std::shared_ptr<NodeType> parse_type();

        std::shared_ptr<NodeStrLit> parse_str_lit(Token *token);

        std::shared_ptr<NodeNumLit> parse_num_lit(Token *token);

        std::shared_ptr<NodeVarDecl> parse_var_decl(bool is_required);

        std::shared_ptr<Ast> parse_block_expr(bool is_required);

        std::shared_ptr<Ast> parse_non_block_expr(bool is_required);

        std::shared_ptr<NodeIfExpr> parse_if_expr(bool is_required);

        std::shared_ptr<Ast> parse_expr(bool is_required);

        std::shared_ptr<Ast> parse_else_expr(bool is_required);

        std::shared_ptr<NodeReturnExpr> parse_return_expr(bool is_required);

        std::shared_ptr<Ast> parse_assign_expr(bool is_required);

        std::shared_ptr<Ast> parse_log_or_expr(bool is_required);

        std::shared_ptr<Ast> parse_log_and_expr(bool is_required);

        std::shared_ptr<Ast> parse_cmp_expr(bool is_required);

        std::shared_ptr<Ast> parse_bit_or_expr(bool is_required);

        std::shared_ptr<Ast> parse_bit_xor_expr(bool is_required);

        std::shared_ptr<Ast> parse_bit_and_expr(bool is_required);

        std::shared_ptr<Ast> parse_shift_expr(bool is_required);

        std::shared_ptr<Ast> parse_add_expr(bool is_required);

        std::shared_ptr<Ast> parse_mul_expr(bool is_required);

        std::shared_ptr<Ast> parse_cast_expr(bool is_required);

        std::shared_ptr<Ast> parse_unary_expr(bool is_required);

        std::shared_ptr<Ast> parse_postfix_expr(bool is_required);

        std::shared_ptr<Ast> parse_primary_expr(bool is_required);

        std::vector<std::shared_ptr<Ast>> parse_call_params();

        std::shared_ptr<Ast> parse_group_expr(bool is_required);
        //----------------------------//

        //----- Utility functions -----//
        void expect_token(Token *token, TokenKind kind, std::string msg = "");

        std::string token_value(Token *token);

        [[noreturn]]
        void error(Token *token, const std::string &msg);
        //----------------------------//

    public:
        explicit Parser(std::string source, std::vector<Token> tokens) : m_source(std::move(source)),
                                                                         m_tokens(std::move(tokens)) {}

        std::shared_ptr<NodeRoot> parse();
    };

} // dal

#endif //DAL_PARSER_HPP

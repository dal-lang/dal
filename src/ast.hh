/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_AST_HPP
#define DAL_AST_HPP

#include <string>
#include <utility>
#include <vector>
#include <memory>
#include "src/utils/span.hpp"

namespace dal::ast {

    enum ast_kind {
        ast_kind_root,
        ast_kind_fn_proto,
        ast_kind_fn_def,
        ast_kind_fn_decl,
        ast_kind_param_decl,
        ast_kind_type,
        ast_kind_block,
        ast_kind_extern_block,
        ast_kind_attribute,
        ast_kind_return_expr,
        ast_kind_var_decl,
        ast_kind_binop_expr,
        ast_kind_cast_expr,
        ast_kind_num_lit,
        ast_kind_str_lit,
        ast_kind_noreturn,
        ast_kind_ident,
        ast_kind_unop_expr,
        ast_kind_call_expr,
        ast_kind_array_index,
        ast_kind_import,
        ast_kind_void,
        ast_kind_bool_lit,
        ast_kind_if_expr,
    };

    class ast_node {
    public:
        ast_kind m_ast_kind;
        span m_span{};

        explicit ast_node(ast_kind ast_kind) : m_ast_kind(ast_kind) {}

        virtual ~ast_node() = default;

        void merge_span(const std::shared_ptr<ast_node> &ast);

        [[nodiscard]] std::string kind_string() const;

        std::string to_string(int indent = 0);
    };

    class node_root : public ast_node {
    public:
        std::vector<std::shared_ptr<ast_node>> m_children;

        node_root() : ast_node(ast_kind_root) {}
    };

    class node_void : public ast_node {
    public:
        node_void() : ast_node(ast_kind_void) {}
    };

    class node_no_return : public ast_node {
    public:
        node_no_return() : ast_node(ast_kind_noreturn) {}
    };

    class node_str_lit : public ast_node {
    public:
        std::string m_value;

        node_str_lit() : ast_node(ast_kind_str_lit) {}
    };

    class node_num_lit : public ast_node {
    public:
        std::string m_value;

        node_num_lit() : ast_node(ast_kind_num_lit) {}
    };

    class node_bool_lit : public ast_node {
    public:
        bool m_value = false;

        node_bool_lit() : ast_node(ast_kind_bool_lit) {}
    };

    class node_import : public ast_node {
    public:
        std::shared_ptr<node_str_lit> m_path;

        node_import() : ast_node(ast_kind_import) {}
    };

    class node_ident : public ast_node {
    public:
        std::string m_value;

        node_ident() : ast_node(ast_kind_ident) {}
    };

    class node_attribute : public ast_node {
    public:
        std::string m_name;
        std::string m_param;

        node_attribute() : ast_node(ast_kind_attribute) {}
    };

    enum type_kind {
        TYPE_PRIMITIVE,
        TYPE_ARRAY,
        TYPE_POINTER,
    };

    class node_type : public ast_node {
    public:
        type_kind m_type = TYPE_PRIMITIVE;

        explicit node_type(type_kind type) : ast_node(ast_kind_type), m_type(type) {}
    };

    class node_type_primitive : public node_type {
    public:
        std::string m_name;

        node_type_primitive() : node_type(TYPE_PRIMITIVE) {}
    };

    class node_type_array : public node_type {
    public:
        std::shared_ptr<node_type> m_elem_type;
        std::shared_ptr<node_num_lit> m_size;

        node_type_array() : node_type(TYPE_ARRAY) {}
    };

    class node_type_pointer : public node_type {
    public:
        std::shared_ptr<node_type> m_elem_type;
        bool m_is_mut = false;

        node_type_pointer() : node_type(TYPE_POINTER) {}
    };

    class node_param_decl : public ast_node {
    public:
        std::string m_name;
        std::shared_ptr<node_type> m_type;

        node_param_decl() : ast_node(ast_kind_param_decl) {}
    };

    class node_fn_proto : public ast_node {
    public:
        std::vector<std::shared_ptr<node_attribute>> m_attributes;
        bool m_is_public = false;
        std::string m_name;
        std::vector<std::shared_ptr<node_param_decl>> m_params;
        bool m_is_variadic = false;
        std::shared_ptr<node_type> m_return_type;

        node_fn_proto() : ast_node(ast_kind_fn_proto) {}
    };

    class node_fn_decl : public ast_node {
    public:
        std::shared_ptr<node_fn_proto> m_proto;

        node_fn_decl() : ast_node(ast_kind_fn_decl) {}
    };

    class node_block : public ast_node {
    public:
        std::vector<std::shared_ptr<ast_node>> m_children;

        node_block() : ast_node(ast_kind_block) {}
    };

    class node_fn_def : public ast_node {
    public:
        std::shared_ptr<node_fn_proto> m_proto;
        std::shared_ptr<node_block> m_block;

        node_fn_def() : ast_node(ast_kind_fn_def) {}
    };

    class node_fn_block : public ast_node {
    public:
        std::vector<std::shared_ptr<node_attribute>> m_attributes;
        std::vector<std::shared_ptr<node_fn_decl>> m_children;

        node_fn_block() : ast_node(ast_kind_extern_block) {}
    };

    class node_var_decl : public ast_node {
    public:
        std::string m_name;
        std::shared_ptr<node_type> m_type;
        std::shared_ptr<ast_node> m_value;
        bool m_is_const = true;

        node_var_decl() : ast_node(ast_kind_var_decl) {}
    };

    class node_return_expr : public ast_node {
    public:
        std::shared_ptr<ast_node> m_value;

        node_return_expr() : ast_node(ast_kind_return_expr) {}
    };

    class node_call_expr : public ast_node {
    public:
        std::shared_ptr<node_ident> m_name;
        std::vector<std::shared_ptr<ast_node>> m_params;

        node_call_expr() : ast_node(ast_kind_call_expr) {}
    };

    class node_array_index : public ast_node {
    public:
        std::shared_ptr<ast_node> m_array;
        std::shared_ptr<ast_node> m_index;

        node_array_index() : ast_node(ast_kind_array_index) {}
    };

    class node_if_expr : public ast_node {
    public:
        std::shared_ptr<ast_node> m_condition;
        std::shared_ptr<node_block> m_then_block;
        std::shared_ptr<ast_node> m_else_block;

        node_if_expr() : ast_node(ast_kind_if_expr) {}
    };

    class node_cast_expr : public ast_node {
    public:
        std::shared_ptr<ast_node> m_value;
        std::shared_ptr<node_type> m_type;

        node_cast_expr() : ast_node(ast_kind_cast_expr) {}
    };

    enum bin_op {
        op_assign,
        op_log_or,
        op_log_and,
        op_cmp_eq,
        op_cmp_neq,
        op_cmp_lt,
        op_cmp_gt,
        op_cmp_lte,
        op_cmp_gte,
        op_bit_or,
        op_bit_xor,
        op_bit_and,
        op_bit_shl,
        op_bit_shr,
        op_add,
        op_sub,
        op_mul,
        op_div,
        op_mod,
    };

    class node_binop_expr : public ast_node {
    public:
        bin_op m_op;
        std::shared_ptr<ast_node> m_left;
        std::shared_ptr<ast_node> m_right;

        explicit node_binop_expr(bin_op op) : ast_node(ast_kind_binop_expr), m_op(op) {}
    };

    enum un_op {
        op_bit_not,
        op_neg,
        op_log_not,
    };

    class node_unop_expr : public ast_node {
    public:
        un_op m_op;
        std::shared_ptr<ast_node> m_value;

        explicit node_unop_expr(un_op op) : ast_node(ast_kind_unop_expr), m_op(op) {}
    };

} // dal::ast

#endif //DAL_AST_HPP

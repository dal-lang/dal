/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "ast.hpp"

namespace dal::ast {

        void ast_node::merge_span(const std::shared_ptr<ast_node> &ast) {
            if (!ast) {
                return;
            }
            this->m_span = this->m_span.merge(ast->m_span);
        }

        std::string ast_node::kind_string() const {
            switch (this->m_ast_kind) {
                case ast_kind_root:
                    return "Root";
                case ast_kind_fn_proto:
                    return "FnProto";
                case ast_kind_fn_def:
                    return "FnDef";
                case ast_kind_fn_decl:
                    return "FnDecl";
                case ast_kind_param_decl:
                    return "ParamDecl";
                case ast_kind_type:
                    return "Type";
                case ast_kind_block:
                    return "Block";
                case ast_kind_extern_block:
                    return "ExternBlock";
                case ast_kind_attribute:
                    return "Attribute";
                case ast_kind_return_expr:
                    return "ReturnExpr";
                case ast_kind_var_decl:
                    return "VarDecl";
                case ast_kind_binop_expr:
                    return "BinOpExpr";
                case ast_kind_cast_expr:
                    return "CastExpr";
                case ast_kind_num_lit:
                    return "NumLit";
                case ast_kind_str_lit:
                    return "StrLit";
                case ast_kind_noreturn:
                    return "NoReturn";
                case ast_kind_ident:
                    return "Ident";
                case ast_kind_unop_expr:
                    return "UnOpExpr";
                case ast_kind_call_expr:
                    return "FnCallExpr";
                case ast_kind_array_index:
                    return "ArrayAccessExpr";
                case ast_kind_import:
                    return "Import";
                case ast_kind_void:
                    return "Void";
                case ast_kind_bool_lit:
                    return "BoolLit";
                case ast_kind_if_expr:
                    return "IfExpr";
            }
        }

        std::string ast_node::to_string(int indent) {
            std::string str;
            for (int i = 0; i < indent; i++) {
                str += " ";
            }
            switch (this->m_ast_kind) {
                case ast_kind_root: {
                    str += this->kind_string() + "\n";
                    auto *root = dynamic_cast<node_root *>(this);
                    for (const auto &child: root->m_children) {
                        str += child->to_string(indent + 2);
                    }
                    break;
                }
                case ast_kind_fn_proto: {
                    str += this->kind_string() + "\n";
                    auto *fn_proto = dynamic_cast<node_fn_proto *>(this);
                    str += fn_proto->m_return_type->to_string(indent + 2);
                    std::string child_indent;
                    for (int i = 0; i < indent + 2; i++) {
                        child_indent += " ";
                    }
                    str += child_indent + "Name: " + fn_proto->m_name + "\n";
                    str += child_indent + "Public: ";
                    str += fn_proto->m_is_public ? "true" : "false";
                    str += "\n";
                    str += child_indent + "Variadic: ";
                    str += fn_proto->m_is_variadic ? "true" : "false";
                    str += "\n";
                    for (const auto &param: fn_proto->m_params) {
                        str += param->to_string(indent + 2);
                    }
                    break;
                }
                case ast_kind_fn_def: {
                    str += this->kind_string() + "\n";
                    auto *fn_def = dynamic_cast<node_fn_def *>(this);
                    str += fn_def->m_proto->to_string(indent + 2);
                    str += fn_def->m_block->to_string(indent + 2);
                    break;
                }
                case ast_kind_fn_decl: {
                    str += this->kind_string() + "\n";
                    auto *fn_decl = dynamic_cast<node_fn_decl *>(this);
                    str += fn_decl->m_proto->to_string(indent + 2);
                    break;
                }
                case ast_kind_param_decl: {
                    str += this->kind_string() + "\n";
                    auto *param_decl = dynamic_cast<node_param_decl *>(this);
                    std::string child_indent;
                    for (int i = 0; i < indent + 2; i++) {
                        child_indent += " ";
                    }
                    str += child_indent + "Name: " + param_decl->m_name + "\n";
                    str += param_decl->m_type->to_string(indent + 2);
                    break;
                }
                case ast_kind_type: {
                    str += this->kind_string() + "\n";
                    auto *type = dynamic_cast<node_type *>(this);
                    std::string child_indent;
                    for (int i = 0; i < indent + 2; i++) {
                        child_indent += " ";
                    }
                    if (type->m_type == TYPE_PRIMITIVE) {
                        auto *primitive = dynamic_cast<node_type_primitive *>(type);
                        str += child_indent + "Primitive: " + primitive->m_name + "\n";
                    } else if (type->m_type == TYPE_ARRAY) {
                        auto *array = dynamic_cast<node_type_array *>(type);
                        str += child_indent + "Array: " + array->m_size->to_string() + "\n";
                        str += array->m_elem_type->to_string(indent + 2);
                    } else if (type->m_type == TYPE_POINTER) {
                        auto *pointer = dynamic_cast<node_type_pointer *>(type);
                        str += child_indent + "Pointer: ";
                        str += pointer->m_is_mut ? "mut" : "const";
                        str += "\n";
                        str += pointer->m_elem_type->to_string(indent + 2);
                    }
                    break;
                }
                case ast_kind_block: {
                    str += this->kind_string() + "\n";
                    auto *block = dynamic_cast<node_block *>(this);
                    for (const auto &child: block->m_children) {
                        str += child->to_string(indent + 2);
                    }
                    break;
                }
                case ast_kind_extern_block: {
                    str += this->kind_string() + "\n";
                    auto *extern_block = dynamic_cast<node_fn_block *>(this);
                    if (!extern_block->m_attributes.empty()) {
                        for (const auto &attribute: extern_block->m_attributes) {
                            str += attribute->to_string(indent + 2);
                        }
                    }
                    for (const auto &child: extern_block->m_children) {
                        str += child->to_string(indent + 2);
                    }
                    break;
                }
                case ast_kind_attribute: {
                    auto *attribute = dynamic_cast<node_attribute *>(this);
                    str += "Attribute: " + attribute->m_name + "\n";
                    for (int i = 0; i < indent + 2; i++) {
                        str += " ";
                    }
                    str += "Value: " + attribute->m_param + "\n";
                    break;
                }
                case ast_kind_return_expr:
                    break;
                case ast_kind_var_decl: {
                    str += this->kind_string() + ": ";
                    auto *var_decl = dynamic_cast<node_var_decl *>(this);
                    str += var_decl->m_is_const ? "const " : "mut ";
                    str += var_decl->m_name + "\n";
                    std::string child_indent;
                    for (int i = 0; i < indent + 2; i++) {
                        child_indent += " ";
                    }
                    str += child_indent + "Name: " + var_decl->m_name + "\n";
                    if (var_decl->m_type)
                        str += var_decl->m_type->to_string(indent + 2);
                    if (var_decl->m_value)
                        str += var_decl->m_value->to_string(indent + 2);
                    break;
                }
                case ast_kind_binop_expr:
                    break;
                case ast_kind_cast_expr:
                    break;
                case ast_kind_num_lit: {
                    str += this->kind_string() + ": ";
                    auto *num_lit = dynamic_cast<node_num_lit *>(this);
                    str += num_lit->m_value + "\n";
                    break;
                }
                case ast_kind_str_lit:
                    str += this->kind_string() + ": '" + dynamic_cast<node_str_lit *>(this)->m_value + "'\n";
                    break;
                case ast_kind_noreturn:
                    break;
                case ast_kind_ident: {
                    str += this->kind_string() + ": " + dynamic_cast<node_ident *>(this)->m_value + "\n";
                    break;
                }
                case ast_kind_unop_expr:
                    break;
                case ast_kind_call_expr: {
                    str += this->kind_string() + "\n";
                    auto *fn_call = dynamic_cast<node_call_expr *>(this);
                    str += fn_call->m_name->to_string(indent + 2);
                    for (const auto &param: fn_call->m_params) {
                        str += param->to_string(indent + 2);
                    }
                    break;
                }
                case ast_kind_array_index:
                    break;
                case ast_kind_import: {
                    str += this->kind_string() + "\n";
                    auto *import = dynamic_cast<node_import *>(this);
                    str += import->m_path->to_string(indent + 2);
                    break;
                }
                case ast_kind_void:
                    break;
                case ast_kind_bool_lit:
                    break;
                case ast_kind_if_expr:
                    break;
            }

            return str;
        }

} // dal
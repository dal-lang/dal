/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "Ast.hpp"

namespace dal {

    void Ast::merge_span(const std::shared_ptr<Ast> &ast) {
        if (!ast) {
            return;
        }
        this->m_span = this->m_span.merge(ast->m_span);
    }

    std::string Ast::kind_string() const {
        switch (this->ast_kind) {
            case AstKindRoot:
                return "Root";
            case AstKindFnProto:
                return "FnProto";
            case AstKindFnDef:
                return "FnDef";
            case AstKindFnDecl:
                return "FnDecl";
            case AstKindParamDecl:
                return "ParamDecl";
            case AstKindType:
                return "Type";
            case AstKindBlock:
                return "Block";
            case AstKindExternBlock:
                return "ExternBlock";
            case AstKindAttribute:
                return "Attribute";
            case AstKindReturnExpr:
                return "ReturnExpr";
            case AstKindVarDecl:
                return "VarDecl";
            case AstKindBinOpExpr:
                return "BinOpExpr";
            case AstKindCastExpr:
                return "CastExpr";
            case AstKindNumLit:
                return "NumLit";
            case AstKindStrLit:
                return "StrLit";
            case AstKindNoReturn:
                return "NoReturn";
            case AstKindIdent:
                return "Ident";
            case AstKindUnOpExpr:
                return "UnOpExpr";
            case AstKindFnCallExpr:
                return "FnCallExpr";
            case AstKindArrayAccessExpr:
                return "ArrayAccessExpr";
            case AstKindImport:
                return "Import";
            case AstKindVoid:
                return "Void";
            case AstKindBoolLit:
                return "BoolLit";
            case AstKindIfExpr:
                return "IfExpr";
        }
    }

    std::string Ast::to_string(int indent) {
        std::string str;
        for (int i = 0; i < indent; i++) {
            str += " ";
        }
        switch (this->ast_kind) {
            case AstKindRoot: {
                str += this->kind_string() + "\n";
                auto *root = dynamic_cast<NodeRoot *>(this);
                for (const auto &child: root->m_children) {
                    str += child->to_string(indent + 2);
                }
                break;
            }
            case AstKindFnProto: {
                str += this->kind_string() + "\n";
                auto *fn_proto = dynamic_cast<NodeFnProto *>(this);
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
            case AstKindFnDef: {
                str += this->kind_string() + "\n";
                auto *fn_def = dynamic_cast<NodeFnDef *>(this);
                str += fn_def->m_proto->to_string(indent + 2);
                str += fn_def->m_block->to_string(indent + 2);
                break;
            }
            case AstKindFnDecl: {
                str += this->kind_string() + "\n";
                auto *fn_decl = dynamic_cast<NodeFnDecl *>(this);
                str += fn_decl->m_proto->to_string(indent + 2);
                break;
            }
            case AstKindParamDecl: {
                str += this->kind_string() + "\n";
                auto *param_decl = dynamic_cast<NodeParamDecl *>(this);
                std::string child_indent;
                for (int i = 0; i < indent + 2; i++) {
                    child_indent += " ";
                }
                str += child_indent + "Name: " + param_decl->m_name + "\n";
                str += param_decl->m_type->to_string(indent + 2);
                break;
            }
            case AstKindType: {
                str += this->kind_string() + "\n";
                auto *type = dynamic_cast<NodeType *>(this);
                std::string child_indent;
                for (int i = 0; i < indent + 2; i++) {
                    child_indent += " ";
                }
                if (type->m_type == TypeKindPrimitive) {
                    auto *primitive = dynamic_cast<NodeTypePrimitive *>(type);
                    str += child_indent + "Primitive: " + primitive->m_name + "\n";
                } else if (type->m_type == TypeKindArray) {
                    auto *array = dynamic_cast<NodeTypeArray *>(type);
                    str += child_indent + "Array: " + array->m_size->to_string() + "\n";
                    str += array->m_elem_type->to_string(indent + 2);
                } else if (type->m_type == TypeKindPointer) {
                    auto *pointer = dynamic_cast<NodeTypePointer *>(type);
                    str += child_indent + "Pointer: ";
                    str += pointer->m_is_mut ? "mut" : "const";
                    str += "\n";
                    str += pointer->m_elem_type->to_string(indent + 2);
                }
                break;
            }
            case AstKindBlock: {
                str += this->kind_string() + "\n";
                auto *block = dynamic_cast<NodeBlock *>(this);
                for (const auto &child: block->m_children) {
                    str += child->to_string(indent + 2);
                }
                break;
            }
            case AstKindExternBlock: {
                str += this->kind_string() + "\n";
                auto *extern_block = dynamic_cast<NodeExternBlock *>(this);
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
            case AstKindAttribute: {
                auto *attribute = dynamic_cast<NodeAttribute *>(this);
                str += "Attribute: " + attribute->m_name + "\n";
                for (int i = 0; i < indent + 2; i++) {
                    str += " ";
                }
                str += "Value: " + attribute->m_param + "\n";
                break;
            }
            case AstKindReturnExpr:
                break;
            case AstKindVarDecl: {
                str += this->kind_string() + ": ";
                auto *var_decl = dynamic_cast<NodeVarDecl *>(this);
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
            case AstKindBinOpExpr:
                break;
            case AstKindCastExpr:
                break;
            case AstKindNumLit: {
                str += this->kind_string() + ": ";
                auto *num_lit = dynamic_cast<NodeNumLit *>(this);
                str += num_lit->m_value + "\n";
                break;
            }
            case AstKindStrLit:
                str += this->kind_string() + ": '" + dynamic_cast<NodeStrLit *>(this)->m_value + "'\n";
                break;
            case AstKindNoReturn:
                break;
            case AstKindIdent: {
                str += this->kind_string() + ": " + dynamic_cast<NodeIdent *>(this)->m_value + "\n";
                break;
            }
            case AstKindUnOpExpr:
                break;
            case AstKindFnCallExpr: {
                str += this->kind_string() + "\n";
                auto *fn_call = dynamic_cast<NodeFnCallExpr *>(this);
                str += fn_call->m_name->to_string(indent + 2);
                for (const auto &param: fn_call->m_params) {
                    str += param->to_string(indent + 2);
                }
                break;
            }
            case AstKindArrayAccessExpr:
                break;
            case AstKindImport: {
                str += this->kind_string() + "\n";
                auto *import = dynamic_cast<NodeImport *>(this);
                str += import->m_path->to_string(indent + 2);
                break;
            }
            case AstKindVoid:
                break;
            case AstKindBoolLit:
                break;
            case AstKindIfExpr:
                break;
        }

        return str;
    }

} // dal
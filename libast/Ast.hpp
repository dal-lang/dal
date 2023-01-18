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
#include <libspan/Span.hpp>
#include <memory>

namespace dal {

    enum AstKind {
        AstKindRoot,
        AstKindFnProto,
        AstKindFnDef,
        AstKindFnDecl,
        AstKindParamDecl,
        AstKindType,
        AstKindBlock,
        AstKindExternBlock,
        AstKindAttribute,
        AstKindReturnExpr,
        AstKindVarDecl,
        AstKindBinOpExpr,
        AstKindCastExpr,
        AstKindNumLit,
        AstKindStrLit,
        AstKindNoReturn,
        AstKindIdent,
        AstKindUnOpExpr,
        AstKindFnCallExpr,
        AstKindArrayAccessExpr,
        AstKindImport,
        AstKindVoid,
        AstKindBoolLit,
        AstKindIfExpr,
    };

    class Ast {
    public:
        AstKind ast_kind;
        Span m_span{};

        explicit Ast(AstKind ast_kind) : ast_kind(ast_kind) {}

        virtual ~Ast() = default;

        void merge_span(const std::shared_ptr<Ast> &ast);

        [[nodiscard]] std::string kind_string() const;

        std::string to_string(int indent = 0);
    };

    class NodeRoot : public Ast {
    public:
        std::vector<std::shared_ptr<Ast>> m_children;

        NodeRoot() : Ast(AstKindRoot) {}
    };

    class NodeVoid : public Ast {
    public:
        NodeVoid() : Ast(AstKindVoid) {}
    };

    class NodeNoReturn : public Ast {
    public:
        NodeNoReturn() : Ast(AstKindNoReturn) {}
    };

    class NodeStrLit : public Ast {
    public:
        std::string m_value;

        NodeStrLit() : Ast(AstKindStrLit) {}
    };

    class NodeNumLit : public Ast {
    public:
        std::string m_value;

        NodeNumLit() : Ast(AstKindNumLit) {}
    };

    class NodeBoolLit : public Ast {
    public:
        bool m_value = false;

        NodeBoolLit() : Ast(AstKindBoolLit) {}
    };

    class NodeImport : public Ast {
    public:
        std::shared_ptr<NodeStrLit> m_path;

        NodeImport() : Ast(AstKindImport) {}
    };

    class NodeIdent : public Ast {
    public:
        std::string m_value;

        NodeIdent() : Ast(AstKindIdent) {}
    };

    class NodeAttribute : public Ast {
    public:
        std::string m_name;
        std::string m_param;

        NodeAttribute() : Ast(AstKindAttribute) {}
    };

    enum TypeKind {
        TypeKindPrimitive,
        TypeKindArray,
        TypeKindPointer,
    };

    class NodeType : public Ast {
    public:
        TypeKind m_type = TypeKindPrimitive;

        explicit NodeType(TypeKind type) : Ast(AstKindType), m_type(type) {}
    };

    class NodeTypePrimitive : public NodeType {
    public:
        std::string m_name;

        NodeTypePrimitive() : NodeType(TypeKindPrimitive) {}
    };

    class NodeTypeArray : public NodeType {
    public:
        std::shared_ptr<NodeType> m_elem_type;
        std::shared_ptr<NodeNumLit> m_size;

        NodeTypeArray() : NodeType(TypeKindArray) {}
    };

    class NodeTypePointer : public NodeType {
    public:
        std::shared_ptr<NodeType> m_elem_type;
        bool m_is_mut = false;

        NodeTypePointer() : NodeType(TypeKindPointer) {}
    };

    class NodeParamDecl : public Ast {
    public:
        std::string m_name;
        std::shared_ptr<NodeType> m_type;

        NodeParamDecl() : Ast(AstKindParamDecl) {}
    };

    class NodeFnProto : public Ast {
    public:
        std::vector<std::shared_ptr<NodeAttribute>> m_attributes;
        bool m_is_public = false;
        std::string m_name;
        std::vector<std::shared_ptr<NodeParamDecl>> m_params;
        bool m_is_variadic = false;
        std::shared_ptr<NodeType> m_return_type;

        NodeFnProto() : Ast(AstKindFnProto) {}
    };

    class NodeFnDecl : public Ast {
    public:
        std::shared_ptr<NodeFnProto> m_proto;

        NodeFnDecl() : Ast(AstKindFnDecl) {}
    };

    class NodeBlock : public Ast {
    public:
        std::vector<std::shared_ptr<Ast>> m_children;

        NodeBlock() : Ast(AstKindBlock) {}
    };

    class NodeFnDef : public Ast {
    public:
        std::shared_ptr<NodeFnProto> m_proto;
        std::shared_ptr<NodeBlock> m_block;

        NodeFnDef() : Ast(AstKindFnDef) {}
    };

    class NodeExternBlock : public Ast {
    public:
        std::vector<std::shared_ptr<NodeAttribute>> m_attributes;
        std::vector<std::shared_ptr<NodeFnDecl>> m_children;

        NodeExternBlock() : Ast(AstKindExternBlock) {}
    };

    class NodeVarDecl : public Ast {
    public:
        std::string m_name;
        std::shared_ptr<NodeType> m_type;
        std::shared_ptr<Ast> m_value;
        bool m_is_const = true;

        NodeVarDecl() : Ast(AstKindVarDecl) {}
    };

    class NodeReturnExpr : public Ast {
    public:
        std::shared_ptr<Ast> m_value;

        NodeReturnExpr() : Ast(AstKindReturnExpr) {}
    };

    class NodeFnCallExpr : public Ast {
    public:
        std::shared_ptr<NodeIdent> m_name;
        std::vector<std::shared_ptr<Ast>> m_params;

        NodeFnCallExpr() : Ast(AstKindFnCallExpr) {}
    };

    class NodeArrayAccessExpr : public Ast {
    public:
        std::shared_ptr<Ast> m_array;
        std::shared_ptr<Ast> m_index;

        NodeArrayAccessExpr() : Ast(AstKindArrayAccessExpr) {}
    };

    class NodeIfExpr : public Ast {
    public:
        std::shared_ptr<Ast> m_condition;
        std::shared_ptr<NodeBlock> m_then_block;
        std::shared_ptr<Ast> m_else_block;

        NodeIfExpr() : Ast(AstKindIfExpr) {}
    };

    class NodeCastExpr : public Ast {
    public:
        std::shared_ptr<Ast> m_value;
        std::shared_ptr<NodeType> m_type;

        NodeCastExpr() : Ast(AstKindCastExpr) {}
    };

    enum BinOp {
        BinOpAssign,
        BinOpLogicalOr,
        BinOpLogicalAnd,
        BinOpCmpEq,
        BinOpCmpNeq,
        BinOpCmpLt,
        BinOpCmpGt,
        BinOpCmpLte,
        BinOpCmpGte,
        BinOpBitOr,
        BinOpBitXor,
        BinOpBitAnd,
        BinOpBitShiftLeft,
        BinOpBitShiftRight,
        BinOpAdd,
        BinOpSub,
        BinOpMul,
        BinOpDiv,
        BinOpMod,
    };

    class NodeBinOpExpr : public Ast {
    public:
        BinOp m_op;
        std::shared_ptr<Ast> m_left;
        std::shared_ptr<Ast> m_right;

        explicit NodeBinOpExpr(BinOp op) : Ast(AstKindBinOpExpr), m_op(op) {}
    };

    enum UnOp {
        UnOpBitNot,
        UnOpNeg,
        UnOpLogicalNot,
    };

    class NodeUnOpExpr : public Ast {
    public:
        UnOp m_op;
        std::shared_ptr<Ast> m_value;

        explicit NodeUnOpExpr(UnOp op) : Ast(AstKindUnOpExpr), m_op(op) {}
    };

} // dal

#endif //DAL_AST_HPP

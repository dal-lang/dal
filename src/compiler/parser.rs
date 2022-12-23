// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use crate::{
    codegen::analyze::ImportTableEntry,
    compiler::ast::{AsmToken, AsmTokenKind},
    ErrColor, ErrMsg,
};

use super::{
    ast::{
        AsmExpr, AsmInput, AsmOutput, BinOpKind, CastExpr, FnProtoVisibMod, Node,
        NodeArrayAccessExpr, NodeBinOpExpr, NodeBlock, NodeCallExpr, NodeData, NodeExtern,
        NodeFnDecl, NodeFnDef, NodeFnProto, NodeIfExpr, NodeImport, NodeKind, NodeParamDecl,
        NodeReturn, NodeRoot, NodeType, NodeUnaryOpExpr, NodeVarDecl, SrcPos, TypeKind,
        UnaryOpKind,
    },
    tokenizer::{Token, TokenKind},
};
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
struct ParseContext<'a> {
    src: String,
    root: Option<Node>,
    tokens: Vec<Token>,
    owner: RefCell<&'a mut ImportTableEntry>,
    err_color: ErrColor,
}

impl<'a> ParseContext<'a> {
    fn new(owner: &'a mut ImportTableEntry) -> Self {
        Self {
            src: String::new(),
            root: None,
            tokens: Vec::new(),
            owner: RefCell::new(owner),
            err_color: ErrColor::Off,
        }
    }

    fn ast_error(&self, token: &Token, msg: String) -> ! {
        let err = ErrMsg {
            line_start: token.start_line,
            col_start: token.start_col,
            msg,
            path: self.owner.borrow().path.clone(),
            src: self.owner.borrow().src_code.clone(),
            line_offsets: self.owner.borrow().line_offsets.clone(),
        };
        err.print(&self.err_color);
    }

    fn invalid_token_error(&self, token: &Token) -> ! {
        self.ast_error(
            token,
            format!("Invalid token error: '{}'", token.kind.to_str()),
        );
    }

    fn expect_token(&self, current: &Token, expected: TokenKind) {
        if current.kind != expected {
            self.invalid_token_error(current);
        }
    }

    fn tok_val(&self, token: &Token) -> &str {
        &self.src[token.start_pos..token.end_pos]
    }

    fn ast_asm_error(&self, node: &Node, offset: usize, msg: String) -> ! {
        assert_eq!(node.kind(), NodeKind::AsmExpr);
        let src_pos = &node.data().asm_expr().borrow().offset_map.borrow().clone()[offset];
        let err = ErrMsg {
            line_start: src_pos.line.clone(),
            col_start: src_pos.col.clone(),
            msg,
            path: self.owner.borrow().path.clone(),
            src: self.owner.borrow().src_code.clone(),
            line_offsets: self.owner.borrow().line_offsets.clone(),
        };
        err.print(&self.err_color);
    }

    fn create_node_no_line_info(&self, kind: NodeKind) -> Node {
        let node = Node::new(kind);
        match kind {
            NodeKind::Root => {
                node.set_data(NodeData::Root(RefCell::new(NodeRoot::new())));
            }
            NodeKind::FnProto => {
                node.set_data(NodeData::FnProto(RefCell::new(NodeFnProto::new())));
            }
            NodeKind::FnDef => {
                node.set_data(NodeData::FnDef(RefCell::new(NodeFnDef::new())));
            }
            NodeKind::FnDecl => {
                node.set_data(NodeData::FnDecl(RefCell::new(NodeFnDecl::new())));
            }
            NodeKind::ParamDecl => {
                node.set_data(NodeData::ParamDecl(RefCell::new(NodeParamDecl::new())));
            }
            NodeKind::Type => {
                node.set_data(NodeData::Type(RefCell::new(NodeType::new())));
            }
            NodeKind::Block => {
                node.set_data(NodeData::Block(RefCell::new(NodeBlock::new())));
            }
            NodeKind::ExternBlock => {
                node.set_data(NodeData::ExternBlock(RefCell::new(NodeExtern::new())));
            }
            NodeKind::Import => {
                node.set_data(NodeData::Import(RefCell::new(NodeImport::new())));
            }
            NodeKind::Return => {
                node.set_data(NodeData::Return(RefCell::new(NodeReturn::new())));
            }
            NodeKind::VarDecl => {
                node.set_data(NodeData::VarDecl(RefCell::new(NodeVarDecl::new())));
            }
            NodeKind::BinOpExpr => {
                node.set_data(NodeData::BinOpExpr(RefCell::new(NodeBinOpExpr::new())));
            }
            NodeKind::UnaryOpExpr => {
                node.set_data(NodeData::UnaryOpExpr(RefCell::new(NodeUnaryOpExpr::new())));
            }
            NodeKind::CallExpr => {
                node.set_data(NodeData::CallExpr(RefCell::new(NodeCallExpr::new())));
            }
            NodeKind::Ident => {
                node.set_data(NodeData::Ident(RefCell::new(String::new())));
            }
            NodeKind::ArrayAccessExpr => {
                node.set_data(NodeData::ArrayAccessExpr(RefCell::new(
                    NodeArrayAccessExpr::new(),
                )));
            }
            NodeKind::CastExpr => {
                node.set_data(NodeData::CastExpr(RefCell::new(CastExpr::new())));
            }
            NodeKind::IfExpr => {
                node.set_data(NodeData::IfExpr(RefCell::new(NodeIfExpr::new())));
            }
            NodeKind::AsmExpr => {
                node.set_data(NodeData::AsmExpr(RefCell::new(AsmExpr::new())));
            }
            NodeKind::StrLit => {
                node.set_data(NodeData::StrLit(RefCell::new(String::new())));
            }
            NodeKind::NumLit => {
                node.set_data(NodeData::NumLit(RefCell::new(String::new())));
            }
            NodeKind::BoolLit => {
                node.set_data(NodeData::BoolLit(RefCell::new(false)));
            }
            NodeKind::Void => {}
        }
        node
    }

    fn update_node_line_info(&self, node: &Node, token: &Token) {
        node.set_line(token.start_line);
        node.set_col(token.start_col);
    }

    fn create_node(&self, kind: NodeKind, token: &Token) -> Node {
        let node = self.create_node_no_line_info(kind);
        self.update_node_line_info(&node, token);
        node
    }

    fn create_node_with_node(&self, kind: NodeKind, node: &Node) -> Node {
        let new_node = self.create_node_no_line_info(kind);
        new_node.set_line(node.line.borrow().clone());
        new_node.set_col(node.col.borrow().clone());
        new_node
    }

    fn create_void_type_node(&self, token: &Token) -> Node {
        let node = self.create_node(NodeKind::Type, token);
        node.data
            .borrow()
            .type_()
            .borrow()
            .set_kind(TypeKind::Primitive);
        node.data
            .borrow()
            .type_()
            .borrow()
            .set_name("void".to_string());
        node
    }

    fn parse_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let tok = &self.tokens[*tok_index];

        let block = self.parse_block_expr(tok_index, false);
        if block.is_some() {
            return block;
        }

        let non_block = self.parse_non_block_expr(tok_index, false);
        if non_block.is_some() {
            return non_block;
        }

        if mandatory {
            self.invalid_token_error(tok);
        }

        None
    }

    fn parse_type(&self, tok_index: usize, new_tok_index: &mut usize) -> Node {
        let mut tok_index = tok_index;
        let tok = &self.tokens[tok_index];
        tok_index += 1;

        let node = self.create_node(NodeKind::Type, tok);
        if tok.kind == TokenKind::KwVoid {
            node.data
                .borrow()
                .type_()
                .borrow()
                .set_kind(TypeKind::Primitive);
            node.data
                .borrow()
                .type_()
                .borrow()
                .set_name("void".to_string());
        } else if tok.kind == TokenKind::Ident {
            node.data
                .borrow()
                .type_()
                .borrow()
                .set_kind(TypeKind::Primitive);
            node.data
                .borrow()
                .type_()
                .borrow()
                .set_name(self.tok_val(tok).to_string());
        } else if tok.kind == TokenKind::Star {
            node.data
                .borrow()
                .type_()
                .borrow()
                .set_kind(TypeKind::Pointer);
            let const_or_mut = &self.tokens[tok_index];
            tok_index += 1;
            if const_or_mut.kind == TokenKind::KwMut {
                node.data.borrow().type_().borrow().set_is_const(false);
            } else if const_or_mut.kind == TokenKind::KwConst {
                node.data.borrow().type_().borrow().set_is_const(true);
            } else {
                self.invalid_token_error(const_or_mut);
            }
            node.data
                .borrow()
                .type_()
                .borrow()
                .set_child_type(self.parse_type(tok_index, &mut tok_index));
        } else if tok.kind == TokenKind::LBracket {
            node.data
                .borrow()
                .type_()
                .borrow()
                .set_kind(TypeKind::Array);
            node.data
                .borrow()
                .type_()
                .borrow()
                .set_child_type(self.parse_type(tok_index, &mut tok_index));
            let semi = &self.tokens[tok_index];
            tok_index += 1;
            self.expect_token(semi, TokenKind::Semicolon);
            node.data
                .borrow()
                .type_()
                .borrow()
                .set_array_size(self.parse_expr(&mut tok_index, true).unwrap());
            let rbracket = &self.tokens[tok_index];
            tok_index += 1;
            self.expect_token(rbracket, TokenKind::RBracket);
        } else {
            self.invalid_token_error(tok);
        }

        *new_tok_index = tok_index;
        node
    }

    fn parse_param_decl(&self, tok_index: usize, new_tok_index: &mut usize) -> Option<Node> {
        let mut tok_index = tok_index;
        let param_name = &self.tokens[tok_index];
        tok_index += 1;

        if param_name.kind == TokenKind::Ident {
            let node = self.create_node(NodeKind::ParamDecl, param_name);
            node.data
                .borrow()
                .param_decl()
                .borrow()
                .set_name(self.tok_val(param_name).to_string());
            let colon = &self.tokens[tok_index];
            tok_index += 1;
            self.expect_token(colon, TokenKind::Colon);
            node.data
                .borrow()
                .param_decl()
                .borrow()
                .set_param_type(self.parse_type(tok_index, &mut tok_index));
            *new_tok_index = tok_index;
            Some(node)
        } else {
            self.invalid_token_error(param_name);
        }
    }

    fn parse_param_decl_list(
        &self,
        tok_index: usize,
        new_tok_index: &mut usize,
        params: &NodeFnProto,
    ) {
        let mut tok_index = tok_index;
        let lparen = &self.tokens[tok_index];
        tok_index += 1;
        self.expect_token(lparen, TokenKind::LParen);

        let token = &self.tokens[tok_index];
        if token.kind == TokenKind::RParen {
            tok_index += 1;
            *new_tok_index = tok_index;
            return;
        }

        loop {
            let param_decl_node = self.parse_param_decl(tok_index, &mut tok_index);
            if param_decl_node.is_some() {
                params.push_param(param_decl_node.unwrap());
            }

            let token = &self.tokens[tok_index];
            tok_index += 1;
            if token.kind == TokenKind::RParen {
                *new_tok_index = tok_index;
                return;
            } else {
                self.expect_token(token, TokenKind::Comma);
            }
        }
    }

    fn parse_var_decl_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let let_tok = &self.tokens[*tok_index];
        if let_tok.kind == TokenKind::KwLet {
            *tok_index += 1;
            let node = self.create_node(NodeKind::VarDecl, let_tok);
            let name_tok: &Token;
            let tok = &self.tokens[*tok_index];
            if tok.kind == TokenKind::KwMut {
                node.data.borrow().var_decl().borrow().set_is_const(false);
                *tok_index += 1;
                name_tok = &self.tokens[*tok_index];
                self.expect_token(name_tok, TokenKind::Ident);
            } else if tok.kind == TokenKind::Ident {
                node.data.borrow().var_decl().borrow().set_is_const(true);
                name_tok = tok;
            } else {
                self.invalid_token_error(tok);
            }
            *tok_index += 1;
            node.data
                .borrow()
                .var_decl()
                .borrow()
                .set_name(self.tok_val(name_tok).to_string());

            let assign_or_colon = &self.tokens[*tok_index];
            *tok_index += 1;
            if assign_or_colon.kind == TokenKind::Assign {
                node.data
                    .borrow()
                    .var_decl()
                    .borrow()
                    .set_expr(self.parse_expr(tok_index, true).unwrap());
                return Some(node);
            } else if assign_or_colon.kind == TokenKind::Colon {
                node.data
                    .borrow()
                    .var_decl()
                    .borrow()
                    .set_var_type(self.parse_type(*tok_index, tok_index));

                let assign = &self.tokens[*tok_index];
                if assign.kind == TokenKind::Assign {
                    *tok_index += 1;
                    node.data
                        .borrow()
                        .var_decl()
                        .borrow()
                        .set_expr(self.parse_expr(tok_index, true).unwrap());
                }
                return Some(node);
            } else {
                self.invalid_token_error(assign_or_colon);
            }
        } else if mandatory {
            self.invalid_token_error(let_tok);
        } else {
            return None;
        }
    }

    fn parse_else_or_else_if(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let else_tok = &self.tokens[*tok_index];
        if else_tok.kind != TokenKind::KwElse {
            if mandatory {
                self.invalid_token_error(else_tok);
            } else {
                return None;
            }
        }
        *tok_index += 1;
        let if_expr = self.parse_if_expr(tok_index, false);
        if if_expr.is_some() {
            return if_expr;
        }
        self.parse_block(tok_index, true)
    }

    fn parse_if_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let if_tok = &self.tokens[*tok_index];
        if if_tok.kind != TokenKind::KwIf {
            if mandatory {
                self.invalid_token_error(if_tok);
            } else {
                return None;
            }
        }
        *tok_index += 1;

        let node = self.create_node(NodeKind::IfExpr, if_tok);
        node.data
            .borrow()
            .if_expr()
            .borrow()
            .set_cond(self.parse_expr(tok_index, true).unwrap());
        node.data
            .borrow()
            .if_expr()
            .borrow()
            .set_then(self.parse_block_expr(tok_index, true).unwrap());
        if let Some(else_expr) = self.parse_else_or_else_if(tok_index, false) {
            node.data.borrow().if_expr().borrow().set_else(else_expr);
        }

        Some(node)
    }

    fn parse_block_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let tok = &self.tokens[*tok_index];

        let if_expr = self.parse_if_expr(tok_index, false);
        if if_expr.is_some() {
            return if_expr;
        }

        let block = self.parse_block(tok_index, false);
        if block.is_some() {
            return block;
        }

        if mandatory {
            self.invalid_token_error(tok);
        }
        None
    }

    fn parse_return_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let return_tok = &self.tokens[*tok_index];
        if return_tok.kind == TokenKind::KwReturn {
            *tok_index += 1;
            let node = self.create_node(NodeKind::Return, return_tok);
            if let Some(expr) = self.parse_expr(tok_index, false) {
                node.data.borrow().return_().borrow().set_expr(expr);
            }

            Some(node)
        } else if mandatory {
            self.invalid_token_error(return_tok);
        } else {
            None
        }
    }

    fn parse_grouped_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let lparen = &self.tokens[*tok_index];
        if lparen.kind != TokenKind::LParen {
            if mandatory {
                self.invalid_token_error(lparen);
            } else {
                return None;
            }
        }
        *tok_index += 1;

        let node = self.parse_expr(tok_index, true);

        let rparen = &self.tokens[*tok_index];
        *tok_index += 1;
        self.expect_token(rparen, TokenKind::RParen);

        node
    }

    fn parse_primary_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let tok = &self.tokens[*tok_index];

        if tok.kind == TokenKind::NumLit {
            let node = self.create_node(NodeKind::NumLit, tok);
            node.set_data(NodeData::NumLit(RefCell::new(
                self.tok_val(tok).to_string(),
            )));
            *tok_index += 1;
            return Some(node);
        } else if tok.kind == TokenKind::StrLit {
            let node = self.create_node(NodeKind::StrLit, tok);
            node.set_data(NodeData::StrLit(RefCell::new(String::new())));
            self.parse_string_literal(tok, node.data.borrow().str_lit(), None);
            *tok_index += 1;
            return Some(node);
        } else if tok.kind == TokenKind::KwVoid {
            let node = self.create_node(NodeKind::Void, tok);
            *tok_index += 1;
            return Some(node);
        } else if tok.kind == TokenKind::KwTrue {
            let node = self.create_node(NodeKind::BoolLit, tok);
            node.set_data(NodeData::BoolLit(RefCell::new(true)));
            *tok_index += 1;
            return Some(node);
        } else if tok.kind == TokenKind::KwFalse {
            let node = self.create_node(NodeKind::BoolLit, tok);
            node.set_data(NodeData::BoolLit(RefCell::new(false)));
            *tok_index += 1;
            return Some(node);
        } else if tok.kind == TokenKind::Ident {
            let node = self.create_node(NodeKind::Ident, tok);
            node.set_data(NodeData::Ident(RefCell::new(self.tok_val(tok).to_string())));
            *tok_index += 1;
            return Some(node);
        }

        let grouped_expr = self.parse_grouped_expr(tok_index, false);
        if grouped_expr.is_some() {
            return grouped_expr;
        }

        if !mandatory {
            return None;
        }

        self.invalid_token_error(tok);
    }

    fn parse_call_params(&self, tok_index: usize, new_tok_index: &mut usize, call: &NodeCallExpr) {
        let mut tok_index = tok_index;
        let tok = &self.tokens[tok_index];
        if tok.kind == TokenKind::RParen {
            tok_index += 1;
            *new_tok_index = tok_index;
            return;
        }

        loop {
            let expr = self.parse_expr(&mut tok_index, true);
            call.add_arg(expr.unwrap());
            let tok = &self.tokens[tok_index];
            tok_index += 1;
            if tok.kind == TokenKind::RParen {
                *new_tok_index = tok_index;
                return;
            } else {
                self.expect_token(tok, TokenKind::Comma);
            }
        }
    }

    fn parse_post_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let primary = self.parse_primary_expr(tok_index, mandatory);
        if primary.is_none() {
            return None;
        }

        let tok = &self.tokens[*tok_index];
        if tok.kind == TokenKind::LParen {
            *tok_index += 1;
            let node = self.create_node(NodeKind::CallExpr, tok);
            node.data
                .borrow()
                .call_expr()
                .borrow()
                .set_callee(primary.unwrap());
            self.parse_call_params(
                *tok_index,
                tok_index,
                &node.data.borrow().call_expr().borrow(),
            );

            Some(node)
        } else if tok.kind == TokenKind::LBracket {
            *tok_index += 1;
            let node = self.create_node(NodeKind::ArrayAccessExpr, tok);
            node.data
                .borrow()
                .array_access_expr()
                .borrow()
                .set_array(primary.unwrap());
            node.data
                .borrow()
                .array_access_expr()
                .borrow()
                .set_index(self.parse_expr(tok_index, true).unwrap());
            let rbracket = &self.tokens[*tok_index];
            *tok_index += 1;
            self.expect_token(rbracket, TokenKind::RBracket);
            Some(node)
        } else {
            primary
        }
    }

    fn tok_to_unary_op(&self, tok: &Token) -> UnaryOpKind {
        match tok.kind {
            TokenKind::Bang => UnaryOpKind::BoolNot,
            TokenKind::Dash => UnaryOpKind::Neg,
            TokenKind::BitNot => UnaryOpKind::Not,
            _ => UnaryOpKind::Invalid,
        }
    }

    fn parse_unary_op(&self, tok_index: &mut usize, mandatory: bool) -> UnaryOpKind {
        let tok = &self.tokens[*tok_index];
        let res = self.tok_to_unary_op(tok);
        if res == UnaryOpKind::Invalid {
            if mandatory {
                self.invalid_token_error(tok);
            } else {
                return UnaryOpKind::Invalid;
            }
        }
        *tok_index += 1;
        res
    }

    fn parse_unary_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let tok = &self.tokens[*tok_index];
        let unary_op = self.parse_unary_op(tok_index, false);
        if unary_op == UnaryOpKind::Invalid {
            return self.parse_post_expr(tok_index, mandatory);
        }

        let unary_expr = self.parse_post_expr(tok_index, true);

        let node = self.create_node(NodeKind::UnaryOpExpr, tok);
        node.data
            .borrow()
            .unary_op_expr()
            .borrow()
            .set_expr(unary_expr.unwrap());
        node.data.borrow().unary_op_expr().borrow().set_op(unary_op);
        Some(node)
    }

    fn parse_cast_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let un = self.parse_unary_expr(tok_index, mandatory);
        if un.is_none() {
            return None;
        }

        let as_kw = &self.tokens[*tok_index];
        if as_kw.kind != TokenKind::KwAs {
            return un;
        }
        *tok_index += 1;

        let node = self.create_node(NodeKind::CastExpr, as_kw);
        node.data
            .borrow()
            .cast_expr()
            .borrow()
            .set_expr(un.unwrap());
        node.data
            .borrow()
            .cast_expr()
            .borrow()
            .set_cast_type(self.parse_type(*tok_index, tok_index));
        Some(node)
    }

    fn tok_to_mul_op(&self, tok: &Token) -> BinOpKind {
        match tok.kind {
            TokenKind::Star => BinOpKind::Mul,
            TokenKind::Slash => BinOpKind::Div,
            TokenKind::Percent => BinOpKind::Mod,
            _ => BinOpKind::Invalid,
        }
    }

    fn parse_mul_op(&self, tok_index: &mut usize, mandatory: bool) -> BinOpKind {
        let tok = &self.tokens[*tok_index];
        let res = self.tok_to_mul_op(tok);
        if res == BinOpKind::Invalid {
            if mandatory {
                self.invalid_token_error(tok);
            } else {
                return BinOpKind::Invalid;
            }
        }
        *tok_index += 1;
        res
    }

    fn parse_mul_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let op1 = self.parse_cast_expr(tok_index, mandatory);
        if op1.is_none() {
            return None;
        }

        let tok = &self.tokens[*tok_index];
        let mul_op = self.parse_mul_op(tok_index, false);
        if mul_op == BinOpKind::Invalid {
            return op1;
        }
        *tok_index += 1;

        let op2 = self.parse_cast_expr(tok_index, true);

        let node = self.create_node(NodeKind::BinOpExpr, tok);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_lhs(op1.unwrap());
        node.data.borrow().bin_op_expr().borrow().set_op(mul_op);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_rhs(op2.unwrap());
        Some(node)
    }

    fn tok_to_add_op(&self, tok: &Token) -> BinOpKind {
        match tok.kind {
            TokenKind::Plus => BinOpKind::Add,
            TokenKind::Dash => BinOpKind::Sub,
            _ => BinOpKind::Invalid,
        }
    }

    fn parse_add_op(&self, tok_index: &mut usize, mandatory: bool) -> BinOpKind {
        let tok = &self.tokens[*tok_index];
        let res = self.tok_to_add_op(tok);
        if res == BinOpKind::Invalid {
            if mandatory {
                self.invalid_token_error(tok);
            } else {
                return BinOpKind::Invalid;
            }
        }
        *tok_index += 1;
        res
    }

    fn parse_add_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let op1 = self.parse_mul_expr(tok_index, mandatory);
        if op1.is_none() {
            return None;
        }

        let tok = &self.tokens[*tok_index];
        let add_op = self.parse_add_op(tok_index, false);
        if add_op == BinOpKind::Invalid {
            return op1;
        }

        let op2 = self.parse_mul_expr(tok_index, true);

        let node = self.create_node(NodeKind::BinOpExpr, tok);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_lhs(op1.unwrap());
        node.data.borrow().bin_op_expr().borrow().set_op(add_op);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_rhs(op2.unwrap());
        Some(node)
    }

    fn tok_to_bit_shift_op(&self, tok: &Token) -> BinOpKind {
        match tok.kind {
            TokenKind::BitShl => BinOpKind::Shl,
            TokenKind::BitShr => BinOpKind::Shr,
            _ => BinOpKind::Invalid,
        }
    }

    fn parse_bit_shift_op(&self, tok_index: &mut usize, mandatory: bool) -> BinOpKind {
        let tok = &self.tokens[*tok_index];
        let res = self.tok_to_bit_shift_op(tok);
        if res == BinOpKind::Invalid {
            if mandatory {
                self.invalid_token_error(tok);
            } else {
                return BinOpKind::Invalid;
            }
        }
        *tok_index += 1;
        res
    }

    fn parse_bit_shift_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let op1 = self.parse_add_expr(tok_index, mandatory);
        if op1.is_none() {
            return None;
        }

        let tok = &self.tokens[*tok_index];
        let bit_shift_op = self.parse_bit_shift_op(tok_index, false);
        if bit_shift_op == BinOpKind::Invalid {
            return op1;
        }

        let op2 = self.parse_add_expr(tok_index, true);

        let node = self.create_node(NodeKind::BinOpExpr, tok);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_lhs(op1.unwrap());
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_op(bit_shift_op);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_rhs(op2.unwrap());
        Some(node)
    }

    fn parse_bin_and_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let op1 = self.parse_bit_shift_expr(tok_index, mandatory);
        if op1.is_none() {
            return None;
        }

        let tok = &self.tokens[*tok_index];
        if tok.kind != TokenKind::BoolAnd {
            return op1;
        }
        *tok_index += 1;

        let op2 = self.parse_bit_shift_expr(tok_index, true);

        let node = self.create_node(NodeKind::BinOpExpr, tok);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_lhs(op1.unwrap());
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_op(BinOpKind::BoolAnd);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_rhs(op2.unwrap());
        Some(node)
    }

    fn parse_bin_xor_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let op1 = self.parse_bin_and_expr(tok_index, mandatory);
        if op1.is_none() {
            return None;
        }

        let tok = &self.tokens[*tok_index];
        if tok.kind != TokenKind::BitXor {
            return op1;
        }
        *tok_index += 1;

        let op2 = self.parse_bin_and_expr(tok_index, true);

        let node = self.create_node(NodeKind::BinOpExpr, tok);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_lhs(op1.unwrap());
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_op(BinOpKind::BoolAnd);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_rhs(op2.unwrap());
        Some(node)
    }

    fn parse_bin_or_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let op1 = self.parse_bin_xor_expr(tok_index, mandatory);
        if op1.is_none() {
            return None;
        }

        let tok = &self.tokens[*tok_index];
        if tok.kind != TokenKind::BitOr {
            return op1;
        }
        *tok_index += 1;

        let op2 = self.parse_bin_xor_expr(tok_index, true);

        let node = self.create_node(NodeKind::BinOpExpr, tok);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_lhs(op1.unwrap());
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_op(BinOpKind::Or);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_rhs(op2.unwrap());
        Some(node)
    }

    fn tok_to_cmp_op(&self, tok: &Token) -> BinOpKind {
        match tok.kind {
            TokenKind::Assign => BinOpKind::Assign,
            TokenKind::CmpEq => BinOpKind::Eq,
            TokenKind::CmpNeq => BinOpKind::Neq,
            TokenKind::CmpLt => BinOpKind::Lt,
            TokenKind::CmpGt => BinOpKind::Gt,
            TokenKind::CmpLte => BinOpKind::Lte,
            TokenKind::CmpGte => BinOpKind::Gte,
            _ => BinOpKind::Invalid,
        }
    }

    fn parse_cmp_op(&self, tok_index: &mut usize, mandatory: bool) -> BinOpKind {
        let tok = &self.tokens[*tok_index];
        let res = self.tok_to_cmp_op(tok);
        if res == BinOpKind::Invalid {
            if mandatory {
                self.invalid_token_error(tok);
            } else {
                return BinOpKind::Invalid;
            }
        }
        *tok_index += 1;
        res
    }

    fn parse_cmp_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let op1 = self.parse_bin_or_expr(tok_index, mandatory);
        if op1.is_none() {
            return None;
        }

        let tok = &self.tokens[*tok_index];
        let cmp_op = self.parse_cmp_op(tok_index, false);
        if cmp_op == BinOpKind::Invalid {
            return op1;
        }

        let op2 = self.parse_bin_or_expr(tok_index, true);
        let node = self.create_node(NodeKind::BinOpExpr, tok);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_lhs(op1.unwrap());
        node.data.borrow().bin_op_expr().borrow().set_op(cmp_op);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_rhs(op2.unwrap());
        Some(node)
    }

    fn parse_bool_and_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let op1 = self.parse_cmp_expr(tok_index, mandatory);
        if op1.is_none() {
            return None;
        }

        let tok = &self.tokens[*tok_index];
        if tok.kind != TokenKind::BoolAnd {
            return op1;
        }
        *tok_index += 1;

        let op2 = self.parse_cmp_expr(tok_index, true);

        let node = self.create_node(NodeKind::BinOpExpr, tok);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_lhs(op1.unwrap());
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_op(BinOpKind::BoolAnd);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_rhs(op2.unwrap());
        Some(node)
    }

    fn parse_bool_or_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let op1 = self.parse_bool_and_expr(tok_index, mandatory);
        if op1.is_none() {
            return None;
        }

        let tok = &self.tokens[*tok_index];
        if tok.kind != TokenKind::BoolOr {
            return op1;
        }
        *tok_index += 1;

        let op2 = self.parse_bool_and_expr(tok_index, true);

        let node = self.create_node(NodeKind::BinOpExpr, tok);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_lhs(op1.unwrap());
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_op(BinOpKind::BoolOr);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_rhs(op2.unwrap());
        Some(node)
    }

    fn parse_assignment_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let lhs = self.parse_bool_or_expr(tok_index, mandatory);
        if lhs.is_none() {
            return None;
        }

        let tok = &self.tokens[*tok_index];
        if tok.kind != TokenKind::Assign {
            return lhs;
        }
        *tok_index += 1;

        let rhs = self.parse_bool_or_expr(tok_index, true);
        let node = self.create_node(NodeKind::BinOpExpr, tok);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_lhs(lhs.unwrap());
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_op(BinOpKind::Assign);
        node.data
            .borrow()
            .bin_op_expr()
            .borrow()
            .set_rhs(rhs.unwrap());
        Some(node)
    }

    fn parse_asm_template(&self, node: &Node) {
        let binding = node.data.borrow();
        let asm_expr = binding.asm_expr().borrow();

        enum State {
            Start,
            Percent,
            Template,
            Var,
        }

        assert_eq!(asm_expr.token_list.borrow().len(), 0);

        let mut state = State::Start;

        let mut i = 0;
        while i < asm_expr.asm_template.borrow().len() {
            let c = asm_expr.asm_template.borrow().chars().nth(i).unwrap();
            match state {
                State::Start => {
                    if c == '%' {
                        let new_tok = AsmToken::new();
                        new_tok.set_kind(AsmTokenKind::Percent);
                        new_tok.set_start(i);
                        new_tok.set_end(0);
                        asm_expr.add_token_list(new_tok);
                        state = State::Percent;
                    } else {
                        let new_tok = AsmToken::new();
                        new_tok.set_kind(AsmTokenKind::Template);
                        new_tok.set_start(i);
                        new_tok.set_end(0);
                        asm_expr.add_token_list(new_tok);
                        state = State::Template;
                    }
                }
                State::Percent => {
                    if c == '%' {
                        asm_expr.token_list.borrow().last().unwrap().set_end(i);
                        state = State::Start;
                    } else if c == '[' {
                        asm_expr
                            .token_list
                            .borrow()
                            .last()
                            .unwrap()
                            .set_kind(AsmTokenKind::Var);
                        state = State::Var;
                    } else {
                        self.ast_asm_error(node, i, "invalid asm template".to_string());
                    }
                }
                State::Template => {
                    if c == '%' {
                        asm_expr.token_list.borrow().last().unwrap().set_end(i);
                        i -= 1;
                        state = State::Start;
                    }
                }
                State::Var => {
                    if c == ']' {
                        asm_expr.token_list.borrow().last().unwrap().set_end(i);
                        state = State::Start;
                    } else if (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') || c == '_' {
                        // do nothing
                    } else {
                        self.ast_asm_error(node, i, "invalid asm template".to_string());
                    }
                }
            }
            i += 1;
        }

        match state {
            State::Start => {}
            State::Percent | State::Var => {
                self.ast_asm_error(
                    node,
                    asm_expr.asm_template.borrow().len(),
                    "invalid asm template".to_string(),
                );
            }
            State::Template => {
                asm_expr
                    .token_list
                    .borrow()
                    .last()
                    .unwrap()
                    .set_end(asm_expr.asm_template.borrow().len());
            }
        }
    }

    fn eat_token(&self, tok_index: &mut usize, kind: TokenKind) -> Token {
        let tok = &self.tokens[*tok_index];
        self.expect_token(tok, kind);
        *tok_index += 1;
        tok.clone()
    }

    fn parse_asm_output_item(&self, tok_index: &mut usize, node: &Node) {
        self.eat_token(tok_index, TokenKind::LBracket);
        let alias = self.eat_token(tok_index, TokenKind::Ident);
        self.eat_token(tok_index, TokenKind::RBracket);

        let constraint = self.eat_token(tok_index, TokenKind::StrLit);

        self.eat_token(tok_index, TokenKind::LParen);
        let out_ident = self.eat_token(tok_index, TokenKind::Ident);
        self.eat_token(tok_index, TokenKind::RParen);

        let output = AsmOutput::new();
        output.set_symbolic_name(self.tok_val(&alias).to_string());
        output.set_var_name(self.tok_val(&out_ident).to_string());

        self.parse_string_literal(&constraint, &output.constraint, None);
        node.data
            .borrow()
            .asm_expr()
            .borrow()
            .add_output_list(output);
    }

    fn parse_asm_input_item(&self, tok_index: &mut usize, node: &Node) {
        self.eat_token(tok_index, TokenKind::LBracket);
        let alias = self.eat_token(tok_index, TokenKind::Ident);
        self.eat_token(tok_index, TokenKind::RBracket);

        let constraint = self.eat_token(tok_index, TokenKind::StrLit);

        self.eat_token(tok_index, TokenKind::LParen);
        let expr = self.parse_expr(tok_index, true);
        self.eat_token(tok_index, TokenKind::RParen);

        let input = AsmInput::new();
        input.set_symbolic_name(self.tok_val(&alias).to_string());
        input.set_expr(expr.unwrap());

        self.parse_string_literal(&constraint, &input.constraint, None);
        node.data.borrow().asm_expr().borrow().add_input_list(input);
    }

    fn parse_asm_clobbers(&self, tok_index: &mut usize, node: &Node) {
        let colon = &self.tokens[*tok_index];
        if colon.kind != TokenKind::Colon {
            return;
        }
        *tok_index += 1;

        loop {
            let str = &self.tokens[*tok_index];
            self.expect_token(str, TokenKind::StrLit);
            *tok_index += 1;

            let clobber = RefCell::new(String::new());
            self.parse_string_literal(str, &clobber, None);
            node.data
                .borrow()
                .asm_expr()
                .borrow()
                .add_clobber_list(clobber.borrow().clone());

            let comma = &self.tokens[*tok_index];
            if comma.kind == TokenKind::Comma {
                *tok_index += 1;
            } else {
                break;
            }
        }
    }

    fn parse_asm_input(&self, tok_index: &mut usize, node: &Node) {
        let colon = &self.tokens[*tok_index];
        if colon.kind != TokenKind::Colon {
            return;
        }
        *tok_index += 1;

        loop {
            self.parse_asm_input_item(tok_index, node);
            let comma = &self.tokens[*tok_index];
            if comma.kind == TokenKind::Comma {
                *tok_index += 1;
            } else {
                break;
            }
        }

        self.parse_asm_clobbers(tok_index, node);
    }

    fn parse_asm_output(&self, tok_index: &mut usize, node: &Node) {
        let colon = &self.tokens[*tok_index];
        if colon.kind != TokenKind::Colon {
            return;
        }
        *tok_index += 1;

        loop {
            self.parse_asm_output_item(tok_index, node);
            let comma = &self.tokens[*tok_index];
            if comma.kind == TokenKind::Comma {
                *tok_index += 1;
            } else {
                break;
            }
        }

        self.parse_asm_input(tok_index, node);
    }

    fn parse_asm_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let asm_tok = &self.tokens[*tok_index];
        if asm_tok.kind != TokenKind::KwAsm {
            if mandatory {
                self.invalid_token_error(asm_tok);
            } else {
                return None;
            }
        }

        let node = self.create_node(NodeKind::AsmExpr, asm_tok);
        node.data
            .borrow()
            .asm_expr()
            .borrow()
            .set_asm_template(String::new());
        *tok_index += 1;

        let lparen = &self.tokens[*tok_index];
        self.expect_token(lparen, TokenKind::LParen);
        *tok_index += 1;

        let asm_str = &self.tokens[*tok_index];
        self.expect_token(asm_str, TokenKind::StrLit);
        *tok_index += 1;

        self.parse_string_literal(
            asm_str,
            &node.data.borrow().asm_expr().borrow().asm_template,
            Some(&node.data.borrow().asm_expr().borrow().offset_map),
        );
        self.parse_asm_template(&node);
        self.parse_asm_output(tok_index, &node);

        let rparen = &self.tokens[*tok_index];
        self.expect_token(rparen, TokenKind::RParen);
        *tok_index += 1;

        Some(node)
    }

    fn parse_string_literal(
        &self,
        token: &Token,
        buf: &RefCell<String>,
        offset_map: Option<&RefCell<Vec<SrcPos>>>,
    ) {
        let mut escape = false;
        let mut first = true;
        let mut pos = SrcPos::new();
        pos.set_line(token.start_line);
        pos.set_col(token.start_col);
        let mut i = token.start_pos;
        while i < token.end_pos - 1 {
            let c = self.src.chars().nth(i).unwrap();
            if first {
                first = false;
            } else {
                if escape {
                    match c {
                        '\\' => {
                            buf.borrow_mut().push('\\');
                            if offset_map.is_some() {
                                offset_map.as_ref().unwrap().borrow_mut().push(pos.clone());
                            }
                        }
                        'r' => {
                            buf.borrow_mut().push('\r');
                            if offset_map.is_some() {
                                offset_map.as_ref().unwrap().borrow_mut().push(pos.clone());
                            }
                        }
                        'n' => {
                            buf.borrow_mut().push('\n');
                            if offset_map.is_some() {
                                offset_map.as_ref().unwrap().borrow_mut().push(pos.clone());
                            }
                        }
                        't' => {
                            buf.borrow_mut().push('\t');
                            if offset_map.is_some() {
                                offset_map.as_ref().unwrap().borrow_mut().push(pos.clone());
                            }
                        }
                        '"' => {
                            buf.borrow_mut().push('"');
                            if offset_map.is_some() {
                                offset_map.as_ref().unwrap().borrow_mut().push(pos.clone());
                            }
                        }
                        _ => {}
                    }
                    escape = false;
                } else if c == '\\' {
                    escape = true;
                } else {
                    buf.borrow_mut().push(c);
                    if offset_map.is_some() {
                        offset_map.as_ref().unwrap().borrow_mut().push(pos.clone());
                    }
                }
            }
            if c == '\n' {
                pos.set_line(pos.line.clone() + 1);
                pos.set_col(0);
            } else {
                pos.set_col(pos.col.clone() + 1);
            }
            i += 1;
        }
        if offset_map.is_some() {
            offset_map.as_ref().unwrap().borrow_mut().push(pos.clone());
        }
    }

    fn parse_non_block_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let tok = &self.tokens[*tok_index];

        let ret_expr = self.parse_return_expr(tok_index, false);
        if ret_expr.is_some() {
            return ret_expr;
        }

        let assignment_expr = self.parse_assignment_expr(tok_index, false);
        if assignment_expr.is_some() {
            return assignment_expr;
        }

        let asm_expr = self.parse_asm_expr(tok_index, false);
        if asm_expr.is_some() {
            return asm_expr;
        }

        if mandatory {
            self.invalid_token_error(tok);
        }
        None
    }

    fn parse_block(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let mut last_tok = &self.tokens[*tok_index];

        if last_tok.kind != TokenKind::LBrace {
            if mandatory {
                self.invalid_token_error(last_tok);
            } else {
                return None;
            }
        }
        *tok_index += 1;

        let node = self.create_node(NodeKind::Block, last_tok);
        loop {
            let mut stmt = self.parse_var_decl_expr(tok_index, false);
            if stmt.is_none() {
                stmt = self.parse_block_expr(tok_index, false);
                if stmt.is_none() {
                    stmt = self.parse_non_block_expr(tok_index, false);
                    if stmt.is_none() {
                        stmt = Some(self.create_node(NodeKind::Void, last_tok));
                    }
                }
            }
            node.data.borrow().block().borrow().push(stmt.unwrap());
            last_tok = &self.tokens[*tok_index];

            if last_tok.kind == TokenKind::RBrace {
                *tok_index += 1;
                return Some(node);
            } else {
                continue;
            }
        }
    }

    fn parse_fn_proto(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let token = &self.tokens[*tok_index];

        let visib_mod: FnProtoVisibMod;

        if token.kind == TokenKind::KwPub {
            visib_mod = FnProtoVisibMod::Public;
            *tok_index += 1;
            let fn_token = &self.tokens[*tok_index];
            *tok_index += 1;
            self.expect_token(fn_token, TokenKind::KwFn);
        } else if token.kind == TokenKind::KwFn {
            visib_mod = FnProtoVisibMod::Private;
            *tok_index += 1;
        } else if mandatory {
            self.invalid_token_error(token);
        } else {
            return None;
        }

        let node = self.create_node(NodeKind::FnProto, token);
        node.data
            .borrow()
            .fn_proto()
            .borrow()
            .set_visib_mod(visib_mod);

        let name_token = &self.tokens[*tok_index];
        *tok_index += 1;
        self.expect_token(name_token, TokenKind::Ident);
        node.data
            .borrow()
            .fn_proto()
            .borrow()
            .set_name(self.tok_val(name_token).to_string());
        self.parse_param_decl_list(
            *tok_index,
            tok_index,
            &node.data.borrow().fn_proto().borrow(),
        );

        let arrow = &self.tokens[*tok_index];
        if arrow.kind == TokenKind::Arrow {
            *tok_index += 1;
            node.data
                .borrow()
                .fn_proto()
                .borrow()
                .set_ret_type(self.parse_type(*tok_index, tok_index));
        } else {
            node.data
                .borrow()
                .fn_proto()
                .borrow()
                .set_ret_type(self.create_void_type_node(arrow));
        }
        Some(node)
    }

    fn parse_fn_def(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let fn_proto = self.parse_fn_proto(tok_index, mandatory);
        if fn_proto.is_none() {
            return None;
        }

        let node = self.create_node_with_node(NodeKind::FnDef, &fn_proto.as_ref().unwrap());
        node.data
            .borrow()
            .fn_def()
            .borrow()
            .set_proto(fn_proto.unwrap());
        node.data
            .borrow()
            .fn_def()
            .borrow()
            .set_body(self.parse_block(tok_index, true).unwrap());

        Some(node)
    }

    fn parse_import(&self, tok_index: &mut usize) -> Option<Node> {
        let tok = &self.tokens[*tok_index];
        if tok.kind != TokenKind::KwImport {
            return None;
        }

        *tok_index += 1;

        let node = self.create_node(NodeKind::Import, tok);
        let str = &self.tokens[*tok_index];
        self.parse_string_literal(str, &node.data.borrow().import().borrow().path, None);
        *tok_index += 1;

        Some(node)
    }

    fn parse_fn_decl(&self, tok_index: usize, new_tok_index: &mut usize) -> Option<Node> {
        let mut tok_index = tok_index;
        let fn_proto = self.parse_fn_proto(&mut tok_index, false);
        let node = self.create_node_with_node(NodeKind::FnDecl, &fn_proto.as_ref().unwrap());
        *new_tok_index = tok_index;
        node.data
            .borrow()
            .fn_decl()
            .borrow()
            .set_proto(fn_proto.unwrap());
        Some(node)
    }

    fn parse_extern(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let tok = &self.tokens[*tok_index];
        if tok.kind != TokenKind::KwExtern {
            if mandatory {
                self.invalid_token_error(tok);
            }
            return None;
        }
        *tok_index += 1;

        let node = self.create_node(NodeKind::ExternBlock, tok);
        let lbrace = &self.tokens[*tok_index];
        *tok_index += 1;
        self.expect_token(lbrace, TokenKind::LBrace);

        loop {
            let tok = &self.tokens[*tok_index];
            if tok.kind == TokenKind::RBrace {
                *tok_index += 1;
                return Some(node);
            } else {
                let child = self.parse_fn_decl(*tok_index, tok_index);
                node.data
                    .borrow()
                    .extern_block()
                    .borrow()
                    .push(child.unwrap());
            }
        }
    }

    fn parse_top_level_decls(&mut self, tok_index: &mut usize, root: &NodeRoot) {
        loop {
            let fn_def_node = self.parse_fn_def(tok_index, false);
            if fn_def_node.is_some() {
                root.push(fn_def_node.unwrap());
                continue;
            }

            let import_node = self.parse_import(tok_index);
            if import_node.is_some() {
                root.push(import_node.unwrap());
                continue;
            }

            let extern_node = self.parse_extern(tok_index, false);
            if extern_node.is_some() {
                root.push(extern_node.unwrap());
                continue;
            }

            return;
        }
    }

    fn parse_root(&mut self, tok_index: &mut usize) {
        let node = self.create_node(NodeKind::Root, &self.tokens[*tok_index]);
        self.parse_top_level_decls(tok_index, &node.data.borrow().root().borrow());
        if *tok_index != self.tokens.len() - 1 {
            self.invalid_token_error(&self.tokens[*tok_index]);
        }
        self.root = Some(node);
    }
}

pub fn parse(
    src: &str,
    tokens: Vec<RefCell<Token>>,
    owner: &mut ImportTableEntry,
    err_color: ErrColor,
) -> Node {
    let mut ctx = ParseContext::new(owner);
    ctx.err_color = err_color;
    ctx.src = src.to_string();
    for token in tokens {
        ctx.tokens.push(token.borrow().clone());
    }
    let mut tok_index = 0;
    ctx.parse_root(&mut tok_index);
    ctx.root.unwrap().clone()
}

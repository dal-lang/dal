// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use std::cell::RefCell;

use crate::{
    ast::{AsmInput, AsmToken, AsmTokenKind},
    codegen::ImportTableEntry,
    dal_pointer::{DalPtr, DalWeakPtr},
    err_msg::{ErrColor, ErrMsg},
};

use super::{
    ast::{
        AsmOutput, BinOpNodeKind, CallExprNode, FnProtoNode, FnProtoVisibMod, Node, NodeData,
        NodeKind, RootNode, SrcPos, TypeNodeKind, UnOpNodeKind,
    },
    lexer::{Token, TokenKind},
};

struct ParseContext {
    src: String,
    root: Option<Node>,
    tokens: Vec<Token>,
    err_color: ErrColor,
    owner: DalWeakPtr<ImportTableEntry>,
}

impl ParseContext {
    fn new() -> Self {
        Self {
            src: String::new(),
            root: None,
            tokens: Vec::new(),
            err_color: ErrColor::Auto,
            owner: DalWeakPtr::new(),
        }
    }

    fn err(&self, token: &Token, msg: impl Into<String>) -> ! {
        let mut err_msg = ErrMsg::new();
        err_msg.line_start = token.start_line;
        err_msg.col_start = token.start_col;
        err_msg.msg = msg.into();
        let owner = self.owner.upgrade().unwrap();
        err_msg.path = owner.borrow().path.clone();
        err_msg.src = owner.borrow().src.clone();
        err_msg.line_offsets = owner.borrow().line_offsets.clone();
        err_msg.print(self.err_color.clone())
    }

    fn invalid_token(&self, token: &Token) -> ! {
        self.err(token, format!("Invalid token: {}", token.kind))
    }

    fn asm_error(&self, node: &Node, offset: usize, msg: impl Into<String>) -> ! {
        assert_eq!(node.kind, NodeKind::AsmExpr);
        let src_pos = &node.data.asm_expr().offset_map.clone()[offset];
        let mut err_msg = ErrMsg::new();
        err_msg.line_start = src_pos.line;
        err_msg.col_start = src_pos.col;
        err_msg.msg = msg.into();
        let owner = self.owner.upgrade().unwrap();
        err_msg.path = owner.borrow().path.clone();
        err_msg.src = owner.borrow().src.clone();
        err_msg.line_offsets = owner.borrow().line_offsets.clone();
        err_msg.print(self.err_color.clone())
    }

    fn update_node_line_info(&self, node: &mut Node, token: &Token) {
        node.line = token.start_line;
        node.col = token.start_col;
    }

    fn create_node(&self, kind: NodeKind, token: &Token) -> Node {
        let mut node = Node::new(kind);
        self.update_node_line_info(&mut node, token);
        node
    }

    fn create_node_with_node(&self, kind: NodeKind, node: &Node) -> Node {
        let mut new_node = Node::new(kind);
        new_node.line = node.line;
        new_node.col = node.col;
        new_node
    }

    fn expect_token(&self, current: &Token, expected: TokenKind) {
        if current.kind != expected {
            self.invalid_token(current);
        }
    }

    fn token_value(&self, token: &Token) -> String {
        self.src[token.start_pos..token.end_pos].to_string()
    }

    fn parse_fn_def(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let fn_proto = self.parse_fn_proto(tok_index, mandatory);
        if fn_proto.is_none() {
            return None;
        }

        let node = self.create_node_with_node(NodeKind::FnDef, &fn_proto.as_ref().unwrap());
        node.data.fn_def_mut().proto = fn_proto.unwrap();
        node.data.fn_def_mut().body = self.parse_block(tok_index, true).unwrap();

        Some(node)
    }

    fn parse_import(&self, tok_index: &mut usize) -> Option<Node> {
        let token = &self.tokens[*tok_index];
        if token.kind != TokenKind::KwImport {
            return None;
        }

        *tok_index += 1;

        let node = self.create_node(NodeKind::Import, token);
        let path = &self.tokens[*tok_index];
        self.parse_string_literal(path, &mut node.data.import_mut().path, None);
        *tok_index += 1;

        Some(node)
    }

    fn parse_param_decl(&self, tok_index: usize, new_tok_index: &mut usize) -> Option<Node> {
        let mut tok_index = tok_index;
        let name = &self.tokens[tok_index];
        tok_index += 1;

        if name.kind == TokenKind::Ident {
            let node = self.create_node(NodeKind::ParamDecl, name);
            *node.data.param_decl().name_mut() = self.token_value(name);
            let colon = &self.tokens[tok_index];
            tok_index += 1;
            self.expect_token(colon, TokenKind::Colon);
            *node.data.param_decl().ty_mut() = self.parse_type(tok_index, &mut tok_index);
            *new_tok_index = tok_index;
            Some(node)
        } else {
            self.invalid_token(name);
        }
    }

    fn parse_param_decls(
        &self,
        tok_index: usize,
        new_tok_index: &mut usize,
        proto: &mut FnProtoNode,
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
            let param = self.parse_param_decl(tok_index, &mut tok_index);
            if let Some(param) = param {
                proto.params.push(RefCell::new(param));
            }

            let token = &self.tokens[tok_index];
            tok_index += 1;
            if token.kind == TokenKind::RParen {
                *new_tok_index = tok_index;
                return;
            }

            self.expect_token(token, TokenKind::Comma);
        }
    }

    fn parse_else_or_else_if(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let token = &self.tokens[*tok_index];
        if token.kind != TokenKind::KwElse {
            if mandatory {
                self.invalid_token(token);
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
        let token = &self.tokens[*tok_index];
        if token.kind != TokenKind::KwIf {
            if mandatory {
                self.invalid_token(token);
            } else {
                return None;
            }
        }
        *tok_index += 1;

        let node = self.create_node(NodeKind::IfExpr, token);
        node.data.if_expr_mut().cond = self.parse_expr(tok_index, true).unwrap();
        node.data.if_expr_mut().then = self.parse_block(tok_index, true).unwrap();
        if let Some(else_expr) = self.parse_else_or_else_if(tok_index, false) {
            node.data.if_expr_mut().else_ = Some(else_expr);
        }

        Some(node)
    }

    fn parse_var_decl_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let let_token = &self.tokens[*tok_index];
        if let_token.kind == TokenKind::KwLet {
            *tok_index += 1;
            let node = self.create_node(NodeKind::VarDecl, let_token);
            let name: &Token;
            let token = &self.tokens[*tok_index];
            if token.kind == TokenKind::KwMut {
                *tok_index += 1;
                node.data.var_decl_mut().is_const = false;
                name = &self.tokens[*tok_index];
                self.expect_token(name, TokenKind::Ident);
            } else if token.kind == TokenKind::Ident {
                node.data.var_decl_mut().is_const = true;
                name = token;
            } else {
                self.invalid_token(token);
            }
            *tok_index += 1;
            node.data.var_decl_mut().name = self.token_value(name);

            let assign_or_colon = &self.tokens[*tok_index];
            *tok_index += 1;
            if assign_or_colon.kind == TokenKind::Assign {
                node.data.var_decl_mut().expr = self.parse_expr(tok_index, true);
                return Some(node);
            } else if assign_or_colon.kind == TokenKind::Colon {
                node.data.var_decl_mut().ty = Some(self.parse_type(*tok_index, tok_index));

                let assign = &self.tokens[*tok_index];
                if assign.kind == TokenKind::Assign {
                    *tok_index += 1;
                    node.data.var_decl_mut().expr = self.parse_expr(tok_index, true);
                }
                return Some(node);
            } else {
                self.invalid_token(assign_or_colon);
            }
        } else if mandatory {
            self.invalid_token(let_token);
        } else {
            return None;
        }
    }

    fn parse_block(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let mut token = &self.tokens[*tok_index];

        if token.kind != TokenKind::LBrace {
            if mandatory {
                self.invalid_token(token);
            } else {
                return None;
            }
        }
        *tok_index += 1;

        let node = self.create_node(NodeKind::Block, token);
        loop {
            let mut stmt = self.parse_var_decl_expr(tok_index, false);
            if stmt.is_none() {
                stmt = self.parse_block_expr(tok_index, false);
                if stmt.is_none() {
                    stmt = self.parse_non_block_expr(tok_index, false);
                    if stmt.is_none() {
                        stmt = Some(self.create_node(NodeKind::Void, token));
                    }
                }
            }
            node.data.block_mut().children.push(stmt.unwrap());
            token = &self.tokens[*tok_index];

            if token.kind == TokenKind::RBrace {
                *tok_index += 1;
                return Some(node);
            }
        }
    }

    fn parse_block_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let token = &self.tokens[*tok_index];

        let if_expr = self.parse_if_expr(tok_index, false);
        if if_expr.is_some() {
            return if_expr;
        }

        let block = self.parse_block(tok_index, false);
        if block.is_some() {
            return block;
        }

        if mandatory {
            self.invalid_token(token);
        }

        None
    }

    fn parse_return_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let token = &self.tokens[*tok_index];
        if token.kind == TokenKind::KwReturn {
            *tok_index += 1;
            let node = self.create_node(NodeKind::Return, token);
            if let Some(expr) = self.parse_expr(tok_index, false) {
                node.data.return_mut().expr = Some(expr);
            }
            Some(node)
        } else if mandatory {
            self.invalid_token(token);
        } else {
            None
        }
    }

    fn parse_group_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let lparen = &self.tokens[*tok_index];
        if lparen.kind != TokenKind::LParen {
            if mandatory {
                self.invalid_token(lparen);
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
        let token = &self.tokens[*tok_index];

        if token.kind == TokenKind::NumLit {
            let mut node = self.create_node(NodeKind::NumLit, token);
            node.data = Box::new(NodeData::new_num_lit(self.token_value(token)));
            *tok_index += 1;
            return Some(node);
        } else if token.kind == TokenKind::StrLit {
            let mut node = self.create_node(NodeKind::StrLit, token);
            let mut s = String::new();
            self.parse_string_literal(token, &mut s, None);
            node.data = Box::new(NodeData::new_str_lit(s));
            *tok_index += 1;
            return Some(node);
        } else if token.kind == TokenKind::KwVoid {
            let node = self.create_node(NodeKind::Void, token);
            *tok_index += 1;
            return Some(node);
        } else if token.kind == TokenKind::KwTrue {
            let mut node = self.create_node(NodeKind::BoolLit, token);
            node.data = Box::new(NodeData::new_bool_lit(true));
            *tok_index += 1;
            return Some(node);
        } else if token.kind == TokenKind::KwFalse {
            let mut node = self.create_node(NodeKind::BoolLit, token);
            node.data = Box::new(NodeData::new_bool_lit(false));
            *tok_index += 1;
            return Some(node);
        } else if token.kind == TokenKind::Ident {
            let mut node = self.create_node(NodeKind::Ident, token);
            node.data = Box::new(NodeData::new_ident(self.token_value(token)));
            *tok_index += 1;
            return Some(node);
        }

        let group_expr = self.parse_group_expr(tok_index, false);
        if group_expr.is_some() {
            return group_expr;
        }

        if !mandatory {
            return None;
        }

        self.invalid_token(token);
    }

    fn parse_call_params(
        &self,
        tok_index: usize,
        new_tok_index: &mut usize,
        call_expr: &mut CallExprNode,
    ) {
        let mut tok_index = tok_index;
        let token = &self.tokens[tok_index];
        if token.kind == TokenKind::RParen {
            tok_index += 1;
            *new_tok_index = tok_index;
            return;
        }

        loop {
            let expr = self.parse_expr(&mut tok_index, true);
            call_expr.args.push(expr.unwrap());
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

    fn parse_post_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let primary = self.parse_primary_expr(tok_index, mandatory);
        if primary.is_none() {
            return None;
        }

        let token = &self.tokens[*tok_index];
        if token.kind == TokenKind::LParen {
            *tok_index += 1;
            let node = self.create_node(NodeKind::CallExpr, token);
            node.data.call_expr_mut().callee = primary.unwrap();
            self.parse_call_params(*tok_index, tok_index, &mut node.data.call_expr_mut());
            Some(node)
        } else if token.kind == TokenKind::LBracket {
            *tok_index += 1;
            let node = self.create_node(NodeKind::ArrayAccessExpr, token);
            node.data.array_access_expr_mut().array = primary.unwrap();
            node.data.array_access_expr_mut().index = self.parse_expr(tok_index, true).unwrap();
            let rbracket = &self.tokens[*tok_index];
            *tok_index += 1;
            self.expect_token(rbracket, TokenKind::RBracket);
            Some(node)
        } else {
            primary
        }
    }

    fn token_to_unary_op(&self, token: &Token) -> UnOpNodeKind {
        match token.kind {
            TokenKind::Bang => UnOpNodeKind::BoolNot,
            TokenKind::Dash => UnOpNodeKind::Neg,
            TokenKind::BitNot => UnOpNodeKind::Not,
            _ => UnOpNodeKind::Invalid,
        }
    }

    fn parse_unary_op(&self, tok_index: &mut usize, mandatory: bool) -> UnOpNodeKind {
        let token = &self.tokens[*tok_index];
        let res = self.token_to_unary_op(token);
        if res == UnOpNodeKind::Invalid {
            if mandatory {
                self.invalid_token(token);
            } else {
                return UnOpNodeKind::Invalid;
            }
        }
        *tok_index += 1;
        res
    }

    fn parse_unary_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let token = &self.tokens[*tok_index];
        let op = self.parse_unary_op(tok_index, false);
        if op == UnOpNodeKind::Invalid {
            return self.parse_post_expr(tok_index, mandatory);
        }

        let expr = self.parse_post_expr(tok_index, true);

        let node = self.create_node(NodeKind::UnaryOpExpr, token);
        node.data.un_op_expr_mut().expr = expr.unwrap();
        node.data.un_op_expr_mut().op = op;
        Some(node)
    }

    fn parse_cast_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let unary = self.parse_unary_expr(tok_index, mandatory);
        if unary.is_none() {
            return None;
        }

        let token = &self.tokens[*tok_index];
        if token.kind != TokenKind::KwAs {
            return unary;
        }
        *tok_index += 1;

        let node = self.create_node(NodeKind::CastExpr, token);
        node.data.cast_expr_mut().expr = unary.unwrap();
        node.data.cast_expr_mut().ty = self.parse_type(*tok_index, tok_index);
        Some(node)
    }

    fn token_to_mul_op(&self, token: &Token) -> BinOpNodeKind {
        match token.kind {
            TokenKind::Star => BinOpNodeKind::Mul,
            TokenKind::Slash => BinOpNodeKind::Div,
            TokenKind::Percent => BinOpNodeKind::Mod,
            _ => BinOpNodeKind::Invalid,
        }
    }

    fn parse_mul_op(&self, tok_index: &mut usize, mandatory: bool) -> BinOpNodeKind {
        let token = &self.tokens[*tok_index];
        let res = self.token_to_mul_op(token);
        if res == BinOpNodeKind::Invalid {
            if mandatory {
                self.invalid_token(token);
            } else {
                return BinOpNodeKind::Invalid;
            }
        }
        *tok_index += 1;
        res
    }

    fn parse_mul_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let lhs = self.parse_cast_expr(tok_index, mandatory);
        if lhs.is_none() {
            return None;
        }

        let token = &self.tokens[*tok_index];
        let op = self.parse_mul_op(tok_index, false);
        if op == BinOpNodeKind::Invalid {
            return lhs;
        }

        let rhs = self.parse_cast_expr(tok_index, true);
        let node = self.create_node(NodeKind::BinOpExpr, token);
        node.data.bin_op_expr_mut().lhs = lhs.unwrap();
        node.data.bin_op_expr_mut().rhs = rhs.unwrap();
        node.data.bin_op_expr_mut().op = op;
        Some(node)
    }

    fn token_to_add_op(&self, token: &Token) -> BinOpNodeKind {
        match token.kind {
            TokenKind::Plus => BinOpNodeKind::Add,
            TokenKind::Dash => BinOpNodeKind::Sub,
            _ => BinOpNodeKind::Invalid,
        }
    }

    fn parse_add_op(&self, tok_index: &mut usize, mandatory: bool) -> BinOpNodeKind {
        let token = &self.tokens[*tok_index];
        let res = self.token_to_add_op(token);
        if res == BinOpNodeKind::Invalid {
            if mandatory {
                self.invalid_token(token);
            } else {
                return BinOpNodeKind::Invalid;
            }
        }
        *tok_index += 1;
        res
    }

    fn parse_add_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let lhs = self.parse_mul_expr(tok_index, mandatory);
        if lhs.is_none() {
            return None;
        }

        let token = &self.tokens[*tok_index];
        let op = self.parse_add_op(tok_index, false);
        if op == BinOpNodeKind::Invalid {
            return lhs;
        }

        let rhs = self.parse_mul_expr(tok_index, true);
        let node = self.create_node(NodeKind::BinOpExpr, token);
        node.data.bin_op_expr_mut().lhs = lhs.unwrap();
        node.data.bin_op_expr_mut().rhs = rhs.unwrap();
        node.data.bin_op_expr_mut().op = op;
        Some(node)
    }

    fn token_to_bit_shift_op(&self, token: &Token) -> BinOpNodeKind {
        match token.kind {
            TokenKind::BitShl => BinOpNodeKind::Shl,
            TokenKind::BitShr => BinOpNodeKind::Shr,
            _ => BinOpNodeKind::Invalid,
        }
    }

    fn parse_bit_shift_op(&self, tok_index: &mut usize, mandatory: bool) -> BinOpNodeKind {
        let token = &self.tokens[*tok_index];
        let res = self.token_to_bit_shift_op(token);
        if res == BinOpNodeKind::Invalid {
            if mandatory {
                self.invalid_token(token);
            } else {
                return BinOpNodeKind::Invalid;
            }
        }
        *tok_index += 1;
        res
    }

    fn parse_bit_shift_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let lhs = self.parse_add_expr(tok_index, mandatory);
        if lhs.is_none() {
            return None;
        }

        let token = &self.tokens[*tok_index];
        let op = self.parse_bit_shift_op(tok_index, false);
        if op == BinOpNodeKind::Invalid {
            return lhs;
        }

        let rhs = self.parse_add_expr(tok_index, true);
        let node = self.create_node(NodeKind::BinOpExpr, token);
        node.data.bin_op_expr_mut().lhs = lhs.unwrap();
        node.data.bin_op_expr_mut().rhs = rhs.unwrap();
        node.data.bin_op_expr_mut().op = op;
        Some(node)
    }

    fn parse_bin_and_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let lhs = self.parse_bit_shift_expr(tok_index, mandatory);
        if lhs.is_none() {
            return None;
        }

        let token = &self.tokens[*tok_index];
        if token.kind != TokenKind::BitAnd {
            return lhs;
        }
        *tok_index += 1;

        let rhs = self.parse_bit_shift_expr(tok_index, true);
        let node = self.create_node(NodeKind::BinOpExpr, token);
        node.data.bin_op_expr_mut().lhs = lhs.unwrap();
        node.data.bin_op_expr_mut().rhs = rhs.unwrap();
        node.data.bin_op_expr_mut().op = BinOpNodeKind::And;
        Some(node)
    }

    fn parse_bin_xor_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let lhs = self.parse_bin_and_expr(tok_index, mandatory);
        if lhs.is_none() {
            return None;
        }

        let token = &self.tokens[*tok_index];
        if token.kind != TokenKind::BitXor {
            return lhs;
        }
        *tok_index += 1;

        let rhs = self.parse_bin_and_expr(tok_index, true);
        let node = self.create_node(NodeKind::BinOpExpr, token);
        node.data.bin_op_expr_mut().lhs = lhs.unwrap();
        node.data.bin_op_expr_mut().rhs = rhs.unwrap();
        node.data.bin_op_expr_mut().op = BinOpNodeKind::Xor;
        Some(node)
    }

    fn parse_bin_or_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let lhs = self.parse_bin_xor_expr(tok_index, mandatory);
        if lhs.is_none() {
            return None;
        }

        let token = &self.tokens[*tok_index];
        if token.kind != TokenKind::BitOr {
            return lhs;
        }
        *tok_index += 1;

        let rhs = self.parse_bin_xor_expr(tok_index, true);
        let node = self.create_node(NodeKind::BinOpExpr, token);
        node.data.bin_op_expr_mut().lhs = lhs.unwrap();
        node.data.bin_op_expr_mut().rhs = rhs.unwrap();
        node.data.bin_op_expr_mut().op = BinOpNodeKind::Or;
        Some(node)
    }

    fn token_to_cmp_op(&self, token: &Token) -> BinOpNodeKind {
        match token.kind {
            TokenKind::Assign => BinOpNodeKind::Assign,
            TokenKind::CmpEq => BinOpNodeKind::Eq,
            TokenKind::CmpNeq => BinOpNodeKind::Neq,
            TokenKind::CmpLt => BinOpNodeKind::Lt,
            TokenKind::CmpGt => BinOpNodeKind::Gt,
            TokenKind::CmpLte => BinOpNodeKind::Lte,
            TokenKind::CmpGte => BinOpNodeKind::Gte,
            _ => BinOpNodeKind::Invalid,
        }
    }

    fn parse_cmp_op(&self, tok_index: &mut usize, mandatory: bool) -> BinOpNodeKind {
        let token = &self.tokens[*tok_index];
        let res = self.token_to_cmp_op(token);
        if res == BinOpNodeKind::Invalid {
            if mandatory {
                self.invalid_token(token);
            } else {
                return BinOpNodeKind::Invalid;
            }
        }
        *tok_index += 1;
        res
    }

    fn parse_cmp_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let lhs = self.parse_bin_or_expr(tok_index, mandatory);
        if lhs.is_none() {
            return None;
        }

        let token = &self.tokens[*tok_index];
        let op = self.parse_cmp_op(tok_index, false);
        if op == BinOpNodeKind::Invalid {
            return lhs;
        }

        let rhs = self.parse_bin_or_expr(tok_index, mandatory);
        let node = self.create_node(NodeKind::BinOpExpr, token);
        node.data.bin_op_expr_mut().lhs = lhs.unwrap();
        node.data.bin_op_expr_mut().rhs = rhs.unwrap();
        node.data.bin_op_expr_mut().op = op;
        Some(node)
    }

    fn parse_bool_and_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let lhs = self.parse_cmp_expr(tok_index, mandatory);
        if lhs.is_none() {
            return None;
        }

        let token = &self.tokens[*tok_index];
        if token.kind != TokenKind::BoolAnd {
            return lhs;
        }
        *tok_index += 1;

        let rhs = self.parse_cmp_expr(tok_index, true);
        let node = self.create_node(NodeKind::BinOpExpr, token);
        node.data.bin_op_expr_mut().lhs = lhs.unwrap();
        node.data.bin_op_expr_mut().rhs = rhs.unwrap();
        node.data.bin_op_expr_mut().op = BinOpNodeKind::BoolAnd;
        Some(node)
    }

    fn parse_bool_or_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let lhs = self.parse_bool_and_expr(tok_index, mandatory);
        if lhs.is_none() {
            return None;
        }

        let token = &self.tokens[*tok_index];
        if token.kind != TokenKind::BoolOr {
            return lhs;
        }
        *tok_index += 1;

        let rhs = self.parse_bool_and_expr(tok_index, true);
        let node = self.create_node(NodeKind::BinOpExpr, token);
        node.data.bin_op_expr_mut().lhs = lhs.unwrap();
        node.data.bin_op_expr_mut().rhs = rhs.unwrap();
        node.data.bin_op_expr_mut().op = BinOpNodeKind::BoolOr;
        Some(node)
    }

    fn parse_assign_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let lhs = self.parse_bool_or_expr(tok_index, mandatory);
        if lhs.is_none() {
            return None;
        }

        let token = &self.tokens[*tok_index];
        if token.kind != TokenKind::Assign {
            return lhs;
        }
        *tok_index += 1;

        let rhs = self.parse_bool_or_expr(tok_index, true);
        let node = self.create_node(NodeKind::BinOpExpr, token);
        node.data.bin_op_expr_mut().lhs = lhs.unwrap();
        node.data.bin_op_expr_mut().rhs = rhs.unwrap();
        node.data.bin_op_expr_mut().op = BinOpNodeKind::Assign;
        Some(node)
    }

    fn parse_string_literal(
        &self,
        token: &Token,
        str: &mut String,
        mut offset_map: Option<&mut Vec<SrcPos>>,
    ) {
        let mut escaped = false;
        let mut first = true;
        let mut pos = SrcPos::new();
        pos.line = token.start_line;
        pos.col = token.start_col;
        let mut i = token.start_pos;
        // let src: Vec<char> = self.src.chars().collect();
        while i < token.end_pos - 1 {
            let c = self.src.chars().nth(i).unwrap();
            if first {
                first = false;
            } else {
                if escaped {
                    match c {
                        '\\' => {
                            str.push('\\');
                            if let Some(ref mut offset_map) = offset_map {
                                offset_map.push(pos.clone());
                            }
                        }
                        'r' => {
                            str.push('\r');
                            if let Some(ref mut offset_map) = offset_map {
                                offset_map.push(pos.clone());
                            }
                        }
                        'n' => {
                            str.push('\n');
                            if let Some(ref mut offset_map) = offset_map {
                                offset_map.push(pos.clone());
                            }
                        }
                        't' => {
                            str.push('\t');
                            if let Some(ref mut offset_map) = offset_map {
                                offset_map.push(pos.clone());
                            }
                        }
                        '"' => {
                            str.push('"');
                            if let Some(ref mut offset_map) = offset_map {
                                offset_map.push(pos.clone());
                            }
                        }
                        _ => {}
                    }
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else {
                    str.push(c);
                    if let Some(ref mut offset_map) = offset_map {
                        offset_map.push(pos.clone());
                    }
                }
            }
            if c == '\n' {
                pos.line += 1;
                pos.col = 0;
            } else {
                pos.col += 1;
            }
            i += 1;
        }
        if let Some(ref mut offset_map) = offset_map {
            offset_map.push(pos.clone());
        }
    }

    fn parse_asm_template(&self, node: &Node) {
        enum State {
            Start,
            Percent,
            Template,
            Var,
        }

        assert_eq!(node.data.asm_expr().tokens.len(), 0);

        let mut state = State::Start;
        let mut i = 0;
        let mut asm_expr = node.data.asm_expr_mut();
        let src: Vec<char> = asm_expr.template.chars().collect();
        while i < src.len() {
            let c = src[i];
            match state {
                State::Start => {
                    let mut new_token = AsmToken::new();
                    new_token.start = i;
                    new_token.end = 0;
                    if c == '%' {
                        new_token.kind = AsmTokenKind::Percent;
                        state = State::Percent;
                    } else {
                        new_token.kind = AsmTokenKind::Template;
                        state = State::Template;
                    }
                    asm_expr.tokens.push(new_token);
                }
                State::Percent => {
                    if c == '%' {
                        asm_expr.tokens.last_mut().unwrap().end = i;
                        state = State::Start;
                    } else if c == '[' {
                        asm_expr.tokens.last_mut().unwrap().kind = AsmTokenKind::Var;
                        state = State::Var;
                    } else {
                        self.asm_error(node, i, "Invalid asm template");
                    }
                }
                State::Template => {
                    if c == '%' {
                        asm_expr.tokens.last_mut().unwrap().end = i;
                        i -= 1;
                        state = State::Start;
                    }
                }
                State::Var => {
                    if c == ']' {
                        asm_expr.tokens.last_mut().unwrap().end = i;
                        state = State::Start;
                    } else if (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') || c == '_' {
                        // do nothing
                    } else {
                        self.asm_error(node, i, "Invalid asm template");
                    }
                }
            }
            i += 1;
        }

        match state {
            State::Start => {}
            State::Percent | State::Var => {
                self.asm_error(node, asm_expr.template.len(), "Invalid asm template");
            }
            State::Template => {
                asm_expr.tokens.last_mut().unwrap().end = asm_expr.template.len();
            }
        }
    }

    fn eat_token(&self, tok_index: &mut usize, kind: TokenKind) -> Token {
        let token = &self.tokens[*tok_index];
        self.expect_token(token, kind);
        *tok_index += 1;
        token.clone()
    }

    fn parse_asm_output_item(&self, tok_index: &mut usize, node: &Node) {
        self.eat_token(tok_index, TokenKind::LBracket);
        let alias = self.eat_token(tok_index, TokenKind::Ident);
        self.eat_token(tok_index, TokenKind::RBracket);

        let constraint = self.eat_token(tok_index, TokenKind::StrLit);

        self.eat_token(tok_index, TokenKind::LParen);
        let out_ident = self.eat_token(tok_index, TokenKind::Ident);
        self.eat_token(tok_index, TokenKind::RParen);

        let mut output = AsmOutput::new();
        output.symbolic_name = self.token_value(&alias);
        output.var_name = self.token_value(&out_ident);

        self.parse_string_literal(&constraint, &mut output.constraint, None);
        node.data.asm_expr_mut().outputs.push(output);
    }

    fn parse_asm_input_item(&self, tok_index: &mut usize, node: &Node) {
        self.eat_token(tok_index, TokenKind::LBracket);
        let alias = self.eat_token(tok_index, TokenKind::Ident);
        self.eat_token(tok_index, TokenKind::RBracket);

        let constraint = self.eat_token(tok_index, TokenKind::StrLit);

        self.eat_token(tok_index, TokenKind::LParen);
        let expr = self.parse_expr(tok_index, true);
        self.eat_token(tok_index, TokenKind::RParen);

        let mut input = AsmInput::new();
        input.symbolic_name = self.token_value(&alias);
        input.expr = expr.unwrap();

        self.parse_string_literal(&constraint, &mut input.constraint, None);
        node.data.asm_expr_mut().inputs.push(input);
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

            let mut clobber = String::new();
            self.parse_string_literal(str, &mut clobber, None);
            node.data.asm_expr_mut().clobbers.push(clobber);

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

        self.parse_asm_input(tok_index, node)
    }

    fn parse_asm_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let asm_token = &self.tokens[*tok_index];
        if asm_token.kind != TokenKind::KwAsm {
            if mandatory {
                self.invalid_token(asm_token);
            } else {
                return None;
            }
        }

        let node = self.create_node(NodeKind::AsmExpr, asm_token);
        {
            let mut asm_expr = node.data.asm_expr_mut();
            asm_expr.template = String::new();
            *tok_index += 1;

            let lparen = &self.tokens[*tok_index];
            self.expect_token(lparen, TokenKind::LParen);
            *tok_index += 1;

            let asm_str = &self.tokens[*tok_index];
            self.expect_token(asm_str, TokenKind::StrLit);
            *tok_index += 1;

            let mut offset_map = vec![];
            self.parse_string_literal(asm_str, &mut asm_expr.template, Some(&mut offset_map));
            asm_expr.offset_map = offset_map;
        }
        self.parse_asm_template(&node);
        self.parse_asm_output(tok_index, &node);

        let rparen = &self.tokens[*tok_index];
        self.expect_token(rparen, TokenKind::RParen);
        *tok_index += 1;

        Some(node)
    }

    fn parse_non_block_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let token = &self.tokens[*tok_index];

        let ret_expr = self.parse_return_expr(tok_index, false);
        if ret_expr.is_some() {
            return ret_expr;
        }

        let assign_expr = self.parse_assign_expr(tok_index, false);
        if assign_expr.is_some() {
            return assign_expr;
        }

        let asm_expr = self.parse_asm_expr(tok_index, false);
        if asm_expr.is_some() {
            return asm_expr;
        }

        if mandatory {
            self.invalid_token(token);
        }
        None
    }

    fn parse_expr(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let token = &self.tokens[*tok_index];

        let block = self.parse_block_expr(tok_index, false);
        if block.is_some() {
            return block;
        }

        let non_block = self.parse_non_block_expr(tok_index, false);
        if non_block.is_some() {
            return non_block;
        }

        if mandatory {
            self.invalid_token(token);
        }

        None
    }

    fn parse_type(&self, tok_index: usize, new_tok_index: &mut usize) -> Node {
        let mut tok_index = tok_index;
        let token = &self.tokens[tok_index];
        tok_index += 1;

        let node = self.create_node(NodeKind::Type, token);
        {
            let mut ty = node.data.type_mut();
            if token.kind == TokenKind::KwVoid {
                ty.kind = TypeNodeKind::Primitive;
                ty.name = "void".to_string();
            } else if token.kind == TokenKind::Ident {
                ty.kind = TypeNodeKind::Primitive;
                ty.name = self.token_value(token);
            } else if token.kind == TokenKind::Star {
                ty.kind = TypeNodeKind::Pointer;
                let const_or_mut = &self.tokens[tok_index];
                tok_index += 1;
                if const_or_mut.kind == TokenKind::KwMut {
                    ty.is_const = false;
                } else if const_or_mut.kind == TokenKind::KwConst {
                    ty.is_const = true;
                } else {
                    self.invalid_token(const_or_mut);
                }
                ty.ty = DalPtr::new(self.parse_type(tok_index, &mut tok_index));
            } else if token.kind == TokenKind::LBracket {
                ty.kind = TypeNodeKind::Array;
                ty.ty = DalPtr::new(self.parse_type(tok_index, &mut tok_index));
                let semi = &self.tokens[tok_index];
                tok_index += 1;
                self.expect_token(semi, TokenKind::Semicolon);
                ty.size = DalPtr::new(self.parse_expr(&mut tok_index, true).unwrap());
                let rbracket = &self.tokens[tok_index];
                tok_index += 1;
                self.expect_token(rbracket, TokenKind::RBracket);
            } else {
                self.invalid_token(token);
            }
        }

        *new_tok_index = tok_index;
        node
    }

    fn create_void_type_node(&self, token: &Token) -> Node {
        let node = self.create_node(NodeKind::Type, token);
        {
            let mut ty = node.data.type_mut();
            ty.kind = TypeNodeKind::Primitive;
            ty.name = "void".to_string();
        }
        node
    }

    fn parse_fn_proto(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let token = &self.tokens[*tok_index];
        let visibmod: FnProtoVisibMod;
        if token.kind == TokenKind::KwPub {
            visibmod = FnProtoVisibMod::Public;
            *tok_index += 1;
            let fn_kw = &self.tokens[*tok_index];
            *tok_index += 1;
            self.expect_token(fn_kw, TokenKind::KwFn);
        } else if token.kind == TokenKind::KwFn {
            visibmod = FnProtoVisibMod::Private;
            *tok_index += 1;
        } else if mandatory {
            self.invalid_token(token);
        } else {
            return None;
        }

        let node = self.create_node(NodeKind::FnProto, token);
        node.data.fn_proto_mut().visib_mod = visibmod;

        let name = &self.tokens[*tok_index];
        *tok_index += 1;
        self.expect_token(name, TokenKind::Ident);
        node.data.fn_proto_mut().name = self.token_value(name);
        self.parse_param_decls(*tok_index, tok_index, &mut node.data.fn_proto_mut());

        let arrow = &self.tokens[*tok_index];
        if arrow.kind == TokenKind::Arrow {
            *tok_index += 1;
            node.data.fn_proto_mut().ret_type = self.parse_type(*tok_index, tok_index);
        } else {
            node.data.fn_proto_mut().ret_type = self.create_void_type_node(arrow);
        }
        Some(node)
    }

    fn parse_fn_decl(&self, tok_index: usize, new_tok_index: &mut usize) -> Option<Node> {
        let mut tok_index = tok_index;
        let fn_proto = self.parse_fn_proto(&mut tok_index, true).unwrap();
        let node = self.create_node_with_node(NodeKind::FnDecl, &fn_proto);
        *new_tok_index = tok_index;
        node.data.fn_decl_mut().proto = DalPtr::new(fn_proto);
        Some(node)
    }

    fn parse_extern_node(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let token = &self.tokens[*tok_index];
        if token.kind != TokenKind::KwExtern {
            if mandatory {
                self.invalid_token(token);
            }
            return None;
        }
        *tok_index += 1;

        let node = self.create_node(NodeKind::ExternBlock, token);
        let lbrace = &self.tokens[*tok_index];
        *tok_index += 1;
        self.expect_token(lbrace, TokenKind::LBrace);

        loop {
            let token = &self.tokens[*tok_index];
            if token.kind == TokenKind::RBrace {
                *tok_index += 1;
                return Some(node);
            } else {
                let child = self.parse_fn_decl(*tok_index, tok_index);
                node.data
                    .extern_mut()
                    .fn_decls
                    .push(DalPtr::new(child.unwrap()));
            }
        }
    }

    fn parse_top_level_decls(&mut self, tok_index: &mut usize, root: &mut RootNode) {
        loop {
            let fn_def_node = self.parse_fn_def(tok_index, false);
            if let Some(node) = fn_def_node {
                root.children.push(DalPtr::new(node));
                continue;
            }

            let import_node = self.parse_import(tok_index);
            if let Some(node) = import_node {
                root.children.push(DalPtr::new(node));
                continue;
            }

            let extern_node = self.parse_extern_node(tok_index, false);
            if let Some(node) = extern_node {
                root.children.push(DalPtr::new(node));
                continue;
            }

            return;
        }
    }

    fn parse_root(&mut self, tok_index: &mut usize) {
        let node = self.create_node(NodeKind::Root, &self.tokens[*tok_index]);
        self.parse_top_level_decls(tok_index, &mut node.data.root_mut());

        if *tok_index != self.tokens.len() - 1 {
            self.invalid_token(&self.tokens[*tok_index]);
        }
        self.root = Some(node);
    }
}

pub fn parse(src: &str, tokens: Vec<Token>, owner: DalWeakPtr<ImportTableEntry>) -> Node {
    let mut ctx = ParseContext::new();
    ctx.src = src.to_string();
    ctx.tokens = tokens;
    ctx.owner = owner;
    let mut tok_index = 0;
    ctx.parse_root(&mut tok_index);
    ctx.root.unwrap()
}

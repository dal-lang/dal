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
    ast::{FnProtoVisibMod, Node, NodeData, NodeKind, TypeKind},
    tokenizer::{Token, TokenKind},
};
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
struct ParseContext<'a> {
    src: String,
    root: Option<RefCell<Node>>,
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

    fn invalid_token_error(&self, token: &Token) {
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
        assert_eq!(node.kind, NodeKind::AsmExpr);
        let src_pos = &node.data.asm_expr().borrow().offset_map.clone()[offset];
        let err = ErrMsg {
            line_start: src_pos.borrow().line,
            col_start: src_pos.borrow().col,
            msg,
            path: self.owner.borrow().path.clone(),
            src: self.owner.borrow().src_code.clone(),
            line_offsets: self.owner.borrow().line_offsets.clone(),
        };
        err.print(&self.err_color);
    }

    fn create_node_no_line_info(&self, kind: NodeKind) -> Node {
        Node {
            kind,
            line: 0,
            col: 0,
            owner: std::ptr::null_mut(),
            data: Box::new(NodeData::None),
        }
    }

    fn update_node_line_info(&self, node: &mut Node, token: &Token) {
        node.line = token.start_line;
        node.col = token.start_col;
    }

    fn create_node(&self, kind: NodeKind, token: &Token) -> Node {
        let mut node = self.create_node_no_line_info(kind);
        self.update_node_line_info(&mut node, token);
        node
    }

    fn create_node_with_node(&self, kind: NodeKind, node: &Node) -> Node {
        let mut new_node = self.create_node_no_line_info(kind);
        new_node.line = node.line;
        new_node.col = node.col;
        new_node
    }

    fn create_void_type_node(&self, token: &Token) -> Node {
        let node = self.create_node(NodeKind::Type, token);
        node.data.type_().borrow_mut().kind = TypeKind::Primitive;
        node.data.type_().borrow_mut().name = "void".to_string();
        node
    }

    fn parse_asm_template(&self, node: &Node) {
        let asm_template = &node.data.asm_expr().borrow_mut().asm_template;

        enum State {
            Start,
            Percent,
            Template,
            Var,
        }

        let tok_list = &mut node.data.asm_expr().borrow_mut().token_list;
        assert_eq!(tok_list.len(), 0);

        let mut cur_tok: Option<&RefCell<AsmToken>> = None;

        let mut state = State::Start;

        let mut i = 0;
        while i < asm_template.len() {
            let c = asm_template.chars().nth(i).unwrap();
            match state {
                State::Start => {
                    if c == '%' {
                        tok_list.push(RefCell::new(AsmToken {
                            kind: AsmTokenKind::Percent,
                            start: i,
                            end: 0,
                        }));
                        state = State::Percent;
                        cur_tok = tok_list.last();
                    } else {
                        tok_list.push(RefCell::new(AsmToken {
                            kind: AsmTokenKind::Template,
                            start: i,
                            end: 0,
                        }));
                        state = State::Template;
                        cur_tok = tok_list.last();
                    }
                }
                State::Percent => {
                    if c == '%' {
                        cur_tok.unwrap().borrow_mut().end = i;
                        state = State::Start;
                    } else if c == '[' {
                        cur_tok.unwrap().borrow_mut().kind = AsmTokenKind::Var;
                        state = State::Var;
                    } else {
                        self.ast_asm_error(node, i, "invalid asm template".to_string());
                    }
                }
                State::Template => {
                    if c == '%' {
                        cur_tok.unwrap().borrow_mut().end = i;
                        i -= 1;
                        cur_tok = None;
                        state = State::Start;
                    }
                }
                State::Var => {
                    if c == ']' {
                        cur_tok.unwrap().borrow_mut().end = i;
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
                self.ast_asm_error(node, asm_template.len(), "invalid asm template".to_string());
            }
            State::Template => {
                cur_tok.unwrap().borrow_mut().end = asm_template.len();
            }
        }
    }

    fn parse_expr(&self, tok_index: &mut usize, mandatory: bool) -> Node {
        unimplemented!()
    }

    fn parse_type(&self, tok_index: usize, new_tok_index: &mut usize) -> Node {
        let mut tok_index = tok_index;
        let token = &self.tokens[tok_index];
        tok_index += 1;
        let mut node = self.create_node(NodeKind::Type, token);
        if token.kind == TokenKind::KwVoid {
            node.data.type_().borrow_mut().kind = TypeKind::Primitive;
            node.data.type_().borrow_mut().name = "void".to_string();
        } else if token.kind == TokenKind::Ident {
            node.data.type_().borrow_mut().kind = TypeKind::Primitive;
            node.data.type_().borrow_mut().name = self.tok_val(token).to_string();
        } else if token.kind == TokenKind::Star {
            node.data.type_().borrow_mut().kind = TypeKind::Pointer;
            let const_or_mut = &self.tokens[tok_index];
            tok_index += 1;
            if const_or_mut.kind == TokenKind::KwMut {
                node.data.type_().borrow_mut().is_const = false;
            } else if const_or_mut.kind == TokenKind::KwConst {
                node.data.type_().borrow_mut().is_const = true;
            } else {
                self.invalid_token_error(const_or_mut);
            }
            node.data.type_().borrow_mut().child_type =
                Some(RefCell::new(self.parse_type(tok_index, &mut tok_index)));
        } else if token.kind == TokenKind::LBracket {
            node.data.type_().borrow_mut().kind = TypeKind::Array;
            node.data.type_().borrow_mut().child_type =
                Some(RefCell::new(self.parse_type(tok_index, &mut tok_index)));
            let semi = &self.tokens[tok_index];
            tok_index += 1;
            self.expect_token(semi, TokenKind::Semicolon);
            node.data.type_().borrow_mut().array_size =
                Some(RefCell::new(self.parse_expr(&mut tok_index, true)));
            let rbracket = &self.tokens[tok_index];
            tok_index += 1;
            self.expect_token(rbracket, TokenKind::RBracket);
        } else {
            self.invalid_token_error(token);
        }

        node
    }

    fn parse_param_decl(&self, tok_index: usize, new_tok_index: &mut usize) -> Option<Node> {
        let mut tok_index = tok_index;
        let param_name = &self.tokens[tok_index];
        tok_index += 1;

        if param_name.kind == TokenKind::Ident {
            let mut node = self.create_node(NodeKind::ParamDecl, param_name);
            node.data.param_decl().borrow_mut().name = self.tok_val(param_name).to_string();
            let colon = &self.tokens[tok_index];
            tok_index += 1;
            self.expect_token(colon, TokenKind::Colon);
            node.data.param_decl().borrow_mut().param_type =
                RefCell::new(self.parse_type(tok_index, &mut tok_index));
            *new_tok_index = tok_index;
            Some(node)
        } else {
            self.invalid_token_error(param_name);
            None
        }
    }

    fn parse_param_decl_list(
        &self,
        tok_index: usize,
        new_tok_index: &mut usize,
        params: &mut Vec<RefCell<Node>>,
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
                params.push(RefCell::new(param_decl_node.unwrap()));
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

    fn parse_fn_proto(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let token = &self.tokens[*tok_index];
        let mut visib_mod = FnProtoVisibMod::Private;
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

        let mut node = self.create_node(NodeKind::FnProto, token);
        node.data.fn_proto().borrow_mut().visib_mod = visib_mod;

        let name_token = &self.tokens[*tok_index];
        *tok_index += 1;
        self.expect_token(name_token, TokenKind::Ident);
        node.data.fn_proto().borrow_mut().name = self.tok_val(name_token).to_string();
        self.parse_param_decl_list(
            *tok_index,
            tok_index,
            &mut node.data.fn_proto().borrow_mut().params,
        );
        None
    }

    fn parse_fn_def(&self, tok_index: &mut usize, mandatory: bool) -> Option<Node> {
        let fn_proto = self.parse_fn_proto(tok_index, mandatory);
        None
    }

    fn parse_top_level_decls(&mut self, tok_index: &mut usize, children: &mut Vec<RefCell<Node>>) {
        loop {
            let fn_def_node = self.parse_fn_def(tok_index, false);
        }
    }

    fn parse_root(&mut self, tok_index: &mut usize) {
        let mut node = self.create_node(NodeKind::Root, &self.tokens[*tok_index]);
        self.parse_top_level_decls(tok_index, &mut node.data.root().borrow_mut().children);
    }
}

pub fn parse(
    src: &str,
    tokens: Vec<RefCell<Token>>,
    owner: &mut ImportTableEntry,
    err_color: ErrColor,
) {
    let mut ctx = ParseContext::new(owner);
    ctx.err_color = err_color;
    ctx.src = src.to_string();
    for token in tokens {
        ctx.tokens.push(token.borrow().clone());
    }
    let mut tok_index = 0;
    ctx.parse_root(&mut tok_index);
}

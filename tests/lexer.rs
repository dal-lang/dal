#[cfg(test)]
mod test_lexer {
    use dal::lexer::*;

    #[test]
    fn parse_keywords() {
        let src = String::from(
            "@import @asm pub fn true false let mut const if else as void extern return",
        );
        let mut out = Tokenization::new();
        tokenize(&src, &mut out);
        assert!(out.errors.is_none());
        assert_eq!(out.tokens.len(), 16);
        assert_eq!(out.tokens[0].kind, TokenKind::KwImport);
        assert_eq!(out.tokens[1].kind, TokenKind::KwAsm);
        assert_eq!(out.tokens[2].kind, TokenKind::KwPub);
        assert_eq!(out.tokens[3].kind, TokenKind::KwFn);
        assert_eq!(out.tokens[4].kind, TokenKind::KwTrue);
        assert_eq!(out.tokens[5].kind, TokenKind::KwFalse);
        assert_eq!(out.tokens[6].kind, TokenKind::KwLet);
        assert_eq!(out.tokens[7].kind, TokenKind::KwMut);
        assert_eq!(out.tokens[8].kind, TokenKind::KwConst);
        assert_eq!(out.tokens[9].kind, TokenKind::KwIf);
        assert_eq!(out.tokens[10].kind, TokenKind::KwElse);
        assert_eq!(out.tokens[11].kind, TokenKind::KwAs);
        assert_eq!(out.tokens[12].kind, TokenKind::KwVoid);
        assert_eq!(out.tokens[13].kind, TokenKind::KwExtern);
        assert_eq!(out.tokens[14].kind, TokenKind::KwReturn);
        assert_eq!(out.tokens[15].kind, TokenKind::Eof);
    }

    #[test]
    fn test_str_lit() {
        let src = String::from("\"Hello, world!\"");
        let mut out = Tokenization::new();
        tokenize(&src, &mut out);
        assert!(out.errors.is_none());
        assert_eq!(out.tokens.len(), 2);
        assert_eq!(out.tokens[0].kind, TokenKind::StrLit);
        let chars: Vec<char> = src.chars().collect();
        assert_eq!(
            chars[out.tokens[0].start_pos..out.tokens[0].end_pos],
            "\"Hello, world!\"".chars().collect::<Vec<char>>()
        );
    }

    #[test]
    fn test_ident() {
        let src = String::from(
            "HelloWorld HELLOWORLD helloworld hello_world _hello_world _123 HELLO_WORLD",
        );
        let mut out = Tokenization::new();
        tokenize(&src, &mut out);
        assert!(out.errors.is_none());
        assert_eq!(out.tokens.len(), 8);
        for i in 0..6 {
            assert_eq!(out.tokens[i].kind, TokenKind::Ident);
        }
        assert_eq!(out.tokens[7].kind, TokenKind::Eof);
    }

    #[test]
    fn test_number() {
        let src = String::from("0123456789 123");
        let mut out = Tokenization::new();
        tokenize(&src, &mut out);
        assert!(out.errors.is_none());
        assert_eq!(out.tokens.len(), 3);
        assert_eq!(out.tokens[0].kind, TokenKind::NumLit);
        assert_eq!(out.tokens[1].kind, TokenKind::NumLit);
        assert_eq!(out.tokens[2].kind, TokenKind::Eof);
    }

    #[test]
    fn test_op() {
        let src = String::from("= + - * / % & | ^ ~ < > <= >= == != && ||");
        let mut out = Tokenization::new();
        tokenize(&src, &mut out);
        assert!(out.errors.is_none());
        assert_eq!(out.tokens.len(), 19);
        assert_eq!(out.tokens[0].kind, TokenKind::Assign);
        assert_eq!(out.tokens[1].kind, TokenKind::Plus);
        assert_eq!(out.tokens[2].kind, TokenKind::Dash);
        assert_eq!(out.tokens[3].kind, TokenKind::Star);
        assert_eq!(out.tokens[4].kind, TokenKind::Slash);
        assert_eq!(out.tokens[5].kind, TokenKind::Percent);
        assert_eq!(out.tokens[6].kind, TokenKind::BitAnd);
        assert_eq!(out.tokens[7].kind, TokenKind::BitOr);
        assert_eq!(out.tokens[8].kind, TokenKind::BitXor);
        assert_eq!(out.tokens[9].kind, TokenKind::BitNot);
        assert_eq!(out.tokens[10].kind, TokenKind::CmpLt);
        assert_eq!(out.tokens[11].kind, TokenKind::CmpGt);
        assert_eq!(out.tokens[12].kind, TokenKind::CmpLte);
        assert_eq!(out.tokens[13].kind, TokenKind::CmpGte);
        assert_eq!(out.tokens[14].kind, TokenKind::CmpEq);
        assert_eq!(out.tokens[15].kind, TokenKind::CmpNeq);
        assert_eq!(out.tokens[16].kind, TokenKind::BoolAnd);
        assert_eq!(out.tokens[17].kind, TokenKind::BoolOr);
        assert_eq!(out.tokens[18].kind, TokenKind::Eof);
    }

    #[test]
    fn test_special_symbol() {
        let src = String::from("-> . .. : ; , ( ) [ ] { }");
        let mut out = Tokenization::new();
        tokenize(&src, &mut out);
        assert!(out.errors.is_none());
        assert_eq!(out.tokens.len(), 13);
        assert_eq!(out.tokens[0].kind, TokenKind::Arrow);
        assert_eq!(out.tokens[1].kind, TokenKind::Dot);
        assert_eq!(out.tokens[2].kind, TokenKind::DotDot);
        assert_eq!(out.tokens[3].kind, TokenKind::Colon);
        assert_eq!(out.tokens[4].kind, TokenKind::Semicolon);
        assert_eq!(out.tokens[5].kind, TokenKind::Comma);
        assert_eq!(out.tokens[6].kind, TokenKind::LParen);
        assert_eq!(out.tokens[7].kind, TokenKind::RParen);
        assert_eq!(out.tokens[8].kind, TokenKind::LBracket);
        assert_eq!(out.tokens[9].kind, TokenKind::RBracket);
        assert_eq!(out.tokens[10].kind, TokenKind::LBrace);
        assert_eq!(out.tokens[11].kind, TokenKind::RBrace);
        assert_eq!(out.tokens[12].kind, TokenKind::Eof);
    }
}

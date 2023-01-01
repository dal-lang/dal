#[cfg(test)]
mod test {
    use dal::{
        ast::{NodeKind, TypeNodeKind},
        lexer::{self, Tokenization},
        parser::*,
    };

    #[test]
    fn test_simple_parsing() {
        let src = "fn main() { println(\"Hello, world!\") }".to_string();
        let mut out = Tokenization::new();
        lexer::tokenize(&src, &mut out);
        assert!(out.errors.is_none());
        let ast = parse(&src, out.tokens);
        let root = ast.data().root().clone();
        assert_eq!(root.children.borrow().len(), 1);
        assert_eq!(
            *root.children.borrow()[0].borrow().kind.borrow(),
            NodeKind::FnDef
        );
        let fn_def = root.children.borrow()[0].borrow().data().fn_def().clone();
        assert_eq!(
            *fn_def.proto.borrow().data().fn_proto().name.borrow(),
            "main"
        );
        let ret_ty = fn_def
            .proto
            .borrow()
            .data()
            .fn_proto()
            .ret_type
            .borrow()
            .clone();
        assert_eq!(
            *ret_ty.data().type_().kind.borrow(),
            TypeNodeKind::Primitive
        );
        assert_eq!(*ret_ty.data().type_().name.borrow(), "void");
    }
}

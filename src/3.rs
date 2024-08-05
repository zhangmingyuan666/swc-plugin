use swc_core::ecma::{
    ast::{*},
    transforms::testing::test_inline,
    visit::{as_folder, Fold, FoldWith, VisitMut, VisitMutWith},
};
use swc_core::common::DUMMY_SP;
pub struct ImportTransformer;

impl Fold for ImportTransformer {
    fn fold_var_declarator(&mut self, decl: VarDeclarator) -> VarDeclarator {
        // 递归处理所子节点
        let mut decl = decl.fold_children_with(self); 

        if let Some(init) = &decl.init {
            if let Expr::Call(call_expr) = &**init {
                if let ExprOrSuper::Expr(expr) = &call_expr.callee {
                    if let Expr::Ident(ident) = &**expr {
                        if ident.sym == *"s1sAsyncImport" {
                            if let Some(arg) = call_expr.args.first() {
                                if let Expr::Lit(Lit::Str(str_lit)) = &*arg.expr {
                                    let import_path = str_lit.value.clone();

                                    let new_expr = Box::new(Expr::Arrow(ArrowExpr {
                                        params: vec![],
                                        body: BlockStmtOrExpr::BlockStmt(BlockStmt {
                                            stmts: vec![Stmt::Return(ReturnStmt {
                                                arg: Some(Box::new(Expr::Call(CallExpr {
                                                    callee: ExprOrSuper::Expr(Box::new(Expr::Import(Import { span: DUMMY_SP }))),
                                                    args: vec![ExprOrSpread {
                                                        spread: None,
                                                        expr: Box::new(Expr::Lit(Lit::Str(Str {
                                                            value: import_path,
                                                            span: DUMMY_SP,
                                                            has_escape: false,
                                                            kind: StrKind::Synthesized,
                                                        }))),
                                                    },
                                                    ],
                                                    type_args: None,
                                                })))
                                            })]
                                        }),
                                        is_async: false,
                                        is_generator: false,
                                        span: DUMMY_SP,
                                        return_type: None,
                                        type_params: None,
                                    }));

                                    decl.init = Some(new_expr);
                                } 
                            }
                        }
                    }
                }
            }
        }
        decl
    }
}
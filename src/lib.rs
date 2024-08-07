use swc_core::{common::DUMMY_SP, ecma::{
    ast::*,
    transforms::testing::test_inline,
    visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
}};
use swc_core::ecma::atoms::JsWord;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::common::SyntaxContext;
use std::{ptr::null, sync::Arc};
use swc_core::common::SourceMap;
use swc_ecma_codegen::{Emitter, text_writer::JsWriter};


pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
    fn visit_mut_var_declarator(&mut self, e: &mut VarDeclarator) {
        e.visit_mut_children_with(self);

        let mut should_wrap = Some(false);
        
        if let Some(Expr::Call(CallExpr {
            callee: Callee::Expr(callee),
            ..
        })) = e.init.as_deref().as_mut()
        {
            if let Expr::Ident(ident) = &**callee {
                // 如果发现是此函数，要给
                if ident.sym == *"s1sAsyncImport" {
                    should_wrap = Some(true)
                }
            }
        }

        match should_wrap {
            // 应该进行处理
            Some(true) => {
                println!("True");

                let init = e.init.as_mut().unwrap();

                *init = Box::new(Expr::Arrow(ArrowExpr {
                    span: DUMMY_SP,
                    params: vec![],
                    is_async: false,
                    is_generator: false,
                    type_params: None,
                    return_type: None,
                    ctxt: SyntaxContext::empty(),
                    body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                        span: DUMMY_SP,
                        ctxt: SyntaxContext::from_u32(3),
                        stmts: vec![Stmt::Return(ReturnStmt {
                            span: DUMMY_SP,
                            arg: None
                        }) ]
                    }))
                }))
            }

            // 无需进行处理
            Some(false) => {
                println!("False");
            }

            _ => {
                println!("False");
            }
        }
    }
}

/// An example plugin function with macro support.
/// `plugin_transform` macro interop pointers into deserialized structs, as well
/// as returning ptr back to host.
///
/// It is possible to opt out from macro by writing transform fn manually
/// if plugin need to handle low-level ptr directly via
/// `__transform_plugin_process_impl(
///     ast_ptr: *const u8, ast_ptr_len: i32,
///     unresolved_mark: u32, should_enable_comments_proxy: i32) ->
///     i32 /*  0 for success, fail otherwise.
///             Note this is only for internal pointer interop result,
///             not actual transform result */`
///
/// This requires manual handling of serialization / deserialization from ptrs.
/// Refer swc_plugin_macro to see how does it work internally.
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}

// An example to test plugin transform.
// Recommended strategy to test plugin's transform is verify
// the Visitor's behavior, instead of trying to run `process_transform` with mocks
// unless explicitly required to do so.
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    simple_transform_global_var,
    // Input codes
    r#"let isDev = s1sAsyncImport(1);"#,
    r#"let isDev = s1sAsyncImport(1);"#
);

test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    acllk,
    // Input codes
    r#"let isDev = test(1);"#,
    r#"let isDev = false;"#
);
use crate::parser::functions::return_handler::{ReturnTypeContext, ReturnType};

pub fn analyze(ctx: &mut ReturnTypeContext) {
    ctx.register_fn("print", ReturnType::Void);
    ctx.register_fn("sqrt", ReturnType::Float);
}

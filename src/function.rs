use meval::{eval_str, Expr, Context};
struct Function<'a> {
    ctx: Context<'a>,
    expr: String,
}

//TODO: implementation ideas
//- Have function be parsed via evaluate_input before the whole string is evaluated.
//- input variables must follow the requirements from valid_variable_name






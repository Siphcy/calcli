use crate::error::EvalError;
use crate::eval::evaluate_input;
use crate::eval_context::EvalContext;
use crate::parser::format_variables_with_exclusion;
use std::borrow::Cow;



#[derive(Clone)]
pub struct Function {
    pub func_name: Cow<'static, str>,
    pub var_name: Cow<'static, str>,
    pub expr: Cow<'static, str>,
}

impl Function {
    pub fn new(func_name: String, var_name: String, expr: String) -> Self {
        Self {
            func_name: Cow::Owned(func_name),
            var_name: Cow::Owned(var_name),
            expr: Cow::Owned(expr),
        }
    }
    pub const fn define_new(func_name: &'static str, var_name: &'static str, expr: &'static str) -> Self {
        Self {
            func_name: Cow::Borrowed(func_name),
            var_name: Cow::Borrowed(var_name),
            expr: Cow::Borrowed(expr),
        }
    }
   pub fn evaluate_func(&self, ctx: &mut EvalContext, value: &str) -> Result<f64, EvalError> {
        // First, evaluate the input value to get a number
        let evaluated_value = evaluate_input(ctx, value)?;

        // Format variables but exclude the function parameter (keep it in brackets)
        let new_expr = format_variables_with_exclusion(self.expr.to_string(), ctx, Some(self.var_name.as_ref()));

        // Replace all instances of [var_name] with the evaluated value
        let search_pattern = format!("[{}]", self.var_name);
        let replacement = format!("({})", evaluated_value);
        let input = new_expr.replace(&search_pattern, &replacement);

        // Evaluate the modified expression
        evaluate_input(ctx, &input)
    }


}






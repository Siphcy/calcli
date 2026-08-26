use fancy_regex::Regex;
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

        // Create a regex to match the specific variable in brackets (e.g., [x])
        let pattern = format!(r"\[{}\]", self.var_name);
        let var_regex = Regex::new(&pattern).unwrap();

        // Replace all instances of [var_name] with the evaluated value
        let input = var_regex.replace_all(&new_expr, format!("({})", evaluated_value.to_string())).to_string();

        // Evaluate the modified expression
        evaluate_input(ctx, &input)
    }


}






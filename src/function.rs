use fancy_regex::Regex;
use crate::eval::{EvalError, evaluate_input};
use crate::eval_context::EvalContext;
use crate::parser::format_variables_with_exclusion;

#[derive(Clone)]
pub struct Function {
    pub func_name: String,
    pub var_name: String,
    pub expr: String,
}

impl Function {
    pub fn new(func_name: String, var_name: String, expr: String) -> Self {
        Self {
            func_name,
            var_name,
            expr,
        }
    }

   pub fn evaluate_func(&self, ctx: &mut EvalContext, value: &str) -> Result<f64, EvalError> {
        // First, evaluate the input value to get a number
        let evaluated_value = evaluate_input(ctx, value)?;

        // Format variables but exclude the function parameter (keep it in brackets)
        let new_expr = format_variables_with_exclusion(self.expr.clone(), ctx, Some(&self.var_name));

        // Create a regex to match the specific variable in brackets (e.g., [x])
        let pattern = format!(r"\[{}\]", self.var_name);
        let var_regex = Regex::new(&pattern).unwrap();

        // Replace all instances of [var_name] with the evaluated value
        let input = var_regex.replace_all(&new_expr, format!("({})", evaluated_value.to_string())).to_string();

        // Evaluate the modified expression
        evaluate_input(ctx, &input)
    }


}

//TODO: implementation ideas
//- Have function be parsed via evaluate_input before the whole string is evaluated.
//- input variables must follow the requirements from valid_variable_name
//
//





use meval::{Context};
use std::collections::HashMap;

pub struct EvalContext<'a> {
    pub ctx:  Context<'a>,
    pub parsed_results:  Vec<f64>,
    pub counter:  usize,
    pub defined_vars: HashMap<String, f64>,
}
impl<'a> EvalContext<'a> {
      pub fn new() -> Self {
          Self {
              ctx: Context::new(),
              parsed_results: Vec::new(),
              counter: 1,
              defined_vars: HashMap::new(),
          }
      }
  }



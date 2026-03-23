use meval::{Context};
use indexmap::IndexMap;
use crate::function::Function;

pub struct EvalContext<'a> {
    pub ctx:  Context<'a>,
    pub parsed_results:  Vec<f64>,
    pub counter:  usize,
    pub defined_vars: IndexMap<String, f64>,
    pub defined_funcs: IndexMap<String, Function>,
}
impl<'a> EvalContext<'a> {
      pub fn new() -> Self {
          Self {
              ctx: Context::new(),
              parsed_results: Vec::new(),
              counter: 1,
              defined_vars: IndexMap::new(),
              defined_funcs: IndexMap::new(),
          }
      }
  }



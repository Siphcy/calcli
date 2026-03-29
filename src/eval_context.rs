use meval::{Context};
use indexmap::IndexMap;
use crate::definition_handler::function::Function;

pub struct EvalContext<'a> {
    pub ctx:  Context<'a>,
    pub counter:  usize,
    pub defined_vars: IndexMap<String, f64>,
    pub defined_funcs: IndexMap<String, Function>,
    pub history_entries: Vec<(String, f64)>,
    pub recently_assigned: Vec<(String, String)>

}
impl<'a> EvalContext<'a> {
      pub fn new() -> Self {
          Self {
              ctx: Context::new(),
              counter: 0,
              defined_vars: IndexMap::new(),
              defined_funcs: IndexMap::new(),
              history_entries: Vec::new(),
              recently_assigned: Vec::new(),
          }
      }
  }



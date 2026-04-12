use meval::{Context};
use indexmap::IndexMap;
use crate::definition_handler::function::Function;
use crate::conversion_handler::scientific_notation::format_number;
use crate::constant::CONSTANT_VAR;
use crate::constant::CONSTANT_FUNC;

pub struct EvalContext<'a> {
    pub ctx:  Context<'a>,
    pub counter:  usize,
    pub defined_vars: IndexMap<String, f64>,
    pub defined_funcs: IndexMap<String, Function>,
    pub history_entries: Vec<(String, f64)>,
    pub recently_assigned: Vec<(String, String)>,
    pub sci_notation_enabled: bool,
    pub precision: usize,
    pub digit_threshold: usize,
}

impl<'a> EvalContext<'a> {
      pub fn new() -> Self {

        let mut init_ctx = Context::new();
        let mut var_map = IndexMap::new();
        for (name, value) in CONSTANT_VAR {
            init_ctx.var(*name, *value);
            var_map.insert(name.to_string(), *value);
            }
        let mut func_map = IndexMap::new();
        for (name, function) in CONSTANT_FUNC {
            func_map.insert(name.to_string(), function.clone());
            }

          Self {
              ctx: init_ctx,
              counter: 0,
              defined_vars: var_map,
              defined_funcs: func_map,
              history_entries: Vec::new(),
              recently_assigned: Vec::new(),
              sci_notation_enabled: true,
              precision: 6,
              digit_threshold: 6,  // Trigger sci notation for 6+ digits (100000+) or 6+ leading zeros (0.000001-)
          }
      }

      /// Format a number according to current scientific notation settings
      pub fn format_result(&self, value: f64) -> String {
          format_number(
              value,
              self.sci_notation_enabled,
              self.precision,
              self.digit_threshold,
          )
      }
  }



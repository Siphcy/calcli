use meval::{Context};

pub struct EvalContext<'a> {
    pub ctx:  Context<'a>,
    pub parsed_results:  Vec<f64>,
    pub counter:  usize
}
impl<'a> EvalContext<'a> {
      pub fn new() -> Self {
          Self {
              ctx: Context::new(),
              parsed_results: Vec::new(),
              counter: 1,
          }
      }
  }



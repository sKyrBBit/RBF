use super::data::Data;
use std::collections::HashMap;

/// Wrapper of HashMap
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub(crate) struct Symbols {
  table: HashMap<Box<str>, Data>,
  enclosing: Option<Box<Symbols>>,
}
impl Symbols {
  pub(crate) fn new() -> Self {
    Self {
      table: HashMap::with_capacity(32),
      enclosing: None,
    }
  }
  pub(crate) fn enclose(&mut self, enclosing: Symbols) {
    self.enclosing = Some(Box::from(enclosing));
  }
  pub(crate) fn disclose(&mut self) -> Option<Self> {
    std::mem::take(&mut self.enclosing).map(|s| *s)
  }
  pub(crate) fn insert(&mut self, key: Box<str>, value: Data) {
    self.table.insert(key, value);
  }
  pub(crate) fn get(&self, key: &Box<str>) -> Option<&Data> {
    self.table
      .get(key)
      .or_else(|| if let Some(table) = &self.enclosing {
        table.get(key)
      } else {
        None
      })
  }
}
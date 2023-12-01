mod ban;
mod clear;
mod help;
mod kick;
mod util;

use gooby::prelude::*;

pub struct Commands;

impl Commands {
  pub fn all() -> Vec<PoiseCommand> {
    vec![help::help(), clear::clear(), kick::kick(), ban::ban()]
  }
}

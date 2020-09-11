mod print;
mod to_string;

pub use to_string::to_string;

use sflyn_parser::tokens::Token;

use super::{
  BuiltIn,
  Error,
  Objects,
};

pub fn get_builtin_for_identifier(identifier: Token) -> Box<Objects> {
  // Print
  if identifier.value == "print" {
    return Box::new(Objects::BUILTIN(BuiltIn {
      obj: None,
      fun: Some(print::print),
    }));
  }

  // Default
  Error::new(
    format!("`{}` identifier not found.", identifier.value.clone()),
    identifier.clone(),
  )
}

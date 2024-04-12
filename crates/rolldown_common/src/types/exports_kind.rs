use bincode::{Decode, Encode};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Encode, Decode)]
pub enum ExportsKind {
  Esm,
  CommonJs,
  None,
}

impl Default for ExportsKind {
  fn default() -> Self {
    Self::None
  }
}

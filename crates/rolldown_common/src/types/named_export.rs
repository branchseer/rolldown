use crate::SymbolRef;
use bincode::{Decode, Encode};

/// This is a representation for statements like
/// - Case A: `export function foo() {}`
/// - Case B: `const foo = 1; export { foo }`
/// - Case C: `const foo = 1; export { foo as foo2 }`
#[derive(Debug, Encode, Decode)]
pub struct LocalExport {
  pub referenced: SymbolRef,
}

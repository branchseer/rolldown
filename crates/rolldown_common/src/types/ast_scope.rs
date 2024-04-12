use bincode::{Decode, Encode};
use index_vec::IndexVec;
use oxc::semantic::{Reference, ReferenceId, ScopeTree, SymbolId};

#[derive(Debug, Decode, Encode)]
pub struct AstScope {
  inner: ScopeTree,
  references: IndexVec<ReferenceId, Reference>,
  resolved_references: IndexVec<SymbolId, Vec<ReferenceId>>,
}

impl AstScope {
  pub fn new(
    inner: ScopeTree,
    references: IndexVec<ReferenceId, Reference>,
    resolved_references: IndexVec<SymbolId, Vec<ReferenceId>>,
  ) -> Self {
    Self { inner, references, resolved_references }
  }

  pub fn is_unresolved(&self, reference_id: ReferenceId) -> bool {
    self.references[reference_id].symbol_id().is_none()
  }

  pub fn symbol_id_for(&self, reference_id: ReferenceId) -> Option<SymbolId> {
    self.references[reference_id].symbol_id()
  }

  pub fn get_resolved_references(
    &self,
    symbol_id: SymbolId,
  ) -> impl Iterator<Item = &Reference> + '_ {
    self.resolved_references[symbol_id].iter().map(|reference_id| &self.references[*reference_id])
  }
}

impl std::ops::Deref for AstScope {
  type Target = ScopeTree;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

use std::{fmt::Debug, sync::Arc};

use crate::OxcCompiler;
use bincode::{Decode, Encode};
use oxc::{allocator::Allocator, ast::ast::Program, span::SourceType};

use self_cell::self_cell;

mod helpers;

self_cell!(
  pub(crate) struct Inner {
    owner: (Arc<str>, Allocator),

    #[covariant]
    dependent: Program,
  }
);

/// `OxcAst` is a wrapper of `Program` that provides a safe way to treat `Program<'ast>` as as owned value without considering the lifetime of `'ast`.
pub struct OxcAst {
  pub(crate) inner: Inner,
}

impl OxcAst {
  pub fn source(&self) -> &Arc<str> {
    &self.inner.borrow_owner().0
  }

  pub fn allocator(&self) -> &Allocator {
    &self.inner.borrow_owner().1
  }

  pub fn program(&self) -> &Program {
    self.inner.borrow_dependent()
  }

  /// Visit all fields including `&mut Program` within a closure.
  ///
  /// ## Example
  ///
  /// ```ignore
  /// let mut ast = OxcCompiler::parse("", SourceType::default());
  /// ast.with_mut(|fields| {
  ///   fields.source; // &Arc<str>
  ///   fields.allocator; // &Allocator
  ///   fields.program; // &mut Program
  /// });
  /// ```
  pub fn with_mut<'outer, Ret>(
    &'outer mut self,
    func: impl for<'inner> ::core::ops::FnOnce(WithFieldsMut<'outer, 'inner>) -> Ret,
  ) -> Ret {
    self.inner.with_dependent_mut::<'outer, Ret>(|owner, program| {
      func(WithFieldsMut { source: &owner.0, allocator: &owner.1, program })
    })
  }
}

impl Encode for OxcAst {
  fn encode<E: bincode::enc::Encoder>(
    &self,
    encoder: &mut E,
  ) -> Result<(), bincode::error::EncodeError> {
    self.inner.borrow_owner().0.encode(encoder)?;
    self.inner.borrow_dependent().encode(encoder)?;
    Ok(())
  }
}

impl<C> Decode<C> for OxcAst {
  fn decode<D: bincode::de::Decoder<Ctx = C>>(
    decoder: &mut D,
  ) -> Result<Self, bincode::error::DecodeError> {
    let source = Arc::<str>::decode(decoder)?;
    let inner = Inner::try_new((source, Allocator::default()), |owner| {
      let mut allocator = &owner.1;
      Program::decode(&mut decoder.with_ctx(&mut allocator))
    })?;
    Ok(Self { inner })
  }
}

bincode::impl_borrow_decode!(OxcAst);

pub struct WithFieldsMut<'outer, 'inner> {
  pub source: &'inner Arc<str>,
  pub allocator: &'inner Allocator,
  pub program: &'outer mut Program<'inner>,
}

impl Debug for OxcAst {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Ast").field("source", &self.inner.borrow_owner().0).finish_non_exhaustive()
  }
}

impl Default for OxcAst {
  fn default() -> Self {
    OxcCompiler::parse("", SourceType::default())
  }
}

unsafe impl Send for OxcAst {}
unsafe impl Sync for OxcAst {}

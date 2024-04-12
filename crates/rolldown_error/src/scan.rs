use std::sync::Arc;

use bincode::{Decode, Encode};
use oxc::span::Span;

use crate::{
  events::{forbid_const_assign::ForbidConstAssign, unsupported_eval::UnsupportedEval, BuildEvent},
  BuildError,
};

#[derive(Debug, Encode, Decode)]
pub struct ScanWarning(ScanWarningKind);

#[derive(Debug, Encode, Decode)]
enum ScanWarningKind {
  UnsupportedEval(UnsupportedEval),
  ForbidConstAssign(ForbidConstAssign),
}

impl ScanWarning {
  pub fn unsupported_eval(filename: String, source: Arc<str>, span: Span) -> Self {
    Self(ScanWarningKind::UnsupportedEval(UnsupportedEval { filename, source, eval_span: span }))
  }

  pub fn forbid_const_assign(
    filename: String,
    source: Arc<str>,
    name: String,
    reference_span: Span,
    re_assign_span: Span,
  ) -> Self {
    Self(ScanWarningKind::ForbidConstAssign(ForbidConstAssign {
      filename,
      source,
      name,
      reference_span,
      re_assign_span,
    }))
  }
}

impl From<ScanWarning> for BuildError {
  fn from(value: ScanWarning) -> Self {
    Self::new_inner(match value.0 {
      ScanWarningKind::UnsupportedEval(unsupported_eval) => {
        Box::new(unsupported_eval) as Box<dyn BuildEvent>
      }
      ScanWarningKind::ForbidConstAssign(forbid_const_assign) => {
        Box::new(forbid_const_assign) as Box<dyn BuildEvent>
      }
    })
    .with_severity_warning()
  }
}

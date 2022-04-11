use crate::{self as turbo_tasks};

/// Just an empty type.
/// [NothingVc] can be used as return value instead of `()`
/// to have a concrete reference that can be awaited.
#[turbo_tasks::value]
#[derive(PartialEq, Eq)]
pub struct Nothing;

#[turbo_tasks::value_impl]
impl NothingVc {
    pub fn new() -> Self {
        NothingVc::slot(Nothing)
    }
}
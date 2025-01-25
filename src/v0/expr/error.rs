use thiserror::Error;

use crate::v0::tokens::ExprToken;

#[derive(Debug, Error)]
#[error("Expected {expected}, but found {found}.")]
pub struct NonMatchingExprError {
    pub expected: ExprToken,
    pub found: ExprToken,
}

use crate::{common::traits::private::Sealed, v0::expr::ExprTree};

/// Contents of a [raw formula file](https://github.com/jiricekcz/fef-specification/blob/main/file_content_types/Raw%20Formula.md)
///
/// Expression is represented as an expression tree using the [`ExprTree`] type.
#[derive(Debug, Clone, PartialEq)]
pub struct RawFormulaFile {
    pub(crate) expression: ExprTree,
}

impl RawFormulaFile {
    /// Root of the expression tree
    pub fn root_expression(&self) -> &ExprTree {
        &self.expression
    }

    /// Decompose the file into the [`ExprTree`] root
    pub fn into_root_expression(self) -> ExprTree {
        self.expression
    }
}

impl Sealed for RawFormulaFile {}

impl Into<ExprTree> for RawFormulaFile {
    fn into(self) -> ExprTree {
        self.expression
    }
}

use crate::{common::traits::private::Sealed, v0::expr::ExprTree};

#[derive(Debug, Clone, PartialEq)]
pub struct RawFormulaFile {
    pub(crate) expression: ExprTree,
}

impl RawFormulaFile {
    pub fn root_expression(&self) -> &ExprTree {
        &self.expression
    }

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

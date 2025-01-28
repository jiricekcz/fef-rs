// It is very probable, that this can be written better, however between questionably working
// orphan rule and possible conflicting implementations from downstream crates, this is the
// only way I got this to work.
//
// I'm however very sure this doesn't leak any implementation details to the API.
//
// The whole point of this file is for all expression objects to implement the `ReadFrom<R: ?Sized + Read>`
// trait. Currently this is achieved Multiple macros (reducing them to one is a future task) and several
// helper functions.
//
// Note, that the read_from method of an expr object should read from the byte stream with the assumption,
// that the expression identifier has already been read and identified.

use std::io::Read;

use crate::v0::{
    config::Config,
    expr::{
        error::ExprReadError, ExprAddition, ExprCube, ExprCubeRoot, ExprDivision, ExprFalseLiteral,
        ExprIntDivision, ExprIntRoot, ExprModulo, ExprMultiplication, ExprNegation, ExprPower,
        ExprReciprocal, ExprRoot, ExprSquare, ExprSquareRoot, ExprSubtraction, ExprTrueLiteral,
        ExprVariable,
    },
    traits::ReadFrom,
};

mod read_expr {
    use std::io::Read;

    use crate::v0::{
        config::Config,
        expr::{
            error::ExprReadError,
            traits::{BinaryOperationExpr, EnumExpr, PureExpr, UnaryOperationExpr},
        },
        raw::VariableLengthEnum,
        traits::ReadFrom,
    };

    pub fn read_pure_expr<R: ?Sized + Read, S: ReadFrom<R>, O: PureExpr<S>, C: ?Sized + Config>(
        _reader: &mut R,
        _configuration: &C,
    ) -> Result<O, ExprReadError> {
        Ok(O::from(()))
    }

    pub fn read_enum_expr<R: ?Sized + Read, S: ReadFrom<R>, O: EnumExpr<S>, C: ?Sized + Config>(
        reader: &mut R,
        configuration: &C,
    ) -> Result<O, ExprReadError>
    where
        ExprReadError: From<<O as TryFrom<VariableLengthEnum>>::Error>,
    {
        let variable_length_enum = VariableLengthEnum::read_from(reader, configuration)?;
        Ok(O::try_from(variable_length_enum)?)
    }

    pub fn read_unary_operation_expr<
        R: ?Sized + Read,
        S: ReadFrom<R>,
        O: UnaryOperationExpr<S>,
        C: ?Sized + Config,
    >(
        reader: &mut R,
        configuration: &C,
    ) -> Result<O, ExprReadError>
    where
        ExprReadError: From<S::ReadError>,
    {
        let inner_expr: S = S::read_from(reader, configuration)?;
        Ok(O::from(inner_expr))
    }

    pub fn read_binary_operation_expr<
        R: ?Sized + Read,
        S: ReadFrom<R>,
        O: BinaryOperationExpr<S>,
        C: ?Sized + Config,
    >(
        reader: &mut R,
        configuration: &C,
    ) -> Result<O, ExprReadError>
    where
        ExprReadError: From<S::ReadError>,
    {
        let left_expr: S = S::read_from(reader, configuration)?;
        let right_expr: S = S::read_from(reader, configuration)?;
        Ok(O::from((left_expr, right_expr)))
    }
}

macro_rules! impl_read_from_pure_expr {
    ($($t:ty), +) => {
        $(
            impl<R: ?Sized + Read, S: Sized + ReadFrom<R>> ReadFrom<R> for $t
            where ExprReadError: From<S::ReadError>
             {
                type ReadError = ExprReadError;

                fn read_from<C: ?Sized + Config>(reader: &mut R, configuration: &C) -> Result<Self, ExprReadError> {
                    read_expr::read_pure_expr::<R, S, Self, C>(reader, configuration)
                }
            }
        )+
    };
}

macro_rules! impl_read_from_enum_expr {
    ($($t:ty), +) => {
        $(
            impl<R: ?Sized + Read, S: Sized + ReadFrom<R>> ReadFrom<R> for $t
            where ExprReadError: From<S::ReadError>
            {
                type ReadError = ExprReadError;

                fn read_from<C: ?Sized + Config>(reader: &mut R, configuration: &C) -> Result<Self, ExprReadError> {
                    read_expr::read_enum_expr::<R, S, Self, C>(reader, configuration)
                }
            }
        )+
    };
}

macro_rules! impl_read_from_unary_operation_expr {
    ($($t:ty), +) => {
        $(
            impl<R: ?Sized + Read, S: Sized + ReadFrom<R>> ReadFrom<R> for $t
            where ExprReadError: From<S::ReadError>
            {
                type ReadError = ExprReadError;

                fn read_from<C: ?Sized + Config>(reader: &mut R, configuration: &C) -> Result<Self, ExprReadError> {
                    read_expr::read_unary_operation_expr::<R, S, Self, C>(reader, configuration)
                }
            }
        )+
    };
}

macro_rules! impl_read_from_binary_operation_expr {
    ($($t:ty), +) => {
        $(
            impl<R: ?Sized + Read, S: Sized + ReadFrom<R>> ReadFrom<R> for $t
            where ExprReadError: From<S::ReadError>
            {
                type ReadError = ExprReadError;

                fn read_from<C: ?Sized + Config>(reader: &mut R, configuration: &C) -> Result<Self, ExprReadError> {
                    read_expr::read_binary_operation_expr::<R, S, Self, C>(reader, configuration)
                }
            }
        )+
    };
}

impl_read_from_pure_expr!(ExprTrueLiteral<S>, ExprFalseLiteral<S>);

impl_read_from_enum_expr!(ExprVariable<S>);

impl_read_from_unary_operation_expr!(
    ExprNegation<S>,
    ExprReciprocal<S>,
    ExprSquare<S>,
    ExprCube<S>,
    ExprSquareRoot<S>,
    ExprCubeRoot<S>
);

impl_read_from_binary_operation_expr!(
    ExprAddition<S>,
    ExprSubtraction<S>,
    ExprMultiplication<S>,
    ExprDivision<S>,
    ExprModulo<S>,
    ExprPower<S>,
    ExprIntDivision<S>,
    ExprRoot<S>,
    ExprIntRoot<S>
);

//! Implementations of the [TryReadFromAndComposeWithContext] for all expressions.
//!
//! This module contains implementations of the [TryReadFromAndComposeWithContext] trait for all expressions.
//! When this trait is called on an expression object, it should assume, that the corresponding expression
//! token has already been read from the input stream and the reader is positioned at the beginning of the
//! expression body.
//!
//! This module should never leak any implementation details. It will probably be very macro heavy and
//! subject to a lot of refactoring.
use std::io::Read;

use crate::v0::{
    config::Config,
    expr::{
        error::ExprReadWithComposerError,
        traits::{ComposeIntoWithContext, TryReadFromAndComposeWithContext},
    },
};

use super::{ExprFalseLiteral, ExprTrueLiteral};

macro_rules! impl_read_from_pure_expr {
    ($($t:ty), +) => {
        $(
            impl<S: Sized, CTX: ?Sized> TryReadFromAndComposeWithContext<S, CTX> for $t
            where
                $t: ComposeIntoWithContext<S, CTX>,
            {
                fn try_read_from_and_compose_with_context<R: ?Sized + Read, C: ?Sized + Config>(
                    _byte_stream: &mut R,
                    _configuration: &C,
                    context: &CTX,
                ) -> Result<S, ExprReadWithComposerError<<Self as ComposeIntoWithContext<S, CTX>>::Error>> {
                    let expr = <$t>::from(());
                    let composed = expr.compose_into(context);
                    match composed {
                        Ok(composed) => Ok(composed),
                        Err(error) => Err(ExprReadWithComposerError::from_composer_error(error)),
                    }
                }
            }
        )+
    };
}

impl_read_from_pure_expr!(ExprTrueLiteral<S>, ExprFalseLiteral<S>);

use std::io::Read;

use crate::v0::{config::Config, expr::Expr};

pub fn parse_expression<R: ?Sized + Read, C: ?Sized + Config, S: Sized>(
    byte_stream: &mut R,
    config: &C,
    process: fn(Expr<S>) -> S,
) -> () {
}

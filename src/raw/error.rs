//! Errors for the raw module.

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error, Hash)]
pub enum IntegerReadError {}

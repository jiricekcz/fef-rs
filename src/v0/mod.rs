#![doc = include_str!("../../doc/v0.md")]

use crate::common::version::SpecVersion;

/// Currently implemented version of the FEF specification.
pub const IMPLEMENTED_SPECIFICATION_VERSION: SpecVersion = SpecVersion::new(0, 3, 0);

pub mod raw;

pub mod traits;

pub mod config;

pub mod tokens;

pub mod read;

pub mod expr;

pub mod write;

pub mod metadata;

pub mod file;

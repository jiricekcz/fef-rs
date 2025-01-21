#![doc = include_str!("../doc/crate.md")] // Create documentation is large and thus loaded from a separate file
#![cfg_attr(docsrs, feature(doc_auto_cfg))] // This is a nightly-only feature to generate docs for cfg attributes, run only on docs.rs
                                            // #![doc(html_playground_url = SOME_LINK)] - When we get a playground that has fef installed, we can add this to the docs

pub mod common;

#[cfg(feature = "v0")]
pub mod v0;

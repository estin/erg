//! defines the compiler for Erg (ergc).
extern crate common;
pub extern crate parser;

mod compile;
pub use compile::*;
mod codegen;
pub mod effectcheck;
pub mod error;
pub mod eval;
pub mod hir;
pub mod initialize;
pub mod lower;
pub use lower::ASTLowerer;
pub mod optimize;
pub mod ownercheck;
pub mod table;
pub mod varinfo;

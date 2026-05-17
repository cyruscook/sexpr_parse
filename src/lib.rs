//! Parser for a small S-expression syntax.
//!
//! The parser accepts top-level atoms, quoted text values, and parenthesized nodes.
//! Each node starts with a symbol name followed by zero or more child items.
//!
//! # Examples
//!
//! ```
//! use sexpr_parse::{parse_sexpr_stream, SExprItem};
//!
//! let parsed = parse_sexpr_stream(r#"(typ "m" (inst (alias nat)))"#)?;
//!
//! assert_eq!(
//!     parsed,
//!     vec![SExprItem::Node(
//!         "typ".to_string(),
//!         vec![
//!             SExprItem::Text("m".to_string()),
//!             SExprItem::Node(
//!                 "inst".to_string(),
//!                 vec![SExprItem::Node(
//!                     "alias".to_string(),
//!                     vec![SExprItem::Atom("nat".to_string())]
//!                 )]
//!             )
//!         ]
//!     )]
//! );
//! # Ok::<(), sexpr_parse::SExprError>(())
//! ```
#![deny(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::pedantic
)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic))]
#![allow(clippy::doc_markdown, clippy::missing_errors_doc)]

mod error;
mod parse;
mod reader;
mod sexpr;

pub use error::SExprError;
pub use parse::parse_sexpr_stream;
pub use sexpr::SExprItem;

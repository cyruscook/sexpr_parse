use std::string::FromUtf8Error;
use thiserror::Error;

/// Errors returned while parsing S-expressions.
#[derive(Error, Debug)]
pub enum SExprError {
    /// An I/O-style failure produced by the internal reader at the given byte offset.
    #[error("io error at {position}: {source}")]
    Io {
        #[source]
        source: std::io::Error,
        position: usize,
    },
    #[error("utf8 error at {position}: {source}")]
    /// The parser collected bytes that were not valid UTF-8 at the given byte offset.
    Utf8 {
        #[source]
        source: FromUtf8Error,
        position: usize,
    },
    #[error("unexpected byte at {position}: found {unexpected:#x}, expected {expected:#x}")]
    /// A specific byte was expected but a different byte was found instead.
    UnexpectedByte {
        unexpected: u8,
        expected: u8,
        position: usize,
    },
    #[error("expected symbol at {position}")]
    /// A node started with `(` but did not provide the required symbol name.
    ExpectedSymbol { position: usize },
    #[error("missing separator before byte {unexpected:#x} at {position}")]
    /// Two adjacent items were not separated by whitespace or `)`.
    MissingSeparator { unexpected: u8, position: usize },
    #[error("{context}: {source}")]
    /// Additional context attached while bubbling up another parsing error.
    WithContext {
        #[source]
        source: Box<SExprError>,
        context: String,
    },
}

impl SExprError {
    /// Attaches additional human-readable context to this error.
    #[must_use]
    pub fn with_context<R: AsRef<str>>(self, context: R) -> Self {
        Self::WithContext {
            source: Box::new(self),
            context: context.as_ref().to_string(),
        }
    }
}

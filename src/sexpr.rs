use std::fmt::Display;

/// A single parsed S-expression item.
#[derive(Clone, Debug, PartialEq)]
pub enum SExprItem {
    /// An unquoted symbol.
    Atom(String),
    /// A quoted text value with escape characters preserved as literal bytes.
    Text(String),
    /// A parenthesized node with its name and child items.
    Node(String, Vec<SExprItem>),
}

impl Display for SExprItem {
    /// Formats the item back into S-expression syntax.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SExprItem::Atom(s) => write!(f, "{s}"),
            SExprItem::Text(s) => write!(f, "\"{s}\""),
            SExprItem::Node(name, items) => {
                write!(f, "({name}")?;
                for item in items {
                    write!(f, " {item}")?;
                }
                write!(f, ")")
            }
        }
    }
}

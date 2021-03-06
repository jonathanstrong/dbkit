// vim: set ts=4 sw=4 et :

use std::fmt;
use std::heap::AllocErr;
use std::io::{Error as IOError};


/// Query execution errors
pub enum DBError {
    Unknown,
    /// An underlying IO operation caused an error
    IO(IOError),
    // Unknown type (parsing external type information)
    UnknownType(String),
    /// Referencing a missing schema attribute (name or position)
    AttributeMissing(String),
    /// Mismatched expectation about attributes nullability
    AttributeNullability(String),
    /// Mismatched expectation about attribute types
    AttributeType(String),
    /// Duplicate attribute in result schema
    AttributeDuplicate(String),
    ///
    ExpressionInputType(String),
    ExpressionInputCount(String),
    ExpressionNotCost,
    ///
    RowOutOfBounds,
    /// Unknown memory allocation error
    Memory(AllocErr),
    /// Memory allocation limit reached (via policy)
    MemoryLimit,
}

impl DBError {
    pub fn make_column_not_nullable(name: String) -> DBError {
        DBError::AttributeNullability(name)
    }

    pub fn make_column_unknown_pos(pos: usize) -> DBError {
        DBError::AttributeMissing(format!("(pos: {})", pos))
    }
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DBError::Unknown =>
                write!(f, "Unknown Error"),
            DBError::IO(ref e) =>
                write!(f, "IO Error {}", e),
            DBError::UnknownType(ref t) =>
                write!(f, "Unknown/Unexpected Type {}", t),
            DBError::AttributeMissing(ref attr) =>
                write!(f, "Unknown Attribute {}", attr),
            DBError::AttributeNullability(ref attr) =>
                write!(f, "Attribute Not Nullable {}", attr),
            DBError::AttributeType(ref attr) =>
                write!(f, "Attribute Type Mismatch {}", attr),
            DBError::AttributeDuplicate(ref attr) =>
                write!(f, "Duplicate Attribute name {} in output schema", attr),
            DBError::ExpressionInputType(ref str) =>
                write!(f, "Invalid expression input type: {}", str),
            DBError::ExpressionInputCount(ref str) =>
                write!(f, "Invalid expression input count: {}", str),
            DBError::ExpressionNotCost =>
                write!(f, "Expression expected to be const"),
            DBError::RowOutOfBounds =>
                write!(f, "Row out of bounds"),
            DBError::Memory(ref e) =>
                write!(f, "Memory allocation failure: {}", e),
            DBError::MemoryLimit =>
                write!(f, "Memory allocation failure due to policy limit"),
        }
    }
}

impl fmt::Debug for DBError {
    // Dummy implementation for Option / Result unwrap()
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

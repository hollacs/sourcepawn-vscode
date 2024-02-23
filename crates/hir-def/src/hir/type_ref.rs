use syntax::TSKind;
use tree_sitter::Node;

use crate::item_tree::Name;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TypeRef {
    /// Reference to a type definition (e.g. enum struct, enum, methodmap, etc.)
    Name(Name), // TODO: Disambiguate between new types and old types.

    /// int
    Int,

    /// bool
    Bool,

    /// float
    Float,

    /// char
    Char,

    /// void
    Void,

    /// any
    Any,

    /// String
    OldString,

    /// Float
    OldFloat,
}

impl TypeRef {
    pub fn from_node(node: &Node, source: &str) -> Self {
        match TSKind::from(node) {
            TSKind::anon_int => Self::Int,
            TSKind::anon_bool => Self::Bool,
            TSKind::anon_float => Self::Float,
            TSKind::anon_char => Self::Char,
            TSKind::anon_void => Self::Void,
            TSKind::any_type => Self::Any,
            TSKind::anon_String => Self::OldString,
            TSKind::anon_Float => Self::Float,
            _ => TypeRef::Name(Name::from_node(node, source)),
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            TypeRef::Name(name) => String::from(name.clone()), //TODO: Can we avoid this clone?
            TypeRef::Int => "int".to_string(),
            TypeRef::Bool => "bool".to_string(),
            TypeRef::Float => "float".to_string(),
            TypeRef::Char => "char".to_string(),
            TypeRef::Void => "void".to_string(),
            TypeRef::Any => "any".to_string(),
            TypeRef::OldString => "String".to_string(),
            TypeRef::OldFloat => "Float".to_string(),
        }
    }
}

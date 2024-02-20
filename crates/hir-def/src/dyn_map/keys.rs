//! keys to be used with `DynMap`

use crate::{EnumId, EnumStructId, FieldId, FunctionId, GlobalId, MacroId, NodePtr, VariantId};

pub type Key<K, V> = crate::dyn_map::Key<K, V>;

pub const FUNCTION: Key<NodePtr, FunctionId> = Key::new();
pub const MACRO: Key<NodePtr, MacroId> = Key::new();
pub const GLOBAL: Key<NodePtr, GlobalId> = Key::new();
pub const ENUM_STRUCT: Key<NodePtr, EnumStructId> = Key::new();
pub const ENUM: Key<NodePtr, EnumId> = Key::new();
pub const ENUM_VARIANT: Key<NodePtr, VariantId> = Key::new();
pub const FIELD: Key<NodePtr, FieldId> = Key::new();

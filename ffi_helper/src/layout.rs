//! Structures and types for implementing `_TypeInfoImpl`.
//!
//! Whatever you're looking for probably isn't here, since `_TypeInfoImpl` is
//! usually automatically derived using the [`TypeInfo`][derive@crate::TypeInfo] derive
//! procedural macro.

use crate::types::{SBox, SOption, SStr, SVec};

/// A list of all defined types with their [`TypeUid`]s
pub type DefinedTypes = Vec<(TypeUid, DefinedType)>;

/// Unique type ID
#[derive(PartialEq, Debug, Clone)]
pub struct TypeUid {
    /// The Rust source path to the type (`my_crate::path::to::MyType`)
    pub rustpath: SStr<'static>,
    /// The path of the file where the type is defined.
    pub file: SStr<'static>,
    /// The line in the source file where the type is defined
    pub line: u32,
    /// The column in the source file where the type is defined
    pub column: u32,
}

/// Layout and all defined types including their [`TypeUid`]s
/// Only for internal use
pub struct FullLayout {
    pub layout: Layout,
    pub defined_types: DefinedTypes,
}

/// Type definition (name and layout)
#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct DefinedType {
    pub name: SStr<'static>,
    pub ty: TypeType,
}

/// The type of a type (`struct`, `enum`, etc)
#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum TypeType {
    StructNamed {
        fields: SVec<NamedField>,
    },
    StructUnnamed {
        fields: SVec<Layout>,
    },
    StructUnit,
    Enum {
        variants: SVec<EnumVariant>,
        repr: SOption<SStr<'static>>,
    },
    Union {
        fields: SVec<NamedField>,
    },
}

/// The layout of a single segment
#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum Layout {
    Void,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    USize,
    ISize,
    Bool,
    F32,
    F64,
    Char,
    ConstPtr(SBox<Layout>),
    MutPtr(SBox<Layout>),
    Ref {
        referent: SBox<Layout>,
    },
    MutRef {
        referent: SBox<Layout>,
    },
    Array {
        len: usize,
        layout: SBox<Layout>,
    },
    DefinedType {
        id: usize, // id in the FullLayout.defined_types vec
    },
}

/// A field's name and layout
#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct NamedField {
    pub name: SStr<'static>,
    pub layout: Layout,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct EnumVariant {
    pub name: SStr<'static>,
    pub ty: EnumVariantType,
    pub discriminant: i64,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub enum EnumVariantType {
    Unit,
    Tuple(SVec<Layout>),
    Struct(SVec<NamedField>),
}

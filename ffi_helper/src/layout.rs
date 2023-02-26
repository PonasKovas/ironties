//! Structures and types for implementing `_TypeInfoImpl`.
//!
//! Whatever you're looking for probably isn't here, since `_TypeInfoImpl` is
//! usually automatically derived using the [`TypeInfo`][derive@crate::TypeInfo] derive
//! procedural macro.

use crate::types::{SBox, SStr, SVec};

/// A list of all defined types with their [`TypeUid`]s
pub type DefinedTypes = Vec<(TypeUid, DefinedType)>;

/// Unique type ID
#[derive(PartialEq, Debug)]
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
#[derive(Debug, PartialEq)]
pub struct FullLayout {
    pub layout: Layout,
    pub defined_types: DefinedTypes,
    pub lifetimes: Vec<Lifetime>,
}

/// Type definition (name and layout)
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct DefinedType {
    pub name: SStr<'static>,
    pub ty: TypeType,
}

/// Describes lifetime bounds and relations
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum Lifetime {
    Unbound,
    /// Indices of other lifetimes that this one must live longer than
    Outlives(SVec<usize>),
    Static,
}

/// The type of a type (`struct`, `enum`, etc)
#[repr(u8)]
#[derive(Debug, PartialEq)]
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
        repr: u8,
    },
}

/// The layout of a single segment
#[repr(u8)]
#[derive(Debug, PartialEq)]
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
        // index of the lifetime
        lifetime: usize,
    },
    MutRef {
        referent: SBox<Layout>,
        // index of the lifetime
        lifetime: usize,
    },
    Array {
        len: usize,
        layout: SBox<Layout>,
    },
    DefinedType(usize), // id in the Layout.defined_types vec
}

/// A field's name and layout
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct NamedField {
    pub name: SStr<'static>,
    pub layout: Layout,
}

/// An enum variant layout
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum EnumVariant {
    Unit,
    Tuple(SVec<Layout>),
    Struct(SVec<NamedField>),
}
use super::abstract_syntax_tree::*;

#[derive(Debug, Clone)]
pub struct ASTSymbol {
    pub id: usize,
    pub select_type: ASTSymbolValueType,
    pub scope: Scope,
    pub address: Address,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ASTSymbolValueType { 
    UnsignedChar,
    Char,
    UnsignedShort,
    Short,
    UnsignedInt,
    Int,
    UnsignedLong,
    Long,
    PtrUnsignedChar,
    PtrChar,
    PtrUnsignedShort,
    PtrShort,
    PtrUnsignedInt,
    PtrInt,
    PtrUnsignedLong,
    PtrLong,
    Unknown,
    Ptr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Scope {
    Local,
    Global,
    Temp,
}

impl ASTSymbol {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            select_type: ASTSymbolValueType::Unknown,
            scope: Scope::Temp,
            address: Address::GR(0),
        }
    }
}

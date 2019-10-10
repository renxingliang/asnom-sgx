use std::default;
use universal;
use structure;

use super::ASNTag;
use common::TagClass;
use std::prelude::v1::*;

#[derive(Clone, Debug, PartialEq)]
pub struct OctetString {
    pub id: u64,
    pub class: TagClass,
    pub inner: Vec<u8>,
}

impl ASNTag for OctetString {
    fn into_structure(self) -> structure::StructureTag {
        structure::StructureTag {
            id: self.id,
            class: self.class,
            payload: structure::PL::P(self.inner),
        }
    }
}

impl default::Default for OctetString {
    fn default() -> Self {
        OctetString {
            id: universal::Types::OctetString as u64,
            class: TagClass::Universal,
            inner: Vec::new(),
        }
    }
}

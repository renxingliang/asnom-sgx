use common::TagClass;
use std::prelude::v1::*;

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct StructureTag {
    pub class: TagClass,
    pub id: u64,
    pub payload: PL
}

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum PL {
    P(Vec<u8>),
    C(Vec<StructureTag>),
}

impl StructureTag {
    pub fn match_class(self, class: TagClass) -> Option<Self> {
        if self.class == class { Some(self) }
        else { None }
    }

    pub fn match_id(self, id: u64) -> Option<Self> {
        if self.id == id { Some(self) }
        else { None }
    }

    pub fn expect_constructed(self) -> Option<Vec<StructureTag>> {
        match self.payload {
            PL::P(_) => {
                None
            },
            PL::C(i) => {
                Some(i)
            }
        }
    }

    pub fn expect_primitive(self) -> Option<Vec<u8>> {
        match self.payload {
            PL::P(i) => {
                Some(i)
            },
            PL::C(_) => {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{TagClass, TagStructure};

    #[test]
    fn expect_exact() {
        let tag = StructureTag {
            class: TagClass::Application,
            id: 65u64,
            payload: PL::C(vec![
                StructureTag {
                    class: TagClass::Universal,
                    id: 2u64,
                    payload: PL::P(vec![0x16, 0x16]),
                }
            ]),
        };

        let out = tag.clone().match_class(TagClass::Application)
            .and_then(|x| x.match_id(65u64));

        assert_eq!(out, Some(tag));
    }

    #[test]
    fn expect_inner() {
        let tag = StructureTag {
            class: TagClass::Application,
            id: 65u64,
            payload: PL::C(vec![
               StructureTag {
                   class: TagClass::Universal,
                   id: 2u64,
                   payload: PL::P(vec![0x16, 0x16]),
               },
               StructureTag {
                   class: TagClass::Application,
                   id: 3u64,
                   payload: PL::P(vec![0x3, 0x3]),
               }
            ]),
        };

        let mut subt = tag.expect_constructed().unwrap();

        let b = subt.pop().unwrap().match_class(TagClass::Application).and_then(|x| x.match_id(3));
        let a = subt.pop().unwrap().match_class(TagClass::Universal).and_then(|x| x.match_id(2));

        assert!(a.is_some());
        assert!(b.is_some());
    }
}

use std::io::{Read, Result, Write};
#[cfg(test)] use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};
use varint_rs::{VarintReader, VarintWriter};

use crate::readable_writable::{Readable, Writable};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct RawTaggedField {
    pub tag: i32,
    pub data: Vec<u8>,
}

impl Readable for RawTaggedField {
    fn read(input: &mut impl std::io::Read) -> Result<Self> {
        let tag = input.read_u32_varint()? as i32;
        let data_len = input.read_u32_varint()? as i32;
        let mut data = vec![0u8; data_len as usize];
        input.read(&mut data)?;
        Ok(RawTaggedField { tag, data })
    }
}

impl Writable for RawTaggedField {
    fn write(&self, output: &mut impl Write) -> Result<()> {
        output.write_u32_varint(self.tag as u32)?;
        output.write_u32_varint(self.data.len() as u32)?;
        output.write(&self.data)?;
        Ok(())
    }
}

pub(crate) fn read_tagged_fields(
    input: &mut impl Read,
    mut callback: impl FnMut(i32, &[u8]) -> Result<bool>,
) -> Result<Vec<RawTaggedField>> {
    todo!()
}

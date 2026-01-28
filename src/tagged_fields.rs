use std::io::{Error, ErrorKind, Read, Result, Write};
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
    let len = input.read_u32_varint()?;
    let mut unknown_tagged_fiels: Vec<RawTaggedField> = Vec::new();
    for _ in 0..len {
        let field = RawTaggedField::read(input)?;
        if !callback(field.tag, &field.data)? {
            unknown_tagged_fiels.push(field);
        }
    }
    Ok(unknown_tagged_fiels)
}

pub(crate) fn write_tagged_fields(
    output: &mut impl Write,
    known_tagged_fields: &[RawTaggedField],
    unknown_tagged_fields: &[RawTaggedField],
) -> Result<()> {
    let mut max_known_tag = -1;
    for tag_pair in known_tagged_fields.windows(2) {
        let tag0 = &tag_pair[0].tag;
        let tag1 = &tag_pair[1].tag;
        if tag0 >= tag1 {
            return Err(Error::new(ErrorKind::Other, format!(
                "Invalid raw tag field list: tag {tag1:?} comes after tag {tag0:?}, but is not higher than it."
            )));
        }
        if *tag0 > max_known_tag {
            max_known_tag = *tag0;
        }
    }
    for tag_pair in unknown_tagged_fields.windows(2) {
        let tag0 = &tag_pair[0].tag;
        let tag1 = &tag_pair[1].tag;
        if tag0 >= tag1 {
            return Err(Error::new(ErrorKind::Other, format!(
                "Invalid raw tag field list: tag {tag1:?} comes after tag {tag0:?}, but is not higher than it."
            )));
        }
        if *tag0 <= max_known_tag {
            return Err(Error::new(ErrorKind::Other, format!(
                "Invalid raw tag field list: tag {tag0:?} is not higher than the maximum known tag {max_known_tag:?}."
            )));
        }
    }

    output.write_u32_varint((known_tagged_fields.len() + unknown_tagged_fields.len()) as u32)?;
    for el in known_tagged_fields {
        el.write(output)?;
    }
    for el in unknown_tagged_fields {
        el.write(output)?;
    }
    Ok(())
}

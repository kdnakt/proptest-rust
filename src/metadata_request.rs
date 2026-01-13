use std::io::{Read, Result, Write};
#[cfg(test)] use proptest_derive::Arbitrary;
use uuid::Uuid;

use crate::arrays::read_nullable_array;
use crate::readable_writable::{Readable, Writable, write_nullable_array};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct MetadataRequest {
    pub topics: Option<Vec<MetadataRequestTopic>>,
    pub allow_auto_topic_creation: bool,
    pub include_topic_authorized_operations: bool,
}

impl Readable for MetadataRequest {
    fn read(input: &mut impl Read) -> Result<Self> {
        let topics = read_nullable_array::<MetadataRequestTopic>(input, "topics", true)?;
        let allow_auto_topic_creation = bool::read(input)?;
        let include_topic_authorized_operations = bool::read(input)?;
        Ok(MetadataRequest {
            topics,
            allow_auto_topic_creation,
            include_topic_authorized_operations,
        })
    }
}

impl Writable for MetadataRequest {
    fn write(&self, output: &mut impl Write) -> Result<()> {
        write_nullable_array(output, "topics", self.topics.as_deref(), true)?;
        self.allow_auto_topic_creation.write(output)?;
        self.include_topic_authorized_operations.write(output)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct MetadataRequestTopic {
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    pub name: Option<String>,
}

impl Readable for MetadataRequestTopic {
    fn read(input: &mut impl Read) -> Result<Self> {
        let topic_id = Uuid::read(input)?;
        let name = Option::<String>::read_ext(input, "name", true)?;
        Ok(MetadataRequestTopic { topic_id, name })
    }
}

impl Writable for MetadataRequestTopic {
    fn write(&self, output: &mut impl Write) -> Result<()> {
        self.topic_id.write(output)?;
        self.name.write_ext(output, "name", true)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Seek, SeekFrom};
    use proptest::prelude::*;
    use super::*;

    proptest! {
        #[test]
        fn test_serde(data: MetadataRequest) {
            // Serialize
            let mut cur = Cursor::new(Vec::<u8>::new());
            data.write(&mut cur).unwrap();
            // Deserialize
            cur.seek(SeekFrom::Start(0)).unwrap();
            let data_read = MetadataRequest::read(&mut cur).unwrap();
            // Compare
            prop_assert_eq!(data_read, data.clone());
        }
    }
}

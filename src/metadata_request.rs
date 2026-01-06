use std::io::{Read, Result};
use uuid::Uuid;

use crate::arrays::read_nullable_array;
use crate::readable_writable::Readable;

#[derive(Clone, PartialEq)]
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

#[derive(Clone, PartialEq)]
pub struct MetadataRequestTopic {
    pub topic_id: Uuid,
    pub name: Option<String>,
}

impl Readable for MetadataRequestTopic {
    fn read(input: &mut impl Read) -> Result<Self> {
        let topic_id = Uuid::read(input)?;
        let name = Option::<String>::read_ext(input, "name", true)?;
        todo!()
    }
}

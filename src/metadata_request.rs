use uuid::Uuid;
use std::io::{Read, Result};

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
        todo!()
    }
}

#[derive(Clone, PartialEq)]
pub struct MetadataRequestTopic {
    pub topic_id: Uuid,
    pub name: Option<String>,
}

impl Readable for MetadataRequestTopic {
    fn read(input: &mut impl Read) -> Result<Self> {
        todo!()
    }
}

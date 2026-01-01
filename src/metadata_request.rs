use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct MetadataRequest {
    pub topics: Option<Vec<MetadataRequestTopic>>,
    pub allow_auto_topic_creation: bool,
    pub include_topic_authorized_operations: bool,
}

#[derive(Clone, PartialEq)]
pub struct MetadataRequestTopic {
    pub topic_id: Uuid,
    pub name: Option<String>,
}

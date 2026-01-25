use proptest::prelude::*;
use proptest::collection;
use uuid::Uuid;

use crate::tagged_fields::RawTaggedField;

pub(crate) fn uuid() -> impl Strategy<Value = Uuid> {
    (any::<u128>()).prop_map(|num| Uuid::from_u128(num))
}

pub(crate) fn bytes() -> impl Strategy<Value = Vec<u8>> {
    collection::vec(prop::num::u8::ANY, collection::size_range(0..10))
}

pub(crate) fn unknown_tagged_fields() -> impl Strategy<Value = Vec<RawTaggedField>> {
    prop_oneof![
        Just(Vec::<RawTaggedField>::new()),
        bytes().prop_map(|data| vec![RawTaggedField { tag: 999, data }]),
    ]
}

use proptest::prelude::*;
use uuid::Uuid;

pub(crate) fn uuid() -> impl Strategy<Value = Uuid> {
    (any::<u128>()).prop_map(|num| Uuid::from_u128(num))
}

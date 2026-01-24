use proptest::prelude::*;
use proptest::collection;
use uuid::Uuid;

pub(crate) fn uuid() -> impl Strategy<Value = Uuid> {
    (any::<u128>()).prop_map(|num| Uuid::from_u128(num))
}

pub(crate) fn vec<T>() -> impl Strategy<Value = Vec<T>>
where
    T: Arbitrary,
{
    collection::vec(any::<T>(), collection::size_range(0..10))
}

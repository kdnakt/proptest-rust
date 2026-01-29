use proptest::prelude::TestCaseError;

use std::fmt::Debug;
use crate::readable_writable::{Readable, Writable};

#[cfg(test)]

pub(crate) mod proptest_strategies;

pub(crate) fn test_serde<T>(data: &T) -> Result<(), TestCaseError>
where
    T: Readable + Writable + Debug + PartialEq + Clone,
{
    todo!()
}

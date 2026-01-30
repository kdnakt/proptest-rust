use proptest::{prelude::TestCaseError, prop_assert_eq};

use std::{fmt::Debug, io::{Cursor, Seek, SeekFrom}};
use crate::readable_writable::{Readable, Writable};

#[cfg(test)]

pub(crate) mod proptest_strategies;

pub(crate) fn test_serde<T>(data: &T) -> Result<(), TestCaseError>
where
    T: Readable + Writable + Debug + PartialEq + Clone,
{
    let mut cur = Cursor::new(Vec::<u8>::new());
    data.write(&mut cur).unwrap();

    cur.seek(SeekFrom::Start(0)).unwrap();
    let data_read = T::read(&mut cur).unwrap();
    prop_assert_eq!(data_read, data.clone());
    Ok(())
}

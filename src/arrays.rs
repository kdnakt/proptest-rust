use std::io::{Read, Result};

use crate::readable_writable::Readable;

#[inline]
pub(crate) fn read_nullable_array<T>(
    input: &mut impl Read,
    field_name: &str,
    compact: bool,
) -> Result<Option<Vec<T>>>
where
    T: Readable,
{
    // Placeholder implementation
    Ok(None)
}

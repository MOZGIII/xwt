//! Implementations related to datagrams.

use crate::types::*;

impl<T> AsRef<[u8]> for Datagram<T>
where
    T: AsRef<[u8]>,
{
    fn as_ref(&self) -> &[u8] {
        T::as_ref(&self.0)
    }
}

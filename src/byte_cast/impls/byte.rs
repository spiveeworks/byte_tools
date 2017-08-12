use byte_cast::{AsBytes};

use std::iter;


impl AsBytes for u8
{
    type Iter = iter::Once<u8>;
    fn from_bytes<I: Iterator<Item=u8>> (stream: &mut I) -> Option<u8>
      { stream.next() }
    fn into_bytes (self) -> Self::Iter
      { iter::once(self) }
}

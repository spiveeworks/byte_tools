use stream_container::{StreamContainer};

use std::iter;


impl<T> StreamContainer<T> for T
{
    type Iter = iter::Once<T>;
    fn fill_with<I: Iterator<Item=T>> (stream: &mut I) -> T
      { stream.next() }
    fn into_stream (self) -> Self::Iter
      { iter::once(self) }
}

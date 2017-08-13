use byte_cast::{AsBytes};

use std::vec;
use std::iter;


macro_rules! dummy
{
    ($x:expr) => ();
}

// replace each term after the semicolon with a copy of the _expression_ before the semicolon
macro_rules! array_dummy_map
{
    [$x: expr; $($ns:tt),*,] => {array_dummy_map![$x; $($ns),*]};
    [$x: expr; $($ns:tt),*] => {[$({dummy!($ns); $x}),*]};
}


macro_rules! impl_array_as_bytes
{
    {$n:expr} =>
    {
        impl_array_as_bytes!{$n,}
    };
    {$n:expr, $($ns:expr),*} =>
    {
        impl<T: AsBytes> AsBytes for [T; $n]
        {
            type Iter = iter::FlatMap<vec::IntoIter<T>, T::Iter, fn(T) -> T::Iter>;
            //type Iter = vec::IntoIter<u8>; // this needs to be FlatMap<IntoIter, T::Iter, _something>
            #[allow(unused_variables)] // stream is unused in the [T; 0] case, but this is fine
            fn from_bytes<I: Iterator<Item = u8>> (stream: &mut I) -> Option<Self>
            {
                Some(array_dummy_map![
                     try_from_bytes!(stream);
                     $($ns),*])
            }
            fn into_bytes(self) -> Self::Iter
            {
                let box_self: Box<[T]> = Box::new(self);
                box_self.into_vec()
                        .into_iter()
                        .flat_map(AsBytes::into_bytes)
            }
        }

        impl_array_as_bytes!{$($ns),*}
    };

    {} => {}
}

impl_array_as_bytes!
{
                                32, 31, 30,
    29, 28, 27, 26, 25, 24, 23, 22, 21, 20,
    19, 18, 17, 16, 15, 14, 13, 12, 11, 10,
     9,  8,  7,  6,  5,  4,  3,  2,  1,  0
}

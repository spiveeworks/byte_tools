
use std::marker;


trait AsBytes
{
    type Iter: Iterator<Item = u8>;
    fn from_bytes<I: Iterator<Item = u8>>(bytes: &mut I) -> Option<Self>
        where Self: marker::Sized;
    fn into_bytes(self) -> Self::Iter;
}


trait AsBytesIntermediate
{
    type Base: AsBytes;
    fn from_base(Self::Base) -> Self;
    fn into_base(self) -> Self::Base;
}

macro_rules! compose_bytes_methods
// this is encapsulated separately in case implementors wish to define extra constraints or blanket implementations
{
    ($Self: ty) =>
    {
        type Iter = <<$Self as AsBytesIntermediate>::Base as AsBytes>::Iter;
        fn from_bytes<I: Iterator<Item = u8>>(stream: &mut I) -> Option<$Self>
        {
            let result_base = <<$Self as AsBytesIntermediate>::Base as AsBytes>::from_bytes(stream);
            result_base.map(|base| <$Self as AsBytesIntermediate>::from_base(base))
        }
        fn into_bytes(self: $Self) -> <$Self as AsBytes>::Iter
        {
            let base = <$Self as AsBytesIntermediate>::into_base(self);
            <<$Self as AsBytesIntermediate>::Base as AsBytes>::into_bytes(base)
        }
    }
}

macro_rules! compose_bytes_traits
{
    ($C: ty) =>
    {
        impl AsBytes for $C
            where $C: AsBytesIntermediate
        {
            compose_bytes_methods!(Self);
        }
    }
}


#[macro_use]
mod tests;



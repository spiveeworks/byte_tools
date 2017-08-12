use byte_cast::{AsBytes, AsBytesIntermediate};

use std::iter;


impl<A, B> AsBytes for (A, B)
    where A: AsBytes,
          B: AsBytes,
{
    type Iter = iter::Chain<<A as AsBytes>::Iter, <B as AsBytes>::Iter>;
    fn from_bytes<I: Iterator<Item=u8>> (stream: &mut I) -> Option<Self>
    {
        let a = try_from_bytes!(stream);
        let b = try_from_bytes!(stream);
        Some((a, b))
    }
    fn into_bytes(self) -> Self::Iter
    {
        self.0.into_bytes().chain(self.1.into_bytes())
    }
}

/*
impl AsBytes for ()
{
    type Iter = iter::Empty<u8>;
    fn fill_with<I> (_stream: &mut I) -> Option<Self>
      { Some(()) }
    fn into_stream(self) -> Self::Iter
      { iter::empty() }
}


impl<T> AsBytesIntermediate for (T,)
    where T: AsBytes
{
    type Base = T;
    fn into_base(self) -> T
      { self.0 }
    fn from_base(base: T) -> Self
      { (base,) }
}
*/

macro_rules! impl_tuple_as_bytes
{
    {
        $N1: ident : $T1: ident,
        $N2: ident : $T2: ident,
        $($Ns: ident : $Ts: ident),+
    } =>
    {
        impl<$T1, $T2, $($Ts),+> AsBytesIntermediate for ($T1, $T2, $($Ts),+)
            where $T1: AsBytes,
                  $T2: AsBytes,
                  $($Ts: AsBytes),+
        {
            type Base = (($T1, $T2), $($Ts),+);
            fn from_base(base: Self::Base) -> Self
            {
                let (($N1, $N2), $($Ns),+) = base;
                ($N1, $N2, $($Ns),+)
            }
            fn into_base(self) -> Self::Base
            {
                let ($N1, $N2, $($Ns),+) = self;
                (($N1, $N2), $($Ns),+)
            }
        }

        impl<$T1, $T2, $($Ts),+> AsBytes for ($T1, $T2, $($Ts),+)
            where Self: AsBytesIntermediate // which implies the constraints defined above
        {
            // use the internal macro since we need to use different template args + restrictions
            compose_bytes_methods!(Self);
        }

        impl_tuple_as_bytes!
        {
            $N2: $T2,
            $($Ns: $Ts),+
        }
    };

    {
        $N1: ident : $T1: ident,
        $N2: ident : $T2: ident
    } => 
    {
        // pair has its own implementation
    };

}

impl_tuple_as_bytes!
{
    a: A,
    b: B,
    c: C
}




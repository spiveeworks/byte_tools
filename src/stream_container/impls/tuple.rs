use stream_container::{StreamContainer, StreamCast};

use std::iter;


impl<T, A, B> StreamContainer<T> for (A, B)
    where A: StreamContainer<T>,
          B: StreamContainer<T>,
{
    type Iter = iter::Chain<<A as StreamContainer<T>>::Iter, <B as StreamContainer<T>>::Iter>;
    fn fill_with<I: Iterator<Item=T>> (stream: &mut I) -> Option<Self>
    {
        if let Some(a) = StreamContainer::<T>::fill_with(stream)
        {
            if let Some(b) = StreamContainer::<T>::fill_with(stream)
              { Some((a, b)) }
            else
              { None }
        }
        else
          { None }
    }
    fn into_stream(self) -> Self::Iter
    {
        self.0.into_stream().chain(self.1.into_stream())
    }
}

/*
impl<T> StreamContainer<T> for ()
{
    type Iter = iter::Empty<T>;
    fn fill_with<I> (_stream: &mut I) -> Option<Self>
      { Some(()) }
    fn into_stream(self) -> Self::Iter
      { iter::empty() }
}
*/

impl<T> StreamCast<T> for (T,)
    where T: StreamContainer<T> // implemented by default elsewhere
{
    type Base = T;
    fn into_base(self) -> T
      { self.0 }
    fn from_base(base: T) -> Self
      { (base,) }
}

macro_rules! tuple_impl_stream_container
{
    {
        $N1: ident : $T1: ident,
        $N2: ident : $T2: ident,
        $($Ns: ident : $Ts: ident),+
    } =>
    {
        impl<T, $T1, $T2, $($Ts),+> StreamCast<T> for ($T1, $T2, $($Ts),+)
            where $T1: StreamContainer<T>,
                  $T2: StreamContainer<T>,
                  $($Ts: StreamContainer<T>),+
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

        impl<T, $T1, $T2, $($Ts),+> StreamContainer<T> for ($T1, $T2, $($Ts),+)
            where Self: StreamCast<T> // offload the actual requirements to StreamCast
        {
            // use the internal macro since we need to use different template args + restrictions
            container_by_cast_items!(Self, T);
        }

        tuple_impl_stream_container!
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

tuple_impl_stream_container!
{
    a: A,
    b: B,
    c: C
}




#![allow(dead_code)]





#[macro_use]
mod impls
{
    
    macro_rules! try_option 
    {
        {$x:expr} => 
        {
            match $x
            {
                Some(x) => x,
                None => return None,
            }
        };
    }
    #[macro_use]
    mod array;
    #[macro_use]
    mod tuple;

    // implements StreamContainer<T> for T
    mod single;
}

#[macro_use]
mod tests;



/*
unsafe impl TransmuteStream<u8> for Test
{
    type UseBase = [u8;5];
}

unsafe trait TransmuteStream<T> 
  where Self: Sized,
        T: Sized,
{
    type UseBase: StreamContainer<T>;
}
impl<C,T> StreamCast<T> for C
    where C: TransmuteStream<T>
{
    type Base = C::UseBase;
    fn into_base(self) -> Self::Base
      {unsafe{std::mem::transmute::<Self,Self::Base>(self)}}
    fn from_base(base: Self::Base) -> Self
      {unsafe{std::mem::transmute::<Self::Base,Self>(base)}}
}
*/





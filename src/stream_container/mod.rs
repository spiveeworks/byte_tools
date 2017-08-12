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



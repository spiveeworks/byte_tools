#![cfg(test)]

use super::*;

use std::mem;


#[derive(PartialEq, Eq, Clone, Default, Debug)]
struct Test(u8, u32);


impl StreamCast<u8> for Test
{
    type Base = (u8, [u8; 4]);
    fn into_base(self) -> Self::Base
    {
        let Test(x, y) = self;
        let ys = unsafe
        {
            mem::transmute::<u32,[u8; 4]>(y)
        };
        (x,ys)
    }
    fn from_base(base: Self::Base) -> Self
    {
        let (x, ys) = base;
        let y = unsafe
        {
            mem::transmute::<[u8;4],u32>(ys)
        };
        Test(x, y)
    }
}

container_by_cast!(Test);

#[test]
fn struct_streaming() 
{
    const X: u8 = 5;
    const Y: u32 = 1000000;
    let a = Test(X,Y);
    let b = a.clone();

    let mut a_bytes = a.into_stream();
    let maybe_a = StreamContainer::<u8>::fill_with(&mut a_bytes);
    let a: Test = maybe_a.expect("Ran out of bytes when reconstructing Test");

    assert_eq!(a, b);
}



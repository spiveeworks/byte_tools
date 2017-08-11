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

#[test]
fn tuple_streaming()
{
    const TERMS: [u8; 5] = [5,6,7,9,2];

    let arr = TERMS;
    let arr_as_stream = arr.into_stream();
    let maybe_arr_as_tup = StreamContainer::<u8>::fill_with(arr_as_stream);
    let arr_as_tup = maybe_arr_as_tup.expect("Ran out of bytes when constructing tuple");
    assert_eq!(arr_as_tup, (TERMS[0], TERMS[1], TERMS[2], TERMS[3], TERMS[4]));

    let tup = arr_as_tup;
    let tup_as_stream = tup.into_stream();
    let maybe_tup_as_arr = StreamContainer::<u8>::fill_with(tup_as_stream);
    let tup_as_arr = maybe_tup_as_arr.expect("Ran out of bytes when constructing array");
    assert_eq!(tup_as_arr, TERMS);
}


#![cfg(test)]

use super::*;

use std::mem;
use std::vec;


#[derive(PartialEq, Eq, Clone, Default, Debug)]
struct Test(u8, u32);

impl AsBytes for [u8; 4]
{
    type Iter = vec::IntoIter<u8>;
    fn from_bytes<I: Iterator<Item = u8>>(stream: &mut I) -> Option<Self>
    {
        let mut result = [0; 4];
        for i in 0..4
        {
            result[i] = try_from_bytes!(stream);
        }
        Some(result)
    }
    fn into_bytes(self) -> Self::Iter
    {
        let mut result = Vec::with_capacity(4);
        result.extend_from_slice(&self);
        result.into_iter()
    }
}

impl AsBytesIntermediate for Test
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

compose_bytes_traits!(Test);

#[test]
fn struct_streaming()
{
    const X: u8 = 5;
    const Y: u32 = 1000000;
    let a = Test(X,Y);
    let b = a.clone();

    let mut a_bytes = a.into_bytes();
    let maybe_a = AsBytes::from_bytes(&mut a_bytes);
    let a: Test = maybe_a.expect("Ran out of bytes when reconstructing Test");

    assert_eq!(a, b);
}

#[test]
fn tuple_streaming()
{
    const TERMS: [u8; 4] = [5,6,7,9];


    let arr = TERMS;
    let mut arr_as_stream = arr.into_bytes();
    let maybe_arr_as_tup = AsBytes::from_bytes(&mut arr_as_stream);

    let arr_as_tup: (u8, u8, u8, u8);
    arr_as_tup = maybe_arr_as_tup.expect("Ran out of bytes when constructing tuple");
    assert!(arr_as_stream.next() == None, "Excess bytes when constructing array");
    assert_eq!(arr_as_tup, (TERMS[0], TERMS[1], TERMS[2], TERMS[3]));


    let tup = arr_as_tup;
    let mut tup_as_stream = tup.into_bytes();
    let maybe_tup_as_arr = AsBytes::from_bytes(&mut tup_as_stream);

    let tup_as_arr: [u8; 4];
    tup_as_arr = maybe_tup_as_arr.expect("Ran out of bytes when constructing array");
    assert!(tup_as_stream.next() == None, "Excess bytes when constructing array");
    assert_eq!(tup_as_arr, TERMS);
}


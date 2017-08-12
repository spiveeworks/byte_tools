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



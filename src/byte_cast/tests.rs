#![cfg(test)]

use super::*;

use std::mem;
use std::vec;


#[derive(PartialEq, Eq, Clone, Default, Debug)]
struct Test(u8, u32);

struct TestBase(u8, [u8; 4]);

impl AsBytes for TestBase
{
    type Iter = vec::IntoIter<u8>;
    fn from_bytes<I: Iterator<Item = u8>>(stream: &mut I) -> Option<Self>
    {
        if let Some(head) = stream.next()
        {
            let mut tail = [0; 4];
            for i in 0..4
            {
                if let Some(x) = stream.next()
                {
                    tail[i] = x;
                }
                else
                {
                    return None;
                }
            }
            Some(TestBase(head, tail))
        }
        else
        {
            None
        }
    }
    fn into_bytes(self) -> Self::Iter
    {
        let mut result = Vec::with_capacity(5);
        result.push(self.0);
        for &x in &self.1
        {
            result.push(x);
        }

        result.into_iter()
    }
}


impl AsBytesIntermediate for Test
{
    type Base = TestBase;
    fn into_base(self) -> Self::Base
    {
        let Test(x, y) = self;
        let ys = unsafe
        {
            mem::transmute::<u32,[u8; 4]>(y)
        };
        TestBase(x,ys)
    }
    fn from_base(base: Self::Base) -> Self
    {
        let TestBase(x, ys) = base;
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



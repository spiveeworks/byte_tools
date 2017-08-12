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


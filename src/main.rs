use std::cmp;
use std::iter::FromIterator;

use roaring::RoaringBitmap;

mod array;

fn roaring_serialize_test(integers: &[u32]) -> bool {
    let bitmap = RoaringBitmap::from_iter(integers.iter().cloned());

    let mut vec = Vec::with_capacity(bitmap.serialized_size());
    bitmap.serialize_into(&mut vec).unwrap();
    let deser_bitmap = RoaringBitmap::deserialize_from(&vec[..]).unwrap();

    bitmap == deser_bitmap
}

// https://www.fuzzingbook.org/html/Reducer.html#Delta-Debugging
fn main() {
    let mut input = array::ARRAY.to_vec();

    assert!(!roaring_serialize_test(&input));

    let mut n = 2; // initial granularity
    while input.len() >= 2 {
        let mut start = 0;
        let subset_length = input.len() / n;
        let mut some_complement_is_failing = false;

        while start + subset_length < input.len() {
            // complement generation
            let left = &input[..start];
            let right = &input[start + subset_length..];
            let mut complement = Vec::with_capacity(left.len() + right.len());
            complement.extend_from_slice(left);
            complement.extend_from_slice(right);

            if !roaring_serialize_test(&complement) {
                input = complement;
                n = cmp::max(n - 1, 2);
                some_complement_is_failing = true;
                break;
            }

            start += subset_length;
        }

        if !some_complement_is_failing {
            if n == input.len() {
                break;
            }
            n = cmp::min(n * 2, input.len());
        }
    }

    println!("reduced input is {:?}", input);
}

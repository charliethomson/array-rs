extern crate better_array;
use std::i32;
use better_array::array::Array;
use std::io::Result as IOResult;

fn main() -> IOResult<()> {
    
    let mut array: Array<i32> = Array::new(64)?;

    array.fill(i32::MAX);

    for (idx, item) in array.clone().into_iter().enumerate() {
        if item == i32::MAX {
            array.pop(idx);
        }
    }

    drop(array);

    let mut array: Array<usize> = Array::new(64)?;

    for idx in 0..array.cap() {
        array.set(idx, 10usize);
    }

    assert_eq!(array.count(10), array.cap());
    
    
    Ok(())

}

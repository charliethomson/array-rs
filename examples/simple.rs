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
    let mut array: Array<usize> = Array::from(0..60);

    for idx in 0..array.cap() {
//        array.set(idx, 10usize);
        eprintln!("arr[{}] = {}", idx, array.get(idx)?);
    }

    for item in array.into_iter() {
        eprintln!("item: {}", item);
    }

    eprintln!("{} -> {}", array.count(10), array.cap());
    
    
    Ok(())

}

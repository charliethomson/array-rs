
use better_array::prelude::*;
use std::{
    ptr::NonNull,
};

#[test]
fn alloc_test() -> Result<(), ArrayError> {
    let addr: NonNull<u8> = alloc::<u8>(32)?;
    unsafe {
        for offset in 0..128 {
            let a = addr.as_ptr().add(offset);
            // eprintln!("{:?} (offset {:x}) -> {}", a, offset, *a)
        }
    }

    Ok(())
}

#[test]
fn array_test() -> Result<(), ArrayError> {
    use std::ptr::copy;

    let mut copy_to = Array::<u8>::new(32)?;

    let mut arr = Array::<u8>::new(32)?;
    arr.fill(0x38);

    eprintln!("{:?} -> {:?}", arr, arr.clone());


    Ok(())
}

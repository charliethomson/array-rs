use std::{
    alloc::{
        alloc_zeroed, Layout
    },
    mem::{size_of, align_of},
    io::{
        Result as IOResult,
        Error, ErrorKind,
    },
    ptr::NonNull,
};

/// Allocate space for an array [T; `len`]
///
/// Error states: 
/// * if `len` is 0
/// * `len * size_of::<T>()` overflows
/// 
/// Returns a non-null pointer to the beginning of the Array
pub fn alloc<T>(len: usize) -> IOResult<NonNull<T>> {
    let size = match size_of::<T>().checked_mul(len) {
        Some(0) => return Err(Error::new(ErrorKind::Other, "Cannot allocate zero sized value")),
        Some(n) => n,
        None => return Err(Error::new(ErrorKind::Other, "Overflow when getting layout size")),
    };
    let align = align_of::<T>();
    let layout = match Layout::from_size_align(size, align) {
        Ok(n) => n,
        Err(_) => return Err(Error::new(ErrorKind::Other, format!("Failed to create layout from (size: {}, align: {})", size, align)))
    };
    // alloc_zeroed's behaviour is only undefined when trying to
    // allocate zero sized values
    unsafe {
        let ptr = alloc_zeroed(layout) as *mut T;
        if let Some(p) = NonNull::new(ptr) {
            return Ok(p);
        } else {
            return Err(Error::new(ErrorKind::Other, "Failed to allocate memory for the Array"));
        }
    }
}
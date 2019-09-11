use std::{
    fmt::{ Debug, Display, Formatter, Result as FmtResult },
    io::{ Error, ErrorKind, Result as IOResult },
    cmp::Ordering,
    ptr::{ NonNull, write, read },
};

/// Safe, generic implementation of an Array
pub struct Array<T> {
    ptr: NonNull<T>,
    cap: usize,
}

// Private Methods
impl<T> Array<T> {

    fn allocate(size: usize) -> IOResult<NonNull<T>> {
        use crate::alloc::alloc;
        alloc::<T>(size)
    }

    /// copy self.cap items from src to self.ptr    
    unsafe fn copy_from(&self, src: NonNull<T>) {
        use std::ptr::copy_nonoverlapping;

        copy_nonoverlapping(self.as_ptr(), src.as_ptr(), self.cap);
    }
}

// Public Methods
impl<T> Array<T> {
    
    /// Create a new Array of size `size`
    /// 
    /// Error states:
    ///  * see [`::alloc::alloc`]: ../alloc/fn.alloc.html
    pub fn new(size: usize) -> IOResult<Self> {
        Ok(Array {
            ptr: Self::allocate(size)?,
            cap: size,
        })
    }

    /// Create a new Array, ignoring checks
    pub unsafe fn new_unchecked(size: usize) -> Self {
        Self::new(size).unwrap()
    }
    
    /// Create an `Array<T>` from a raw block of memory
    /// 
    /// # Safety
    /// This is unsafe because it relies on the user to give the correct pointer and length
    pub unsafe fn from_raw(ptr: NonNull<T>, len: usize) -> IOResult<Self> {
        let arr = Array::new(len)?;
        arr.copy_from(ptr);
        Ok(arr)
    }

    /// Fills `self` with `with`
    pub fn fill(&self, with: T) where T: Copy {
        for offs in 0..self.cap {
            unsafe {
                // This write should always be safe ( ptr + offs should never leave the allocated space )
                write(self.as_ptr().add(offs), with);
            }
        }
    }

    /// Get the value at `idx`
    /// 
    /// Error states:
    ///  * `idx` is greater than the length of the array
    pub fn get(&self, idx: usize) -> IOResult<T> {
        if idx > self.cap {
            return Err(Error::new(ErrorKind::Other, "index out of range"));
        } else {
            unsafe {
                // This is safe because of (^)
                Ok(read(self.as_ptr().add(idx)))
            }
        }
    }

    /// Set the value at `idx` to `val`
    /// 
    /// Error states:
    ///  * `idx` is greater than the length of the array
    pub fn set(&self, idx: usize, val: T) -> IOResult<()> {
        if idx > self.cap {
            return Err(Error::new(ErrorKind::Other, "index out of range"));
        } else {
            unsafe {
                write(self.as_ptr().add(idx), val);
            }
        }
        Ok(())        
    }

    /// Delete and return the value at `idx`
    /// 
    /// Error states:
    ///  * `idx` is greater than the length of the array
    pub fn pop(&self, idx: usize) -> IOResult<T> {
        if idx > self.cap {
            return Err(Error::new(ErrorKind::Other, "index out of range"));
        } else {
            unsafe {
                let addr = self.as_ptr().add(idx);
                let val: T = read(addr);
                addr.write_bytes(0, 1);
                Ok(val)
            }
        }
    }

    /// Get a pointer to the `Array<T>`
    /// 
    /// Guaranteed to be non-null
    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Return the amount of times `val` appears in the `Array`
    pub fn count(&self, val: T) -> usize
    where T: Clone + PartialEq
    {
        let v: Vec<T> = self.clone().into_iter().filter(|v| v == &val).collect();
        v.len() 
    }

    pub fn cap(&self) -> usize { self.cap }
}

// Trait implemetations

// impl<T: Display> Display for Array<T> {
//     fn fmt(&self, f: &mut Formatter) -> FmtResult {
//         write!(f,)
//     }
// }


impl<T: PartialEq + Clone> PartialEq for Array<T> {
    fn eq(&self, other: &Self) -> bool {
        let items = self.clone().into_iter().zip(other.clone().into_iter());
        for (s, o) in items {
            if s != o { return false }
        }
        true
    }
}

impl<T, U> From<U> for Array<T>
where U: Iterator<Item=T>
{
    fn from(i: U) -> Self {
        let v: Vec<T> = i.collect();
        let arr: Array<T> = Self::new(v.len()).expect("");
        let vec_ptr: NonNull<T> = NonNull::new(v.as_ptr() as *mut T).expect("Vec.as_ptr() returned a null pointer");
        unsafe {
            // This should be safe, assuming the Vec allocated correctly
            // arr.cap == vec.len() && !vec_ptr.is_null()
            arr.copy_from(vec_ptr);
        }
        arr
    }
}

impl<T: Clone> Clone for Array<T> {
    fn clone(&self) -> Self {
        // Create a new array
        let arr = match Self::new(self.cap) {
            // Get the array
            Ok(a) => a,
            Err(e) => {
                if let Some(msg) = e.into_inner() { panic!("Encountered an error when cloning -> {}", msg); }
                // V This V means that there was no message given, pls investigate
                else { panic!("Unknown error encountered! Code: 00"); }
            }
        };
        unsafe {
            // copies `arr.cap` values from `self.ptr`
            arr.copy_from(self.ptr);
        }
        arr
    }
}

impl<T: Copy + Clone> Copy for Array<T> {}

impl<T> IntoIterator for Array<T> {
    type IntoIter = ArrayIter<T>;
    type Item = T;

    fn into_iter(self) -> ArrayIter<T> {
        ArrayIter::new(self)
    }
}

/// Iterator for `Array<T>`
pub struct ArrayIter<T> {
    arr: Array<T>,
    idx: usize
} impl<T> ArrayIter<T> {
    /// Create a new `ArrayIter<T>` (consumes `arr`)
    fn new(arr: Array<T>) -> Self {
        ArrayIter { arr, idx: 0 }
    }
} impl<T> Iterator for ArrayIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.arr.cap {
            None
        } else {
            self.idx += 1;
            // This unwrap is guaranteed safe (idx will be less than cap)
            Some(self.arr.get(self.idx - 1).unwrap())
        }
    }
}
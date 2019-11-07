
use {
    crate::{
        error::ArrayError,
    },
    std::{
        fmt::{
            Debug, Display,
            Formatter, Result as FmtResult
        },
        cmp::Ordering,
        ptr::{ NonNull, write, read },
        alloc::{
            alloc_zeroed, Layout
        },
        mem::{size_of, align_of},
    },
};


/// Allocate space for an array [T; `len`]
///
/// Error states: 
/// * if `len` is 0
/// * `len * size_of::<T>()` overflows
/// 
/// Returns a non-null pointer to the beginning of the Array
pub fn alloc<T>(len: usize) -> Result<NonNull<T>, ArrayError> {
    // Get the size of the allocation
    let size = match size_of::<T>().checked_mul(len) {
        Some(0) => return Err(ArrayError::new("Cannot allocate zero sized value")),
        Some(n) => n,
        None => return Err(ArrayError::new("Overflow when getting layout size")),
    };
    // Get the align of T
    let align = align_of::<T>();
    let layout = match Layout::from_size_align(size, align) {
        Ok(n) => n,
        Err(_) => return Err(ArrayError::new(format!("Failed to create layout from (size: {}, align: {})", size, align)))
    };
    // alloc_zeroed's behaviour is only undefined when trying to
    // allocate zero sized values
    unsafe {
        let ptr = alloc_zeroed(layout) as *mut T;
        // Create a NonNull<T> from a *mut T
        // Will fail if the *mut T is null somehow
        if let Some(p) = NonNull::new(ptr) {
            return Ok(p);
        } else {
            return Err(ArrayError::new("Failed to allocate memory for the Array"));
        }
    }
}

/// Safe, generic implementation of an Array
pub struct Array<T> {
    ptr: NonNull<T>,
    cap: usize,
}

// Private Methods
impl<T> Array<T> {

    fn allocate(size: usize) -> Result<NonNull<T>, ArrayError> {
        alloc::<T>(size)
    }

    /// copy self.cap items from src to self.ptr    
    unsafe fn copy_from(&self, src: NonNull<T>) {
        use std::ptr::copy;

        copy(src.as_ptr(), self.as_ptr(), self.cap);
    }

    fn in_bounds(&self, idx: usize) -> Option<ArrayError> {
        if idx > self.cap {
            return Some(ArrayError::new("index out of range"));
        }

        None
    }
}

// Public Methods
impl<T> Array<T> {
    
    /// Create a new Array of size `size`
    /// 
    /// Error states:
    ///  * see [`::alloc::alloc`]: ../alloc/fn.alloc.html
    pub fn new(size: usize) -> Result<Self, ArrayError> {
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
    pub unsafe fn from_raw(ptr: NonNull<T>, len: usize) -> Result<Self, ArrayError> {
        let arr = Array::new(len)?;
        arr.copy_from(ptr);
        Ok(arr)
    }

    /// Fills `self` with `with`
    pub fn fill(&mut self, with: T) where T: Copy {
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
    pub fn get(&self, idx: usize) -> Result<T, ArrayError> {
        if let Some(err) = self.in_bounds(idx) {
            return Err(err);
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
    pub fn set(&mut self, idx: usize, val: T) -> Result<(), ArrayError> {
        if let Some(err) = self.in_bounds(idx) {
            return Err(err);
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
    pub fn pop(&mut self, idx: usize) -> Result<T, ArrayError> {
        if let Some(err) = self.in_bounds(idx) {
            return Err(err);
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
    where T: Clone + PartialEq + Display
    {
        self.clone().into_iter().filter(|v| v == &val).collect::<Vec<T>>().len() 
    }

    pub fn cap(&self) -> usize { self.cap }
}

// Trait implemetations

impl<T: Debug + Copy> Debug for Array<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", &self.clone().into_iter().collect::<Vec<T>>()[..])
    }
}

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
                panic!("Encountered an error when cloning -> {}", e.msg())
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
    fn new(arr: Array<T>) -> Self  {
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
pub mod array;
pub mod error;

pub mod prelude {
    pub use crate::{
        array::{ Array, ArrayIter, alloc },
        error::{ ArrayError },
    };
}

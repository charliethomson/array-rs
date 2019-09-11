pub mod alloc;
pub mod array;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use std::iter::FromIterator;
        let a = Vec::from_iter(0..10);
        let b = Vec::from_iter(0..8);
        for (x, y) in a.into_iter().zip(b.into_iter()) {
            eprintln!("x: {}, y: {}", x, y);
        }
        assert_eq!(2 + 2, 4);
    }
}

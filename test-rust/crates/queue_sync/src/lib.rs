pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(all(test, feature="with-bench"))]
mod bench {
    use super::*;

    #[test]
    fn it_works(bh: & mut test::Bencher) {
        bh.iter( || {
            add(2, 2);
        });
    }
}

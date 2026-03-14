pub fn st_iterator_main() {
    let array = &[1, 2, 3];
    let iterator = array.iter();

    iterator.for_each(|e| print!("{}, ", e));
}
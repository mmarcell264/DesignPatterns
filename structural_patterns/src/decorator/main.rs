use std::io::{BufReader, Cursor, Read};

pub fn decorator_main() {
    let mut buf = [0u8; 10];

    // A buffered reader decorates a vector reader which wraps input data.
    let mut input = BufReader::new(Cursor::new("Input data"));

    input.read(&mut buf).ok();

    for byte in buf {
        print!("{}", char::from(byte));
    }

    println!();
}

use anerror_impl::AnError;

#[derive(AnError)]
enum FooError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

// fn get_line_int() -> Result<u32, FooError> {
//     let mut buf = String::new();
//     std::io::stdin().read_line(&mut buf)?;
//
//     Ok(buf.trim().parse()?)
// }

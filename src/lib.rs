enum FooError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

// impl From<std::io::Error> for FooError {
//     fn from(value: std::io::Error) -> Self {
//         Self::Io(value)
//     }
// }
//
// impl From<std::num::ParseIntError> for FooError {
//     fn from(value: std::num::ParseIntError) -> Self {
//         Self::Parse(value)
//     }
// }

fn get_line_int() -> Result<u32, FooError> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    Ok(buf.trim().parse()?)
}

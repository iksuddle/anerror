use anerror_impl::AnError;

#[derive(AnError)]
enum FooError {
    IO(std::io::Error),
    Parse(std::num::ParseIntError),
}

pub fn do_something() -> Result<(), FooError> {
    let num: u32 = "123".parse()?;
    println!("{num}");

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    Ok(())
}

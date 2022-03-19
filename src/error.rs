#[macro_export]
macro_rules! runtime_error {
    () => ({
        ::std::eprintln!("runtime error");
        ::std::process::exit(1)
    });
    ($($arg:tt)*) => ({
        ::std::eprintln!($($arg)*);
        ::std::process::exit(1)
    })
}

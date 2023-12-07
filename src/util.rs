use std::io;

pub fn not_implemented_error(day: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("{} not implemented", day))
}

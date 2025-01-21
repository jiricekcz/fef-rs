//! Set of functions to work with [std::io::Bytes].

use std::io::Bytes;

/// Reads a fixed number of bytes from the given reader.
///
/// # Examples
/// ```rust
/// # use std::io::Read;
/// # use fef::raw::bytes::read_exact;
/// # use std::io::Bytes;
/// # fn main() -> Result<(), std::io::Error> {
/// let file = vec![0x01u8, 0x02u8, 0x03u8];
/// let mut bytes = file.bytes();
///
/// let value = read_exact::<2, _>(&mut bytes)?;
/// let int_value = u16::from_be_bytes(value);
/// assert_eq!(int_value, 0x0102);
/// # Ok(())
/// # }
pub fn read_exact<const N: usize, R: std::io::Read>(
    bytes: &mut Bytes<R>,
) -> Result<[u8; N], std::io::Error> {
    let mut buffer = [0; N];

    for i in 0..N {
        buffer[i] = match bytes.next() {
            Some(byte) => byte?,
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "unexpected end of stream",
                ))
            }
        };
    }
    Ok(buffer)
}

/// Reads a dynamic number of bytes from the given reader.
///
/// # Examples
/// ```rust
/// # use std::io::Read;
/// # use fef::raw::bytes::read_dynamic;
/// # use std::io::Bytes;
/// # fn main() -> Result<(), std::io::Error> {
/// let file = vec![0x01u8, 0x02u8, 0x03u8];
/// let mut bytes = file.bytes();
///
/// let value = read_dynamic(&mut bytes, 2)?;
/// assert_eq!(value, vec![0x01u8, 0x02u8]);
/// # Ok(())
/// # }
pub fn read_dynamic<R: std::io::Read>(
    bytes: &mut Bytes<R>,
    length: usize,
) -> Result<Vec<u8>, std::io::Error> {
    let mut buffer = Vec::with_capacity(length);

    for _ in 0..length {
        buffer.push(match bytes.next() {
            Some(byte) => byte?,
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "unexpected end of stream",
                ))
            }
        });
    }
    Ok(buffer)
}

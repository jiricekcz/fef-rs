use std::{cmp::Ordering, io::Bytes};

/// Represents a variable length enum in the FEF specification.
///
/// Holds an unsigned integer of arbitrary size. Implementation of this type is not stabilized.
///
/// # Examples
/// ```rust
/// # use fef::raw::VariableLengthEnum;
/// fn get_variable_length_enum() -> VariableLengthEnum {
///     /* Some code providing a variable length enum */
/// #   VariableLengthEnum::from(42)
/// }
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let variable_length_enum = get_variable_length_enum();
/// let value: usize = variable_length_enum.try_into()?;
/// if value == 42 {
///     println!("The value is 42!");
/// #   assert!(true);
/// } else {
///     println!("The value is not 42, it is {}", value);
/// #   assert!(false);
/// }
///
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VariableLengthEnum {
    // This is not public because settling on an implementation may be dangerous, since we don't know what the future FEF versions will use the enum for.
    // Instead we will encourage users to use try_from and into to convert to and from this type.
    // Most likely, these conversions will be rare, so them not being ergonomic is not a big issue.
    value: VariableLengthEnumStorage,
}

/// Stores value of a variable length enum.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum VariableLengthEnumStorage {
    /// This variant is selected when the value fits into a u64
    U64(u64),
    /// If it doesn't fit into a u64, it is stored as a Vec<u8> according to the FEF specification without leading `0x80` bytes.
    Overflow(Vec<u8>),
}

impl PartialOrd for VariableLengthEnumStorage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VariableLengthEnumStorage {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            VariableLengthEnumStorage::U64(self_u64) => match other {
                VariableLengthEnumStorage::U64(other_u64) => self_u64.cmp(other_u64), // Both fit into u64, compare them
                VariableLengthEnumStorage::Overflow(_) => Ordering::Less, // self fits into u64, other doesn't, self is less
            },
            VariableLengthEnumStorage::Overflow(self_overflow) => match other {
                VariableLengthEnumStorage::U64(_) => Ordering::Greater, // self doesn't fit into u64, other does, self is greater
                VariableLengthEnumStorage::Overflow(other_overflow) => {
                    // Both don't fit into u64, compare their lengths first
                    let len_cmp = self_overflow.len().cmp(&other_overflow.len());
                    if len_cmp != Ordering::Equal {
                        return len_cmp;
                    }
                    // If lengths are equal, compare the bytes
                    self_overflow.iter().rev().cmp(other_overflow.iter().rev())
                }
            },
        }
    }
}

/// Creating a variable length enum from a usize.
///
/// # Examples
/// ```rust
/// # use fef::raw::VariableLengthEnum;
/// let selector = 42;
///
/// let variable_length_enum = VariableLengthEnum::from(selector);
/// ```
///
impl From<usize> for VariableLengthEnum {
    fn from(value: usize) -> Self {
        VariableLengthEnum {
            value: VariableLengthEnumStorage::U64(value as u64),
        }
    }
}

/// Reading a variable length enum from a byte stream.
///
/// This reads from a bytes reader and interprets the bytes as a variable length enum.
/// It ends when it finished reading a variable length enum.
/// It returns an error if it encounters an unexpected end of the byte stream or other I/O errors.
///
/// # Examples
///
/// Simple reading of a small variable length enum:
/// ```rust
/// # use fef::raw::VariableLengthEnum;
/// # use std::io::Read;
/// # fn main() -> Result<(), std::io::Error> {
/// let file: Vec<u8> = vec![0x81, 0x80, 0x00, 0x12];
/// let mut bytes = file.bytes();
///
/// let variable_length_enum = VariableLengthEnum::try_from(&mut bytes)?;
///
/// assert_eq!(variable_length_enum, VariableLengthEnum::from(0b1_0000000_0000000));
/// assert_eq!(bytes.next().unwrap()?, 0x12);
/// assert!(bytes.next().is_none());
/// # Ok(())
/// # }
/// ```
///
/// Reading a large variable length enum with leading `0x80` bytes:
/// ```rust
/// # use fef::raw::VariableLengthEnum;
/// # use std::io::Read;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let file: Vec<u8> = vec![0x80, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00, 0x12];
/// let mut bytes1 = file.bytes();
/// let mut bytes2 = file.bytes();
///
/// bytes1.next().ok_or("First exists")?; // Skip the first leading `0x80` byte. It has no effect
///
/// let variable_length_enum = VariableLengthEnum::try_from(&mut bytes1)?;
/// let variable_length_enum2 = VariableLengthEnum::try_from(&mut bytes2)?;
///
/// assert_eq!(variable_length_enum, variable_length_enum2);
///
/// assert_eq!(bytes1.next().unwrap()?, 0x12);
/// assert!(bytes1.next().is_none());
///
/// assert_eq!(bytes2.next().unwrap()?, 0x12);
/// assert!(bytes2.next().is_none());
/// # Ok(())
/// # }
/// ```
///
/// Reading from a passed `&mut Bytes<R>`:
/// ```rust
/// # use fef::raw::VariableLengthEnum;
/// # use std::io::Read;
/// # use std::io::Bytes;
/// fn read_two_variable_length_enums<R: std::io::Read>(bytes: &mut Bytes<R>) -> Result<(VariableLengthEnum, VariableLengthEnum), std::io::Error> {
///     let enum1 = VariableLengthEnum::try_from(&mut *bytes)?; // Notice the reborrowing here
///     let enum2 = VariableLengthEnum::try_from(&mut *bytes)?;
///
///     Ok((enum1, enum2))
/// }
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let file = vec![0x80, 0x80, 0x00, 0x82, 0x80, 0x04, 0x12];
/// let mut bytes = file.bytes();
///
/// let (enum1, enum2) = read_two_variable_length_enums(&mut bytes)?;
/// assert_eq!(enum1, VariableLengthEnum::from(0));
/// assert_eq!(enum2, VariableLengthEnum::from(0b10_0000000_0000100));
///
/// assert_eq!(bytes.next().unwrap()?, 0x12);
/// assert!(bytes.next().is_none());
/// # Ok(())
/// # }
impl<R> TryFrom<&mut Bytes<R>> for VariableLengthEnum
where
    R: std::io::Read,
{
    type Error = std::io::Error;

    fn try_from(bytes: &mut Bytes<R>) -> Result<Self, Self::Error> {
        let mut byte_vec = Vec::new();
        let mut accumulator: Option<u64> = Some(0);

        loop {
            // We read the next byte from the stream
            let byte = if let Some(byte) = bytes.next() {
                byte?
            } else {
                // If we reached the end of the stream, we return an error
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "Unexpected end of stream",
                ));
            };
            if byte == 0x80 && byte_vec.is_empty() {
                // Leading 0x80 is ignored
                // This is only padding as defined in the FEF specification, so we ignore it
                continue;
            }

            // We save the value to both the accumulator and the byte_vec, choosing later which one to use
            byte_vec.push(byte);

            accumulator = if let Some(inner) = accumulator {
                if inner.leading_zeros() < 7 {
                    // If the accumulator has less than 7 leading zeros, shifting it left by 7 bits would overflow
                    None
                } else {
                    // We have enough space to shift the accumulator left by 7 bits and add the new byte
                    Some(inner << 7 | (byte & 0x7F) as u64)
                }
            } else {
                None
            };

            if byte < 0x80 {
                // The leading bit is 0, so this is the last byte, we stop reading
                break;
            }
        }
        if let Some(accumulator) = accumulator {
            // If we have an accumulator, we use it as the value
            Ok(VariableLengthEnum {
                value: VariableLengthEnumStorage::U64(accumulator),
            })
        } else {
            // If we don't have an accumulator, we use the byte_vec as the value
            Ok(VariableLengthEnum {
                value: VariableLengthEnumStorage::Overflow(byte_vec),
            })
        }
    }
}

/// Converting a variable length enum to a usize for easier use.
///
/// This conversion is fallible, as the value may be too large to fit into a usize.
/// This is however very unlikely - the FEF specification uses variable length enums to store values that are not too large.
///
/// # Examples
/// ```rust
/// # use fef::raw::VariableLengthEnum;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let variable_length_enum = VariableLengthEnum::from(42);
///
/// let value: usize = variable_length_enum.try_into()?;
///
/// match value {
///     42 => assert!(true),
///      _ => assert!(false),
/// }
///
/// # Ok(())
/// # }
/// ```
///
/// Example of a value that is too large to fit into a usize:
/// ```rust
/// # use fef::raw::VariableLengthEnum;
/// # use std::io::Read;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut bytes = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00].bytes();
/// let variable_length_enum = VariableLengthEnum::try_from(&mut bytes)?;
/// let value: Result<usize, _> = variable_length_enum.try_into(); // This will error
///
/// assert!(value.is_err());
/// # Ok(())
/// # }
/// ```
impl TryInto<usize> for VariableLengthEnum {
    type Error = &'static str; // This is a placeholder, we can change it to a more specific error type later

    fn try_into(self) -> Result<usize, Self::Error> {
        match self.value {
            VariableLengthEnumStorage::U64(u64_value) => u64_value
                .try_into()
                .map_err(|_| "Value is too large to fit into usize"),
            VariableLengthEnumStorage::Overflow(_) => Err("Value is too large to fit into usize"),
        }
    }
}
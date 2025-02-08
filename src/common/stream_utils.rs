use std::io::Read;

pub(crate) fn skip_bytes<R: ?Sized + Read>(
    reader: &mut R,
    count: usize,
) -> Result<(), std::io::Error> {
    let mut buffer = [0; 1];
    for _ in 0..count {
        reader.read_exact(&mut buffer)?;
    }
    Ok(())
}

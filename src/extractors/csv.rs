use crate::{error::Error, utils::decompress};
use colored::Colorize;
use std::{fs, path::Path};

/// Processes encoded, raw `.csv` file data.
///
/// The data passed here must be **compressed/raw**. Passing uncompressed or decoded
/// csv file data will result in [`Error::DecompressionError`].
///
/// If decompression is unsuccessful, [`Error::DecompressionError`] is returned.
///
/// ## Arguments
///
/// * `data`: Raw `.csv` file data.
/// * `path`: Path to the `.csv` file. It is used to get file name.
/// * `out_dir`: Directory to store extracted csv files.
///
/// [`Error::DecompressionError`]: ./error/enum.Error.html#variant.DecompressionError
pub fn process_csv(data: &[u8], file_name: &str, out_dir: &Path) -> Result<(), Error> {
    let decompressed = match decompress(data) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    println!("\nExtracting {} file...", file_name.green().bold());

    fs::write(out_dir.join(file_name), decompressed.get_ref())?;

    Ok(())
}

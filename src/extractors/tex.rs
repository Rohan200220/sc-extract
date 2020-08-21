use crate::{
    error::Error,
    utils::{decompress, Reader},
};
use colored::Colorize;
use image::{Rgba, RgbaImage};
use std::path::Path;

/// Reads some data from the stream and returns appropriate pixel data.
///
/// The bitwise transformations depend on the type of the pixel. One of the following
/// types is valid: `0, 1, 2, 3, 4, 6, 10`.
///
/// If `pixel_type` is not one of the above, `UnknownPixel` is raised. Otherwise, an array
/// of four `u8`s is returned, wrapped around by `Ok`.
///
/// ## Arguments
///
/// * `reader`: `Reader` representing the data stream.
/// * `pixel_type`: The type of pixel. For `_tex.sc` data, it is the image sub-type.
fn convert_pixel(reader: &mut Reader, pixel_type: u8) -> Result<[u8; 4], Error> {
    match pixel_type {
        // RGB8888
        0 | 1 => {
            let pixel = reader.read(4);
            Ok([pixel[0], pixel[1], pixel[2], pixel[3]])
        }
        // RGB4444
        2 => {
            let pixel = reader.read_uint16();
            Ok([
                (((pixel >> 12) & 0xF) << 4) as u8,
                (((pixel >> 8) & 0xF) << 4) as u8,
                (((pixel >> 4) & 0xF) << 4) as u8,
                ((pixel & 0xF) << 4) as u8,
            ])
        }
        // RGBA5551
        3 => {
            let pixel = reader.read_uint16();
            Ok([
                (((pixel >> 11) & 0x1F) << 3) as u8,
                (((pixel >> 6) & 0x1F) << 3) as u8,
                (((pixel >> 1) & 0x1F) << 3) as u8,
                ((pixel & 0xFF) << 7) as u8,
            ])
        }
        // RGB565
        4 => {
            let pixel = reader.read_uint16();
            Ok([
                (((pixel >> 11) & 0x1F) << 3) as u8,
                (((pixel >> 5) & 0x3F) << 2) as u8,
                ((pixel & 0x1F) << 3) as u8,
                // Alpha channel must always be 255 for type 4.
                255,
            ])
        }
        // LA88
        6 => {
            let pixel = reader.read_uint16();
            Ok([
                (pixel >> 8) as u8,
                (pixel >> 8) as u8,
                (pixel >> 8) as u8,
                (pixel & 0xFF) as u8,
            ])
        }
        10 => {
            let pixel = reader.read_byte();
            Ok([pixel; 4])
        }
        _ => Err(Error::UnknownPixel(format!(
            "Unknown pixel type ({}).",
            pixel_type
        ))),
    }
}

/// Adjusts some pixels.
fn adjust_pixels(img: &mut RgbaImage, pixels: Vec<[u8; 4]>, height: u32, width: u32) {
    let mut i = 0;
    let block_size = 32;
    let h_limit = (height as f64 / block_size as f64).ceil() as u32;
    let w_limit = (width as f64 / block_size as f64).ceil() as u32;

    for _h in 0..h_limit {
        for _w in 0..w_limit {
            let mut h = _h * block_size;
            while h != (_h + 1) * block_size && h < height as u32 {
                let mut w = _w * block_size;
                while w != (_w + 1) * block_size && w < width as u32 {
                    img.put_pixel(
                        w,
                        h,
                        Rgba([pixels[i][0], pixels[i][1], pixels[i][2], pixels[i][3]]),
                    );
                    i += 1;
                    w += 1;
                }
                h += 1;
            }
        }
    }
}

/// Processes compressed, raw `_tex.sc` file data.
///
/// If decompressing and pixel conversion is successful, the resultant png
/// image is saved in the output directory (`out_dir`).
///
/// If decompression is unsuccessful, [`Error::DecompressionError`] is returned.
/// Pixel conversion errors are handled in the function itself.
///
/// A single `_tex.sc` file can contain data for multiple sprites. All of the
/// sprites are extracted and saved by this process. `_`s are appended to the
/// file name in cases of multiple sprites.
///
/// ## Arguments
///
/// * `data`: Raw `_tex.sc` file data.
/// * `path`: Path to the `_tex.sc` file. It is used to get file name.
/// * `out_dir`: Directory to store extracted images.
/// * `parallelize`: Whether files are processed in parallel or not.
///
/// [`Error::DecompressionError`]: ./error/enum.Error.html#variant.DecompressionError
pub fn process_tex(
    data: &[u8],
    file_name: &str,
    out_dir: &Path,
    parallelize: bool,
) -> Result<(), Error> {
    if data.len() < 35 {
        return Err(Error::DecompressionError(
            "Size of file is too small".to_string(),
        ));
    }

    let decompressed = decompress(&data[26..])?;

    let mut reader = Reader::new(decompressed);

    let mut pic_count = 0;
    let possible_types = [1, 24, 27, 28];

    if !parallelize {
        println!("\nExtracting {} image(s)...", file_name);
    }

    'main: while reader.len() > 0 {
        let file_type = reader.read_byte();
        let file_size = reader.read_uint32();

        if !possible_types.contains(&file_type) {
            reader.read(file_size as usize);
            continue;
        }

        let sub_type = reader.read_byte();
        let width = reader.read_uint16() as u32;
        let height = reader.read_uint16() as u32;

        println!(
            "file_type: {}, file_size: {}, sub_type: {}, width: {}, height: {}",
            file_type.to_string().cyan().bold(),
            file_size.to_string().cyan().bold(),
            sub_type.to_string().cyan().bold(),
            width.to_string().cyan().bold(),
            height.to_string().cyan().bold()
        );

        let mut pixels = Vec::new();
        let mut img = RgbaImage::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let pixel_data = match convert_pixel(&mut reader, sub_type) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("Error: {}", e.inner().red());
                        continue 'main;
                    }
                };
                pixels.push(pixel_data);

                img.put_pixel(x, y, Rgba(pixel_data));
            }
        }

        if file_type == 27 || file_type == 28 {
            adjust_pixels(&mut img, pixels, height, width);
        }

        let initial_path = out_dir.join(file_name.replace(".sc", ""));
        let path = format!("{}{}.png", initial_path.display(), "_".repeat(pic_count));
        if let Err(_) = img.save(path) {
            return Err(Error::IoError("Failed to save image!".red().to_string()));
        }

        pic_count += 1;
    }

    Ok(())
}

use crate::wrapper::create_file_name;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use pixels_graphics_lib::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn convert_format(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    let output_file_name = create_file_name(&input, output, "ici")?;
    let input_file = process_input(input)?;
    let ici = convert(input_file)?;
    let bytes = ici.to_file_contents(&FilePalette::Colors)?;
    let mut file = File::create(output_file_name)?;
    file.write_all(&bytes)?;

    Ok(())
}

fn process_input(file_path: PathBuf) -> Result<Image> {
    let image = open_image(file_path)?;
    Ok(image)
}

fn convert(image: Image) -> Result<IndexedImage> {
    let width = image.width();
    let height = image.height();
    let colors: HashSet<Color> = HashSet::from_iter(image.pixels().iter().copied());

    if width >= 256 || height >= 256 {
        return Err(eyre!(
            "Image is too big, max width and height are 255 (was {width} x {height})"
        ));
    }

    if colors.len() >= 256 {
        return Err(eyre!(
            "Image has too many colours, max is 255 (was {})",
            colors.len()
        ));
    }

    let pixels = image
        .pixels()
        .iter()
        .map(|p| colors.iter().position(|c| c == p).unwrap() as u8)
        .collect();

    Ok(IndexedImage::new(
        width as u8,
        height as u8,
        colors.into_iter().collect(),
        pixels,
    )?)
}

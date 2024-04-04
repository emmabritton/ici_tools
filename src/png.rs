use color_eyre::Result;
use image::{save_buffer_with_format, ExtendedColorType, ImageFormat};
use pixels_graphics_lib::prelude::{
    Color, FilePalette, Graphics, Image, IndexedImage, JascPalette,
};
use std::fs;
use std::path::PathBuf;

use crate::wrapper::create_file_name;

pub fn to_png(input: PathBuf, palette: Option<PathBuf>, output: Option<PathBuf>) -> Result<()> {
    let output_file_name = create_file_name(&input, output, "png")?;
    let palette = process_palette(palette);
    let input = process_input(input, palette)?;
    let image = convert(input);
    save(output_file_name, image)?;

    Ok(())
}

fn process_palette(file_path: Option<PathBuf>) -> Option<Vec<Color>> {
    if let Some(path) = file_path {
        let colors = JascPalette::from_file_contents(&fs::read_to_string(path).unwrap())
            .unwrap()
            .colors;
        Some(colors)
    } else {
        None
    }
}

fn process_input(file_path: PathBuf, palette: Option<Vec<Color>>) -> Result<IndexedImage> {
    let bytes = fs::read(file_path)?;
    let (mut image, fp) = IndexedImage::from_file_contents(&bytes)?;
    if matches!(
        fp,
        FilePalette::ID(_) | FilePalette::Name(_) | FilePalette::NoData
    ) && palette.is_none()
    {
        panic!("Image has no palette data and no palette was provided");
    }
    if let Some(colors) = palette {
        if image.get_palette().len() != colors.len() {
            panic!(
                "Image and provided palette have different lengths ({} and {})",
                image.get_palette().len(),
                colors.len()
            );
        }
        image.set_palette(&colors)?;
    }
    Ok(image)
}

fn convert(indexed: IndexedImage) -> Image {
    let mut buffer = Graphics::create_buffer(indexed.width() as usize, indexed.height() as usize);
    let mut graphics = Graphics::new(
        &mut buffer,
        indexed.width() as usize,
        indexed.height() as usize,
    )
    .unwrap();
    graphics.draw_indexed_image((0, 0), &indexed);
    graphics.copy_to_image()
}

fn save(file: PathBuf, image: Image) -> Result<()> {
    let values: Vec<u8> = image.pixels().iter().flat_map(|c| c.as_array()).collect();
    save_buffer_with_format(
        file,
        &values,
        image.width() as u32,
        image.height() as u32,
        ExtendedColorType::Rgba8,
        ImageFormat::Png,
    )?;
    Ok(())
}

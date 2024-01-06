use std::fs;
use std::path::PathBuf;
use color_eyre::Result;
use crate::wrapper::{create_file_name, load, open_ici_file};
use pixels_graphics_lib::prelude::*;

pub fn palette_extract(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    let (bytes, _) = open_ici_file(input.clone())?;

    let image = load(bytes, true)?;

    let palette = image.get_palette();
    let output_file_name = create_file_name(&input, output, "pal")?;

    let file_contents = JascPalette::from(palette).to_file_contents();

    fs::write(output_file_name, file_contents)?;

    Ok(())
}
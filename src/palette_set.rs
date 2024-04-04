use crate::wrapper::{load, open_ici_file};
use color_eyre::eyre::eyre;
use color_eyre::Result;
use pixels_graphics_lib::prelude::*;
use std::fs;
use std::path::PathBuf;

pub fn palette_set(input: PathBuf, others: Vec<PathBuf>) -> Result<()> {
    let palette = read_palette(input)?;

    for file in others {
        if let Err(e) = assign_palette(&file, &palette) {
            eprintln!("Error for {}: {e:?}", file.to_string_lossy());
        }
    }

    Ok(())
}

fn read_palette(file: PathBuf) -> Result<Vec<Color>> {
    if let Some(ext) = file.extension().and_then(|s| s.to_str()) {
        return match ext {
            "ici" | "ica" => {
                let (bytes, _) = open_ici_file(file)?;
                let image = load(bytes, true)?;

                Ok(image.get_palette().to_vec())
            }
            "pal" | "jasc" => {
                let str = fs::read_to_string(file)?;
                let palette = JascPalette::from_file_contents(&str)?;
                Ok(palette.colors)
            }
            _ => Err(eyre!(
                "Unsupported file type {}, only ici, ica and pal files are supported",
                file.to_string_lossy()
            )),
        };
    }
    Err(eyre!("Unable to read file {}", file.to_string_lossy()))
}

fn assign_palette(file: &PathBuf, palette: &[Color]) -> Result<()> {
    let (bytes, _) = open_ici_file(file.clone())?;

    let mut image = load(bytes, false)?;
    if image.get_palette().len() != palette.len() {
        return Err(eyre!(
            "Palette wrong size, was {} much be {}",
            image.get_palette().len(),
            palette.len()
        ));
    }

    image.set_palette(palette)?;

    let bytes = match image {
        IndexedWrapper::Static(img) => img.to_file_contents(&FilePalette::Colors)?,
        IndexedWrapper::Animated(img) => img.to_file_contents(&FilePalette::Colors)?,
    };

    fs::write(file, bytes)?;

    Ok(())
}

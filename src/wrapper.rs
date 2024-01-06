use std::fs;
use std::path::{Path, PathBuf};
use color_eyre::eyre::eyre;
use pixels_graphics_lib::prelude::{AnimatedIndexedImage, FilePalette, IndexedImage};
use color_eyre::Result;
use pixels_graphics_lib::prelude::prelude::IndexedWrapper;

pub fn open_ici_file(input: PathBuf) -> Result<(Vec<u8>, Option<String>)> {
    let filename = input
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string());

    let bytes = fs::read(input)?;
    Ok((bytes, filename))
}

pub fn check_palette(pal: FilePalette) -> color_eyre::Result<()> {
    if pal != FilePalette::Colors {
        return Err(eyre!("Palette type {pal:?} is unsupported"));
    }
    Ok(())
}

pub fn load(bytes: Vec<u8>, validate_palette: bool) -> Result<IndexedWrapper> {
    let image;
    let image_pal;

    match IndexedImage::from_file_contents(&bytes) {
        Ok((img, palette)) => {
            image = IndexedWrapper::Static(img);
            image_pal = palette;
        }
        Err(static_err) => match AnimatedIndexedImage::from_file_contents(&bytes) {
            Ok((img, palette)) => {
                image = IndexedWrapper::Animated(img);
                image_pal = palette;
            }
            Err(anim_err) => {
                return if static_err.to_string() == anim_err.to_string() {
                    Err(eyre!(static_err))
                } else {
                    Err(eyre!(anim_err))
                };
            }
        }
    }

    if validate_palette {
        check_palette(image_pal)?;
    }

    Ok(image)
}

pub fn create_file_name(input: &Path, output_file: Option<PathBuf>, ext: &str) -> Result<PathBuf> {
    if let Some(path) = output_file {
        Ok(path)
    } else {
        let input_file_name = input.file_stem();
        let dir = input.parent();
        if let (Some(file_name), Some(dir)) = (input_file_name.and_then(|s| s.to_str()), dir) {
            let mut path = PathBuf::from(dir);
            path.push(format!("{file_name}.{ext}"));
            Ok(path)
        } else {
            Err(eyre!(
                "Couldn't create file name as input file is incomplete/invalid"
            ))
        }
    }
}
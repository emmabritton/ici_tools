use clap::Parser;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use pixels_graphics_lib::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about="Converts image files to ICIs", long_about = None)]
struct Args {
    #[arg(value_name = "FILE", help = "Image file to convert")]
    input: PathBuf,

    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let output_file_name = create_file_name(&args.input, args.output)?;
    let input_file = process_input(args.input)?;
    let ici = convert(input_file)?;
    let bytes = ici.to_file_contents(&FilePalette::Colors)?;
    let mut file = File::create(output_file_name)?;
    file.write_all(&bytes)?;

    Ok(())
}

fn process_input(file_path: PathBuf) -> Result<Image> {
    let file = File::open(file_path.clone())?;
    let mut bytes = vec![];
    BufReader::new(file).read_to_end(&mut bytes)?;
    let bytes = Cursor::new(bytes);
    match file_path.extension().and_then(|ext| ext.to_str()) {
        None => Err(eyre!("Unknown image file type, can't find file extension")),
        Some("png") => Ok(load_image(bytes, ImageFormat::Png)?),
        Some("bmp") => Ok(load_image(bytes, ImageFormat::Bmp)?),
        Some("tiff") => Ok(load_image(bytes, ImageFormat::Tiff)?),
        Some("tga") => Ok(load_image(bytes, ImageFormat::Tga)?),
        Some("jpeg") | Some("jpg") => Ok(load_image(bytes, ImageFormat::Jpeg)?),
        Some("webp") => Ok(load_image(bytes, ImageFormat::WebP)?),
        Some(_) => Err(eyre!("Unsupported image file type")),
    }
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

    let palette: Vec<IciColor> = colors.iter().map(|c| c.to_ici()).collect();
    let pixels = image
        .pixels()
        .iter()
        .map(|p| palette.iter().position(|c| c.to_color() == *p).unwrap() as u8)
        .collect();

    Ok(IndexedImage::new(
        width as u8,
        height as u8,
        palette,
        pixels,
    )?)
}

fn create_file_name(input: &Path, output_file: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(path) = output_file {
        Ok(path)
    } else {
        let input_file_name = input.file_stem();
        let dir = input.parent();
        if let (Some(file_name), Some(dir)) = (input_file_name.and_then(|s| s.to_str()), dir) {
            let mut path = PathBuf::from(dir);
            path.push(format!("{file_name}.ici"));
            Ok(path)
        } else {
            Err(eyre!(
                "Couldn't create file name as input file is incomplete/invalid"
            ))
        }
    }
}
